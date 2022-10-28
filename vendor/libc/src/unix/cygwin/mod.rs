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
pub const RAND_MAX: ::c_int = 0x7fffffff;
pub const FILENAME_MAX: ::c_uint = 4096;

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
pub const S_ISUID: ::mode_t = 0004000;
pub const S_ISGID: ::mode_t = 0002000;
pub const S_ISVTX: ::mode_t = 0001000;
pub const S_IREAD: ::mode_t = 0000400;
pub const S_IWRITE: ::mode_t = 0000200;
pub const S_IEXEC: ::mode_t = 0000100;
pub const S_ENFMT: ::mode_t = 0002000;
pub const S_IFMT: ::mode_t = 0170000;
pub const S_IFDIR: ::mode_t = 0040000;
pub const S_IFCHR: ::mode_t = 0020000;
pub const S_IFBLK: ::mode_t = 0060000;
pub const S_IFREG: ::mode_t = 0100000;
pub const S_IFLNK: ::mode_t = 0120000;
pub const S_IFSOCK: ::mode_t = 0140000;
pub const S_IFIFO: ::mode_t = 0010000;
pub const S_IRWXU: ::mode_t = S_IRUSR | S_IWUSR | S_IXUSR;
pub const S_IRUSR: ::mode_t = 0000400;
pub const S_IWUSR: ::mode_t = 0000200;
pub const S_IXUSR: ::mode_t = 0000100;
pub const S_IRWXG: ::mode_t = S_IRGRP | S_IWGRP | S_IXGRP;
pub const S_IRGRP: ::mode_t = 0000040;
pub const S_IWGRP: ::mode_t = 0000020;
pub const S_IXGRP: ::mode_t = 0000010;
pub const S_IRWXO: ::mode_t = S_IROTH | S_IWOTH | S_IXOTH;
pub const S_IROTH: ::mode_t = 0000004;
pub const S_IWOTH: ::mode_t = 0000002;
pub const S_IXOTH: ::mode_t = 0000001;

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

pub const FIONREAD: ::c_ulong = 0x4008667f;
pub const FIONBIO: ::c_ulong = 0x8004667e;
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

    pub struct sockaddr_un {
        pub sun_family: sa_family_t,
        pub sun_path: [::c_char; 108]
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

    pub struct in6_pktinfo {
        pub ipi6_addr: ::in6_addr,
        pub ipi6_ifindex: u32,
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

    /* linux_compat/sys/sendfile.h */

    pub fn sendfile(
        out_fd: ::c_int,
        in_fd: ::c_int,
        offset: *mut off_t,
        count: ::size_t,
    ) -> ::ssize_t;
}
