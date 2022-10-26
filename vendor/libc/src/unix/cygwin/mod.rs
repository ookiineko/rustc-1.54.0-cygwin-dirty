pub type c_char = i8;

/* netdb.h */

/* Error codes returned by getaddrinfo and getnameinfo. */
pub const EAI_SYSTEM: ::c_int = 11;                      /* System error */

/* sys/un.h */

s! {
    pub struct sockaddr_un {
        pub sun_family: ::sa_family_t,   /* address family AF_LOCAL/AF_UNIX */
        pub sun_path: [::c_char; 108],   /* 108 bytes of socket address     */
    }
}

/* sys/socket.h */

/* Supported address families. */
/*
 * Address families.
 */

s! {
    pub struct sockaddr_storage {
        pub ss_family: ::sa_family_t,
        pub ss_pad1: [::c_char; 6],
        pub ss_align: i64,
        pub ss_pad2: [::c_char; 112],
    }
}

pub const AF_UNIX: ::c_int = 1;                         /* local to host (pipes, portals) */

/* Flags we can use with send/ and recv. */
pub const MSG_PEEK: ::c_int = 0x0002;                   /* peek at incoming message */

/* dlfcn.h */

/* following doesn't exist in Win32 API .... */
pub const RTLD_DEFAULT: *mut ::c_void = 0i64 as *mut ::c_void;
