pub type clock_t = ::c_ulong;
pub type c_char = i8;
pub type wchar_t = u16;

pub type c_long = i64;
pub type c_ulong = u64;

pub type time_t = ::c_long;
pub type time64_t = time_t;
pub type suseconds_t = ::c_long;
pub type rlim_t = c_ulong;
pub type mode_t = u32;
pub type socklen_t = ::c_int;
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
pub type clockid_t = ::c_ulong;
pub type pthread_t = *mut ::c_void;
pub type pthread_mutex_t = *mut ::c_void;
// Must be usize due to libstd/sys_common/thread_local.rs,
// should technically be *mut ::c_void
pub type pthread_key_t = usize;
pub type pthread_attr_t = *mut ::c_void;
pub type pthread_mutexattr_t = *mut ::c_void;
pub type pthread_condattr_t = *mut ::c_void;
pub type pthread_cond_t = *mut ::c_void;
pub type pthread_rwlock_t = *mut ::c_void;
pub type pthread_rwlockattr_t = *mut ::c_void;

/* spawn.h */

pub type posix_spawnattr_t = *mut ::c_void;
pub type posix_spawn_file_actions_t = *mut ::c_void;

/* elf.h */

pub type Elf32_Half = u16;
pub type Elf64_Half = u16;
pub type Elf32_Word = u32;
pub type Elf32_Sword = i32;
pub type Elf64_Word = u32;
pub type Elf64_Sword = i32;
pub type Elf32_Xword = u64;
pub type Elf32_Sxword = i64;
pub type Elf64_Xword = u64;
pub type Elf64_Sxword = i64;
pub type Elf32_Addr = u32;
pub type Elf64_Addr = u64;
pub type Elf32_Off = u32;
pub type Elf64_Off = u64;
pub type Elf32_Section = u16;
pub type Elf64_Section = u16;
pub type Elf32_Versym = Elf32_Half;
pub type Elf64_Versym = Elf64_Half;

pub const SO_TIMESTAMP: ::c_int = 0x300A;
pub const NCCS: usize = 18;
const ULONG_SIZE: usize = 64;
pub const FD_SETSIZE: usize = 64;
pub const RAND_MAX: ::c_int = 0x7fffffff;
pub const FILENAME_MAX: ::c_uint = 4096;

#[cfg_attr(feature = "extra_traits", derive(Debug))]
pub enum timezone {}
impl ::Copy for timezone {}
impl ::Clone for timezone {
    fn clone(&self) -> timezone {
        *self
    }
}

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

/* sys/stat.h */

pub const S_BLKSIZE: ::mode_t = 1024;
pub const S_ISUID: ::mode_t = 0o004000;
pub const S_ISGID: ::mode_t = 0o002000;
pub const S_ISVTX: ::mode_t = 0o001000;
pub const S_IREAD: ::mode_t = 0o000400;
pub const S_IWRITE: ::mode_t = 0o000200;
pub const S_IEXEC: ::mode_t = 0o000100;
pub const S_ENFMT: ::mode_t = 0o002000;
pub const S_IFMT: ::mode_t = 0o170000;
pub const S_IFDIR: ::mode_t = 0o040000;
pub const S_IFCHR: ::mode_t = 0o020000;
pub const S_IFBLK: ::mode_t = 0o060000;
pub const S_IFREG: ::mode_t = 0o100000;
pub const S_IFLNK: ::mode_t = 0o120000;
pub const S_IFSOCK: ::mode_t = 0o140000;
pub const S_IFIFO: ::mode_t = 0o010000;
pub const S_IRWXU: ::mode_t = S_IRUSR | S_IWUSR | S_IXUSR;
pub const S_IRUSR: ::mode_t = 0o000400;
pub const S_IWUSR: ::mode_t = 0o000200;
pub const S_IXUSR: ::mode_t = 0o000100;
pub const S_IRWXG: ::mode_t = S_IRGRP | S_IWGRP | S_IXGRP;
pub const S_IRGRP: ::mode_t = 0o000040;
pub const S_IWGRP: ::mode_t = 0o000020;
pub const S_IXGRP: ::mode_t = 0o000010;
pub const S_IRWXO: ::mode_t = S_IROTH | S_IWOTH | S_IXOTH;
pub const S_IROTH: ::mode_t = 0o000004;
pub const S_IWOTH: ::mode_t = 0o000002;
pub const S_IXOTH: ::mode_t = 0o000001;

pub const UTIME_NOW: c_long = -2;
pub const UTIME_OMIT: c_long = -1;

/* stdlib.h */

pub const EXIT_FAILURE: ::c_int = 1;
pub const EXIT_SUCCESS: ::c_int = 0;

/* sys/unistd.h */

pub const F_OK: ::c_int = 0;
pub const R_OK: ::c_int = 4;
pub const W_OK: ::c_int = 2;
pub const X_OK: ::c_int = 1;
pub const SEEK_SET: ::c_int = 0;
pub const SEEK_CUR: ::c_int = 1;
pub const SEEK_END: ::c_int = 2;
pub const STDIN_FILENO: ::c_int = 0;
pub const STDOUT_FILENO: ::c_int = 1;
pub const STDERR_FILENO: ::c_int = 2;

pub const _SC_ARG_MAX: ::c_int = 0;
pub const _SC_CHILD_MAX: ::c_int = 1;
pub const _SC_CLK_TCK: ::c_int = 2;
pub const _SC_NGROUPS_MAX: ::c_int = 3;
pub const _SC_OPEN_MAX: ::c_int = 4;
pub const _SC_JOB_CONTROL: ::c_int = 5;
pub const _SC_SAVED_IDS: ::c_int = 6;
pub const _SC_VERSION: ::c_int = 7;
pub const _SC_PAGESIZE: ::c_int = 8;
pub const _SC_PAGE_SIZE: ::c_int = _SC_PAGESIZE;
pub const _SC_NPROCESSORS_CONF: ::c_int = 9;
pub const _SC_NPROCESSORS_ONLN: ::c_int = 10;
pub const _SC_PHYS_PAGES: ::c_int = 11;
pub const _SC_AVPHYS_PAGES: ::c_int = 12;
pub const _SC_MQ_OPEN_MAX: ::c_int = 13;
pub const _SC_MQ_PRIO_MAX: ::c_int = 14;
pub const _SC_RTSIG_MAX: ::c_int = 15;
pub const _SC_SEM_NSEMS_MAX: ::c_int = 16;
pub const _SC_SEM_VALUE_MAX: ::c_int = 17;
pub const _SC_SIGQUEUE_MAX: ::c_int = 18;
pub const _SC_TIMER_MAX: ::c_int = 19;
pub const _SC_TZNAME_MAX: ::c_int = 20;
pub const _SC_ASYNCHRONOUS_IO: ::c_int = 21;
pub const _SC_FSYNC: ::c_int = 22;
pub const _SC_MAPPED_FILES: ::c_int = 23;
pub const _SC_MEMLOCK: ::c_int = 24;
pub const _SC_MEMLOCK_RANGE: ::c_int = 25;
pub const _SC_MEMORY_PROTECTION: ::c_int = 26;
pub const _SC_MESSAGE_PASSING: ::c_int = 27;
pub const _SC_PRIORITIZED_IO: ::c_int = 28;
pub const _SC_REALTIME_SIGNALS: ::c_int = 29;
pub const _SC_SEMAPHORES: ::c_int = 30;
pub const _SC_SHARED_MEMORY_OBJECTS: ::c_int = 31;
pub const _SC_SYNCHRONIZED_IO: ::c_int = 32;
pub const _SC_TIMERS: ::c_int = 33;
pub const _SC_AIO_LISTIO_MAX: ::c_int = 34;
pub const _SC_AIO_MAX: ::c_int = 35;
pub const _SC_AIO_PRIO_DELTA_MAX: ::c_int = 36;
pub const _SC_DELAYTIMER_MAX: ::c_int = 37;
pub const _SC_THREAD_KEYS_MAX: ::c_int = 38;
pub const _SC_THREAD_STACK_MIN: ::c_int = 39;
pub const _SC_THREAD_THREADS_MAX: ::c_int = 40;
pub const _SC_TTY_NAME_MAX: ::c_int = 41;
pub const _SC_THREADS: ::c_int = 42;
pub const _SC_THREAD_ATTR_STACKADDR: ::c_int = 43;
pub const _SC_THREAD_ATTR_STACKSIZE: ::c_int = 44;
pub const _SC_THREAD_PRIORITY_SCHEDULING: ::c_int = 45;
pub const _SC_THREAD_PRIO_INHERIT: ::c_int = 46;
pub const _SC_THREAD_PRIO_PROTECT: ::c_int = 47;
pub const _SC_THREAD_PRIO_CEILING: ::c_int = _SC_THREAD_PRIO_PROTECT;
pub const _SC_THREAD_PROCESS_SHARED: ::c_int = 48;
pub const _SC_THREAD_SAFE_FUNCTIONS: ::c_int = 49;
pub const _SC_GETGR_R_SIZE_MAX: ::c_int = 50;
pub const _SC_GETPW_R_SIZE_MAX: ::c_int = 51;
pub const _SC_LOGIN_NAME_MAX: ::c_int = 52;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: ::c_int = 53;
pub const _SC_ADVISORY_INFO: ::c_int = 54;
pub const _SC_ATEXIT_MAX: ::c_int = 55;
pub const _SC_BARRIERS: ::c_int = 56;
pub const _SC_BC_BASE_MAX: ::c_int = 57;
pub const _SC_BC_DIM_MAX: ::c_int = 58;
pub const _SC_BC_SCALE_MAX: ::c_int = 59;
pub const _SC_BC_STRING_MAX: ::c_int = 60;
pub const _SC_CLOCK_SELECTION: ::c_int = 61;
pub const _SC_COLL_WEIGHTS_MAX: ::c_int = 62;
pub const _SC_CPUTIME: ::c_int = 63;
pub const _SC_EXPR_NEST_MAX: ::c_int = 64;
pub const _SC_HOST_NAME_MAX: ::c_int = 65;
pub const _SC_IOV_MAX: ::c_int = 66;
pub const _SC_IPV6: ::c_int = 67;
pub const _SC_LINE_MAX: ::c_int = 68;
pub const _SC_MONOTONIC_CLOCK: ::c_int = 69;
pub const _SC_RAW_SOCKETS: ::c_int = 70;
pub const _SC_READER_WRITER_LOCKS: ::c_int = 71;
pub const _SC_REGEXP: ::c_int = 72;
pub const _SC_RE_DUP_MAX: ::c_int = 73;
pub const _SC_SHELL: ::c_int = 74;
pub const _SC_SPAWN: ::c_int = 75;
pub const _SC_SPIN_LOCKS: ::c_int = 76;
pub const _SC_SPORADIC_SERVER: ::c_int = 77;
pub const _SC_SS_REPL_MAX: ::c_int = 78;
pub const _SC_SYMLOOP_MAX: ::c_int = 79;
pub const _SC_THREAD_CPUTIME: ::c_int = 80;
pub const _SC_THREAD_SPORADIC_SERVER: ::c_int = 81;
pub const _SC_TIMEOUTS: ::c_int = 82;
pub const _SC_TRACE: ::c_int = 83;
pub const _SC_TRACE_EVENT_FILTER: ::c_int = 84;
pub const _SC_TRACE_EVENT_NAME_MAX: ::c_int = 85;
pub const _SC_TRACE_INHERIT: ::c_int = 86;
pub const _SC_TRACE_LOG: ::c_int = 87;
pub const _SC_TRACE_NAME_MAX: ::c_int = 88;
pub const _SC_TRACE_SYS_MAX: ::c_int = 89;
pub const _SC_TRACE_USER_EVENT_MAX: ::c_int = 90;
pub const _SC_TYPED_MEMORY_OBJECTS: ::c_int = 91;
pub const _SC_V7_ILP32_OFF32: ::c_int = 92;
pub const _SC_V6_ILP32_OFF32: ::c_int = _SC_V7_ILP32_OFF32;
pub const _SC_XBS5_ILP32_OFF32: ::c_int = _SC_V7_ILP32_OFF32;
pub const _SC_V7_ILP32_OFFBIG: ::c_int = 93;
pub const _SC_V6_ILP32_OFFBIG: ::c_int = _SC_V7_ILP32_OFFBIG;
pub const _SC_XBS5_ILP32_OFFBIG: ::c_int = _SC_V7_ILP32_OFFBIG;
pub const _SC_V7_LP64_OFF64: ::c_int = 94;
pub const _SC_V6_LP64_OFF64: ::c_int = _SC_V7_LP64_OFF64;
pub const _SC_XBS5_LP64_OFF64: ::c_int = _SC_V7_LP64_OFF64;
pub const _SC_V7_LPBIG_OFFBIG: ::c_int = 95;
pub const _SC_V6_LPBIG_OFFBIG: ::c_int = _SC_V7_LPBIG_OFFBIG;
pub const _SC_XBS5_LPBIG_OFFBIG: ::c_int = _SC_V7_LPBIG_OFFBIG;
pub const _SC_XOPEN_CRYPT: ::c_int = 96;
pub const _SC_XOPEN_ENH_I18N: ::c_int = 97;
pub const _SC_XOPEN_LEGACY: ::c_int = 98;
pub const _SC_XOPEN_REALTIME: ::c_int = 99;
pub const _SC_STREAM_MAX: ::c_int = 100;
pub const _SC_PRIORITY_SCHEDULING: ::c_int = 101;
pub const _SC_XOPEN_REALTIME_THREADS: ::c_int = 102;
pub const _SC_XOPEN_SHM: ::c_int = 103;
pub const _SC_XOPEN_STREAMS: ::c_int = 104;
pub const _SC_XOPEN_UNIX: ::c_int = 105;
pub const _SC_XOPEN_VERSION: ::c_int = 106;
pub const _SC_2_CHAR_TERM: ::c_int = 107;
pub const _SC_2_C_BIND: ::c_int = 108;
pub const _SC_2_C_DEV: ::c_int = 109;
pub const _SC_2_FORT_DEV: ::c_int = 110;
pub const _SC_2_FORT_RUN: ::c_int = 111;
pub const _SC_2_LOCALEDEF: ::c_int = 112;
pub const _SC_2_PBS: ::c_int = 113;
pub const _SC_2_PBS_ACCOUNTING: ::c_int = 114;
pub const _SC_2_PBS_CHECKPOINT: ::c_int = 115;
pub const _SC_2_PBS_LOCATE: ::c_int = 116;
pub const _SC_2_PBS_MESSAGE: ::c_int = 117;
pub const _SC_2_PBS_TRACK: ::c_int = 118;
pub const _SC_2_SW_DEV: ::c_int = 119;
pub const _SC_2_UPE: ::c_int = 120;
pub const _SC_2_VERSION: ::c_int = 121;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: ::c_int = 122;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: ::c_int = 123;
pub const _SC_XOPEN_UUCP: ::c_int = 124;
pub const _SC_LEVEL1_ICACHE_SIZE: ::c_int = 125;
pub const _SC_LEVEL1_ICACHE_ASSOC: ::c_int = 126;
pub const _SC_LEVEL1_ICACHE_LINESIZE: ::c_int = 127;
pub const _SC_LEVEL1_DCACHE_SIZE: ::c_int = 128;
pub const _SC_LEVEL1_DCACHE_ASSOC: ::c_int = 129;
pub const _SC_LEVEL1_DCACHE_LINESIZE: ::c_int = 130;
pub const _SC_LEVEL2_CACHE_SIZE: ::c_int = 131;
pub const _SC_LEVEL2_CACHE_ASSOC: ::c_int = 132;
pub const _SC_LEVEL2_CACHE_LINESIZE: ::c_int = 133;
pub const _SC_LEVEL3_CACHE_SIZE: ::c_int = 134;
pub const _SC_LEVEL3_CACHE_ASSOC: ::c_int = 135;
pub const _SC_LEVEL3_CACHE_LINESIZE: ::c_int = 136;
pub const _SC_LEVEL4_CACHE_SIZE: ::c_int = 137;
pub const _SC_LEVEL4_CACHE_ASSOC: ::c_int = 138;
pub const _SC_LEVEL4_CACHE_LINESIZE: ::c_int = 139;
pub const _SC_POSIX_26_VERSION: ::c_int = 140;

