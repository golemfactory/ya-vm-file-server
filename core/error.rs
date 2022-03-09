//! 9P error representations.
//!
//! In 9P2000 errors are represented as strings.
//! All the error strings in this module are imported from include/net/9p/error.c of Linux kernel.
//!
//! By contrast, in 9P2000.L, errors are represented as numbers (errno).
//! Using the Linux system errno numbers is the expected behaviour.
use self::errno::*;
use std::io::ErrorKind::*;
use std::{fmt, io};

pub mod libc_linux {
    pub type CInt = isize;

    pub const EPERM: CInt = 1;
    pub const ENOENT: CInt = 2;
    pub const ESRCH: CInt = 3;
    pub const EINTR: CInt = 4;
    pub const EIO: CInt = 5;
    pub const ENXIO: CInt = 6;
    pub const E2BIG: CInt = 7;
    pub const ENOEXEC: CInt = 8;
    pub const EBADF: CInt = 9;
    pub const ECHILD: CInt = 10;
    pub const EAGAIN: CInt = 11;
    pub const ENOMEM: CInt = 12;
    pub const EACCES: CInt = 13;
    pub const EFAULT: CInt = 14;
    pub const ENOTBLK: CInt = 15;
    pub const EBUSY: CInt = 16;
    pub const EEXIST: CInt = 17;
    pub const EXDEV: CInt = 18;
    pub const ENODEV: CInt = 19;
    pub const ENOTDIR: CInt = 20;
    pub const EISDIR: CInt = 21;
    pub const EINVAL: CInt = 22;
    pub const ENFILE: CInt = 23;
    pub const EMFILE: CInt = 24;
    pub const ENOTTY: CInt = 25;
    pub const ETXTBSY: CInt = 26;
    pub const EFBIG: CInt = 27;
    pub const ENOSPC: CInt = 28;
    pub const ESPIPE: CInt = 29;
    pub const EROFS: CInt = 30;
    pub const EMLINK: CInt = 31;
    pub const EPIPE: CInt = 32;
    pub const EDOM: CInt = 33;
    pub const ERANGE: CInt = 34;
    pub const EDEADLK: CInt = 35;
    pub const ENAMETOOLONG: CInt = 36;
    pub const ENOLCK: CInt = 37;
    pub const ENOSYS: CInt = 38;
    pub const ENOTEMPTY: CInt = 39;
    pub const ELOOP: CInt = 40;
    pub const ENOMSG: CInt = 42;
    pub const EIDRM: CInt = 43;
    pub const ECHRNG: CInt = 44;
    pub const EL2NSYNC: CInt = 45;
    pub const EL3HLT: CInt = 46;
    pub const EL3RST: CInt = 47;
    pub const ELNRNG: CInt = 48;
    pub const EUNATCH: CInt = 49;
    pub const ENOCSI: CInt = 50;
    pub const EL2HLT: CInt = 51;
    pub const EBADE: CInt = 52;
    pub const EBADR: CInt = 53;
    pub const EXFULL: CInt = 54;
    pub const ENOANO: CInt = 55;
    pub const EBADRQC: CInt = 56;
    pub const EBADSLT: CInt = 57;
    pub const EBFONT: CInt = 59;
    pub const ENOSTR: CInt = 60;
    pub const ENODATA: CInt = 61;
    pub const ETIME: CInt = 62;
    pub const ENOSR: CInt = 63;
    pub const ENONET: CInt = 64;
    pub const ENOPKG: CInt = 65;
    pub const EREMOTE: CInt = 66;
    pub const ENOLINK: CInt = 67;
    pub const EADV: CInt = 68;
    pub const ESRMNT: CInt = 69;
    pub const ECOMM: CInt = 70;
    pub const EPROTO: CInt = 71;
    pub const EMULTIHOP: CInt = 72;
    pub const EDOTDOT: CInt = 73;
    pub const EDEADLOCK: CInt = EDEADLK;
    pub const EOVERFLOW: CInt = 75;
    pub const ENOTUNIQ: CInt = 76;
    pub const EBADFD: CInt = 77;
    pub const EBADMSG: CInt = 74;
    pub const EREMCHG: CInt = 78;
    pub const ELIBACC: CInt = 79;
    pub const ELIBBAD: CInt = 80;
    pub const ELIBSCN: CInt = 81;
    pub const ELIBMAX: CInt = 82;
    pub const ELIBEXEC: CInt = 83;
    pub const EILSEQ: CInt = 84;
    pub const ERESTART: CInt = 85;
    pub const ESTRPIPE: CInt = 86;
    pub const EUSERS: CInt = 87;
    pub const ENOTSOCK: CInt = 88;
    pub const EDESTADDRREQ: CInt = 89;
    pub const EMSGSIZE: CInt = 90;
    pub const EPROTOTYPE: CInt = 91;
    pub const ENOPROTOOPT: CInt = 92;
    pub const EPROTONOSUPPORT: CInt = 93;
    pub const ESOCKTNOSUPPORT: CInt = 94;
    pub const EOPNOTSUPP: CInt = 95;
    pub const EPFNOSUPPORT: CInt = 96;
    pub const EAFNOSUPPORT: CInt = 97;
    pub const EADDRINUSE: CInt = 98;
    pub const EADDRNOTAVAIL: CInt = 99;
    pub const ENETDOWN: CInt = 100;
    pub const ENETUNREACH: CInt = 101;
    pub const ENETRESET: CInt = 102;
    pub const ECONNABORTED: CInt = 103;
    pub const ECONNRESET: CInt = 104;
    pub const ENOBUFS: CInt = 105;
    pub const EISCONN: CInt = 106;
    pub const ENOTCONN: CInt = 107;
    pub const ESHUTDOWN: CInt = 108;
    pub const ETOOMANYREFS: CInt = 109;
    pub const ETIMEDOUT: CInt = 110;
    pub const ECONNREFUSED: CInt = 111;
    pub const EHOSTDOWN: CInt = 112;
    pub const EHOSTUNREACH: CInt = 113;
    pub const EALREADY: CInt = 114;
    pub const EINPROGRESS: CInt = 115;
    pub const ESTALE: CInt = 116;
    pub const EUCLEAN: CInt = 117;
    pub const ENOTNAM: CInt = 118;
    pub const ENAVAIL: CInt = 119;
    pub const EISNAM: CInt = 120;
    pub const EREMOTEIO: CInt = 121;
    pub const EDQUOT: CInt = 122;
    pub const ENOMEDIUM: CInt = 123;
    pub const EMEDIUMTYPE: CInt = 124;
    pub const ECANCELED: CInt = 125;
    pub const ENOKEY: CInt = 126;
    pub const EKEYEXPIRED: CInt = 127;
    pub const EKEYREVOKED: CInt = 128;
    pub const EKEYREJECTED: CInt = 129;
    pub const EOWNERDEAD: CInt = 130;
    pub const ENOTRECOVERABLE: CInt = 131;
    pub const EHWPOISON: CInt = 133;
    pub const ERFKILL: CInt = 132;
}

