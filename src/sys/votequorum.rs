/* automatically generated by rust-bindgen 0.57.0 */

pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
extern "C" {
    pub fn __errno_location() -> *mut ::std::os::raw::c_int;
}
pub type clock_t = __clock_t;
pub type time_t = __time_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tm {
    pub tm_sec: ::std::os::raw::c_int,
    pub tm_min: ::std::os::raw::c_int,
    pub tm_hour: ::std::os::raw::c_int,
    pub tm_mday: ::std::os::raw::c_int,
    pub tm_mon: ::std::os::raw::c_int,
    pub tm_year: ::std::os::raw::c_int,
    pub tm_wday: ::std::os::raw::c_int,
    pub tm_yday: ::std::os::raw::c_int,
    pub tm_isdst: ::std::os::raw::c_int,
    pub tm_gmtoff: ::std::os::raw::c_long,
    pub tm_zone: *const ::std::os::raw::c_char,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
pub type timer_t = __timer_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigevent {
    _unused: [u8; 0],
}
pub type pid_t = __pid_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn clock() -> clock_t;
}
extern "C" {
    pub fn time(__timer: *mut time_t) -> time_t;
}
extern "C" {
    pub fn difftime(__time1: time_t, __time0: time_t) -> f64;
}
extern "C" {
    pub fn mktime(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn strftime(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
    ) -> usize;
}
extern "C" {
    pub fn strftime_l(
        __s: *mut ::std::os::raw::c_char,
        __maxsize: usize,
        __format: *const ::std::os::raw::c_char,
        __tp: *const tm,
        __loc: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn gmtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn localtime(__timer: *const time_t) -> *mut tm;
}
extern "C" {
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
extern "C" {
    pub fn asctime(__tp: *const tm) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime(__timer: *const time_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn asctime_r(
        __tp: *const tm,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ctime_r(
        __timer: *const time_t,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tzset();
}
extern "C" {
    pub fn timegm(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn timelocal(__tp: *mut tm) -> time_t;
}
extern "C" {
    pub fn dysize(__year: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getres(__clock_id: clockid_t, __res: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_settime(__clock_id: clockid_t, __tp: *const timespec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_nanosleep(
        __clock_id: clockid_t,
        __flags: ::std::os::raw::c_int,
        __req: *const timespec,
        __rem: *mut timespec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clock_getcpuclockid(__pid: pid_t, __clock_id: *mut clockid_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_create(
        __clock_id: clockid_t,
        __evp: *mut sigevent,
        __timerid: *mut timer_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_delete(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_settime(
        __timerid: timer_t,
        __flags: ::std::os::raw::c_int,
        __value: *const itimerspec,
        __ovalue: *mut itimerspec,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_gettime(__timerid: timer_t, __value: *mut itimerspec) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timer_getoverrun(__timerid: timer_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn timespec_get(
        __ts: *mut timespec,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type suseconds_t = __suseconds_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
pub type sigset_t = __sigset_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timezone {
    pub tz_minuteswest: ::std::os::raw::c_int,
    pub tz_dsttime: ::std::os::raw::c_int,
}
extern "C" {
    pub fn gettimeofday(
        __tv: *mut timeval,
        __tz: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn settimeofday(__tv: *const timeval, __tz: *const timezone) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn adjtime(__delta: *const timeval, __olddelta: *mut timeval) -> ::std::os::raw::c_int;
}
pub const ITIMER_REAL: __itimer_which = 0;
pub const ITIMER_VIRTUAL: __itimer_which = 1;
pub const ITIMER_PROF: __itimer_which = 2;
pub type __itimer_which = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
pub type __itimer_which_t = ::std::os::raw::c_int;
extern "C" {
    pub fn getitimer(__which: __itimer_which_t, __value: *mut itimerval) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn utimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lutimes(
        __file: *const ::std::os::raw::c_char,
        __tvp: *const timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn futimes(__fd: ::std::os::raw::c_int, __tvp: *const timeval) -> ::std::os::raw::c_int;
}
pub type cs_time_t = i64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs_name_t {
    pub length: u16,
    pub value: [u8; 256usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct cs_version_t {
    pub releaseCode: ::std::os::raw::c_char,
    pub majorVersion: ::std::os::raw::c_uchar,
    pub minorVersion: ::std::os::raw::c_uchar,
}
pub const CS_DISPATCH_ONE: cs_dispatch_flags_t = 1;
pub const CS_DISPATCH_ALL: cs_dispatch_flags_t = 2;
pub const CS_DISPATCH_BLOCKING: cs_dispatch_flags_t = 3;
pub const CS_DISPATCH_ONE_NONBLOCKING: cs_dispatch_flags_t = 4;
pub type cs_dispatch_flags_t = ::std::os::raw::c_uint;
pub const CS_OK: cs_error_t = 1;
pub const CS_ERR_LIBRARY: cs_error_t = 2;
pub const CS_ERR_VERSION: cs_error_t = 3;
pub const CS_ERR_INIT: cs_error_t = 4;
pub const CS_ERR_TIMEOUT: cs_error_t = 5;
pub const CS_ERR_TRY_AGAIN: cs_error_t = 6;
pub const CS_ERR_INVALID_PARAM: cs_error_t = 7;
pub const CS_ERR_NO_MEMORY: cs_error_t = 8;
pub const CS_ERR_BAD_HANDLE: cs_error_t = 9;
pub const CS_ERR_BUSY: cs_error_t = 10;
pub const CS_ERR_ACCESS: cs_error_t = 11;
pub const CS_ERR_NOT_EXIST: cs_error_t = 12;
pub const CS_ERR_NAME_TOO_LONG: cs_error_t = 13;
pub const CS_ERR_EXIST: cs_error_t = 14;
pub const CS_ERR_NO_SPACE: cs_error_t = 15;
pub const CS_ERR_INTERRUPT: cs_error_t = 16;
pub const CS_ERR_NAME_NOT_FOUND: cs_error_t = 17;
pub const CS_ERR_NO_RESOURCES: cs_error_t = 18;
pub const CS_ERR_NOT_SUPPORTED: cs_error_t = 19;
pub const CS_ERR_BAD_OPERATION: cs_error_t = 20;
pub const CS_ERR_FAILED_OPERATION: cs_error_t = 21;
pub const CS_ERR_MESSAGE_ERROR: cs_error_t = 22;
pub const CS_ERR_QUEUE_FULL: cs_error_t = 23;
pub const CS_ERR_QUEUE_NOT_AVAILABLE: cs_error_t = 24;
pub const CS_ERR_BAD_FLAGS: cs_error_t = 25;
pub const CS_ERR_TOO_BIG: cs_error_t = 26;
pub const CS_ERR_NO_SECTIONS: cs_error_t = 27;
pub const CS_ERR_CONTEXT_NOT_FOUND: cs_error_t = 28;
pub const CS_ERR_TOO_MANY_GROUPS: cs_error_t = 30;
pub const CS_ERR_SECURITY: cs_error_t = 100;
pub type cs_error_t = ::std::os::raw::c_uint;
extern "C" {
    pub fn qb_to_cs_error(result: ::std::os::raw::c_int) -> cs_error_t;
}
extern "C" {
    pub fn cs_strerror(err: cs_error_t) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn hdb_error_to_cs(res: ::std::os::raw::c_int) -> cs_error_t;
}
pub type votequorum_handle_t = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct votequorum_info {
    pub node_id: ::std::os::raw::c_uint,
    pub node_state: ::std::os::raw::c_uint,
    pub node_votes: ::std::os::raw::c_uint,
    pub node_expected_votes: ::std::os::raw::c_uint,
    pub highest_expected: ::std::os::raw::c_uint,
    pub total_votes: ::std::os::raw::c_uint,
    pub quorum: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub qdevice_votes: ::std::os::raw::c_uint,
    pub qdevice_name: [::std::os::raw::c_char; 255usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct votequorum_node_t {
    pub nodeid: u32,
    pub state: u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct votequorum_ring_id_t {
    pub nodeid: u32,
    pub seq: u64,
}
pub type votequorum_quorum_notification_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        handle: votequorum_handle_t,
        context: u64,
        quorate: u32,
        node_list_entries: u32,
        node_list: *mut votequorum_node_t,
    ),
>;
pub type votequorum_nodelist_notification_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        handle: votequorum_handle_t,
        context: u64,
        ring_id: votequorum_ring_id_t,
        node_list_entries: u32,
        node_list: *mut u32,
    ),
>;
pub type votequorum_expectedvotes_notification_fn_t = ::std::option::Option<
    unsafe extern "C" fn(handle: votequorum_handle_t, context: u64, expected_votes: u32),
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct votequorum_callbacks_t {
    pub votequorum_quorum_notify_fn: votequorum_quorum_notification_fn_t,
    pub votequorum_expectedvotes_notify_fn: votequorum_expectedvotes_notification_fn_t,
    pub votequorum_nodelist_notify_fn: votequorum_nodelist_notification_fn_t,
}
extern "C" {
    pub fn votequorum_initialize(
        handle: *mut votequorum_handle_t,
        callbacks: *mut votequorum_callbacks_t,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_finalize(handle: votequorum_handle_t) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_dispatch(
        handle: votequorum_handle_t,
        dispatch_types: cs_dispatch_flags_t,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_fd_get(
        handle: votequorum_handle_t,
        fd: *mut ::std::os::raw::c_int,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_getinfo(
        handle: votequorum_handle_t,
        nodeid: ::std::os::raw::c_uint,
        info: *mut votequorum_info,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_setexpected(
        handle: votequorum_handle_t,
        expected_votes: ::std::os::raw::c_uint,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_setvotes(
        handle: votequorum_handle_t,
        nodeid: ::std::os::raw::c_uint,
        votes: ::std::os::raw::c_uint,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_trackstart(
        handle: votequorum_handle_t,
        context: u64,
        flags: ::std::os::raw::c_uint,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_trackstop(handle: votequorum_handle_t) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_context_get(
        handle: votequorum_handle_t,
        context: *mut *mut ::std::os::raw::c_void,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_context_set(
        handle: votequorum_handle_t,
        context: *mut ::std::os::raw::c_void,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_qdevice_register(
        handle: votequorum_handle_t,
        name: *const ::std::os::raw::c_char,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_qdevice_unregister(
        handle: votequorum_handle_t,
        name: *const ::std::os::raw::c_char,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_qdevice_update(
        handle: votequorum_handle_t,
        oldname: *const ::std::os::raw::c_char,
        newname: *const ::std::os::raw::c_char,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_qdevice_poll(
        handle: votequorum_handle_t,
        name: *const ::std::os::raw::c_char,
        cast_vote: ::std::os::raw::c_uint,
        ring_id: votequorum_ring_id_t,
    ) -> cs_error_t;
}
extern "C" {
    pub fn votequorum_qdevice_master_wins(
        handle: votequorum_handle_t,
        name: *const ::std::os::raw::c_char,
        allow: ::std::os::raw::c_uint,
    ) -> cs_error_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