pub const _PC_LINK_MAX: ::c_int = 0;
pub const _PC_MAX_CANON: ::c_int = 1;
pub const _PC_MAX_INPUT: ::c_int = 2;
pub const _PC_NAME_MAX: ::c_int = 3;
pub const _PC_PATH_MAX: ::c_int = 4;
pub const _PC_PIPE_BUF: ::c_int = 5;
pub const _PC_CHOWN_RESTRICTED: ::c_int = 6;
pub const _PC_NO_TRUNC: ::c_int = 7;
pub const _PC_VDISABLE: ::c_int = 8;
pub const _PC_ASYNC_IO: ::c_int = 9;
pub const _PC_PRIO_IO: ::c_int = 10;
pub const _PC_SYNC_IO: ::c_int = 11;
pub const _PC_FILESIZEBITS: ::c_int = 12;
pub const _PC_2_SYMLINKS: ::c_int = 13;
pub const _PC_SYMLINK_MAX: ::c_int = 14;
pub const _PC_ALLOC_SIZE_MIN: ::c_int = 15;
pub const _PC_REC_INCR_XFER_SIZE: ::c_int = 16;
pub const _PC_REC_MAX_XFER_SIZE: ::c_int = 17;
pub const _PC_REC_MIN_XFER_SIZE: ::c_int = 18;
pub const _PC_REC_XFER_ALIGN: ::c_int = 19;
pub const _PC_TIMESTAMP_RESOLUTION: ::c_int = 20;
pub const _PC_POSIX_PERMISSIONS: ::c_int = 90;
pub const _PC_POSIX_SECURITY: ::c_int = 91;
pub const _PC_CASE_INSENSITIVE: ::c_int = 92;

/* cygwin/if.h */

pub const IFF_UP: ::c_int = 0x1;
pub const IFF_BROADCAST: ::c_int = 0x2;
pub const IFF_LOOPBACK: ::c_int = 0x8;
pub const IFF_POINTOPOINT: ::c_int = 0x10;
pub const IFF_NOTRAILERS: ::c_int = 0x20;
pub const IFF_RUNNING: ::c_int = 0x40;
pub const IFF_NOARP: ::c_int = 0x80;
pub const IFF_PROMISC: ::c_int = 0x100;
pub const IFF_MULTICAST: ::c_int = 0x1000;
pub const IFF_LOWER_UP: ::c_int = 0x10000;
pub const IFF_DORMANT: ::c_int = 0x20000;

/* asm/socket.h */

pub const FIONREAD: ::c_int = 0x4008667f;
pub const FIONBIO: ::c_int = 0x8004667e;
pub const SIOCGIFCONF: ::c_ulong = 0x80107364;
pub const SIOCGIFFLAGS: ::c_ulong = 0x80507365;
pub const SIOCGIFADDR: ::c_ulong = 0x80507366;
pub const SIOCGIFBRDADDR: ::c_ulong = 0x80507367;
pub const SIOCGIFNETMASK: ::c_ulong = 0x80507368;
pub const SIOCGIFHWADDR: ::c_ulong = 0x80507369;
pub const SIOCGIFMETRIC: ::c_ulong = 0x8050736a;
pub const SIOCGIFMTU: ::c_ulong = 0x8050736b;
pub const SIOCGIFDSTADDR: ::c_ulong = 0x8050736e;

pub const SOL_SOCKET: ::c_int = 0xffff;

pub const SO_DEBUG: ::c_int = 0x0001;
pub const SO_ACCEPTCONN: ::c_int = 0x0002;
pub const SO_REUSEADDR: ::c_int = 0x0004;
pub const SO_KEEPALIVE: ::c_int = 0x0008;
pub const SO_DONTROUTE: ::c_int = 0x0010;
pub const SO_BROADCAST: ::c_int = 0x0020;
pub const SO_LINGER: ::c_int = 0x0080;
pub const SO_OOBINLINE: ::c_int = 0x0100;
pub const SO_PEERCRED: ::c_int = 0x0200;
pub const SO_PASSCRED: ::c_int = 0x0400;

pub const SO_SNDBUF: ::c_int = 0x1001;
pub const SO_RCVBUF: ::c_int = 0x1002;
pub const SO_SNDLOWAT: ::c_int = 0x1003;
pub const SO_RCVLOWAT: ::c_int = 0x1004;
pub const SO_SNDTIMEO: ::c_int = 0x1005;
pub const SO_RCVTIMEO: ::c_int = 0x1006;
pub const SO_ERROR: ::c_int = 0x1007;
pub const SO_TYPE: ::c_int = 0x1008;

/* sys/termios.h */

pub const TIOCMGET: ::c_ulong = 0x5415;
pub const TIOCMBIS: ::c_ulong = 0x5416;
pub const TIOCMBIC: ::c_ulong = 0x5417;
pub const TIOCMSET: ::c_ulong = 0x5418;
pub const TIOCINQ: ::c_ulong = 0x541B;
pub const TIOCSCTTY: ::c_ulong = 0x540E;
pub const TIOCSBRK: ::c_ulong = 0x5427;
pub const TIOCCBRK: ::c_ulong = 0x540E;
pub const TIOCM_DTR: ::c_int = 0x002;
pub const TIOCM_RTS: ::c_int = 0x004;
pub const TIOCM_CTS: ::c_int = 0x020;
pub const TIOCM_CAR: ::c_int = 0x040;
pub const TIOCM_RNG: ::c_int = 0x080;
pub const TIOCM_DSR: ::c_int = 0x100;
pub const TIOCM_CD: ::c_int = TIOCM_CAR;
pub const TIOCM_RI: ::c_int = TIOCM_RNG;

pub const TCOOFF: ::c_int = 0;
pub const TCOON: ::c_int = 1;
pub const TCIOFF: ::c_int = 2;
pub const TCION: ::c_int = 3;
pub const TCGETA: ::c_int = 5;
pub const TCSETA: ::c_int = 6;
pub const TCSETAW: ::c_int = 7;
pub const TCSETAF: ::c_int = 8;
pub const TCIFLUSH: ::c_int = 0;
pub const TCOFLUSH: ::c_int = 1;
pub const TCIOFLUSH: ::c_int = 2;
pub const TCFLSH: ::c_int = 3;
pub const TCSAFLUSH: ::c_int = 1;
pub const TCSANOW: ::c_int = 2;
pub const TCSADRAIN: ::c_int = 3;
pub const TCSADFLUSH: ::c_int = 4;
pub const TIOCPKT: ::c_int = 6;

pub const IGNBRK: ::tcflag_t = 0x00001;
pub const BRKINT: ::tcflag_t = 0x00002;
pub const IGNPAR: ::tcflag_t = 0x00003;
pub const IMAXBEL: ::tcflag_t = 0x00008;
pub const INPCK: ::tcflag_t = 0x00010;
pub const ISTRIP: ::tcflag_t = 0x00020;
pub const INLCR: ::tcflag_t = 0x00040;
pub const IGNCR: ::tcflag_t = 0x00080;
pub const ICRNL: ::tcflag_t = 0x00100;
pub const IXON: ::tcflag_t = 0x00400;
pub const IXOFF: ::tcflag_t = 0x01000;
pub const IUCLC: ::tcflag_t = 0x04000;
pub const IXANY: ::tcflag_t = 0x08000;
pub const PARMRK: ::tcflag_t = 0x10000;
pub const IUTF8: ::tcflag_t = 0x20000;