/// The system errno definitions.
///
/// # Protocol
/// 9P2000.L

pub mod errno {
    use crate::core::error::libc_linux;

    #[derive(Debug, Copy, Clone)]
    pub enum Errno {
        UnknownErrno = 0,
        EPERM = libc_linux::EPERM,
        ENOENT = libc_linux::ENOENT,
        ESRCH = libc_linux::ESRCH,
        EINTR = libc_linux::EINTR,
        EIO = libc_linux::EIO,
        ENXIO = libc_linux::ENXIO,
        E2BIG = libc_linux::E2BIG,
        ENOEXEC = libc_linux::ENOEXEC,
        EBADF = libc_linux::EBADF,
        ECHILD = libc_linux::ECHILD,
        EAGAIN = libc_linux::EAGAIN,
        ENOMEM = libc_linux::ENOMEM,
        EACCES = libc_linux::EACCES,
        EFAULT = libc_linux::EFAULT,
        ENOTBLK = libc_linux::ENOTBLK,
        EBUSY = libc_linux::EBUSY,
        EEXIST = libc_linux::EEXIST,
        EXDEV = libc_linux::EXDEV,
        ENODEV = libc_linux::ENODEV,
        ENOTDIR = libc_linux::ENOTDIR,
        EISDIR = libc_linux::EISDIR,
        EINVAL = libc_linux::EINVAL,
        ENFILE = libc_linux::ENFILE,
        EMFILE = libc_linux::EMFILE,
        ENOTTY = libc_linux::ENOTTY,
        ETXTBSY = libc_linux::ETXTBSY,
        EFBIG = libc_linux::EFBIG,
        ENOSPC = libc_linux::ENOSPC,
        ESPIPE = libc_linux::ESPIPE,
        EROFS = libc_linux::EROFS,
        EMLINK = libc_linux::EMLINK,
        EPIPE = libc_linux::EPIPE,
        EDOM = libc_linux::EDOM,
        ERANGE = libc_linux::ERANGE,
        EDEADLK = libc_linux::EDEADLK,
        ENAMETOOLONG = libc_linux::ENAMETOOLONG,
        ENOLCK = libc_linux::ENOLCK,
        ENOSYS = libc_linux::ENOSYS,
        ENOTEMPTY = libc_linux::ENOTEMPTY,
        ELOOP = libc_linux::ELOOP,
        ENOMSG = libc_linux::ENOMSG,
        EIDRM = libc_linux::EIDRM,
        ECHRNG = libc_linux::ECHRNG,
        EL2NSYNC = libc_linux::EL2NSYNC,
        EL3HLT = libc_linux::EL3HLT,
        EL3RST = libc_linux::EL3RST,
        ELNRNG = libc_linux::ELNRNG,
        EUNATCH = libc_linux::EUNATCH,
        ENOCSI = libc_linux::ENOCSI,
        EL2HLT = libc_linux::EL2HLT,
        EBADE = libc_linux::EBADE,
        EBADR = libc_linux::EBADR,
        EXFULL = libc_linux::EXFULL,
        ENOANO = libc_linux::ENOANO,
        EBADRQC = libc_linux::EBADRQC,
        EBADSLT = libc_linux::EBADSLT,
        EBFONT = libc_linux::EBFONT,
        ENOSTR = libc_linux::ENOSTR,
        ENODATA = libc_linux::ENODATA,
        ETIME = libc_linux::ETIME,
        ENOSR = libc_linux::ENOSR,
        ENONET = libc_linux::ENONET,
        ENOPKG = libc_linux::ENOPKG,
        EREMOTE = libc_linux::EREMOTE,
        ENOLINK = libc_linux::ENOLINK,
        EADV = libc_linux::EADV,
        ESRMNT = libc_linux::ESRMNT,
        ECOMM = libc_linux::ECOMM,
        EPROTO = libc_linux::EPROTO,
        EMULTIHOP = libc_linux::EMULTIHOP,
        EDOTDOT = libc_linux::EDOTDOT,
        EBADMSG = libc_linux::EBADMSG,
        EOVERFLOW = libc_linux::EOVERFLOW,
        ENOTUNIQ = libc_linux::ENOTUNIQ,
        EBADFD = libc_linux::EBADFD,
        EREMCHG = libc_linux::EREMCHG,
        ELIBACC = libc_linux::ELIBACC,
        ELIBBAD = libc_linux::ELIBBAD,
        ELIBSCN = libc_linux::ELIBSCN,
        ELIBMAX = libc_linux::ELIBMAX,
        ELIBEXEC = libc_linux::ELIBEXEC,
        EILSEQ = libc_linux::EILSEQ,
        ERESTART = libc_linux::ERESTART,
        ESTRPIPE = libc_linux::ESTRPIPE,
        EUSERS = libc_linux::EUSERS,
        ENOTSOCK = libc_linux::ENOTSOCK,
        EDESTADDRREQ = libc_linux::EDESTADDRREQ,
        EMSGSIZE = libc_linux::EMSGSIZE,
        EPROTOTYPE = libc_linux::EPROTOTYPE,
        ENOPROTOOPT = libc_linux::ENOPROTOOPT,
        EPROTONOSUPPORT = libc_linux::EPROTONOSUPPORT,
        ESOCKTNOSUPPORT = libc_linux::ESOCKTNOSUPPORT,
        EOPNOTSUPP = libc_linux::EOPNOTSUPP,
        EPFNOSUPPORT = libc_linux::EPFNOSUPPORT,
        EAFNOSUPPORT = libc_linux::EAFNOSUPPORT,
        EADDRINUSE = libc_linux::EADDRINUSE,
        EADDRNOTAVAIL = libc_linux::EADDRNOTAVAIL,
        ENETDOWN = libc_linux::ENETDOWN,
        ENETUNREACH = libc_linux::ENETUNREACH,
        ENETRESET = libc_linux::ENETRESET,
        ECONNABORTED = libc_linux::ECONNABORTED,
        ECONNRESET = libc_linux::ECONNRESET,
        ENOBUFS = libc_linux::ENOBUFS,
        EISCONN = libc_linux::EISCONN,
        ENOTCONN = libc_linux::ENOTCONN,
        ESHUTDOWN = libc_linux::ESHUTDOWN,
        ETOOMANYREFS = libc_linux::ETOOMANYREFS,
        ETIMEDOUT = libc_linux::ETIMEDOUT,
        ECONNREFUSED = libc_linux::ECONNREFUSED,
        EHOSTDOWN = libc_linux::EHOSTDOWN,
        EHOSTUNREACH = libc_linux::EHOSTUNREACH,
        EALREADY = libc_linux::EALREADY,
        EINPROGRESS = libc_linux::EINPROGRESS,
        ESTALE = libc_linux::ESTALE,
        EUCLEAN = libc_linux::EUCLEAN,
        ENOTNAM = libc_linux::ENOTNAM,
        ENAVAIL = libc_linux::ENAVAIL,
        EISNAM = libc_linux::EISNAM,
        EREMOTEIO = libc_linux::EREMOTEIO,
        EDQUOT = libc_linux::EDQUOT,
        ENOMEDIUM = libc_linux::ENOMEDIUM,
        EMEDIUMTYPE = libc_linux::EMEDIUMTYPE,
        ECANCELED = libc_linux::ECANCELED,
        ENOKEY = libc_linux::ENOKEY,
        EKEYEXPIRED = libc_linux::EKEYEXPIRED,
        EKEYREVOKED = libc_linux::EKEYREVOKED,
        EKEYREJECTED = libc_linux::EKEYREJECTED,
        EOWNERDEAD = libc_linux::EOWNERDEAD,
        ENOTRECOVERABLE = libc_linux::ENOTRECOVERABLE,
        #[cfg(not(any(target_os = "android", target_arch = "mips")))]
        ERFKILL = libc_linux::ERFKILL,
        #[cfg(not(any(target_os = "android", target_arch = "mips")))]
        EHWPOISON = libc_linux::EHWPOISON,
    }

