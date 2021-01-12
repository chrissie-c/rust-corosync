#[macro_use]
extern crate lazy_static;
pub mod cpg;
pub mod quorum;
pub mod votequorum;
pub mod cfg;

use std::fmt;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

// This needs to be kept up-to-date!
/// Error codes returned from the corosync libraries
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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
    CsErrRustCompat = 999, // Set if we get a unknown return from corosync
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
	}
    }
}

// This is dependant on the num_enum crate, converts a C cs_error_t into the Rust enum
// There seems to be some debate as to whether this should be part of the language:
// https://internals.rust-lang.org/t/pre-rfc-enum-from-integer/6348/25
fn cs_error_to_enum(cserr: u32) -> CsError
{
    match CsError::try_from(cserr) {
	Ok(e) => e,
	Err(_) => CsError::CsErrRustCompat
    }
}

/// flags to use with dispatch functions, eg [cpg::dispatch]
#[derive(Copy, Clone)]
// The numbers match the C enum, of course.
pub enum DispatchFlags {
    One = 1,
    All = 2,
    Blocking = 3,
    OneNonblocking = 4,
}

#[derive(Copy, Clone)]
// Same here
pub enum TrackFlags {
    Current = 1,
    Changes = 2,
    ChangesOnly = 4,
}


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
