pub type clock_t = ::c_ulong;
pub type c_char = u8;
pub type wchar_t = u16;

pub type c_long = i64;
pub type c_ulong = u64;

pub type time_t = ::c_long;
pub type suseconds_t = ::c_long;
pub type rlim_t = c_ulong;
pub type mode_t = u32;
pub type socklen_t = ::c_int;
pub type pthread_t = ::c_char;  // __dummy
pub type off_t = ::c_long;
pub type off64_t = off_t;
pub type fsblkcnt_t = ::c_ulong;
pub type fsfilcnt_t = ::c_ulong;
pub type dev_t = u32;
pub type ino_t = u64;
pub type nlink_t = ::c_ushort;
pub type blksize_t = i32;
pub type blkcnt_t = i64;

s! {
    pub struct msghdr {
        pub msg_name: *mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: *mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: *mut ::c_void,
        pub msg_controllen: ::socklen_t,
        pub msg_flags: ::c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: ::size_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct sem_t {
        __dummy: ::c_char,
    }

    pub struct pthread_attr_t {
        __dummy: ::c_char,
    }

    pub struct statfs {
        pub f_type: ::c_long,
        pub f_bsize: ::c_long,
        pub f_blocks: ::c_long,
        pub f_bfree: ::c_long,
        pub f_bavail: ::c_long,
        pub f_files: ::c_long,
        pub f_ffree: ::c_long,

        pub f_fsid: ::c_long,
        pub f_namelen: ::c_long,
        pub f_spare: [::c_long; 6],
    }

    pub struct statvfs {
        pub f_bsize: ::c_ulong,
        pub f_frsize: ::c_ulong,
        pub f_blocks: fsblkcnt_t,
        pub f_bfree: fsblkcnt_t,
        pub f_bavail: fsblkcnt_t,
        pub f_files: fsfilcnt_t,
        pub f_ffree: fsfilcnt_t,
        pub f_favail: fsfilcnt_t,
        pub f_fsid: ::c_ulong,
        pub f_flag: ::c_ulong,
        pub f_namemax: ::c_ulong,
    }

    pub struct stat {
        pub st_dev: ::dev_t,
        pub st_ino: ::ino_t,
        pub st_mode: ::mode_t,
        pub st_nlink: ::nlink_t,
        pub st_uid: ::uid_t,
        pub st_gid: ::gid_t,
        pub st_rdev: ::dev_t,
        pub st_size: ::off_t,
        pub st_atime: ::time_t,
        pub st_atime_nsec: ::c_long,
        pub st_mtime: ::time_t,
        pub st_mtime_nsec: ::c_long,
        pub st_ctime: ::time_t,
        pub st_ctime_nsec: ::c_long,
        pub st_blksize: ::blksize_t,
        pub st_blocks: ::blkcnt_t,
        pub st_birthtim: ::time_t,
        pub st_birthtim_nsec: ::c_long,
    }

    pub struct dirent {
        pub __d_version: u32,
        pub d_ino: ::ino_t,
        pub d_type: ::c_uchar,
        pub __d_unused1:[::c_uchar; 3],
        pub __d_internal1: u32,
        pub d_name: [::c_char; 256],
    }

    pub struct pthread_condattr_t {
        __dummy: ::c_char,
    }

    pub struct pthread_mutexattr_t {
        __dummy: ::c_char,
    }

    pub struct pthread_rwlockattr_t {
        __dummy: ::c_char,
    }
}

pub type statfs64 = statfs;
pub type statvfs64 = statvfs;
pub type stat64 = stat;
pub type dirent64 = dirent;

pub const O_CLOEXEC: ::c_int = 0o1000000;
pub const SO_TIMESTAMP: ::c_int = 0x300A;
