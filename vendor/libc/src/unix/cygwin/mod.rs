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
pub type sa_family_t = u16;

pub const O_CLOEXEC: ::c_int = 0o1000000;
pub const SO_TIMESTAMP: ::c_int = 0x300A;
pub const NCCS: usize = 18;
const ULONG_SIZE: usize = 64;
pub const FD_SETSIZE: usize = 64;

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

/* cygwin/socket.h */

pub const SCM_RIGHTS: ::c_int = 0x01;
pub const SCM_CREDENTIALS: ::c_int = 0x02;

pub const SOCK_STREAM: ::c_int = 1;
pub const SOCK_DGRAM: ::c_int = 2;
pub const SOCK_RAW: ::c_int = 3;
pub const SOCK_RDM: ::c_int = 4;
pub const SOCK_SEQPACKET: ::c_int = 5;

pub const AF_UNSPEC: ::c_int = 0;
pub const AF_UNIX: ::c_int = 1;
pub const AF_LOCAL: ::c_int = 1;
pub const AF_INET: ::c_int = 2;
pub const AF_IMPLINK: ::c_int = 3;
pub const AF_PUP: ::c_int = 4;
pub const AF_CHAOS: ::c_int = 5;
pub const AF_NS: ::c_int = 6;
pub const AF_ISO: ::c_int = 7;
pub const AF_OSI: ::c_int = AF_ISO;
pub const AF_ECMA: ::c_int = 8;
pub const AF_DATAKIT: ::c_int = 9;
pub const AF_CCITT: ::c_int = 10;
pub const AF_SNA: ::c_int = 11;
pub const AF_DECnet: ::c_int = 12;
pub const AF_DLI: ::c_int = 13;
pub const AF_LAT: ::c_int = 14;
pub const AF_HYLINK: ::c_int = 15;
pub const AF_APPLETALK: ::c_int = 16;
pub const AF_NETBIOS: ::c_int = 17;
pub const AF_INET6: ::c_int = 23;
pub const AF_MAX: ::c_int = 32;

pub const PF_UNSPEC: ::c_int = AF_UNSPEC;
pub const PF_UNIX: ::c_int = AF_UNIX;
pub const PF_LOCAL: ::c_int = AF_LOCAL;
pub const PF_INET: ::c_int = AF_INET;
pub const PF_IMPLINK: ::c_int = AF_IMPLINK;
pub const PF_PUP: ::c_int = AF_PUP;
pub const PF_CHAOS: ::c_int = AF_CHAOS;
pub const PF_NS: ::c_int = AF_NS;
pub const PF_ISO: ::c_int = AF_ISO;
pub const PF_OSI: ::c_int = AF_OSI;
pub const PF_ECMA: ::c_int = AF_ECMA;
pub const PF_DATAKIT: ::c_int = AF_DATAKIT;
pub const PF_CCITT: ::c_int = AF_CCITT;
pub const PF_SNA: ::c_int = AF_SNA;
pub const PF_DECnet: ::c_int = AF_DECnet;
pub const PF_DLI: ::c_int = AF_DLI;
pub const PF_LAT: ::c_int = AF_LAT;
pub const PF_HYLINK: ::c_int = AF_HYLINK;
pub const PF_APPLETALK: ::c_int = AF_APPLETALK;
pub const PF_NETBIOS: ::c_int = AF_NETBIOS;
pub const PF_INET6: ::c_int = AF_INET6;
pub const PF_MAX: ::c_int = AF_MAX;

pub const SOMAXCONN: ::c_int = 0x7fffffff;

pub const MSG_OOB: ::c_int = 0x1;
pub const MSG_PEEK: ::c_int = 0x2;
pub const MSG_DONTROUTE: ::c_int = 0x4;
pub const MSG_WAITALL: ::c_int = 0x8;
pub const MSG_DONTWAIT: ::c_int = 0x10;
pub const MSG_NOSIGNAL: ::c_int = 0x20;
pub const MSG_TRUNC: ::c_int = 0x0100;
pub const MSG_CTRUNC: ::c_int = 0x0200;
pub const MSG_BCAST: ::c_int = 0x0400;
pub const MSG_MCAST: ::c_int = 0x0800;
pub const MSG_CMSG_CLOEXEC: ::c_int = 0x1000;
pub const MSG_EOR: ::c_int = 0x8000;

pub const SOL_IP: ::c_int = 0;
pub const SOL_IPV6: ::c_int = 41;
pub const SOL_IPX: ::c_int = 256;
pub const SOL_AX25: ::c_int = 257;
pub const SOL_ATALK: ::c_int = 258;
pub const SOL_NETROM: ::c_int = 259;
pub const SOL_TCP: ::c_int = 6;
pub const SOL_UDP: ::c_int = 17;

pub const IPTOS_LOWDELAY: ::c_int = 0x10;
pub const IPTOS_THROUGHPUT: ::c_int = 0x08;
pub const IPTOS_RELIABILITY: ::c_int = 0x04;

pub const IP_DEFAULT_MULTICAST_TTL: ::c_int = 1;
pub const IP_DEFAULT_MULTICAST_LOOP: ::c_int = 1;
pub const IP_MAX_MEMBERSHIPS: ::c_int = 20;

