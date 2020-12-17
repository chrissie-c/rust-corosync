//CC: TEMP
#![allow(unused_variables,dead_code)]

extern crate rust_corosync_sys as ffi;
use rust_corosync_sys::cpg::cpg_model_data_t;

use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::os::raw::{c_void, c_int};
use std::sync::Mutex;
use std::string::String;
use std::ffi::{CStr, CString};
use std::ptr::copy_nonoverlapping;
use std::slice;
use std::fmt;

// TODO Moved this to 'corosync' level
// This needs to be kept up-to-date!
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


#[derive(Copy, Clone)]
// TODO these are global to all corosync
// The numbers match the C enum, of course.
pub enum DispatchFlags {
    One = 1,
    All = 2,
    Blocking = 3,
    OneNonblocking = 4,
}

pub struct RingId {
    pub nodeid: u32,
    pub seq: u64,
}

// The C enum doesn't have numbers in the code
// so don't assume we can match them
pub enum Guarantee {
    TypeUnordered,
    TypeFifo,
    TypeAgreed,
    TypeSafe,
}

impl Guarantee {
    pub fn to_c (&self) -> u32 {
	match self {
	    Guarantee::TypeUnordered => ffi::cpg::CPG_TYPE_UNORDERED,
	    Guarantee::TypeFifo => ffi::cpg::CPG_TYPE_FIFO,
	    Guarantee::TypeAgreed => ffi::cpg::CPG_TYPE_AGREED,
	    Guarantee::TypeSafe => ffi::cpg::CPG_TYPE_SAFE,
	}
    }
}

pub enum FlowControlState {
    Disabled,
    Enabled
}

#[derive(Copy, Clone)]
pub enum Model1Flags {
    None,
}

pub enum Reason {
    Undefined = 0,
    Join = 1,
    Leave = 2,
    NodeDown = 3,
    NodeUp = 4,
    ProcDown = 5,
}

impl Reason {
    pub fn new(r: u32) ->Reason {
	match r {
	    0 => Reason::Undefined,
	    1 => Reason::Join,
	    2 => Reason::Leave,
	    3 => Reason::NodeDown,
	    4 => Reason::NodeUp,
	    5 => Reason::ProcDown,
	    _ => Reason::Undefined
	}
    }
}
impl fmt::Debug for Reason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    Reason::Undefined => write!(f, "Undefined"),
	    Reason::Join => write!(f, "Join"),
	    Reason::Leave => write!(f, "Leave"),
	    Reason::NodeDown => write!(f, "NodeDown"),
	    Reason::NodeUp => write!(f, "NodeUp"),
	    Reason::ProcDown => write!(f, "ProcDown"),
	}
    }
}

pub struct Address {
    pub nodeid: u32,
    pub pid: u32,
    pub reason: Reason,
}
impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "nodeid: {}, pid: {}, reason: {:?}", self.nodeid, self.pid, self.reason)
    }
}

#[derive(Copy, Clone)]
pub struct Model1Data {
    pub flags: Model1Flags,
    pub deliver_fn: fn(handle: u64,
		       group_name: String,
		       nodeid: u32,
		       pid: u32,
		       msg: &[u8],
		       msg_len: usize,
    ),
    pub confchg_fn: fn(handle: u64,
		       group_name: &str,
		       member_list: Vec<Address>,
		       left_list: Vec<Address>,
		       joined_list: Vec<Address>,
    ),
    pub totem_confchg_fn: fn(handle: u64,
			     ring_id: RingId,
			     member_list: Vec<u32>,
    ),
}

#[derive(Copy, Clone)]
pub enum ModelData {
    ModelNone,
    ModelV1 (Model1Data)
}


// Our CPG internal state
#[derive(Copy, Clone)]
pub struct Handle {
    cpg_handle: u64,
    model_data: ModelData,
}

