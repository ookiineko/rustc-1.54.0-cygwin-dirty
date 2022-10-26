extern crate libc;

use core::ptr;
use Allocator;

/// System setting for Linux
pub struct System {
    _priv: (),
}

impl System {
    pub const fn new() -> System {
        System { _priv: () }
    }
}

#[cfg(feature = "global")]
static mut LOCK: libc::pthread_mutex_t = libc::PTHREAD_MUTEX_INITIALIZER;

unsafe impl Allocator for System {
    fn alloc(&self, size: usize) -> (*mut u8, usize, u32) {
        let addr = unsafe {
            libc::mmap(
                0 as *mut _,
                size,
                libc::PROT_WRITE | libc::PROT_READ,
                libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
                -1,
                0,
            )
        };
        if addr == libc::MAP_FAILED {
            (ptr::null_mut(), 0, 0)
        } else {
            (addr as *mut u8, size, 0)
        }
    }

    fn remap(&self, ptr: *mut u8, oldsize: usize, newsize: usize, can_move: bool) -> *mut u8 {
        let flags = if can_move { libc::MREMAP_MAYMOVE } else { 0 };
        let ptr = unsafe { libc::mremap(ptr as *mut _, oldsize, newsize, flags) };
        if ptr == libc::MAP_FAILED {
            ptr::null_mut()
        } else {
            ptr as *mut u8
        }
    }

    fn free_part(&self, ptr: *mut u8, oldsize: usize, newsize: usize) -> bool {
        unsafe {
            let rc = libc::mremap(ptr as *mut _, oldsize, newsize, 0);
            if rc != libc::MAP_FAILED {
                return true;
            }
            libc::munmap(ptr.offset(newsize as isize) as *mut _, oldsize - newsize) == 0
        }
    }

    fn free(&self, ptr: *mut u8, size: usize) -> bool {
        unsafe { libc::munmap(ptr as *mut _, size) == 0 }
    }

    fn can_release_part(&self, _flags: u32) -> bool {
        true
    }

    fn allocates_zeros(&self) -> bool {
        true
    }

    fn page_size(&self) -> usize {
        4096
    }
}

#[cfg(feature = "global")]
pub fn acquire_global_lock() {
    unsafe { assert_eq!(libc::pthread_mutex_lock(&mut LOCK), 0) }
}

#[cfg(feature = "global")]
pub fn release_global_lock() {
    unsafe { assert_eq!(libc::pthread_mutex_unlock(&mut LOCK), 0) }
}