    pub use Errno::*;
}

fn desc(errno: &Errno) -> &'static str {
    match errno {
        UnknownErrno => "Unknown errno",
        EPERM => "Operation not permitted",
        ENOENT => "No such file or directory",
        ESRCH => "No such process",
        EINTR => "Interrupted system call",
        EIO => "I/O error",
        ENXIO => "No such device or address",
        E2BIG => "Argument list too long",
        ENOEXEC => "Exec format error",
        EBADF => "Bad file number",
        ECHILD => "No child processes",
        EAGAIN => "Try again",
        ENOMEM => "Out of memory",
        EACCES => "Permission denied",
        EFAULT => "Bad address",
        ENOTBLK => "Block device required",
        EBUSY => "Device or resource busy",
        EEXIST => "File exists",
        EXDEV => "Cross-device link",
        ENODEV => "No such device",
        ENOTDIR => "Not a directory",
        EISDIR => "Is a directory",
        EINVAL => "Invalid argument",
        ENFILE => "File table overflow",
        EMFILE => "Too many open files",
        ENOTTY => "Not a typewriter",
        ETXTBSY => "Text file busy",
        EFBIG => "File too large",
        ENOSPC => "No space left on device",
        ESPIPE => "Illegal seek",
        EROFS => "Read-only file system",
        EMLINK => "Too many links",
        EPIPE => "Broken pipe",
        EDOM => "Math argument out of domain of func",
        ERANGE => "Math result not representable",
        EDEADLK => "Resource deadlock would occur",
        ENAMETOOLONG => "File name too long",
        ENOLCK => "No record locks available",
        ENOSYS => "Function not implemented",
        ENOTEMPTY => "Directory not empty",
        ELOOP => "Too many symbolic links encountered",
        ENOMSG => "No message of desired type",
        EIDRM => "Identifier removed",
        EINPROGRESS => "Operation now in progress",
        EALREADY => "Operation already in progress",
        ENOTSOCK => "Socket operation on non-socket",
        EDESTADDRREQ => "Destination address required",
        EMSGSIZE => "Message too long",
        EPROTOTYPE => "Protocol wrong type for socket",
        ENOPROTOOPT => "Protocol not available",
        EPROTONOSUPPORT => "Protocol not supported",
        ESOCKTNOSUPPORT => "Socket type not supported",
        EPFNOSUPPORT => "Protocol family not supported",
        EAFNOSUPPORT => "Address family not supported by protocol",
        EADDRINUSE => "Address already in use",
        EADDRNOTAVAIL => "Cannot assign requested address",
        ENETDOWN => "Network is down",
        ENETUNREACH => "Network is unreachable",
        ENETRESET => "Network dropped connection because of reset",
        ECONNABORTED => "Software caused connection abort",
        ECONNRESET => "Connection reset by peer",
        ENOBUFS => "No buffer space available",
        EISCONN => "Transport endpoint is already connected",
        ENOTCONN => "Transport endpoint is not connected",
        ESHUTDOWN => "Cannot send after transport endpoint shutdown",
        ETOOMANYREFS => "Too many references: cannot splice",
        ETIMEDOUT => "Connection timed out",
        ECONNREFUSED => "Connection refused",
        EHOSTDOWN => "Host is down",
        EHOSTUNREACH => "No route to host",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ECHRNG => "Channel number out of range",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EL2NSYNC => "Level 2 not synchronized",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EL3HLT => "Level 3 halted",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EL3RST => "Level 3 reset",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ELNRNG => "Link number out of range",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EUNATCH => "Protocol driver not attached",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOCSI => "No CSI structure available",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EL2HLT => "Level 2 halted",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EBADE => "Invalid exchange",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EBADR => "Invalid request descriptor",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EXFULL => "Exchange full",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOANO => "No anode",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EBADRQC => "Invalid request code",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EBADSLT => "Invalid slot",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EBFONT => "Bad font file format",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOSTR => "Device not a stream",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENODATA => "No data available",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ETIME => "Timer expired",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOSR => "Out of streams resources",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENONET => "Machine is not on the network",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOPKG => "Package not installed",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EREMOTE => "Object is remote",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOLINK => "Link has been severed",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EADV => "Advertise error",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ESRMNT => "Srmount error",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ECOMM => "Communication error on send",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EPROTO => "Protocol error",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EMULTIHOP => "Multihop attempted",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EDOTDOT => "RFS specific error",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EBADMSG => "Not a data message",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EOVERFLOW => "Value too large for defined data type",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOTUNIQ => "Name not unique on network",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EBADFD => "File descriptor in bad state",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EREMCHG => "Remote address changed",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ELIBACC => "Can not acces a needed shared core",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ELIBBAD => "Accessing a corrupted shared core",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ELIBSCN => ".lib section in a.out corrupted",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ELIBMAX => "Attempting to link in too many shared libraries",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ELIBEXEC => "Cannot exec a shared core directly",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "openbsd"))]
        EILSEQ => "Illegal byte sequence",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ERESTART => "Interrupted system call should be restarted",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ESTRPIPE => "Streams pipe error",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EUSERS => "Too many users",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "netbsd"))]
        EOPNOTSUPP => "Operation not supported on transport endpoint",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ESTALE => "Stale file handle",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EUCLEAN => "Structure needs cleaning",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOTNAM => "Not a XENIX named type file",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENAVAIL => "No XENIX semaphores available",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EISNAM => "Is a named type file",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EREMOTEIO => "Remote I/O error",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EDQUOT => "Quota exceeded",

        #[cfg(any(
            target_os = "linux",
            target_os = "android",
            target_os = "openbsd",
            target_os = "dragonfly"
        ))]
        ENOMEDIUM => "No medium found",

        #[cfg(any(target_os = "linux", target_os = "android", target_os = "openbsd"))]
        EMEDIUMTYPE => "Wrong medium type",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ECANCELED => "Operation canceled",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOKEY => "Required key not available",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EKEYEXPIRED => "Key has expired",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EKEYREVOKED => "Key has been revoked",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EKEYREJECTED => "Key was rejected by service",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        EOWNERDEAD => "Owner died",

        #[cfg(any(target_os = "linux", target_os = "android"))]
        ENOTRECOVERABLE => "State not recoverable",

        #[cfg(all(target_os = "linux", not(target_arch = "mips")))]
        ERFKILL => "Operation not possible due to RF-kill",

        #[cfg(all(target_os = "linux", not(target_arch = "mips")))]
        EHWPOISON => "Memory page has hardware error",

        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
        EDOOFUS => "Programming error",

        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
        EMULTIHOP => "Multihop attempted",

        #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
        ENOLINK => "Link has been severed",

        #[cfg(target_os = "freebsd")]
        ENOTCAPABLE => "Capabilities insufficient",

        #[cfg(target_os = "freebsd")]
        ECAPMODE => "Not permitted in capability mode",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        ENEEDAUTH => "Need authenticator",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EOVERFLOW => "Value too large to be stored in data type",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "netbsd"
        ))]
        EILSEQ => "Illegal byte sequence",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        ENOATTR => "Attribute not found",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "netbsd"
        ))]
        EBADMSG => "Bad message",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "netbsd"
        ))]
        EPROTO => "Protocol error",

        #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "ios"))]
        ENOTRECOVERABLE => "State not recoverable",

        #[cfg(any(target_os = "macos", target_os = "freebsd", target_os = "ios"))]
        EOWNERDEAD => "Previous owner died",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        ENOTSUP => "Operation not supported",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EPROCLIM => "Too many processes",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EUSERS => "Too many users",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EDQUOT => "Disc quota exceeded",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        ESTALE => "Stale NFS file handle",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EREMOTE => "Too many levels of remote in path",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EBADRPC => "RPC struct is bad",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        ERPCMISMATCH => "RPC version wrong",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EPROGUNAVAIL => "RPC prog. not avail",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EPROGMISMATCH => "Program version wrong",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EPROCUNAVAIL => "Bad procedure for program",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EFTYPE => "Inappropriate file type or format",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        EAUTH => "Authentication error",

        #[cfg(any(
            target_os = "macos",
            target_os = "freebsd",
            target_os = "dragonfly",
            target_os = "ios",
            target_os = "openbsd",
            target_os = "netbsd"
        ))]
        ECANCELED => "Operation canceled",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EPWROFF => "Device power is off",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EDEVERR => "Device error, e.g. paper out",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EBADEXEC => "Bad executable",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EBADARCH => "Bad CPU type in executable",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        ESHLIBVERS => "Shared core version mismatch",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EBADMACHO => "Malformed Macho file",

        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "netbsd"))]
        EMULTIHOP => "Reserved",

        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "netbsd"))]
        ENODATA => "No message available on STREAM",

        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "netbsd"))]
        ENOLINK => "Reserved",

        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "netbsd"))]
        ENOSR => "No STREAM resources",

        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "netbsd"))]
        ENOSTR => "Not a STREAM",

        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "netbsd"))]
        ETIME => "STREAM ioctl timeout",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EOPNOTSUPP => "Operation not supported on socket",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        ENOPOLICY => "No such policy registered",

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        EQFULL => "Interface output queue is full",

        #[cfg(target_os = "openbsd")]
        EOPNOTSUPP => "Operation not supported",

        #[cfg(target_os = "openbsd")]
        EIPSEC => "IPsec processing failure",

        #[cfg(target_os = "dragonfly")]
        EUNUSED94 | EUNUSED95 | EUNUSED96 | EUNUSED97 | EUNUSED98 => "Unused",

        #[cfg(target_os = "dragonfly")]
        EASYNC => "Async",

        _ => "No description for error",
    }
}