// Used to convert a CPG handle into one of ours
lazy_static! {
    static ref HANDLE_HASH: Mutex<HashMap<u64, Handle>> = Mutex::new(HashMap::new());
}

// Called from CPG callback function - munge params back to Rust from C
extern "C" fn rust_deliver_fn(
    handle: ffi::cpg::cpg_handle_t,
    group_name: *const ffi::cpg::cpg_name,
    nodeid: u32,
    pid: u32,
    msg: *mut ::std::os::raw::c_void,
    msg_len: usize)
{
    match HANDLE_HASH.lock().unwrap().get(&handle) {
	Some(h) =>  {
	    // Convert group_name into a Rust str.
	    let r_group_name = unsafe {
		CStr::from_ptr(&(*group_name).value[0]).to_string_lossy().into_owned()
	    };

	    let data : &[u8] = unsafe {
		std::slice::from_raw_parts(msg as *const u8, msg_len)
	    };

	    match h.model_data {
		ModelData::ModelV1(md) =>
		    (md.deliver_fn)(handle,
				    r_group_name.to_string(),
				    nodeid,
				    pid,
				    data,
				    msg_len),
		_ => {}
	    }
	}
	None => {}
    }
}

// Convert an array of cpg_addresses to a Vec<cpg::Address>
fn cpg_array_to_vec(list: *const ffi::cpg::cpg_address, list_entries: usize) -> Vec<Address>
{
    let temp: &[ffi::cpg::cpg_address] = unsafe { slice::from_raw_parts(list, list_entries as usize) };
    let mut r_vec = Vec::<Address>::new();

    for i in 0..list_entries as usize {
	let a: Address = Address {nodeid: temp[i].nodeid,
				  pid: temp[i].pid,
				  reason: Reason::new(temp[i].reason)};
	r_vec.push(a);
    }
    r_vec
}


// Called from CPG callback function - munge params back to Rust from C
extern "C" fn rust_confchg_fn(handle: ffi::cpg::cpg_handle_t,
			      group_name: *const ffi::cpg::cpg_name,
			      member_list: *const ffi::cpg::cpg_address,
			      member_list_entries: usize,
			      left_list: *const ffi::cpg::cpg_address,
			      left_list_entries: usize,
			      joined_list: *const ffi::cpg::cpg_address,
			      joined_list_entries: usize)
{
    match HANDLE_HASH.lock().unwrap().get(&handle) {
	Some(h) =>  {
	    let r_group_name = unsafe {
		CStr::from_ptr(&(*group_name).value[0]).to_string_lossy().into_owned()
	    };
	    let r_member_list = cpg_array_to_vec(member_list, member_list_entries);
	    let r_left_list = cpg_array_to_vec(left_list, left_list_entries);
	    let r_joined_list = cpg_array_to_vec(joined_list, joined_list_entries);

	    match h.model_data {
		ModelData::ModelV1(md) =>
		    (md.confchg_fn)(handle,
				    &r_group_name.to_string(),
				    r_member_list,
				    r_left_list,
				    r_joined_list),
		_ => {}
	    }
	}
	None => {}
    }
}

extern "C" fn rust_totem_confchg_fn(handle: ffi::cpg::cpg_handle_t,
				    ring_id: ffi::cpg::cpg_ring_id,
				    member_list_entries: u32,
				    member_list: *const u32)
{
    match HANDLE_HASH.lock().unwrap().get(&handle) {
	Some(h) =>  {
	    let r_ring_id = RingId{nodeid: ring_id.nodeid,
				   seq: ring_id.seq};
	    let mut r_member_list = Vec::<u32>::new();
	    let temp_members: &[u32] = unsafe { slice::from_raw_parts(member_list, member_list_entries as usize) };
	    for i in 0..member_list_entries as usize {
		r_member_list.push(temp_members[i]);
	    }

	    match h.model_data {
		ModelData::ModelV1(md) =>
		    (md.totem_confchg_fn)(handle,
					  r_ring_id,
					  r_member_list),
		_ => {}
	    }
	}
	None => {}
    }
}