pub const OPOST: ::tcflag_t = 0x00001;
pub const OLCUC: ::tcflag_t = 0x00002;
pub const OCRNL: ::tcflag_t = 0x00004;
pub const ONLCR: ::tcflag_t = 0x00008;
pub const ONOCR: ::tcflag_t = 0x00010;
pub const ONLRET: ::tcflag_t = 0x00020;
pub const OFILL: ::tcflag_t = 0x00040;
pub const CRDLY: ::tcflag_t = 0x00180;
pub const CR0: ::tcflag_t = 0x00000;
pub const CR1: ::tcflag_t = 0x00080;
pub const CR2: ::tcflag_t = 0x00100;
pub const CR3: ::tcflag_t = 0x00180;
pub const NLDLY: ::tcflag_t = 0x00200;
pub const NL0: ::tcflag_t = 0x00000;
pub const NL1: ::tcflag_t = 0x00200;
pub const BSDLY: ::tcflag_t = 0x00400;
pub const BS0: ::tcflag_t = 0x00000;
pub const BS1: ::tcflag_t = 0x00400;
pub const TABDLY: ::tcflag_t = 0x01800;
pub const TAB0: ::tcflag_t = 0x00000;
pub const TAB1: ::tcflag_t = 0x00800;
pub const TAB2: ::tcflag_t = 0x01000;
pub const TAB3: ::tcflag_t = 0x01800;
pub const XTABS: ::tcflag_t = 0x01800;
pub const VTDLY: ::tcflag_t = 0x02000;
pub const VT0: ::tcflag_t = 0x00000;
pub const VT1: ::tcflag_t = 0x02000;
pub const FFDLY: ::tcflag_t = 0x04000;
pub const FF0: ::tcflag_t = 0x00000;
pub const FF1: ::tcflag_t = 0x04000;
pub const OFDEL: ::tcflag_t = 0x08000;

pub const CBAUD: ::tcflag_t = 0x0100f;

pub const B0: ::speed_t = 0x00000;
pub const B50: ::speed_t = 0x00001;
pub const B75: ::speed_t = 0x00002;
pub const B110: ::speed_t = 0x00003;
pub const B134: ::speed_t = 0x00004;
pub const B150: ::speed_t = 0x00005;
pub const B200: ::speed_t = 0x00006;
pub const B300: ::speed_t = 0x00007;
pub const B600: ::speed_t = 0x00008;
pub const B1200: ::speed_t = 0x00009;
pub const B1800: ::speed_t = 0x0000a;
pub const B2400: ::speed_t = 0x0000b;
pub const B4800: ::speed_t = 0x0000c;
pub const B9600: ::speed_t = 0x0000d;
pub const B19200: ::speed_t = 0x0000e;
pub const B38400: ::speed_t = 0x0000f;

pub const CSIZE: ::tcflag_t = 0x00030;
pub const CS5: ::tcflag_t = 0x00000;
pub const CS6: ::tcflag_t = 0x00010;
pub const CS7: ::tcflag_t = 0x00020;
pub const CS8: ::tcflag_t = 0x00030;
pub const CSTOPB: ::tcflag_t = 0x00040;
pub const CREAD: ::tcflag_t = 0x00080;
pub const PARENB: ::tcflag_t = 0x00100;
pub const PARODD: ::tcflag_t = 0x00200;
pub const HUPCL: ::tcflag_t = 0x00400;
pub const CLOCAL: ::tcflag_t = 0x00800;

pub const CBAUDEX: ::tcflag_t = 0x0100f;

pub const B57600: ::speed_t = 0x01001;
pub const B115200: ::speed_t = 0x01002;
pub const B128000: ::speed_t = 0x01003;
pub const B230400: ::speed_t = 0x01004;
pub const B256000: ::speed_t = 0x01005;
pub const B460800: ::speed_t = 0x01006;
pub const B500000: ::speed_t = 0x01007;
pub const B576000: ::speed_t = 0x01008;
pub const B921600: ::speed_t = 0x01009;
pub const B1000000: ::speed_t = 0x0100a;
pub const B1152000: ::speed_t = 0x0100b;
pub const B1500000: ::speed_t = 0x0100c;
pub const B2000000: ::speed_t = 0x0100d;
pub const B2500000: ::speed_t = 0x0100e;
pub const B3000000: ::speed_t = 0x0100f;

pub const CRTSCTS: ::tcflag_t = 0x08000;
pub const CMSPAR: ::tcflag_t = 0x40000000;

pub const ISIG: ::tcflag_t = 0x0001;
pub const ICANON: ::tcflag_t = 0x0002;
pub const ECHO: ::tcflag_t = 0x0004;
pub const ECHOE: ::tcflag_t = 0x0008;
pub const ECHOK: ::tcflag_t = 0x0010;
pub const ECHONL: ::tcflag_t = 0x0020;
pub const NOFLSH: ::tcflag_t = 0x0040;
pub const TOSTOP: ::tcflag_t = 0x0080;
pub const IEXTEN: ::tcflag_t = 0x0100;
pub const FLUSHO: ::tcflag_t = 0x0200;
pub const ECHOKE: ::tcflag_t = 0x0400;
pub const ECHOCTL: ::tcflag_t = 0x0800;

pub const VDISCARD: usize = 1;
pub const VEOL: usize = 2;
pub const VEOL2: usize = 3;
pub const VEOF: usize = 4;
pub const VERASE: usize = 5;
pub const VINTR: usize = 6;
pub const VKILL: usize = 7;
pub const VLNEXT: usize = 8;
pub const VMIN: usize = 9;
pub const VQUIT: usize = 10;
pub const VREPRINT: usize = 11;
pub const VSTART: usize = 12;
pub const VSTOP: usize = 13;
pub const VSUSP: usize = 14;
pub const VSWTC: usize = 15;
pub const VTIME: usize = 16;
pub const VWERASE: usize = 17;

pub const TIOCGWINSZ: ::c_int = 0x5401;
pub const TIOCSWINSZ: ::c_int = 0x5402;
pub const TIOCLINUX: ::c_int = 0x5403;
pub const TIOCGPGRP: ::c_int = 0x540f;
pub const TIOCSPGRP: ::c_int = 0x5410;

/* cygwin/in.h */

pub const IPPROTO_IP: ::c_int = 0;
pub const IPPROTO_HOPOPTS: ::c_int = 0;
pub const IPPROTO_ICMP: ::c_int = 1;
pub const IPPROTO_IGMP: ::c_int = 2;
pub const IPPROTO_IPIP: ::c_int = 4;
pub const IPPROTO_TCP: ::c_int = 6;
pub const IPPROTO_EGP: ::c_int = 8;
pub const IPPROTO_PUP: ::c_int = 12;
pub const IPPROTO_UDP: ::c_int = 17;
pub const IPPROTO_IDP: ::c_int = 22;
pub const IPPROTO_IPV6: ::c_int = 41;
pub const IPPROTO_ROUTING: ::c_int = 43;
pub const IPPROTO_FRAGMENT: ::c_int = 44;
pub const IPPROTO_ESP: ::c_int = 50;
pub const IPPROTO_AH: ::c_int = 51;
pub const IPPROTO_ICMPV6: ::c_int = 58;
pub const IPPROTO_NONE: ::c_int = 59;
pub const IPPROTO_DSTOPTS: ::c_int = 60;
pub const IPPROTO_RAW: ::c_int = 255;
pub const IPPROTO_MAX: ::c_int = 256;

/* dlfcn.h */

pub const RTLD_DEFAULT: *mut ::c_void = 0i64 as *mut ::c_void;
pub const RTLD_LOCAL: ::c_int = 0x0;
pub const RTLD_LAZY: ::c_int = 0x1;
pub const RTLD_NOW: ::c_int = 0x2;
pub const RTLD_GLOBAL: ::c_int = 0x4;
pub const RTLD_NODELETE: ::c_int = 0x8;
pub const RTLD_NOLOAD: ::c_int = 0x10;
pub const RTLD_DEEPBIND: ::c_int = 0x20;

/* time.h */

pub const TIMER_ABSTIME: ::c_int = 4;
pub const CLOCK_REALTIME_COARSE: ::clockid_t = 0;
pub const CLOCK_REALTIME: ::clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: ::clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: ::clockid_t = 3;
pub const CLOCK_MONOTONIC: ::clockid_t = 4;
pub const CLOCK_MONOTONIC_RAW: ::clockid_t = 5;
pub const CLOCK_MONOTONIC_COARSE: ::clockid_t = 6;
pub const CLOCK_BOOTTIME: ::clockid_t = 7;
pub const CLOCK_REALTIME_ALARM: ::clockid_t = 8;
pub const CLOCK_BOOTTIME_ALARM: ::clockid_t = 9;

/* locale.h */

pub const LC_ALL: ::c_int = 0;
pub const LC_COLLATE: ::c_int = 1;
pub const LC_CTYPE: ::c_int = 2;
pub const LC_MONETARY: ::c_int = 3;
pub const LC_NUMERIC: ::c_int = 4;
pub const LC_TIME: ::c_int = 5;
pub const LC_MESSAGES: ::c_int = 6;
pub const LC_ALL_MASK: ::c_int = (1<< LC_ALL);
pub const LC_COLLATE_MASK: ::c_int = (1<< LC_COLLATE);
pub const LC_CTYPE_MASK: ::c_int = (1<< LC_CTYPE);
pub const LC_MONETARY_MASK: ::c_int = (1<< LC_MONETARY);
pub const LC_NUMERIC_MASK: ::c_int = (1<< LC_NUMERIC);
pub const LC_TIME_MASK: ::c_int = (1<< LC_TIME);
pub const LC_MESSAGES_MASK: ::c_int = (1<< LC_MESSAGES);

/* sys/mman.h */

pub const PROT_NONE: ::c_int = 0;
pub const PROT_READ: ::c_int = 1;
pub const PROT_WRITE: ::c_int = 2;
pub const PROT_EXEC: ::c_int = 4;

pub const MAP_FILE: ::c_int = 0;
pub const MAP_SHARED: ::c_int = 1;
pub const MAP_PRIVATE: ::c_int = 2;
pub const MAP_TYPE: ::c_int = 0xf;
pub const MAP_FIXED: ::c_int = 0x10;
pub const MAP_ANONYMOUS: ::c_int = 0x20;
pub const MAP_ANON: ::c_int = MAP_ANONYMOUS;
pub const MAP_NORESERVE: ::c_int = 0x4000;
pub const MAP_AUTOGROW: ::c_int = 0x8000;
pub const MAP_FAILED: *mut ::c_void = !0 as *mut ::c_void;

pub const MS_ASYNC: ::c_int = 1;
pub const MS_SYNC: ::c_int = 2;
pub const MS_INVALIDATE: ::c_int = 4;

pub const POSIX_MADV_NORMAL: ::c_int = 0;
pub const POSIX_MADV_SEQUENTIAL: ::c_int = 1;
pub const POSIX_MADV_RANDOM: ::c_int = 2;
pub const POSIX_MADV_WILLNEED: ::c_int = 3;
pub const POSIX_MADV_DONTNEED: ::c_int = 4;

pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_SEQUENTIAL: ::c_int = 1;
pub const MADV_RANDOM: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;

/* sys/error.h */

