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
pub type tcflag_t = ::c_uint;
pub type speed_t = ::c_uint;
pub type sigset_t = ::c_ulong;
pub type nfds_t = ::c_uint;

pub const O_CLOEXEC: ::c_int = 0o1000000;
pub const SO_TIMESTAMP: ::c_int = 0x300A;
pub const NCCS: usize = 18;

/* netdb.h */

pub const AI_PASSIVE: ::c_int = 0x1;
pub const AI_CANONNAME: ::c_int = 0x2;
pub const AI_NUMERICHOST: ::c_int = 0x4;
pub const AI_NUMERICSERV: ::c_int = 0x8;
pub const AI_ALL: ::c_int = 0x100;
pub const AI_ADDRCONFIG: ::c_int = 0x400;
pub const AI_V4MAPPED: ::c_int = 0x800;
pub const AI_IDN: ::c_int = 0x4000;
pub const AI_CANONIDN: ::c_int = 0x8000;
pub const AI_IDN_ALLOW_UNASSIGNED: ::c_int = 0x10000;
pub const AI_IDN_USE_STD3_ASCII_RULES: ::c_int = 0x20000;

pub const NI_NOFQDN: ::c_int = 0x1;
pub const NI_NUMERICHOST: ::c_int = 0x2;
pub const NI_NAMEREQD: ::c_int = 0x4;
pub const NI_NUMERICSERV: ::c_int = 0x8;
pub const NI_DGRAM: ::c_int = 0x10;
pub const NI_IDN: ::c_int = 0x4000;
pub const NI_IDN_ALLOW_UNASSIGNED: ::c_int = 0x10000;
pub const NI_IDN_USE_STD3_ASCII_RULES: ::c_int = 0x20000;
pub const NI_MAXHOST: ::c_int = 1025;
pub const NI_MAXSERV: ::c_int = 32;

pub const EAI_ADDRFAMILY: ::c_int = 1;
pub const EAI_AGAIN: ::c_int = 2;
pub const EAI_BADFLAGS: ::c_int = 3;
pub const EAI_FAIL: ::c_int = 4;
pub const EAI_FAMILY: ::c_int = 5;
pub const EAI_MEMORY: ::c_int = 6;
pub const EAI_NODATA: ::c_int = 7;
pub const EAI_NONAME: ::c_int = 8;
pub const EAI_SERVICE: ::c_int = 9;
pub const EAI_SOCKTYPE: ::c_int = 10;
pub const EAI_SYSTEM: ::c_int = 11;
pub const EAI_BADHINTS: ::c_int = 12;
pub const EAI_PROTOCOL: ::c_int = 13;
pub const EAI_OVERFLOW: ::c_int = 14;
pub const EAI_IDN_ENCODE: ::c_int = 15;

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

    pub struct pthread_condattr_t {
        __dummy: ::c_char,
    }

    pub struct pthread_mutexattr_t {
        __dummy: ::c_char,
    }

    pub struct pthread_rwlockattr_t {
        __dummy: ::c_char,
    }

    pub struct pthread_key_t {
        __dummy: ::c_char,
    }

    pub struct pthread_mutex_t {
        __dummy: ::c_char,
    }

    pub struct pthread_cond_t {
        __dummy: ::c_char,
    }

    pub struct pthread_rwlock_t {
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

    pub struct termios {
        pub c_iflag: ::tcflag_t,
        pub c_oflag: ::tcflag_t,
        pub c_cflag: ::tcflag_t,
        pub c_lflag: ::tcflag_t,
        pub c_line: ::c_char,
        pub c_cc: [::cc_t; ::NCCS],
        pub c_ispeed: ::speed_t,
        pub c_ospeed: ::speed_t,
    }

    pub struct passwd {
        pub pw_name: *mut ::c_char,
        pub pw_passwd: *mut ::c_char,
        pub pw_uid: ::uid_t,
        pub pw_gid: ::gid_t,
        pub pw_comment: *mut ::c_char,
        pub pw_gecos: *mut ::c_char,
        pub pw_dir: *mut ::c_char,
        pub pw_shell: *mut ::c_char,
    }

    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        pub sa_flags: ::c_int,
    }
}

pub type statfs64 = statfs;
pub type statvfs64 = statvfs;
pub type stat64 = stat;
pub type dirent64 = dirent;
