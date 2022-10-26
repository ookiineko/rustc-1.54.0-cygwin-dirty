//! Cygwin-specific raw type definitions.

#![stable(feature = "raw_ext", since = "1.1.0")]
#![rustc_deprecated(
    since = "1.8.0",
    reason = "these type aliases are no longer supported by \
              the standard library, the `libc` crate on \
              crates.io should be used instead for the correct \
              definitions"
)]
#![allow(deprecated)]

use crate::os::raw::c_ulong;

#[stable(feature = "raw_ext", since = "1.1.0")]
pub type dev_t = u32;
#[stable(feature = "raw_ext", since = "1.1.0")]
pub type mode_t = u32;

#[stable(feature = "pthread_t", since = "1.8.0")]
pub type pthread_t = c_ulong;

#[doc(inline)]
#[stable(feature = "raw_ext", since = "1.1.0")]
pub use self::arch::{blkcnt_t, blksize_t, ino_t, nlink_t, off_t, stat, time_t};

#[cfg(target_arch = "x86_64")]
mod arch {
    use crate::os::raw::{c_int, c_long};

    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub type blkcnt_t = i64;
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub type blksize_t = u32;
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub type ino_t = u64;
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub type nlink_t = u16;
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub type off_t = i64;
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub type time_t = i64;

    #[repr(C)]
    #[derive(Clone)]
    #[stable(feature = "raw_ext", since = "1.1.0")]
    pub struct stat {
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_dev: u32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ino: u64,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mode: u32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_nlink: u16,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_uid: u32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_gid: u32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_rdev: u32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_size: i64,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_atime: i64,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_atime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mtime: i64,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_mtime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ctime: i64,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_ctime_nsec: c_long,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_blksize: u32,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_blocks: i64,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_birthtim: i64,
        #[stable(feature = "raw_ext", since = "1.1.0")]
        pub st_birthtim_nsec: c_long,
    }
}