fn errno_from_io_error(e: &io::Error) -> errno::Errno {
    match e.kind() {
        NotFound => ENOENT,
        PermissionDenied => EPERM,
        ConnectionRefused => ECONNREFUSED,
        ConnectionReset => ECONNRESET,
        ConnectionAborted => ECONNABORTED,
        NotConnected => ENOTCONN,
        AddrInUse => EADDRINUSE,
        AddrNotAvailable => EADDRNOTAVAIL,
        BrokenPipe => EPIPE,
        AlreadyExists => EALREADY,
        WouldBlock => EAGAIN,
        InvalidInput => EINVAL,
        InvalidData => EINVAL,
        TimedOut => ETIMEDOUT,
        WriteZero => EAGAIN,
        Interrupted => EINTR,
        Other | _ => EIO,
    }
}

/// 9P error type which is convertible to an errno.
///
/// The value of `Error::errno()` will be used for Rlerror.
///
/// # Protocol
/// 9P2000.L
#[derive(Debug)]
pub enum Error {
    /// System error containing an errno.
    No(errno::Errno),
    /// I/O error.
    Io(io::Error),
}

impl Error {
    /// Get an errno representations.
    pub fn errno(&self) -> errno::Errno {
        match *self {
            Error::No(ref e) => *e,
            Error::Io(ref e) => errno_from_io_error(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::No(ref e) => write!(f, "System error: {}", desc(e)),
            Error::Io(ref e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match *self {
            Error::No(_) => None,
            Error::Io(ref e) => Some(e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::Io(e)
    }
}

impl<'a> From<&'a io::Error> for Error {
    fn from(e: &'a io::Error) -> Self {
        Error::No(errno_from_io_error(e))
    }
}

impl From<errno::Errno> for Error {
    fn from(e: errno::Errno) -> Self {
        Error::No(e)
    }
}
/*
implementation From<Error> for Error {
    fn from(e: nix::Error) -> Self {
        Error::No(e.as_errno().unwrap_or(UnknownErrno))
    }
}*/

/// 9P error strings imported from Linux.
///
/// # Protocol
/// 9P2000
///
pub mod string {
    pub const EPERM: &str = "Operation not permitted";
    pub const EPERM_WSTAT: &str = "wstat prohibited";
    pub const ENOENT: &str = "No such file or directory";
    pub const ENOENT_DIR: &str = "directory entry not found";
    pub const ENOENT_FILE: &str = "file not found";
    pub const EINTR: &str = "Interrupted system call";
    pub const EIO: &str = "Input/output error";
    pub const ENXIO: &str = "No such device or address";
    pub const E2BIG: &str = "Argument list too long";
    pub const EBADF: &str = "Bad file descriptor";
    pub const EAGAIN: &str = "Resource temporarily unavailable";
    pub const ENOMEM: &str = "Cannot allocate memory";
    pub const EACCES: &str = "Permission denied";
    pub const EFAULT: &str = "Bad address";
    pub const ENOTBLK: &str = "Block device required";
    pub const EBUSY: &str = "Device or resource busy";
    pub const EEXIST: &str = "File exists";
    pub const EXDEV: &str = "Invalid cross-device link";
    pub const ENODEV: &str = "No such device";
    pub const ENOTDIR: &str = "Not a directory";
    pub const EISDIR: &str = "Is a directory";
    pub const EINVAL: &str = "Invalid argument";
    pub const ENFILE: &str = "Too many open files in system";
    pub const EMFILE: &str = "Too many open files";
    pub const ETXTBSY: &str = "Text file busy";
    pub const EFBIG: &str = "File too large";
    pub const ENOSPC: &str = "No space left on device";
    pub const ESPIPE: &str = "Illegal seek";
    pub const EROFS: &str = "Read-only file system";
    pub const EMLINK: &str = "Too many links";
    pub const EPIPE: &str = "Broken pipe";
    pub const EDOM: &str = "Numerical argument out of domain";
    pub const ERANGE: &str = "Numerical result out of range";
    pub const EDEADLK: &str = "Resource deadlock avoided";
    pub const ENAMETOOLONG: &str = "File name too long";
    pub const ENOLCK: &str = "No locks available";
    pub const ENOSYS: &str = "Function not implemented";
    pub const ENOTEMPTY: &str = "Directory not empty";
    pub const ELOOP: &str = "Too many levels of symbolic links";
    pub const ENOMSG: &str = "No message of desired type";
    pub const EIDRM: &str = "Identifier removed";
    pub const ENODATA: &str = "No data available";
    pub const ENONET: &str = "Machine is not on the network";
    pub const ENOPKG: &str = "Package not installed";
    pub const EREMOTE: &str = "Object is remote";
    pub const ENOLINK: &str = "Link has been severed";
    pub const ECOMM: &str = "Communication error on send";
    pub const EPROTO: &str = "Protocol error";
    pub const EBADMSG: &str = "Bad message";
    pub const EBADFD: &str = "File descriptor in bad state";
    pub const ESTRPIPE: &str = "Streams pipe error";
    pub const EUSERS: &str = "Too many users";
    pub const ENOTSOCK: &str = "Socket operation on non-socket";
    pub const EMSGSIZE: &str = "Message too long";
    pub const ENOPROTOOPT: &str = "Protocol not available";
    pub const EPROTONOSUPPORT: &str = "Protocol not supported";
    pub const ESOCKTNOSUPPORT: &str = "Socket type not supported";
    pub const EOPNOTSUPP: &str = "Operation not supported";
    pub const EPFNOSUPPORT: &str = "Protocol family not supported";
    pub const ENETDOWN: &str = "Network is down";
    pub const ENETUNREACH: &str = "Network is unreachable";
    pub const ENETRESET: &str = "Network dropped connection on reset";
    pub const ECONNABORTED: &str = "Software caused connection abort";
    pub const ECONNRESET: &str = "Connection reset by peer";
    pub const ENOBUFS: &str = "No buffer space available";
    pub const EISCONN: &str = "Transport endpoint is already connected";
    pub const ENOTCONN: &str = "Transport endpoint is not connected";
    pub const ESHUTDOWN: &str = "Cannot send after transport endpoint shutdown";
    pub const ETIMEDOUT: &str = "Connection timed out";
    pub const ECONNREFUSED: &str = "Connection refused";
    pub const EHOSTDOWN: &str = "Host is down";
    pub const EHOSTUNREACH: &str = "No route to host";
    pub const EALREADY: &str = "Operation already in progress";
    pub const EINPROGRESS: &str = "Operation now in progress";
    pub const EISNAM: &str = "Is a named type file";
    pub const EREMOTEIO: &str = "Remote I/O error";
    pub const EDQUOT: &str = "Disk quota exceeded";
    pub const EBADF2: &str = "fid unknown or out of range";
    pub const EACCES2: &str = "permission denied";
    pub const ENOENT_FILE2: &str = "file does not exist";
    pub const ECONNREFUSED2: &str = "authentication failed";
    pub const ESPIPE2: &str = "bad offset in directory read";
    pub const EBADF3: &str = "bad use of fid";
    pub const EPERM_CONV: &str = "wstat can't convert between files and directories";
    pub const ENOTEMPTY2: &str = "directory is not empty";
    pub const EEXIST2: &str = "file exists";
    pub const EEXIST3: &str = "file already exists";
    pub const EEXIST4: &str = "file or directory already exists";
    pub const EBADF4: &str = "fid already in use";
    pub const ETXTBSY2: &str = "file in use";
    pub const EIO2: &str = "i/o error";
    pub const ETXTBSY3: &str = "file already open for I/O";
    pub const EINVAL2: &str = "illegal mode";
    pub const ENAMETOOLONG2: &str = "illegal name";
    pub const ENOTDIR2: &str = "not a directory";
    pub const EPERM_GRP: &str = "not a member of proposed group";
    pub const EACCES3: &str = "not owner";
    pub const EACCES4: &str = "only owner can change group in wstat";
    pub const EROFS2: &str = "read only file system";
    pub const EPERM_SPFILE: &str = "no access to special file";
    pub const EIO3: &str = "i/o count too large";
    pub const EINVAL3: &str = "unknown group";
    pub const EINVAL4: &str = "unknown user";
    pub const EPROTO2: &str = "bogus wstat buffer";
    pub const EAGAIN2: &str = "exclusive use file already open";
    pub const EIO4: &str = "corrupted directory entry";
    pub const EIO5: &str = "corrupted file entry";
    pub const EIO6: &str = "corrupted block label";
    pub const EIO7: &str = "corrupted meta data";
    pub const EINVAL5: &str = "illegal offset";
    pub const ENOENT_PATH: &str = "illegal path element";
    pub const EIO8: &str = "root of file system is corrupted";
    pub const EIO9: &str = "corrupted super block";
    pub const EPROTO3: &str = "protocol botch";
    pub const ENOSPC2: &str = "file system is full";
    pub const EAGAIN3: &str = "file is in use";
    pub const ENOENT_ALLOC: &str = "directory entry is not allocated";
    pub const EROFS3: &str = "file is read only";
    pub const EIDRM2: &str = "file has been removed";
    pub const EPERM_TRUNCATE: &str = "only support truncation to zero length";
    pub const EPERM_RMROOT: &str = "cannot remove root";
    pub const EFBIG2: &str = "file too big";
    pub const EIO10: &str = "venti i/o error";
}
