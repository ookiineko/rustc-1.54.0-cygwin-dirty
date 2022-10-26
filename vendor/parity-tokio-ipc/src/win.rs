use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::accctrl::*;
use winapi::um::aclapi::*;
use winapi::um::minwinbase::{LPTR, PSECURITY_ATTRIBUTES, SECURITY_ATTRIBUTES};
use winapi::um::securitybaseapi::*;
use winapi::um::winbase::{LocalAlloc, LocalFree};
use winapi::um::winnt::*;

use std::io;
use std::marker;
use std::mem;
use std::ptr;
use futures::Stream;
use tokio::prelude::*;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::path::Path;
use std::mem::MaybeUninit;
use tokio::io::PollEvented;

type NamedPipe = PollEvented<mio_named_pipes::NamedPipe>;

const PIPE_AVAILABILITY_TIMEOUT: u64 = 5000;

/// Endpoint implementation for windows
pub struct Endpoint {
    path: String,
    security_attributes: SecurityAttributes,
}

impl Endpoint {
    /// Stream of incoming connections
    pub fn incoming(mut self) -> io::Result<impl Stream<Item = tokio::io::Result<impl AsyncRead + AsyncWrite>> + 'static> {
        let pipe = self.inner()?;
        Ok(Incoming {
            path: self.path.clone(),
            inner: NamedPipeSupport {
                path: self.path,
                pipe,
                security_attributes: self.security_attributes,
            },
        })
    }

    /// Inner platform-dependant state of the endpoint
    fn inner(&mut self) -> io::Result<NamedPipe> {
        use miow::pipe::NamedPipeBuilder;
        use std::os::windows::io::*;

        let raw_handle = unsafe {
            NamedPipeBuilder::new(&self.path)
                .first(true)
                .inbound(true)
                .accept_remote(false)
                .outbound(true)
                .out_buffer_size(65536)
                .in_buffer_size(65536)
                .with_security_attributes(self.security_attributes.as_ptr())?
                .into_raw_handle()
        };

        let mio_pipe = unsafe { mio_named_pipes::NamedPipe::from_raw_handle(raw_handle) };
        NamedPipe::new(mio_pipe)
    }

    /// Set security attributes for the connection
    pub fn set_security_attributes(&mut self, security_attributes: SecurityAttributes) {
        self.security_attributes = security_attributes;
    }

    /// Returns the path of the endpoint.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Make new connection using the provided path and running event pool.
    pub async fn connect<P: AsRef<Path>>(path: P) -> io::Result<Connection> {
        Ok(Connection::wrap(Self::connect_inner(path.as_ref())?))
    }

    fn connect_inner(path: &Path) -> io::Result<NamedPipe> {
        use std::fs::OpenOptions;
        use std::os::windows::fs::OpenOptionsExt;
        use std::os::windows::io::{FromRawHandle, IntoRawHandle};
        use winapi::um::winbase::FILE_FLAG_OVERLAPPED;

        // Wait for the pipe to become available or fail after 5 seconds.
        miow::pipe::NamedPipe::wait(
            path,
            Some(std::time::Duration::from_millis(PIPE_AVAILABILITY_TIMEOUT)),
        )?;
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(FILE_FLAG_OVERLAPPED)
            .open(path)?;
        let mio_pipe =
            unsafe { mio_named_pipes::NamedPipe::from_raw_handle(file.into_raw_handle()) };
        let pipe = NamedPipe::new(mio_pipe)?;
        Ok(pipe)
    }

    /// New IPC endpoint at the given path
    pub fn new(path: String) -> Self {
        Endpoint {
            path,
            security_attributes: SecurityAttributes::empty(),
        }
    }
}

struct NamedPipeSupport {
    path: String,
    pipe: NamedPipe,
    security_attributes: SecurityAttributes,
}

impl NamedPipeSupport {
    fn replacement_pipe(&mut self) -> io::Result<NamedPipe> {
        use miow::pipe::NamedPipeBuilder;
        use std::os::windows::io::*;

        let raw_handle = unsafe {
            NamedPipeBuilder::new(&self.path)
                .first(false)
                .inbound(true)
                .outbound(true)
                .out_buffer_size(65536)
                .in_buffer_size(65536)
                .with_security_attributes(self.security_attributes.as_ptr())?
                .into_raw_handle()
        };

        let mio_pipe = unsafe { mio_named_pipes::NamedPipe::from_raw_handle(raw_handle) };
        NamedPipe::new(mio_pipe)
    }
}

/// Stream of incoming connections
pub struct Incoming {
    #[allow(dead_code)]
    path: String,
    inner: NamedPipeSupport,
}

impl Stream for Incoming {
    type Item = tokio::io::Result<Connection>;