pub const EPERM: ::c_int = 1;
pub const ENOENT: ::c_int = 2;
pub const ESRCH: ::c_int = 3;
pub const EINTR: ::c_int = 4;
pub const EIO: ::c_int = 5;
pub const ENXIO: ::c_int = 6;
pub const E2BIG: ::c_int = 7;
pub const ENOEXEC: ::c_int = 8;
pub const EBADF: ::c_int = 9;
pub const ECHILD: ::c_int = 10;
pub const EAGAIN: ::c_int = 11;
pub const ENOMEM: ::c_int = 12;
pub const EACCES: ::c_int = 13;
pub const EFAULT: ::c_int = 14;
pub const ENOTBLK: ::c_int = 15;
pub const EBUSY: ::c_int = 16;
pub const EEXIST: ::c_int = 17;
pub const EXDEV: ::c_int = 18;
pub const ENODEV: ::c_int = 19;
pub const ENOTDIR: ::c_int = 20;
pub const EISDIR: ::c_int = 21;
pub const EINVAL: ::c_int = 22;
pub const ENFILE: ::c_int = 23;
pub const EMFILE: ::c_int = 24;
pub const ENOTTY: ::c_int = 25;
pub const ETXTBSY: ::c_int = 26;
pub const EFBIG: ::c_int = 27;
pub const ENOSPC: ::c_int = 28;
pub const ESPIPE: ::c_int = 29;
pub const EROFS: ::c_int = 30;
pub const EMLINK: ::c_int = 31;
pub const EPIPE: ::c_int = 32;
pub const EDOM: ::c_int = 33;
pub const ERANGE: ::c_int = 34;
pub const ENOMSG: ::c_int = 35;
pub const EIDRM: ::c_int = 36;
pub const ECHRNG: ::c_int = 37;
pub const EL2NSYNC: ::c_int = 38;
pub const EL3HLT: ::c_int = 39;
pub const EL3RST: ::c_int = 40;
pub const ELNRNG: ::c_int = 41;
pub const EUNATCH: ::c_int = 42;
pub const ENOCSI: ::c_int = 43;
pub const EL2HLT: ::c_int = 44;
pub const EDEADLK: ::c_int = 45;
pub const ENOLCK: ::c_int = 46;
pub const EBADE: ::c_int = 50;
pub const EBADR: ::c_int = 51;
pub const EXFULL: ::c_int = 52;
pub const ENOANO: ::c_int = 53;
pub const EBADRQC: ::c_int = 54;
pub const EBADSLT: ::c_int = 55;
pub const EDEADLOCK: ::c_int = 56;
pub const EBFONT: ::c_int = 57;
pub const ENOSTR: ::c_int = 60;
pub const ENODATA: ::c_int = 61;
pub const ETIME: ::c_int = 62;
pub const ENOSR: ::c_int = 63;
pub const ENONET: ::c_int = 64;
pub const ENOPKG: ::c_int = 65;
pub const EREMOTE: ::c_int = 66;
pub const ENOLINK: ::c_int = 67;
pub const EADV: ::c_int = 68;
pub const ESRMNT: ::c_int = 69;
pub const ECOMM: ::c_int = 70;
pub const EPROTO: ::c_int = 71;
pub const EMULTIHOP: ::c_int = 74;
pub const ELBIN: ::c_int = 75;
pub const EDOTDOT: ::c_int = 76;
pub const EBADMSG: ::c_int = 77;
pub const EFTYPE: ::c_int = 79;
pub const ENOTUNIQ: ::c_int = 80;
pub const EBADFD: ::c_int = 81;
pub const EREMCHG: ::c_int = 82;
pub const ELIBACC: ::c_int = 83;
pub const ELIBBAD: ::c_int = 84;
pub const ELIBSCN: ::c_int = 85;
pub const ELIBMAX: ::c_int = 86;
pub const ELIBEXEC: ::c_int = 87;
pub const ENOSYS: ::c_int = 88;
pub const ENMFILE: ::c_int = 89;
pub const ENOTEMPTY: ::c_int = 90;
pub const ENAMETOOLONG: ::c_int = 91;
pub const ELOOP: ::c_int = 92;
pub const EOPNOTSUPP: ::c_int = 95;
pub const EPFNOSUPPORT: ::c_int = 96;
pub const ECONNRESET: ::c_int = 104;
pub const ENOBUFS: ::c_int = 105;
pub const EAFNOSUPPORT: ::c_int = 106;
pub const EPROTOTYPE: ::c_int = 107;
pub const ENOTSOCK: ::c_int = 108;
pub const ENOPROTOOPT: ::c_int = 109;
pub const ESHUTDOWN: ::c_int = 110;
pub const ECONNREFUSED: ::c_int = 111;
pub const EADDRINUSE: ::c_int = 112;
pub const ECONNABORTED: ::c_int = 113;
pub const ENETUNREACH: ::c_int = 114;
pub const ENETDOWN: ::c_int = 115;
pub const ETIMEDOUT: ::c_int = 116;
pub const EHOSTDOWN: ::c_int = 117;
pub const EHOSTUNREACH: ::c_int = 118;
pub const EINPROGRESS: ::c_int = 119;
pub const EALREADY: ::c_int = 120;
pub const EDESTADDRREQ: ::c_int = 121;
pub const EMSGSIZE: ::c_int = 122;
pub const EPROTONOSUPPORT: ::c_int = 123;
pub const ESOCKTNOSUPPORT: ::c_int = 124;
pub const EADDRNOTAVAIL: ::c_int = 125;
pub const ENETRESET: ::c_int = 126;
pub const EISCONN: ::c_int = 127;
pub const ENOTCONN: ::c_int = 128;
pub const ETOOMANYREFS: ::c_int = 129;
pub const EPROCLIM: ::c_int = 130;
pub const EUSERS: ::c_int = 131;
pub const EDQUOT: ::c_int = 132;
pub const ESTALE: ::c_int = 133;
pub const ENOTSUP: ::c_int = 134;
pub const ENOMEDIUM: ::c_int = 135;
pub const ENOSHARE: ::c_int = 136;
pub const ECASECLASH: ::c_int = 137;
pub const EILSEQ: ::c_int = 138;
pub const EOVERFLOW: ::c_int = 139;
pub const ECANCELED: ::c_int = 140;
pub const ENOTRECOVERABLE: ::c_int = 141;
pub const EOWNERDEAD: ::c_int = 142;
pub const ESTRPIPE: ::c_int = 143;
pub const EWOULDBLOCK: ::c_int = EAGAIN;

/* pthread.h */

pub const PTHREAD_CREATE_JOINABLE: ::c_int = 0;
pub const PTHREAD_CREATE_DETACHED: ::c_int = 1;
pub const PTHREAD_MUTEX_INITIALIZER: ::pthread_mutex_t = 19 as *mut _;
pub const PTHREAD_COND_INITIALIZER: ::pthread_cond_t = 21 as *mut _;
pub const PTHREAD_RWLOCK_INITIALIZER: ::pthread_rwlock_t = 22 as *mut _;
pub const PTHREAD_MUTEX_NORMAL: ::c_int = 2;
pub const PTHREAD_MUTEX_RECURSIVE: ::c_int = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: ::c_int = 1;
pub const PTHREAD_MUTEX_DEFAULT: ::c_int = PTHREAD_MUTEX_NORMAL;
pub const PTHREAD_PROCESS_PRIVATE: ::c_int = 0;
pub const PTHREAD_PROCESS_SHARED: ::c_int = 1;

/* semaphore.h */

pub const SEM_FAILED: *mut ::sem_t = 0 as *mut sem_t;

/* cygwin/signal.h */

pub const CLD_EXITED: ::c_int = 28;
pub const CLD_KILLED: ::c_int = 29;
pub const CLD_DUMPED: ::c_int = 30;
pub const CLD_TRAPPED: ::c_int =31;
pub const CLD_STOPPED: ::c_int =32;
pub const CLD_CONTINUED: ::c_int = 33;

pub const SIGEV_SIGNAL: ::c_int = 0;
pub const SIGEV_NONE: ::c_int = 1;
pub const SIGEV_THREAD: ::c_int = 2;

pub const SA_NOCLDSTOP: ::c_int = 0x00000001;
pub const SA_SIGINFO: ::c_int = 0x00000002;
pub const SA_RESTART: ::c_int = 0x10000000;
pub const SA_ONSTACK: ::c_int = 0x20000000;
pub const SA_NODEFER: ::c_int = 0x40000000;
pub const SA_RESETHAND: ::c_int = 0x80000000;

pub const MINSIGSTKSZ: ::size_t = 8192;
pub const SIGSTKSZ: ::size_t = 32768;

pub const SIGHUP: ::c_int = 1;
pub const SIGINT: ::c_int = 2;
pub const SIGQUIT: ::c_int = 3;
pub const SIGILL: ::c_int = 4;
pub const SIGTRAP: ::c_int = 5;
pub const SIGABRT: ::c_int = 6;
pub const SIGIOT: ::c_int = SIGABRT;
pub const SIGEMT: ::c_int = 7;
pub const SIGFPE: ::c_int = 8;
pub const SIGKILL: ::c_int = 9;
pub const SIGBUS: ::c_int = 10;
pub const SIGSEGV: ::c_int = 11;
pub const SIGSYS: ::c_int = 12;
pub const SIGPIPE: ::c_int = 13;
pub const SIGALRM: ::c_int = 14;
pub const SIGTERM: ::c_int = 15;
pub const SIGURG: ::c_int = 16;
pub const SIGSTOP: ::c_int = 17;
pub const SIGTSTP: ::c_int = 18;
pub const SIGCONT: ::c_int = 19;
pub const SIGCHLD: ::c_int = 20;
pub const SIGCLD: ::c_int = 20;
pub const SIGTTIN: ::c_int = 21;
pub const SIGTTOU: ::c_int = 22;
pub const SIGIO: ::c_int = 23;
pub const SIGPOLL: ::c_int = SIGIO;
pub const SIGXCPU: ::c_int = 24;
pub const SIGXFSZ: ::c_int = 25;
pub const SIGVTALRM: ::c_int = 26;
pub const SIGPROF: ::c_int = 27;
pub const SIGWINCH: ::c_int = 28;
pub const SIGLOST: ::c_int = 29;
pub const SIGPWR: ::c_int = SIGLOST;
pub const SIGUSR1: ::c_int = 30;
pub const SIGUSR2: ::c_int = 31;

/* sys/signal.h */

pub const SS_ONSTACK: ::c_int = 1;
pub const SS_DISABLE: ::c_int = 2;

pub const SIG_SETMASK: ::c_int = 0;
pub const SIG_BLOCK: ::c_int = 0x000001;
pub const SIG_UNBLOCK: ::c_int = 0x02;

/* sys/sched.h */

pub const SCHED_OTHER: ::c_int = 3;
pub const SCHED_FIFO: ::c_int = 1;
pub const SCHED_RR: ::c_int = 2;

/* sys/_default_fcntl.h */

pub const O_RDONLY: ::c_int = 0;
pub const O_WRONLY: ::c_int = 1;
pub const O_RDWR: ::c_int = 2;
pub const O_ACCMODE: ::c_int = (O_RDONLY|O_WRONLY|O_RDWR);
pub const O_APPEND: ::c_int = 0x0008;
pub const O_CREAT: ::c_int = 0x0200;
pub const O_TRUNC: ::c_int = 0x0400;
pub const O_EXCL: ::c_int = 0x0800;
pub const O_SYNC: ::c_int = 0x2000;
pub const O_NONBLOCK: ::c_int = 0x4000;
pub const O_NOCTTY: ::c_int = 0x8000;
pub const O_CLOEXEC: ::c_int = 0x40000;
pub const O_NOFOLLOW: ::c_int = 0x100000;
pub const O_DIRECTORY: ::c_int = 0x200000;
pub const O_EXEC: ::c_int = 0x400000;
pub const O_SEARCH: ::c_int = 0x400000;
pub const O_DIRECT: ::c_int = 0x80000;
pub const O_BINARY: ::c_int = 0x10000;
pub const O_TEXT: ::c_int = 0x20000;
pub const O_DSYNC: ::c_int = O_SYNC;
pub const O_RSYNC: ::c_int = O_SYNC;
pub const O_TMPFILE: ::c_int = 0x800000;
pub const O_NOATIME: ::c_int = 0x1000000;
pub const O_PATH: ::c_int = 0x2000000;