// This is dependant on the num_enum crate.
// https://internals.rust-lang.org/t/pre-rfc-enum-from-integer/6348/25
fn cs_error_to_enum(cserr: u32) -> CsError
{
    match CsError::try_from(cserr) {
	Ok(e) => e,
	Err(_) => CsError::CsErrRustCompat
    }
}

// Returns the actual CPG handle as .. well, it's easier than inventing something else
pub fn initialize(model_data: &ModelData, context: u64) -> Result<u64>
{
    let mut handle: ffi::cpg::cpg_handle_t = 0;
    let mut m = match model_data {
	ModelData::ModelV1(_v1) => {
	    ffi::cpg::cpg_model_v1_data_t {
		model: ffi::cpg::CPG_MODEL_V1,
		cpg_deliver_fn: Some(rust_deliver_fn),
		cpg_confchg_fn: Some(rust_confchg_fn),
		cpg_totem_confchg_fn: Some(rust_totem_confchg_fn),
		flags: 0, // TODO flags conversion
	    }
	}
	_ => return Err(CsError::CsErrInvalidParam)
    };

    unsafe {
	let c_context: *mut c_void = &mut &context as *mut _ as *mut c_void;
	let c_model:   *mut cpg_model_data_t = &mut m as *mut _ as *mut cpg_model_data_t;
	let res = ffi::cpg::cpg_model_initialize(&mut handle,
						 m.model,
						 c_model,
						 c_context);

	if res == ffi::cpg::CS_OK {
	    let rhandle = Handle{cpg_handle: handle, model_data: *model_data};
	    HANDLE_HASH.lock().unwrap().insert(handle, rhandle);
	    Ok(handle)
	} else {
	    Err(cs_error_to_enum(res))
	}
    }
}

pub fn finalize(handle: u64) -> Result<()>
{
    let res =
	unsafe {
	    ffi::cpg::cpg_finalize(handle)
	};
    if res == ffi::cpg::CS_OK {
	Ok(())
    } else {
	Err(CsError::try_from(res).unwrap())
    }
}

pub fn fd_get(handle: u64) -> Result<i32>
{
    let c_fd: *mut c_int = &mut 0 as *mut _ as *mut c_int;
    let res =
	unsafe {
	    ffi::cpg::cpg_fd_get(handle, c_fd)
	};
    if res == ffi::cpg::CS_OK {
	Ok(c_fd as i32)
    } else {
	Err(cs_error_to_enum(res))
    }
}

/// CC: Some missing here - just so I can test what we have

pub fn dispatch(handle: u64, flags: DispatchFlags) -> Result<()>
{
    let res =
	unsafe {
	    ffi::cpg::cpg_dispatch(handle, flags as u32)
	};
    if res == ffi::cpg::CS_OK {
	Ok(())
    } else {
	Err(cs_error_to_enum(res))
    }
}


pub fn join(handle: u64, group: &String) -> Result<()>
{
    let res =
	unsafe {
	    // fill in the cpg_group struct
	    let c_name = CString::new(group.as_str()).unwrap();
	    let mut c_group = ffi::cpg::cpg_name{length: group.len()as u32, value: [0; 128]};
	    copy_nonoverlapping(c_name.as_ptr(), c_group.value.as_mut_ptr(), group.len());
	    ffi::cpg::cpg_join(handle, &c_group)
	};
    if res == ffi::cpg::CS_OK {
	Ok(())
    } else {
	Err(cs_error_to_enum(res))
    }
}

pub fn leave(handle: u64, group: &String) -> Result<()>
{
    let res =
	unsafe {
	    let c_name = CString::new(group.as_str()).unwrap();
	    let mut c_group = ffi::cpg::cpg_name{length: group.len()as u32, value: [0; 128]};
	    copy_nonoverlapping(c_name.as_ptr(), c_group.value.as_mut_ptr(), group.len());
	    ffi::cpg::cpg_leave(handle, &c_group)
	};
    if res == ffi::cpg::CS_OK {
	Ok(())
    } else {
	Err(cs_error_to_enum(res))
    }
}

