//! Cygwin-specific extensions to primitives in the `std::fs` module.

#![stable(feature = "metadata_ext", since = "1.1.0")]

use crate::fs::Metadata;
use crate::sys_common::AsInner;

#[allow(deprecated)]
use crate::os::cygwin::raw;

/// OS-specific extensions to [`fs::Metadata`].
///
/// [`fs::Metadata`]: crate::fs::Metadata
#[stable(feature = "metadata_ext", since = "1.1.0")]
pub trait MetadataExt {
    /// Gain a reference to the underlying `stat` structure which contains
    /// the raw information returned by the OS.
    ///
    /// The contents of the returned [`stat`] are **not** consistent across
    /// Unix platforms. The `os::unix::fs::MetadataExt` trait contains the
    /// cross-Unix abstractions contained within the raw stat.
    ///
    /// [`stat`]: struct@crate::os::cygwin::raw::stat
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs;
    /// use std::io;
    /// use std::os::cygwin::fs::MetadataExt;
    ///
    /// fn main() -> io::Result<()> {
    ///     let meta = fs::metadata("some_file")?;
    ///     let stat = meta.as_raw_stat();
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "metadata_ext", since = "1.1.0")]
    #[rustc_deprecated(since = "1.8.0", reason = "other methods of this trait are now preferred")]
    #[allow(deprecated)]
    fn as_raw_stat(&self) -> &raw::stat;

    /// Returns the device ID on which this file resides.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs;
    /// use std::io;
    /// use std::os::cygwin::fs::MetadataExt;
    ///
    /// fn main() -> io::Result<()> {
    ///     let meta = fs::metadata("some_file")?;
    ///     println!("{}", meta.st_dev());
    ///     Ok(())
    /// }
    /// ```
    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_dev(&self) -> u32;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ino(&self) -> u64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mode(&self) -> u32;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_nlink(&self) -> u16;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_uid(&self) -> u32;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_gid(&self) -> u32;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_rdev(&self) -> u32;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_size(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_atime(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_atime_nsec(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mtime(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_mtime_nsec(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ctime(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_ctime_nsec(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_blksize(&self) -> u32;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_blocks(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_birthtim(&self) -> i64;

    #[stable(feature = "metadata_ext2", since = "1.8.0")]
    fn st_birthtim_nsec(&self) -> i64;
}

#[stable(feature = "metadata_ext", since = "1.1.0")]
impl MetadataExt for Metadata {
    #[allow(deprecated)]
    fn as_raw_stat(&self) -> &raw::stat {
        unsafe { &*(self.as_inner().as_inner() as *const raw::stat) }
    }
    fn st_dev(&self) -> u32 {
        self.as_inner().as_inner().st_dev as u32
    }
    fn st_ino(&self) -> u64 {
        self.as_inner().as_inner().st_ino as u64
    }
    fn st_mode(&self) -> u32 {
        self.as_inner().as_inner().st_mode as u32
    }
    fn st_nlink(&self) -> u16 {
        self.as_inner().as_inner().st_nlink as u16
    }
    fn st_uid(&self) -> u32 {
        self.as_inner().as_inner().st_uid as u32
    }
    fn st_gid(&self) -> u32 {
        self.as_inner().as_inner().st_gid as u32
    }
    fn st_rdev(&self) -> u32 {
        self.as_inner().as_inner().st_rdev as u32
    }
    fn st_size(&self) -> i64 {
        self.as_inner().as_inner().st_size as i64
    }
    fn st_atime(&self) -> i64 {
        self.as_inner().as_inner().st_atime as i64
    }
    fn st_atime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_atime_nsec as i64
    }
    fn st_mtime(&self) -> i64 {
        self.as_inner().as_inner().st_mtime as i64
    }
    fn st_mtime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_mtime_nsec as i64
    }
    fn st_ctime(&self) -> i64 {
        self.as_inner().as_inner().st_ctime as i64
    }
    fn st_ctime_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_ctime_nsec as i64
    }
    fn st_blksize(&self) -> u32 {
        self.as_inner().as_inner().st_blksize as u32
    }
    fn st_blocks(&self) -> i64 {
        self.as_inner().as_inner().st_blocks as i64
    }
    fn st_birthtim(&self) -> i64 {
        self.as_inner().as_inner().st_birthtim as i64
    }
    fn st_birthtim_nsec(&self) -> i64 {
        self.as_inner().as_inner().st_birthtim_nsec as i64
    }
}