pub const F_DUPFD: ::c_int = 0;
pub const F_GETFD: ::c_int = 1;
pub const F_SETFD: ::c_int = 2;
pub const F_GETFL: ::c_int = 3;
pub const F_SETFL: ::c_int = 4;
pub const F_GETOWN: ::c_int = 5;
pub const F_SETOWN: ::c_int = 6;
pub const F_GETLK: ::c_int = 7;
pub const F_SETLK: ::c_int = 8;
pub const F_SETLKW: ::c_int = 9;
pub const F_RGETLK: ::c_int = 10;
pub const F_RSETLK: ::c_int = 11;
pub const F_CNVT: ::c_int = 12;
pub const F_RSETLKW: ::c_int = 13;
pub const F_DUPFD_CLOEXEC: ::c_int = 14;
pub const F_RDLCK: ::c_int = 1;
pub const F_WRLCK: ::c_int = 2;
pub const F_UNLCK: ::c_int = 3;

pub const AT_FDCWD: ::c_int = -2;
pub const AT_EACCESS: ::c_int = 1;
pub const AT_SYMLINK_NOFOLLOW: ::c_int = 2;
pub const AT_SYMLINK_FOLLOW: ::c_int = 4;
pub const AT_REMOVEDIR: ::c_int = 8;
pub const AT_EMPTY_PATH: ::c_int = 16;

pub const LOCK_SH: ::c_int = 1;
pub const LOCK_EX: ::c_int = 2;
pub const LOCK_NB: ::c_int = 4;
pub const LOCK_UN: ::c_int = 8;

/* fcntl.h */

pub const O_NDELAY: ::c_int = O_NONBLOCK;

pub const POSIX_FADV_NORMAL: ::c_int = 0;
pub const POSIX_FADV_SEQUENTIAL: ::c_int = 1;
pub const POSIX_FADV_RANDOM: ::c_int = 2;
pub const POSIX_FADV_WILLNEED: ::c_int = 3;
pub const POSIX_FADV_DONTNEED: ::c_int = 4;
pub const POSIX_FADV_NOREUSE: ::c_int = 5;

/* sys/poll.h */

pub const POLLIN: ::c_short = 0x1;
pub const POLLPRI: ::c_short = 0x2;
pub const POLLOUT: ::c_short = 0x4;
pub const POLLERR: ::c_short = 0x8;
pub const POLLHUP: ::c_short = 0x10;
pub const POLLNVAL: ::c_short = 0x20;
pub const POLLRDNORM: ::c_short = POLLIN;
pub const POLLRDBAND: ::c_short = POLLPRI;
pub const POLLWRNORM: ::c_short = POLLOUT;
pub const POLLWRBAND: ::c_short = POLLOUT;

/* netinet/tcp.h */

pub const TCP_NODELAY: ::c_int = 1;
pub const TCP_KEEPIDLE: ::c_int = 3;
pub const TCP_MAXSEG: ::c_int = 4;
pub const TCP_QUICKACK: ::c_int = 12;
pub const TCP_USER_TIMEOUT: ::c_int = 14;
pub const TCP_FASTOPEN: ::c_int = 15;
pub const TCP_KEEPCNT: ::c_int = 16;
pub const TCP_KEEPINTVL: ::c_int = 17;

/* cygwin/wait.h */

pub const WNOHANG: ::c_int = 0x00000001;
pub const WUNTRACED: ::c_int = 0x00000002;
pub const WCONTINUED: ::c_int = 0x00000008;

/* limits.h */

pub const PTHREAD_STACK_MIN: ::size_t = 65536;
pub const PATH_MAX: ::c_int = 4096;
pub const PIPE_BUF: usize = 4096;

/* sys/cpuset.h */

pub const CPU_SETSIZE: ::c_int = 0x400;

/* sys/random.h */

pub const GRND_NONBLOCK: ::c_uint = 0x0001;
pub const GRND_RANDOM: ::c_uint = 0x0002;

/* spawn.h */

pub const POSIX_SPAWN_RESETIDS: ::c_int = 0x01;
pub const POSIX_SPAWN_SETPGROUP: ::c_int = 0x02;
pub const POSIX_SPAWN_SETSCHEDPARAM: ::c_int = 0x04;
pub const POSIX_SPAWN_SETSCHEDULER: ::c_int = 0x08;
pub const POSIX_SPAWN_SETSIGDEF: ::c_int = 0x10;
pub const POSIX_SPAWN_SETSIGMASK: ::c_int = 0x20;

/* elf.h */

pub const PT_NULL: u32 = 0x0;
pub const PT_LOAD: u32 = 0x1;
pub const PT_DYNAMIC: u32 = 0x2;
pub const PT_INTERP: u32 = 0x3;
pub const PT_NOTE: u32 = 0x4;
pub const PT_SHLIB: u32 = 0x5;
pub const PT_PHDR: u32 = 0x6;
pub const PT_TLS: u32 = 0x7;
pub const PT_NUM: u32 = 0x8;
pub const PT_LOOS: u32 = 0x60000000;
pub const PT_GNU_EH_FRAME: u32 = 0x6474e550;
pub const PT_GNU_STACK: u32 = 0x6474e551;
pub const PT_GNU_RELRO: u32 = 0x6474e552;

pub const AT_NULL: ::c_ulong = 0;
pub const AT_IGNORE: ::c_ulong = 1;
pub const AT_EXECFD: ::c_ulong = 2;
pub const AT_PHDR: ::c_ulong = 3;
pub const AT_PHENT: ::c_ulong = 4;
pub const AT_PHNUM: ::c_ulong = 5;
pub const AT_PAGESZ: ::c_ulong = 6;
pub const AT_BASE: ::c_ulong = 7;
pub const AT_FLAGS: ::c_ulong = 8;
pub const AT_ENTRY: ::c_ulong = 9;
pub const AT_NOTELF: ::c_ulong = 10;
pub const AT_UID: ::c_ulong = 11;
pub const AT_EUID: ::c_ulong = 12;
pub const AT_GID: ::c_ulong = 13;
pub const AT_EGID: ::c_ulong = 14;
pub const AT_CLKTCK: ::c_ulong = 17;
pub const AT_PLATFORM: ::c_ulong = 15;
pub const AT_HWCAP: ::c_ulong = 16;
pub const AT_FPUCW: ::c_ulong = 18;
pub const AT_DCACHEBSIZE: ::c_ulong = 19;
pub const AT_ICACHEBSIZE: ::c_ulong = 20;
pub const AT_UCACHEBSIZE: ::c_ulong = 21;
pub const AT_IGNOREPPC: ::c_ulong = 22;
pub const AT_SECURE: ::c_ulong = 23;
pub const AT_BASE_PLATFORM: ::c_ulong = 24;
pub const AT_RANDOM: ::c_ulong = 25;
pub const AT_HWCAP2: ::c_ulong = 26;
pub const AT_EXECFN: ::c_ulong = 31;