pub fn local_get(handle: u64) -> Result<u32>
{
    let mut nodeid: u32 = 0;
    let res =
	unsafe {
	    ffi::cpg::cpg_local_get(handle, &mut nodeid)
	};
    if res == ffi::cpg::CS_OK {
	Ok(nodeid)
    } else {
	Err(cs_error_to_enum(res))
    }
}

pub fn membership_get(handle: u64, group: &String) -> Result<Vec::<Address>>
{
    let mut member_list_entries: i32 = 0;
    let member_list = [ffi::cpg::cpg_address{nodeid:0,pid:0,reason:0};128];
    let res =
	unsafe {
	    // fill in the cpg_group struct
	    let c_name = CString::new(group.as_str()).unwrap();
	    let mut c_group = ffi::cpg::cpg_name{length: group.len()as u32, value: [0; 128]};
	    copy_nonoverlapping(c_name.as_ptr(), c_group.value.as_mut_ptr(), group.len());

	    let c_memlist = member_list.as_ptr() as *mut ffi::cpg::cpg_address;

	    ffi::cpg::cpg_membership_get(handle, &mut c_group,
					 &mut *c_memlist,
					 &mut member_list_entries)
	};
    if res == ffi::cpg::CS_OK {
	Ok(cpg_array_to_vec(member_list.as_ptr(), member_list_entries as usize))
    } else {
	Err(cs_error_to_enum(res))
    }
}

pub fn max_atomic_msgsize_get(handle: u64) -> Result<u32>
{
    let mut asize: u32 = 0;
    let res =
	unsafe {
	    ffi::cpg::cpg_max_atomic_msgsize_get(handle, &mut asize)
	};
    if res == ffi::cpg::CS_OK {
	Ok(asize)
    } else {
	Err(cs_error_to_enum(res))
    }
}

pub fn context_get(handle: u64) -> Result<u64>
{
    let mut c_context: *mut c_void = &mut 0 as *mut _ as *mut c_void;
    let (res, context) =
	unsafe {
	    let r = ffi::cpg::cpg_context_get(handle, &mut c_context);
	    let context: u64 = c_context as u64;
	    (r, context)
	};
    if res == ffi::cpg::CS_OK {
	Ok(context)
    } else {
	Err(cs_error_to_enum(res))
    }
}

pub fn context_set(handle: u64, context: u64) -> Result<()>
{
    let res =
	unsafe {
	    let c_context = context as *mut c_void;
	    ffi::cpg::cpg_context_set(handle, c_context)
	};
    if res == ffi::cpg::CS_OK {
	Ok(())
    } else {
	Err(cs_error_to_enum(res))
    }
}

pub fn flow_control_state_get(handle: u64) -> Result<bool>
{
    let mut fc_state: u32 = 0;
    let res =
	unsafe {
	    ffi::cpg::cpg_flow_control_state_get(handle, &mut fc_state)
	};
    if res == ffi::cpg::CS_OK {
	if fc_state == 1 {
	    Ok(true)
	} else {
	    Ok(false)
	}
    } else {
	Err(cs_error_to_enum(res))
    }
}

pub fn mcast_joined(handle: u64, guarantee: Guarantee,
		    msg: &[u8]) -> Result<()>
{
    let c_iovec = ffi::cpg::iovec {
	iov_base: msg.as_ptr() as *mut c_void,
	iov_len: msg.len(),
    };
    let res =
	unsafe {
	    ffi::cpg::cpg_mcast_joined(handle,
				       guarantee.to_c(),
				       &c_iovec, 1)
    };
    if res == ffi::cpg::CS_OK {
	Ok(())
    } else {
	Err(cs_error_to_enum(res))
    }
}
