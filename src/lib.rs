#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
pub mod cpg;
pub mod quorum;
pub mod votequorum;
pub mod cfg;
pub mod cmap;

use std::fmt;
use std::convert::TryFrom;
use std::ptr::copy_nonoverlapping;
use std::ffi::CString;
use std::error::Error;

// This needs to be kept up-to-date!
/// Error codes returned from the corosync libraries
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum CsError {
    CsOk = 1,
    CsErrLibrary = 2,
    CsErrVersion = 3,
    CsErrInit = 4,
    CsErrTimeout = 5,
    CsErrTryAgain = 6,
    CsErrInvalidParam = 7,
    CsErrNoMemory = 8,
    CsErrBadHandle = 9,
    CsErrBusy = 10,
    CsErrAccess = 11,
    CsErrNotExist = 12,
    CsErrNameTooLong = 13,
    CsErrExist = 14,
    CsErrNoSpace = 15,
    CsErrInterrupt = 16,
    CsErrNameNotFound = 17,
    CsErrNoResources = 18,
    CsErrNotSupported = 19,
    CsErrBadOperation = 20,
    CsErrFailedOperation = 21,
    CsErrMessageError = 22,
    CsErrQueueFull = 23,
    CsErrQueueNotAvailable = 24,
    CsErrBadFlags = 25,
    CsErrTooBig = 26,
    CsErrNoSection = 27,
    CsErrContextNotFound = 28,
    CsErrTooManyGroups = 30,
    CsErrSecurity = 100,
    CsErrRustCompat = 998, // Set if we get a unknown return from corosync
    CsErrRustString = 999, // Set if we get a string conversion error
}
pub type Result<T> = ::std::result::Result<T, CsError>;

impl fmt::Display for CsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    CsError::CsOk => write!(f, "OK"),
	    CsError::CsErrLibrary => write!(f, "ErrLibrary"),
	    CsError::CsErrVersion => write!(f, "ErrVersion"),
	    CsError::CsErrInit => write!(f, "ErrInit"),
	    CsError::CsErrTimeout => write!(f, "ErrTimeout"),
	    CsError::CsErrTryAgain => write!(f, "ErrTryAgain"),
	    CsError::CsErrInvalidParam => write!(f, "ErrInvalidParam"),
	    CsError::CsErrNoMemory => write!(f, "ErrNoMemory"),
	    CsError::CsErrBadHandle => write!(f, "ErrbadHandle"),
	    CsError::CsErrBusy => write!(f, "ErrBusy"),
	    CsError::CsErrAccess => write!(f, "ErrAccess"),
	    CsError::CsErrNotExist => write!(f, "ErrNotExist"),
	    CsError::CsErrNameTooLong => write!(f, "ErrNameTooLong"),
	    CsError::CsErrExist => write!(f, "ErrExist"),
	    CsError::CsErrNoSpace => write!(f, "ErrNoSpace"),
	    CsError::CsErrInterrupt => write!(f, "ErrInterrupt"),
	    CsError::CsErrNameNotFound => write!(f, "ErrNameNotFound"),
	    CsError::CsErrNoResources => write!(f, "ErrNoResources"),
	    CsError::CsErrNotSupported => write!(f, "ErrNotSupported"),
	    CsError::CsErrBadOperation => write!(f, "ErrBadOperation"),
	    CsError::CsErrFailedOperation => write!(f, "ErrFailedOperation"),
	    CsError::CsErrMessageError => write!(f, "ErrMEssageError"),
	    CsError::CsErrQueueFull => write!(f, "ErrQueueFull"),
	    CsError::CsErrQueueNotAvailable => write!(f, "ErrQueueNotAvailable"),
	    CsError::CsErrBadFlags => write!(f, "ErrBadFlags"),
	    CsError::CsErrTooBig => write!(f, "ErrTooBig"),
	    CsError::CsErrNoSection => write!(f, "ErrNoSection"),
	    CsError::CsErrContextNotFound => write!(f, "ErrContextNotFound"),
	    CsError::CsErrTooManyGroups => write!(f, "ErrTooManyGroups"),
	    CsError::CsErrSecurity => write!(f, "ErrSecurity"),
	    CsError::CsErrRustCompat => write!(f, "ErrRustCompat"),
	    CsError::CsErrRustString => write!(f, "ErrRustString"),
	}
    }
}

impl Error for CsError {}

// This is dependant on the num_enum crate, converts a C cs_error_t into the Rust enum
// There seems to be some debate as to whether this should be part of the language:
// https://internals.rust-lang.org/t/pre-rfc-enum-from-integer/6348/25
impl From<u32> for CsError {

    fn from(cserr: u32) -> CsError
    {
	match CsError::try_from(cserr) {
	    Ok(e) => e,
	    Err(_) => CsError::CsErrRustCompat
	}
    }
}

/// Flags to use with dispatch functions, eg [cpg::dispatch]
#[derive(Copy, Clone)]
// The numbers match the C enum, of course.
pub enum DispatchFlags {
    One = 1,
    All = 2,
    Blocking = 3,
    OneNonblocking = 4,
}

/// Flags to use with (most) tracking API calls
#[derive(Copy, Clone)]
// Same here
pub enum TrackFlags {
    Current = 1,
    Changes = 2,
    ChangesOnly = 4,
}

/// A corosync nodeid
#[derive(Copy, Clone, Debug)]
pub struct NodeId {
    id: u32,
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{}", self.id)
    }
}

// Conversion from a NodeId to and from u32
impl From<u32> for NodeId {
    fn from(id: u32) -> NodeId {
	NodeId{id}
    }
}

impl From<NodeId> for u32 {
    fn from(nodeid: NodeId) -> u32 {
	nodeid.id
    }
}


// General internal routine to copy bytes from a C array into a Rust String
fn string_from_bytes(bytes: *const ::std::os::raw::c_char, max_length: usize) -> Result<String>
{
    let mut newbytes = Vec::<u8>::new();
    newbytes.resize(max_length, 0u8);

    unsafe {
	// We need to fully copy it, not shallow copy it.
	// Messy casting on both parts of the copy here to get it to work on both signed
	// and unsigned char machines
	copy_nonoverlapping(bytes as *mut i8, newbytes.as_mut_ptr() as *mut i8, max_length);
    }

    // Get length of the string in old-fashioned style
    let mut length: usize = 0;
    let mut count : usize = 0;
    for i in &newbytes {
	if *i == 0 && length == 0 {
	    length = count;
	    break;
	}
	count += 1;
    }

    // Cope with an empty string
    if length == 0 {
	return Ok(String::new());
    }

    let cs = match CString::new(&newbytes[0..length as usize]) {
	Ok(c1) => c1,
	Err(_) => return Err(CsError::CsErrRustString),
    };
    match cs.into_string() {
	Ok(s) => Ok(s),
	Err(_) => Err(CsError::CsErrRustString),
    }
}