// The order of fields in these structs are crucial
// for converting between the Rust and C types.
s! {
    /* cygwin/socket.h */

    pub struct sockaddr {
        pub sa_family: sa_family_t,
        pub sa_data: [::c_char; 14],
    }

    pub struct sockaddr_storage {
        pub ss_family: sa_family_t,
        __ss_pad1: [::c_char; 6],
        __ss_align: i64,
        __ss_pad2: [::c_char; 128 - 2 * 8],
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

    /* cygwin/in.h */

    pub struct in_addr {
        pub s_addr: ::in_addr_t,
    }

    pub struct ip_mreq {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
    }

    pub struct ip_mreq_source {
        pub imr_multiaddr: in_addr,
        pub imr_interface: in_addr,
        pub imr_sourceaddr: in_addr,
    }

    pub struct in_pktinfo {
        pub ipi_addr: ::in_addr,
        pub ipi_ifindex: u32,
    }

    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: ::in_port_t,
        pub sin_addr: ::in_addr,
        pub sin_zero: [u8; 8],
    }

    /* cygwin/in6.h */

    pub struct in6_pktinfo {
        pub ipi6_addr: ::in6_addr,
        pub ipi6_ifindex: u32,
    }

    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: ::in_port_t,
        pub sin6_flowinfo: u32,
        pub sin6_addr: ::in6_addr,
        pub sin6_scope_id: u32,
    }

    /* dlfcn.h */

    pub struct Dl_info {
        pub dli_fname: *const ::c_char,
        pub dli_fbase: *mut ::c_void,
        pub dli_sname: *const ::c_char,
        pub dli_saddr: *mut ::c_void,
    }

    /* netdb.h */

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

    /* time.h */

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

    /* sys/select.h */

    pub struct fd_set {
        fds_bits: [::c_ulong; FD_SETSIZE / ULONG_SIZE],
    }

    /* locale.h */

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

    /* sys/un.h */

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 108]
    }

    /* semaphore.h */

    pub struct sem_t {
        __dummy: ::c_char,
    }

    /* sys/vfs.h */

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

    /* sys/statvfs.h */

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

    /* cygwin/stat.h */

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
        pub st_birthtime: ::time_t,
        pub st_birthtime_nsec: ::c_long,
    }

    /* sys/dirent.h */

    pub struct dirent {
        pub __d_version: u32,
        pub d_ino: ::ino_t,
        pub d_type: ::c_uchar,
        pub __d_unused1:[::c_uchar; 3],
        pub __d_internal1: u32,
        pub d_name: [::c_char; 256],
    }

    /* sys/termios.h */

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

    /* pwd.h */

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

    /* sys/signal.h */
    pub struct stack_t {
        pub ss_sp: *mut ::c_void,
        pub ss_flags: ::c_int,
        pub ss_size: ::size_t
    }

    /* cygwin/if.h */

    pub struct if_nameindex {
        pub if_index: ::c_uint,
        pub if_name: *mut ::c_char,
    }

    /* cygwin/signal.h */

    pub struct _libc_fpxreg {
        pub significand: [u16; 4],
        pub exponent: u16,
        __padding: [u16; 3],
    }

    pub struct _libc_xmmreg {
        pub element: [u32; 4],
    }

    pub struct _libc_fpstate {
        pub cwd: u16,
        pub swd: u16,
        pub ftw: u16,
        pub fop: u16,
        pub rip: u64,
        pub rdp: u64,
        pub mxcsr: u32,
        pub mxcr_mask: u32,
        pub _st: [_libc_fpxreg; 8],
        pub _xmm: [_libc_xmmreg; 16],
        __private: [u64; 12],
    }

    pub struct sigevent {
        pub sigev_value: ::sigval,
        pub sigev_signo: ::c_int,
        pub sigev_notify: ::c_int,
        __unused1: [::c_int; 4],
    }

    pub struct siginfo_t {
        pub si_signo: ::c_int,
        pub si_code: ::c_int,
        pub si_pid: ::pid_t,
        pub si_uid: ::uid_t,
        pub si_errno: ::c_int,
        _align: [u64; 0],
    }

    pub struct sigaction {
        pub sa_sigaction: ::sighandler_t,
        pub sa_mask: ::sigset_t,
        pub sa_flags: ::c_int,
    }

    /* sys/ucontext.h */

    pub struct mcontext_t {
        pub p1home: u64,
        pub p2home: u64,
        pub p3home: u64,
        pub p4home: u64,
        pub p5home: u64,
        pub p6home: u64,
        pub ctxflags: u32,
        pub mxcsr: u32,
        pub cs: u16,
        pub ds: u16,
        pub ed: u16,
        pub fs: u16,
        pub gs: u16,
        pub ss: u16,
        pub eflags: u32,
        pub dr0: u64,
        pub dr1: u64,
        pub dr2: u64,
        pub dr3: u64,
        pub dr4: u64,
        pub dr5: u64,
        pub dr6: u64,
        pub dr7: u64,
        pub rax: u64,
        pub rcx: u64,
        pub rdx: u64,
        pub rbx: u64,
        pub rsp: u64,
        pub rbp: u64,
        pub rsi: u64,
        pub rdi: u64,
        pub r9: u64,
        pub r10: u64,
        pub r11: u64,
        pub r12: u64,
        pub r13: u64,
        pub r14: u64,
        pub r15: u64,
        pub rip: u64,
        pub fpregs: _libc_fpstate,
        pub vregs: [u64; 52],
        pub vcx: u64,
        pub dbc: u64,
        pub btr: u64,
        pub bfr: u64,
        pub etr: u64,
        pub efr: u64,
        pub oldmask: u64,
        pub cr2: u64,
    }

    pub struct ucontext_t {
        pub uc_mcontext: mcontext_t,
        pub uc_link: *mut ucontext_t,
        pub uc_sigmask: ::sigset_t,
        pub uc_stack: ::stack_t,
        pub uc_flags: ::c_ulong,
    }

    /* sys/sched.h */

    pub struct sched_param {
        pub sched_priority: ::c_int,
    }

    /* sys/cpuset.h */

    pub struct cpu_set_t {
        bits: [u64; 16],
    }

    /* cygwin/_ucred.h */

    pub struct ucred {
        pub pid: ::pid_t,
        pub uid: ::uid_t,
        pub gid: ::gid_t,
    }

    /* elf.h */



    pub struct Elf32_Ehdr {
        pub e_ident: [::c_uchar; 16],
        pub e_type: Elf32_Half,
        pub e_machine: Elf32_Half,
        pub e_version: Elf32_Word,
        pub e_entry: Elf32_Addr,
        pub e_phoff: Elf32_Off,
        pub e_shoff: Elf32_Off,
        pub e_flags: Elf32_Word,
        pub e_ehsize: Elf32_Half,
        pub e_phentsize: Elf32_Half,
        pub e_phnum: Elf32_Half,
        pub e_shentsize: Elf32_Half,
        pub e_shnum: Elf32_Half,
        pub e_shstrndx: Elf32_Half,
    }
    pub struct Elf64_Ehdr {
        pub e_ident: [::c_uchar; 16],
        pub e_type: Elf64_Half,
        pub e_machine: Elf64_Half,
        pub e_version: Elf64_Word,
        pub e_entry: Elf64_Addr,
        pub e_phoff: Elf64_Off,
        pub e_shoff: Elf64_Off,
        pub e_flags: Elf64_Word,
        pub e_ehsize: Elf64_Half,
        pub e_phentsize: Elf64_Half,
        pub e_phnum: Elf64_Half,
        pub e_shentsize: Elf64_Half,
        pub e_shnum: Elf64_Half,
        pub e_shstrndx: Elf64_Half,
    }
    pub struct Elf32_Shdr {
        pub sh_name: Elf32_Word,
        pub sh_type: Elf32_Word,
        pub sh_flags: Elf32_Word,
        pub sh_addr: Elf32_Addr,
        pub sh_offset: Elf32_Off,
        pub sh_size: Elf32_Word,
        pub sh_link: Elf32_Word,
        pub sh_info: Elf32_Word,
        pub sh_addralign: Elf32_Word,
        pub sh_entsize: Elf32_Word,
    }
    pub struct Elf64_Shdr {
        pub sh_name: Elf64_Word,
        pub sh_type: Elf64_Word,
        pub sh_flags: Elf64_Xword,
        pub sh_addr: Elf64_Addr,
        pub sh_offset: Elf64_Off,
        pub sh_size: Elf64_Xword,
        pub sh_link: Elf64_Word,
        pub sh_info: Elf64_Word,
        pub sh_addralign: Elf64_Xword,
        pub sh_entsize: Elf64_Xword,
    }
    pub struct Elf32_Chdr {
        pub ch_type: ::Elf32_Word,
        pub ch_size: ::Elf32_Word,
        pub ch_addralign: ::Elf32_Word,
    }
    pub struct Elf64_Chdr {
        pub ch_type: ::Elf64_Word,
        pub ch_reserved: ::Elf64_Word,
        pub ch_size: ::Elf64_Xword,
        pub ch_addralign: ::Elf64_Xword,
    }
    pub struct Elf32_Sym {
        pub st_name: Elf32_Word,
        pub st_value: Elf32_Addr,
        pub st_size: Elf32_Word,
        pub st_info: ::c_uchar,
        pub st_other: ::c_uchar,
        pub st_shndx: Elf32_Section,
    }
    pub struct Elf64_Sym {
        pub st_name: Elf64_Word,
        pub st_info: ::c_uchar,
        pub st_other: ::c_uchar,
        pub st_shndx: Elf64_Section,
        pub st_value: Elf64_Addr,
        pub st_size: Elf64_Xword,
    }
    pub struct Elf32_Phdr {
        pub p_type: Elf32_Word,
        pub p_offset: Elf32_Off,
        pub p_vaddr: Elf32_Addr,
        pub p_paddr: Elf32_Addr,
        pub p_filesz: Elf32_Word,
        pub p_memsz: Elf32_Word,
        pub p_flags: Elf32_Word,
        pub p_align: Elf32_Word,
    }
    pub struct Elf64_Phdr {
        pub p_type: Elf64_Word,
        pub p_flags: Elf64_Word,
        pub p_offset: Elf64_Off,
        pub p_vaddr: Elf64_Addr,
        pub p_paddr: Elf64_Addr,
        pub p_filesz: Elf64_Xword,
        pub p_memsz: Elf64_Xword,
        pub p_align: Elf64_Xword,
    }

    /* machine/types.h */

    pub struct flock {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off_t,
        pub l_len: ::off_t,
        pub l_pid: ::pid_t,
    }

    pub struct flock64 {
        pub l_type: ::c_short,
        pub l_whence: ::c_short,
        pub l_start: ::off64_t,
        pub l_len: ::off64_t,
        pub l_pid: ::pid_t,
    }
}

pub type statfs64 = statfs;
pub type statvfs64 = statvfs;
pub type stat64 = stat;
pub type dirent64 = dirent;