    fn poll_next(mut self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match self.inner.pipe.get_ref().connect() {
            Ok(()) => {
                log::trace!("Incoming connection polled successfully");
                let new_listener = self.inner.replacement_pipe()?;
                Poll::Ready(
                    Some(Ok(Connection::wrap(std::mem::replace(&mut self.inner.pipe, new_listener))))
                )
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock {
                    self.inner.pipe.clear_write_ready(ctx)?;
                    Poll::Pending
                } else {
                    Poll::Ready(Some(Err(e)))
                }
            }
        }
    }
}

/// IPC connection.
pub struct Connection {
    inner: NamedPipe,
}

impl Connection {
    /// Wraps an existing named pipe
    pub fn wrap(pipe: NamedPipe) -> Self {
        Self { inner: pipe }
    }
}

impl AsyncRead for Connection {
    unsafe fn prepare_uninitialized_buffer(&self, buf: &mut [MaybeUninit<u8>]) -> bool {
        self.inner.prepare_uninitialized_buffer(buf)
    }

    fn poll_read(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = Pin::into_inner(self);
        Pin::new(&mut this.inner).poll_read(ctx, buf)
    }
}

impl AsyncWrite for Connection {
    fn poll_write(
        self: Pin<&mut Self>,
        ctx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        let this = Pin::into_inner(self);
        Pin::new(&mut this.inner).poll_write(ctx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let this = Pin::into_inner(self);
        Pin::new(&mut this.inner).poll_flush(ctx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        let this = Pin::into_inner(self);
        Pin::new(&mut this.inner).poll_shutdown(ctx)
    }
}

/// Security attributes.
pub struct SecurityAttributes {
    attributes: Option<InnerAttributes>,
}

pub const DEFAULT_SECURITY_ATTRIBUTES: SecurityAttributes = SecurityAttributes {
    attributes: Some(InnerAttributes {
        descriptor: SecurityDescriptor {
            descriptor_ptr: ptr::null_mut(),
        },
        acl: Acl { acl_ptr: ptr::null_mut() },
        attrs: SECURITY_ATTRIBUTES {
            nLength: mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: ptr::null_mut(),
            bInheritHandle: 0,
        },
    })
};

impl SecurityAttributes {
    /// New default security attributes.
    pub fn empty() -> SecurityAttributes {
        DEFAULT_SECURITY_ATTRIBUTES
    }

    /// New default security attributes that allow everyone to connect.
    pub fn allow_everyone_connect(&self) -> io::Result<SecurityAttributes> {
        let attributes = Some(InnerAttributes::allow_everyone(
            GENERIC_READ | FILE_WRITE_DATA,
        )?);
        Ok(SecurityAttributes { attributes })
    }

    /// Set a custom permission on the socket
    pub fn set_mode(self, _mode: u32) -> io::Result<Self> {
        // for now, does nothing.
        Ok(self)
    }

    /// New default security attributes that allow everyone to create.
    pub fn allow_everyone_create() -> io::Result<SecurityAttributes> {
        let attributes = Some(InnerAttributes::allow_everyone(
            GENERIC_READ | GENERIC_WRITE,
        )?);
        Ok(SecurityAttributes { attributes })
    }

    /// Return raw handle of security attributes.
    pub(crate) unsafe fn as_ptr(&mut self) -> PSECURITY_ATTRIBUTES {
        match self.attributes.as_mut() {
            Some(attributes) => attributes.as_ptr(),
            None => ptr::null_mut(),
        }
    }
}

unsafe impl Send for SecurityAttributes {}

struct Sid {
    sid_ptr: PSID,
}

impl Sid {
    fn everyone_sid() -> io::Result<Sid> {
        let mut sid_ptr = ptr::null_mut();
        let result = unsafe {
            AllocateAndInitializeSid(
                SECURITY_WORLD_SID_AUTHORITY.as_mut_ptr() as *mut _,
                1,
                SECURITY_WORLD_RID,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                &mut sid_ptr,
            )
        };
        if result == 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(Sid { sid_ptr })
        }
    }

    // Unsafe - the returned pointer is only valid for the lifetime of self.
    unsafe fn as_ptr(&self) -> PSID {
        self.sid_ptr
    }
}

impl Drop for Sid {
    fn drop(&mut self) {
        if !self.sid_ptr.is_null() {
            unsafe {
                FreeSid(self.sid_ptr);
            }
        }
    }
}

struct AceWithSid<'a> {
    explicit_access: EXPLICIT_ACCESS_W,
    _marker: marker::PhantomData<&'a Sid>,
}