pub const IP_OPTIONS: ::c_int = 1;
pub const IP_HDRINCL: ::c_int = 2;
pub const IP_TOS: ::c_int = 3;
pub const IP_TTL: ::c_int = 4;
pub const IP_MULTICAST_IF: ::c_int = 9;
pub const IP_MULTICAST_TTL: ::c_int = 10;
pub const IP_MULTICAST_LOOP: ::c_int = 11;
pub const IP_ADD_MEMBERSHIP: ::c_int = 12;
pub const IP_DROP_MEMBERSHIP: ::c_int = 13;
pub const IP_DONTFRAGMENT: ::c_int = 14;
pub const IP_ADD_SOURCE_MEMBERSHIP: ::c_int = 15;
pub const IP_DROP_SOURCE_MEMBERSHIP: ::c_int = 16;
pub const IP_BLOCK_SOURCE: ::c_int = 17;
pub const IP_UNBLOCK_SOURCE: ::c_int = 18;
pub const IP_PKTINFO: ::c_int = 19;
pub const IP_UNICAST_IF: ::c_int = 31;

pub const IPV6_HOPOPTS: ::c_int = 1;
pub const IPV6_UNICAST_HOPS: ::c_int = 4;
pub const IPV6_MULTICAST_IF: ::c_int = 9;
pub const IPV6_MULTICAST_HOPS: ::c_int = 10;
pub const IPV6_MULTICAST_LOOP: ::c_int = 11;
pub const IPV6_ADD_MEMBERSHIP: ::c_int = 12;
pub const IPV6_DROP_MEMBERSHIP: ::c_int = 13;
pub const IPV6_JOIN_GROUP: ::c_int = IPV6_ADD_MEMBERSHIP;
pub const IPV6_LEAVE_GROUP: ::c_int = IPV6_DROP_MEMBERSHIP;
pub const IPV6_DONTFRAG: ::c_int = 14;
pub const IPV6_PKTINFO: ::c_int = 19;
pub const IPV6_HOPLIMIT: ::c_int = 21;
pub const IPV6_CHECKSUM: ::c_int = 26;
pub const IPV6_V6ONLY: ::c_int = 27;
pub const IPV6_UNICAST_IF: ::c_int = 31;
pub const IPV6_RTHDR: ::c_int = 32;
pub const IPV6_RECVRTHDR: ::c_int = 38;
pub const IPV6_TCLASS: ::c_int = 39;
pub const IPV6_RECVTCLASS: ::c_int = 40;

pub const MCAST_JOIN_GROUP: ::c_int = 41;
pub const MCAST_LEAVE_GROUP: ::c_int = 42;
pub const MCAST_BLOCK_SOURCE: ::c_int = 43;
pub const MCAST_UNBLOCK_SOURCE: ::c_int = 44;
pub const MCAST_JOIN_SOURCE_GROUP: ::c_int = 45;
pub const MCAST_LEAVE_SOURCE_GROUP: ::c_int = 46;
pub const MCAST_INCLUDE: ::c_int = 0;
pub const MCAST_EXCLUDE: ::c_int = 1;

pub const SHUT_RD: ::c_int = 0;
pub const SHUT_WR: ::c_int = 1;
pub const SHUT_RDWR: ::c_int = 2;

pub const SOPRI_INTERACTIVE: ::c_int = 0;
pub const SOPRI_NORMAL: ::c_int = 1;
pub const SOPRI_BACKGROUND: ::c_int = 2;

/* cygwin/_socketflags.h */

pub const SOCK_NONBLOCK: ::c_int = 0x01000000;
pub const SOCK_CLOEXEC: ::c_int = 0x02000000;

/* stdlib.h */
pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;

// The order of fields in these structs are crucial
// for converting between the Rust and C types.
s! {
    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct in_addr {
        pub s_addr: ::in_addr_t,
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [u8; 8],
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    pub struct addrinfo {
        pub ai_flags: ::c_int,
        pub ai_family: ::c_int,
        pub ai_socktype: ::c_int,
        pub ai_protocol: ::c_int,
        pub ai_addrlen: socklen_t,
        pub ai_canonname: *mut c_char,
        pub ai_addr: *mut ::sockaddr,
        pub ai_next: *mut addrinfo,
    }

    pub struct tm {
        pub tm_sec: ::c_int,
        pub tm_min: ::c_int,
        pub tm_hour: ::c_int,
        pub tm_mday: ::c_int,
        pub tm_mon: ::c_int,
        pub tm_year: ::c_int,
        pub tm_wday: ::c_int,
        pub tm_yday: ::c_int,
        pub tm_isdst: ::c_int,
        pub tm_gmtoff: ::c_long,
        pub tm_zone: *const ::c_char,
    }

    pub struct fd_set {
        fds_bits: [::c_ulong; FD_SETSIZE / ULONG_SIZE],
    }

    pub struct lconv {
        pub decimal_point: *mut ::c_char,
        pub thousands_sep: *mut ::c_char,
        pub grouping: *mut ::c_char,
        pub int_curr_symbol: *mut ::c_char,
        pub currency_symbol: *mut ::c_char,
        pub mon_decimal_point: *mut ::c_char,
        pub mon_thousands_sep: *mut ::c_char,
        pub mon_grouping: *mut ::c_char,
        pub positive_sign: *mut ::c_char,
        pub negative_sign: *mut ::c_char,
        pub int_frac_digits: ::c_char,
        pub frac_digits: ::c_char,
        pub p_cs_precedes: ::c_char,
        pub p_sep_by_space: ::c_char,
        pub n_cs_precedes: ::c_char,
        pub n_sep_by_space: ::c_char,
        pub p_sign_posn: ::c_char,
        pub n_sign_posn: ::c_char,
        pub int_n_cs_precedes: ::c_char,
        pub int_n_sep_by_space: ::c_char,
        pub int_n_sign_posn: ::c_char,
        pub int_p_cs_precedes: ::c_char,
        pub int_p_sep_by_space: ::c_char,
        pub int_p_sign_posn: ::c_char,
    }

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