extern "C" {
    /* sys/socket.h */

    pub fn bind(socket: ::c_int, address: *const ::sockaddr, address_len: ::socklen_t) -> ::c_int;
    pub fn recvfrom(
        socket: ::c_int,
        buf: *mut ::c_void,
        len: ::size_t,
        flags: ::c_int,
        addr: *mut ::sockaddr,
        addrlen: *mut ::socklen_t,
    ) -> ::ssize_t;
    pub fn accept4(
        fd: ::c_int,
        addr: *mut ::sockaddr,
        len: *mut ::socklen_t,
        flg: ::c_int,
    ) -> ::c_int;
    pub fn recvmsg(fd: ::c_int, msg: *mut ::msghdr, flags: ::c_int) -> ::ssize_t;
    pub fn sendmsg(fd: ::c_int, msg: *const ::msghdr, flags: ::c_int) -> ::ssize_t;

    /* linux_compat/sys/sendfile.h */

    pub fn sendfile(
        out_fd: ::c_int,
        in_fd: ::c_int,
        offset: *mut off_t,
        count: ::size_t,
    ) -> ::ssize_t;

    /* pthread.h */

    pub fn pthread_getattr_np(native: ::pthread_t, attr: *mut ::pthread_attr_t) -> ::c_int;
    pub fn pthread_attr_getstack(
        attr: *const ::pthread_attr_t,
        stackaddr: *mut *mut ::c_void,
        stacksize: *mut ::size_t,
    ) -> ::c_int;
    pub fn pthread_condattr_getclock(
        attr: *const pthread_condattr_t,
        clock_id: *mut clockid_t,
    ) -> ::c_int;
    pub fn pthread_condattr_setclock(
        attr: *mut pthread_condattr_t,
        clock_id: ::clockid_t,
    ) -> ::c_int;
    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: ::c_int) -> ::c_int;
    pub fn pthread_mutexattr_setpshared(
        attr: *mut pthread_mutexattr_t,
        pshared: ::c_int,
    ) -> ::c_int;
    pub fn pthread_rwlockattr_getpshared(
        attr: *const pthread_rwlockattr_t,
        val: *mut ::c_int,
    ) -> ::c_int;
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, val: ::c_int) -> ::c_int;
    pub fn pthread_setschedprio(native: ::pthread_t, priority: ::c_int) -> ::c_int;
    pub fn pthread_getschedparam(
        native: ::pthread_t,
        policy: *mut ::c_int,
        param: *mut ::sched_param,
    ) -> ::c_int;
    pub fn pthread_mutex_timedlock(
        lock: *mut pthread_mutex_t,
        abstime: *const ::timespec,
    ) -> ::c_int;
    pub fn pthread_attr_getguardsize(
        attr: *const ::pthread_attr_t,
        guardsize: *mut ::size_t,
    ) -> ::c_int;
    pub fn pthread_condattr_getpshared(
        attr: *const pthread_condattr_t,
        pshared: *mut ::c_int,
    ) -> ::c_int;
    pub fn pthread_setschedparam(
        native: ::pthread_t,
        policy: ::c_int,
        param: *const ::sched_param,
    ) -> ::c_int;
    pub fn pthread_cancel(thread: ::pthread_t) -> ::c_int;
    pub fn pthread_atfork(
        prepare: ::Option<unsafe extern "C" fn()>,
        parent: ::Option<unsafe extern "C" fn()>,
        child: ::Option<unsafe extern "C" fn()>,
    ) -> ::c_int;
    pub fn pthread_mutexattr_getpshared(
        attr: *const pthread_mutexattr_t,
        pshared: *mut ::c_int,
    ) -> ::c_int;
    pub fn pthread_create(
        native: *mut ::pthread_t,
        attr: *const ::pthread_attr_t,
        f: extern "C" fn(*mut ::c_void) -> *mut ::c_void,
        value: *mut ::c_void,
    ) -> ::c_int;
    pub fn pthread_getaffinity_np(
        thread: ::pthread_t,
        cpusetsize: ::size_t,
        cpuset: *mut ::cpu_set_t,
    ) -> ::c_int;
    pub fn pthread_getname_np(thread: ::pthread_t, name: *mut ::c_char, len: ::size_t) -> ::c_int;
    pub fn pthread_setaffinity_np(
        thread: ::pthread_t,
        cpusetsize: ::size_t,
        cpuset: *const ::cpu_set_t,
    ) -> ::c_int;
    pub fn pthread_setname_np(thread: ::pthread_t, name: *const ::c_char) -> ::c_int;

    /* netdb.h */

    pub fn getnameinfo(
        sa: *const ::sockaddr,
        salen: ::socklen_t,
        host: *mut ::c_char,
        hostlen: ::socklen_t,
        serv: *mut ::c_char,
        sevlen: ::socklen_t,
        flags: ::c_int,
    ) -> ::c_int;

    /* sys/uio.h */

    pub fn writev(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;
    pub fn readv(fd: ::c_int, iov: *const ::iovec, iovcnt: ::c_int) -> ::ssize_t;

    /* sys/stat.h */

    pub fn fstat(fildes: ::c_int, buf: *mut stat) -> ::c_int;
    #[link_name = "fstat"]
    pub fn fstat64(fildes: ::c_int, buf: *mut stat64) -> ::c_int;
    pub fn stat(path: *const c_char, buf: *mut stat) -> ::c_int;
    #[link_name = "stat"]
    pub fn stat64(path: *const c_char, buf: *mut stat64) -> ::c_int;
    pub fn lstat(path: *const c_char, buf: *mut stat) -> ::c_int;
    #[link_name = "lstat"]
    pub fn lstat64(path: *const c_char, buf: *mut stat64) -> ::c_int;
    pub fn fstatat(
        dirfd: ::c_int,
        pathname: *const c_char,
        buf: *mut stat,
        flags: ::c_int,
    ) -> ::c_int;
    #[link_name = "fstatat"]
    pub fn fstatat64(
        dirfd: ::c_int,
        pathname: *const c_char,
        buf: *mut stat64,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn mkfifoat(dirfd: ::c_int, pathname: *const ::c_char, mode: ::mode_t) -> ::c_int;
    pub fn mknodat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        mode: ::mode_t,
        dev: dev_t,
    ) -> ::c_int;
    pub fn utimensat(
        dirfd: ::c_int,
        path: *const ::c_char,
        times: *const ::timespec,
        flag: ::c_int,
    ) -> ::c_int;
    pub fn futimens(fd: ::c_int, times: *const ::timespec) -> ::c_int;

    /* stdlib.h */

    pub fn abs(i: ::c_int) -> ::c_int;
    pub fn atof(s: *const ::c_char) -> ::c_double;
    pub fn labs(i: ::c_long) -> ::c_long;
    pub fn mkostemp(template: *mut ::c_char, flags: ::c_int) -> ::c_int;
    pub fn mkostemps(template: *mut ::c_char, suffixlen: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn mkstemps(template: *mut ::c_char, suffixlen: ::c_int) -> ::c_int;
    pub fn rand() -> ::c_int;
    pub fn srand(seed: ::c_uint);
    pub fn qsort_r(
        base: *mut ::c_void,
        num: ::size_t,
        size: ::size_t,
        compar: ::Option<
            unsafe extern "C" fn(*const ::c_void, *const ::c_void, *mut ::c_void) -> ::c_int,
        >,
        arg: *mut ::c_void,
    );

    /* sys/unistd.h */

    pub fn daemon(nochdir: ::c_int, noclose: ::c_int) -> ::c_int;
    pub fn dup3(oldfd: ::c_int, newfd: ::c_int, flags: ::c_int) -> ::c_int;
    pub fn execvpe(
        file: *const ::c_char,
        argv: *const *const ::c_char,
        envp: *const *const ::c_char,
    ) -> ::c_int;
    pub fn faccessat(
        dirfd: ::c_int,
        pathname: *const ::c_char,
        mode: ::c_int,
        flags: ::c_int,
    ) -> ::c_int;
    pub fn fexecve(
        fd: ::c_int,
        argv: *const *const ::c_char,
        envp: *const *const ::c_char,
    ) -> ::c_int;
    pub fn fdatasync(fd: ::c_int) -> ::c_int;
    pub fn getdomainname(name: *mut ::c_char, len: ::size_t) -> ::c_int;
    pub fn lseek(fd: ::c_int, offset: off_t, whence: ::c_int) -> off_t;
    #[link_name = "lseek"]
    pub fn lseek64(fd: ::c_int, offset: off64_t, whence: ::c_int) -> off64_t;
    pub fn pipe2(fds: *mut ::c_int, flags: ::c_int) -> ::c_int;
    pub fn pread(fd: ::c_int, buf: *mut ::c_void, count: ::size_t, offset: off_t) -> ::ssize_t;
    #[link_name = "pread"]
    pub fn pread64(fd: ::c_int, buf: *mut ::c_void, count: ::size_t, offset: off64_t) -> ::ssize_t;
    pub fn pwrite(
        fd: ::c_int,
        buf: *const ::c_void,
        count: ::size_t,
        offset: off_t,
    ) -> ::ssize_t;
    #[link_name = "pwrite"]
    pub fn pwrite64(
        fd: ::c_int,
        buf: *const ::c_void,
        count: ::size_t,
        offset: off64_t,
    ) -> ::ssize_t;
    pub fn sbrk(increment: ::ptrdiff_t) -> *mut ::c_void;
    pub fn setgroups(ngroups: ::c_int, ptr: *const ::gid_t) -> ::c_int;
    pub fn sethostname(name: *const ::c_char, len: ::size_t) -> ::c_int;
    pub fn setregid(rgid: ::gid_t, egid: ::gid_t) -> ::c_int;
    pub fn setreuid(ruid: ::uid_t, euid: ::uid_t) -> ::c_int;
    pub fn vhangup() -> ::c_int;
    pub fn vfork() -> ::pid_t;
    pub fn ftruncate(fd: ::c_int, length: off_t) -> ::c_int;
    #[link_name = "ftruncate"]
    pub fn ftruncate64(fd: ::c_int, length: off64_t) -> ::c_int;
    pub fn sync();

    /* cygwin/if.h */

    pub fn if_nameindex() -> *mut if_nameindex;
    pub fn if_freenameindex(ptr: *mut if_nameindex);

    /* cygwin/time.h */

    pub fn timegm(tm: *const ::tm) -> ::time_t;
    #[link_name = "timegm"]
    pub fn timegm64(tm: *const ::tm) -> ::time64_t;

    /* time.h */

    pub fn clock_settime(clk_id: ::clockid_t, tp: *const ::timespec) -> ::c_int;
    pub fn clock_gettime(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_getres(clk_id: ::clockid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn clock_nanosleep(
        clk_id: ::clockid_t,
        flags: ::c_int,
        rqtp: *const ::timespec,
        rmtp: *mut ::timespec,
    ) -> ::c_int;
    pub fn clock_getcpuclockid(pid: ::pid_t, clk_id: *mut ::clockid_t) -> ::c_int;

    /* locale.h */

    pub fn newlocale(mask: ::c_int, locale: *const ::c_char, base: ::locale_t) -> ::locale_t;
    pub fn freelocale(loc: ::locale_t);
    pub fn duplocale(base: ::locale_t) -> ::locale_t;
    pub fn uselocale(loc: ::locale_t) -> ::locale_t;

    /* strings.h */

    pub fn explicit_bzero(s: *mut ::c_void, len: ::size_t);

    /* string.h */

    #[cfg_attr(link_name = "__xpg_strerror_r")]
    pub fn strerror_r(errnum: ::c_int, buf: *mut c_char, buflen: ::size_t) -> ::c_int;
    pub fn memmem(
        haystack: *const ::c_void,
        haystacklen: ::size_t,
        needle: *const ::c_void,
        needlelen: ::size_t,
    ) -> *mut ::c_void;
    pub fn memrchr(cx: *const ::c_void, c: ::c_int, n: ::size_t) -> *mut ::c_void;

    /* sys/mman.h */

    pub fn mmap(
        addr: *mut ::c_void,
        len: ::size_t,
        prot: ::c_int,
        flags: ::c_int,
        fd: ::c_int,
        offset: off_t,
    ) -> *mut ::c_void;
    #[link_name = "mmap"]
    pub fn mmap64(
        addr: *mut ::c_void,
        len: ::size_t,
        prot: ::c_int,
        flags: ::c_int,
        fd: ::c_int,
        offset: off64_t,
    ) -> *mut ::c_void;
    pub fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int;
    pub fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int;
    pub fn posix_madvise(addr: *mut ::c_void, len: ::size_t, advice: ::c_int) -> ::c_int;
    pub fn shm_open(name: *const c_char, oflag: ::c_int, mode: mode_t) -> ::c_int;
    pub fn shm_unlink(name: *const ::c_char) -> ::c_int;

    /* linux_compat/sys/errno.h */

    pub fn __errno() -> *mut ::c_int;

    /* semaphore.h */

    pub fn sem_init(sem: *mut sem_t, pshared: ::c_int, value: ::c_uint) -> ::c_int;
    pub fn sem_destroy(sem: *mut sem_t) -> ::c_int;
    pub fn sem_open(name: *const ::c_char, oflag: ::c_int, ...) -> *mut sem_t;
    pub fn sem_close(sem: *mut sem_t) -> ::c_int;
    pub fn sem_unlink(name: *const ::c_char) -> ::c_int;
    pub fn sem_timedwait(sem: *mut sem_t, abstime: *const ::timespec) -> ::c_int;
    pub fn sem_getvalue(sem: *mut sem_t, sval: *mut ::c_int) -> ::c_int;

    /* sys/vfs.h */

    pub fn statfs(path: *const ::c_char, buf: *mut statfs) -> ::c_int;
    #[link_name = "statfs"]
    pub fn statfs64(path: *const ::c_char, buf: *mut statfs64) -> ::c_int;
    pub fn fstatfs(fd: ::c_int, buf: *mut statfs) -> ::c_int;
    #[link_name = "fstatfs"]
    pub fn fstatfs64(fd: ::c_int, buf: *mut statfs64) -> ::c_int;

    /* sys/statvfs.h */

    pub fn statvfs(path: *const ::c_char, buf: *mut statvfs) -> ::c_int;
    #[link_name = "statvfs"]
    pub fn statvfs64(path: *const ::c_char, buf: *mut statvfs64) -> ::c_int;

    /* pwd.h */

    pub fn getpwuid_r(
        uid: ::uid_t,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
    pub fn getpwnam_r(
        name: *const ::c_char,
        pwd: *mut passwd,
        buf: *mut ::c_char,
        buflen: ::size_t,
        result: *mut *mut passwd,
    ) -> ::c_int;
    pub fn getpwent() -> *mut passwd;
    pub fn setpwent();
    pub fn endpwent();

    /* cygwin/signal.h */

    pub fn sigwait(set: *const sigset_t, sig: *mut ::c_int) -> ::c_int;
    pub fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> ::c_int;

    /* sys/signal.h */

    pub fn pthread_sigmask(how: ::c_int, set: *const sigset_t, oldset: *mut sigset_t) -> ::c_int;
    pub fn sigsuspend(mask: *const ::sigset_t) -> ::c_int;
    pub fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> ::c_int;
    pub fn pthread_kill(thread: ::pthread_t, sig: ::c_int) -> ::c_int;
    pub fn sigtimedwait(
        set: *const sigset_t,
        info: *mut siginfo_t,
        timeout: *const ::timespec,
    ) -> ::c_int;

    /* sys/ioctl.h */

    pub fn ioctl(fd: ::c_int, request: ::c_int, ...) -> ::c_int;

    /* sys/_default_fcntl.h */

    pub fn open(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    #[link_name = "open"]
    pub fn open64(path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn openat(fd: ::c_int, path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    #[link_name = "openat"]
    pub fn openat64(fd: ::c_int, path: *const c_char, oflag: ::c_int, ...) -> ::c_int;
    pub fn creat(path: *const c_char, mode: mode_t) -> ::c_int;
    #[link_name = "creat"]
    pub fn creat64(path: *const c_char, mode: mode_t) -> ::c_int;

    /* fcntl.h */

    pub fn posix_fadvise(fd: ::c_int, offset: ::off_t, len: ::off_t, advise: ::c_int) -> ::c_int;
    #[link_name = "posix_fadvise"]
    pub fn posix_fadvise64(
        fd: ::c_int,
        offset: ::off64_t,
        len: ::off64_t,
        advise: ::c_int,
    ) -> ::c_int;
    pub fn posix_fallocate(fd: ::c_int, offset: ::off_t, len: ::off_t) -> ::c_int;
    #[link_name = "posix_fallocate"]
    pub fn posix_fallocate64(fd: ::c_int, offset: ::off64_t, len: ::off64_t) -> ::c_int;

    /* dirent.h */

    pub fn dirfd(dirp: *mut ::DIR) -> ::c_int;
    pub fn readdir(dirp: *mut ::DIR) -> *mut ::dirent;
    #[link_name = "readdir"]
    pub fn readdir64(dirp: *mut ::DIR) -> *mut ::dirent64;
    pub fn readdir_r(
        dirp: *mut ::DIR,
        entry: *mut ::dirent,
        result: *mut *mut ::dirent,
    ) -> ::c_int;
    #[link_name = "readdir_r"]
    pub fn readdir64_r(
        dirp: *mut ::DIR,
        entry: *mut ::dirent64,
        result: *mut *mut ::dirent64,
    ) -> ::c_int;
    pub fn seekdir(dirp: *mut ::DIR, loc: ::c_long);
    pub fn telldir(dirp: *mut ::DIR) -> ::c_long;

    /* sys/random.h */

    pub fn getrandom(buf: *mut ::c_void, buflen: ::size_t, flags: ::c_uint) -> ::ssize_t;
    pub fn getentropy(buf: *mut ::c_void, buflen: ::size_t) -> ::c_int;

    /* spawn.h */

    pub fn posix_spawn(
        pid: *mut ::pid_t,
        path: *const ::c_char,
        file_actions: *const ::posix_spawn_file_actions_t,
        attrp: *const ::posix_spawnattr_t,
        argv: *const *mut ::c_char,
        envp: *const *mut ::c_char,
    ) -> ::c_int;
    pub fn posix_spawnp(
        pid: *mut ::pid_t,
        file: *const ::c_char,
        file_actions: *const ::posix_spawn_file_actions_t,
        attrp: *const ::posix_spawnattr_t,
        argv: *const *mut ::c_char,
        envp: *const *mut ::c_char,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_init(actions: *mut posix_spawn_file_actions_t) -> ::c_int;
    pub fn posix_spawn_file_actions_destroy(actions: *mut posix_spawn_file_actions_t) -> ::c_int;
    pub fn posix_spawn_file_actions_addopen(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
        path: *const ::c_char,
        oflag: ::c_int,
        mode: ::mode_t,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_adddup2(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
        newfd: ::c_int,
    ) -> ::c_int;
    pub fn posix_spawn_file_actions_addclose(
        actions: *mut posix_spawn_file_actions_t,
        fd: ::c_int,
    ) -> ::c_int;
    pub fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> ::c_int;
    pub fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> ::c_int;
    pub fn posix_spawnattr_getflags(
        attr: *const posix_spawnattr_t,
        flags: *mut ::c_short,
    ) -> ::c_int;
    pub fn posix_spawnattr_getpgroup(
        attr: *const posix_spawnattr_t,
        flags: *mut ::pid_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_getschedparam(
        attr: *const posix_spawnattr_t,
        param: *mut ::sched_param,
    ) -> ::c_int;
    pub fn posix_spawnattr_getschedpolicy(
        attr: *const posix_spawnattr_t,
        flags: *mut ::c_int,
    ) -> ::c_int;
    pub fn posix_spawnattr_getsigdefault(
        attr: *const posix_spawnattr_t,
        default: *mut ::sigset_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_getsigmask(
        attr: *const posix_spawnattr_t,
        default: *mut ::sigset_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_setflags(attr: *mut posix_spawnattr_t, flags: ::c_short) -> ::c_int;
    pub fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t, flags: ::pid_t) -> ::c_int;
    pub fn posix_spawnattr_setschedparam(
        attr: *mut posix_spawnattr_t,
        param: *const ::sched_param,
    ) -> ::c_int;
    pub fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t, flags: ::c_int) -> ::c_int;
    pub fn posix_spawnattr_setsigdefault(
        attr: *mut posix_spawnattr_t,
        default: *const ::sigset_t,
    ) -> ::c_int;
    pub fn posix_spawnattr_setsigmask(
        attr: *mut posix_spawnattr_t,
        default: *const ::sigset_t,
    ) -> ::c_int;

    /* sched.h */

    pub fn sched_setparam(pid: ::pid_t, param: *const ::sched_param) -> ::c_int;
    pub fn sched_getparam(pid: ::pid_t, param: *mut ::sched_param) -> ::c_int;
    pub fn sched_setscheduler(
        pid: ::pid_t,
        policy: ::c_int,
        param: *const ::sched_param,
    ) -> ::c_int;
    pub fn sched_getscheduler(pid: ::pid_t) -> ::c_int;
    pub fn sched_get_priority_max(policy: ::c_int) -> ::c_int;
    pub fn sched_get_priority_min(policy: ::c_int) -> ::c_int;
    pub fn sched_rr_get_interval(pid: ::pid_t, tp: *mut ::timespec) -> ::c_int;
    pub fn sched_getcpu() -> ::c_int;
    pub fn sched_getaffinity(pid: ::pid_t, cpusetsize: ::size_t, cpuset: *mut cpu_set_t)
                             -> ::c_int;
    pub fn sched_setaffinity(
        pid: ::pid_t,
        cpusetsize: ::size_t,
        cpuset: *const cpu_set_t,
    ) -> ::c_int;

    /* sys/time.h */

    pub fn futimes(
        fd: ::c_int,
        times: *const ::timeval
    ) -> ::c_int;
    pub fn lutimes(file: *const ::c_char, times: *const ::timeval) -> ::c_int;
    pub fn settimeofday(tv: *const ::timeval, tz: *const ::timezone) -> ::c_int;
    pub fn getitimer(which: ::c_int, curr_value: *mut ::itimerval) -> ::c_int;
    pub fn setitimer(
        which: ::c_int,
        new_value: *const ::itimerval,
        old_value: *mut ::itimerval,
    ) -> ::c_int;
    pub fn gettimeofday(tp: *mut ::timeval, tz: *mut ::c_void) -> ::c_int;
}

/* cygwin/socket.h */

const_fn! {
    {const} fn CMSG_ALIGN(len: usize) -> usize {
        len + ::mem::align_of::<cmsghdr>() - 1 & !(::mem::align_of::<cmsghdr>() - 1)
    }
}

f! {
    pub fn CMSG_LEN(length: ::c_uint) -> ::c_uint {
        CMSG_ALIGN(::mem::size_of::<cmsghdr>()) as ::c_uint + length
    }

    pub {const} fn CMSG_SPACE(length: ::c_uint) -> ::c_uint {
        (CMSG_ALIGN(length as usize) + CMSG_ALIGN(::mem::size_of::<cmsghdr>()))
            as ::c_uint
    }

    pub fn CMSG_FIRSTHDR(mhdr: *const msghdr) -> *mut cmsghdr {
        if (*mhdr).msg_controllen as usize >= ::mem::size_of::<cmsghdr>() {
            (*mhdr).msg_control as *mut cmsghdr
        } else {
            0 as *mut cmsghdr
        }
    }

    pub fn CMSG_DATA(cmsg: *const cmsghdr) -> *mut ::c_uchar {
        cmsg.offset(1) as *mut ::c_uchar
    }

    pub fn CMSG_NXTHDR(mhdr: *const msghdr,
                       cmsg: *const cmsghdr) -> *mut cmsghdr {
        if ((cmsg as usize) + CMSG_SPACE((*cmsg).cmsg_len as ::c_uint) as usize)
           > (((*mhdr).msg_control as usize) + ((*mhdr).msg_controllen as usize)) {
            0 as *mut cmsghdr
        } else {
            ((cmsg as usize) + CMSG_ALIGN((*cmsg).cmsg_len as usize)) as *mut cmsghdr
        }
    }

    /* sys/select.h */

    pub fn FD_CLR(fd: ::c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        (*set).fds_bits[fd / 64] &= !(1 << (fd % 64));
        return
    }

    pub fn FD_ISSET(fd: ::c_int, set: *mut fd_set) -> bool {
        let fd = fd as usize;
        return ((*set).fds_bits[fd / 64] & (1 << (fd % 64))) != 0
    }

    pub fn FD_SET(fd: ::c_int, set: *mut fd_set) -> () {
        let fd = fd as usize;
        (*set).fds_bits[fd / 64] |= 1 << (fd % 64);
        return
    }

    pub fn FD_ZERO(set: *mut fd_set) -> () {
        (*set).fds_bits[0] = 0;
        return
    }

    /* sys/cpuset.h */

    pub fn CPU_ALLOC_SIZE(count: ::c_int) -> ::size_t {
        let _dummy: cpu_set_t = ::mem::zeroed();
        let size_in_bits = 8 * ::mem::size_of_val(&_dummy.bits[0]);
        ((count as ::size_t + size_in_bits - 1) / 8) as ::size_t
    }

    pub fn CPU_COUNT_S(size: usize, cpuset: &cpu_set_t) -> ::c_int {
        let mut s: u32 = 0;
        let size_of_mask = ::mem::size_of_val(&cpuset.bits[0]);
        for i in cpuset.bits[..(size / size_of_mask)].iter() {
            s += i.count_ones();
        };
        s as ::c_int
    }

    pub fn CPU_ZERO(cpuset: &mut cpu_set_t) -> () {
        for slot in cpuset.bits.iter_mut() {
            *slot = 0;
        }
    }

    pub fn CPU_SET(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in_bits
            = 8 * ::mem::size_of_val(&cpuset.bits[0]); // 32, 64 etc
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        cpuset.bits[idx] |= 1 << offset;
        ()
    }

    pub fn CPU_CLR(cpu: usize, cpuset: &mut cpu_set_t) -> () {
        let size_in_bits
            = 8 * ::mem::size_of_val(&cpuset.bits[0]); // 32, 64 etc
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        cpuset.bits[idx] &= !(1 << offset);
        ()
    }

    pub fn CPU_ISSET(cpu: usize, cpuset: &cpu_set_t) -> bool {
        let size_in_bits = 8 * ::mem::size_of_val(&cpuset.bits[0]);
        let (idx, offset) = (cpu / size_in_bits, cpu % size_in_bits);
        0 != (cpuset.bits[idx] & (1 << offset))
    }

    pub fn CPU_COUNT(cpuset: &cpu_set_t) -> ::c_int {
        CPU_COUNT_S(::mem::size_of::<cpu_set_t>(), cpuset)
    }

    pub fn CPU_EQUAL(set1: &cpu_set_t, set2: &cpu_set_t) -> bool {
        set1.bits == set2.bits
    }
}

safe_f! {
    /* cygwin/wait.h */

    pub {const} fn WIFEXITED(status: ::c_int) -> bool {
        (status & 0xff) == 0
    }

    pub {const} fn WIFSIGNALED(status: ::c_int) -> bool {
        (status & 0x7f) > 0 && (status & 0x7f) < 0x7f
    }

    pub {const} fn WIFSTOPPED(status: ::c_int) -> bool {
        (status & 0xff) == 0x7f
    }

    pub {const} fn WIFCONTINUED(status: ::c_int) -> bool {
        (status & 0xffff) == 0xffff
    }

    pub {const} fn WEXITSTATUS(status: ::c_int) -> ::c_int {
        (status >> 8) & 0xff
    }

    pub {const} fn WTERMSIG(status: ::c_int) -> ::c_int {
        status & 0x7f
    }

    pub {const} fn WSTOPSIG(status: ::c_int) -> ::c_int {
        (status >> 8) & 0xff
    }

    pub {const} fn WCOREDUMP(status: ::c_int) -> bool {
        WIFSIGNALED(status) && status & 0x80 != 0
    }
}