impl<'a> AceWithSid<'a> {
    fn new(sid: &'a Sid, trustee_type: u32) -> AceWithSid<'a> {
        let mut explicit_access = unsafe { mem::zeroed::<EXPLICIT_ACCESS_W>() };
        explicit_access.Trustee.TrusteeForm = TRUSTEE_IS_SID;
        explicit_access.Trustee.TrusteeType = trustee_type;
        explicit_access.Trustee.ptstrName = unsafe { sid.as_ptr() as *mut _ };

        AceWithSid {
            explicit_access,
            _marker: marker::PhantomData,
        }
    }

    fn set_access_mode(&mut self, access_mode: u32) -> &mut Self {
        self.explicit_access.grfAccessMode = access_mode;
        self
    }

    fn set_access_permissions(&mut self, access_permissions: u32) -> &mut Self {
        self.explicit_access.grfAccessPermissions = access_permissions;
        self
    }

    fn allow_inheritance(&mut self, inheritance_flags: u32) -> &mut Self {
        self.explicit_access.grfInheritance = inheritance_flags;
        self
    }
}

struct Acl {
    acl_ptr: PACL,
}

impl Acl {
    fn empty() -> io::Result<Acl> {
        Self::new(&mut [])
    }

    fn new(entries: &mut [AceWithSid<'_>]) -> io::Result<Acl> {
        let mut acl_ptr = ptr::null_mut();
        let result = unsafe {
            SetEntriesInAclW(
                entries.len() as u32,
                entries.as_mut_ptr() as *mut _,
                ptr::null_mut(),
                &mut acl_ptr,
            )
        };

        if result != ERROR_SUCCESS {
            return Err(io::Error::from_raw_os_error(result as i32));
        }

        Ok(Acl { acl_ptr })
    }

    unsafe fn as_ptr(&self) -> PACL {
        self.acl_ptr
    }
}

impl Drop for Acl {
    fn drop(&mut self) {
        if !self.acl_ptr.is_null() {
            unsafe { LocalFree(self.acl_ptr as *mut _) };
        }
    }
}

struct SecurityDescriptor {
    descriptor_ptr: PSECURITY_DESCRIPTOR,
}

impl SecurityDescriptor {
    fn new() -> io::Result<Self> {
        let descriptor_ptr = unsafe { LocalAlloc(LPTR, SECURITY_DESCRIPTOR_MIN_LENGTH) };
        if descriptor_ptr.is_null() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to allocate security descriptor",
            ));
        }

        if unsafe {
            InitializeSecurityDescriptor(descriptor_ptr, SECURITY_DESCRIPTOR_REVISION) == 0
        } {
            return Err(io::Error::last_os_error());
        };

        Ok(SecurityDescriptor { descriptor_ptr })
    }

    fn set_dacl(&mut self, acl: &Acl) -> io::Result<()> {
        if unsafe {
            SetSecurityDescriptorDacl(self.descriptor_ptr, true as i32, acl.as_ptr(), false as i32)
                == 0
        } {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    unsafe fn as_ptr(&self) -> PSECURITY_DESCRIPTOR {
        self.descriptor_ptr
    }
}

impl Drop for SecurityDescriptor {
    fn drop(&mut self) {
        if !self.descriptor_ptr.is_null() {
            unsafe { LocalFree(self.descriptor_ptr) };
            self.descriptor_ptr = ptr::null_mut();
        }
    }
}

struct InnerAttributes {
    descriptor: SecurityDescriptor,
    acl: Acl,
    attrs: SECURITY_ATTRIBUTES,
}

impl InnerAttributes {
    fn empty() -> io::Result<InnerAttributes> {
        let descriptor = SecurityDescriptor::new()?;
        let mut attrs = unsafe { mem::zeroed::<SECURITY_ATTRIBUTES>() };
        attrs.nLength = mem::size_of::<SECURITY_ATTRIBUTES>() as u32;
        attrs.lpSecurityDescriptor = unsafe { descriptor.as_ptr() };
        attrs.bInheritHandle = false as i32;

        let acl = Acl::empty().expect("this should never fail");

        Ok(InnerAttributes {
            acl,
            descriptor,
            attrs,
        })
    }

    fn allow_everyone(permissions: u32) -> io::Result<InnerAttributes> {
        let mut attributes = Self::empty()?;
        let sid = Sid::everyone_sid()?;

        let mut everyone_ace = AceWithSid::new(&sid, TRUSTEE_IS_WELL_KNOWN_GROUP);
        everyone_ace
            .set_access_mode(SET_ACCESS)
            .set_access_permissions(permissions)
            .allow_inheritance(false as u32);

        let mut entries = vec![everyone_ace];
        attributes.acl = Acl::new(&mut entries)?;
        attributes.descriptor.set_dacl(&attributes.acl)?;

        Ok(attributes)
    }

    unsafe fn as_ptr(&mut self) -> PSECURITY_ATTRIBUTES {
        &mut self.attrs as *mut _
    }
}

#[cfg(test)]
mod test {
    use super::SecurityAttributes;

    #[test]
    fn test_allow_everyone_everything() {
        SecurityAttributes::allow_everyone_create()
            .expect("failed to create security attributes that allow everyone to create a pipe");
    }

    #[test]
    fn test_allow_eveyone_read_write() {
        SecurityAttributes::empty()
            .allow_everyone_connect()
            .expect("failed to create security attributes that allow everyone to read and write to/from a pipe");
    }

}
