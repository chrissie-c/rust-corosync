// libvotequorum interface for Rust
// Copyright (c) 2021 Red Hat, Inc.
//
// All rights reserved.
//
// Author: Christine Caulfield (ccaulfi@redhat.com)
//


// For the code generated by bindgen
extern crate rust_corosync_sys as ffi;

use std::os::raw::{c_void, c_int};
use std::slice;
use std::collections::HashMap;
use std::sync::Mutex;
use std::ffi::CString;
use std::fmt;

use crate::{CsError, DispatchFlags, TrackFlags, Result, NodeId};
use crate::string_from_bytes;


/// RingId returned by votequorum_notification_fn
pub struct RingId {
    pub nodeid: NodeId,
    pub seq: u64,
}

// Used to convert a VOTEQUORUM handle into one of ours
lazy_static! {
    static ref HANDLE_HASH: Mutex<HashMap<u64, Handle>> = Mutex::new(HashMap::new());
}

/// Current state of a node in the cluster, part of the [NodeInfo] and [Node] structs
pub enum NodeState
{
    Member,
    Dead,
    Leaving,
    Unknown,
}
impl NodeState {
    pub fn new(state: u32) -> NodeState
    {
	match state {
	    1 => NodeState::Member,
	    2 => NodeState::Dead,
	    3 => NodeState::Leaving,
	    _ => NodeState::Unknown,
	}
    }
}
impl fmt::Debug for NodeState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	match self {
	    NodeState::Member => write!(f, "Member"),
	    NodeState::Dead => write!(f, "Dead"),
	    NodeState::Leaving => write!(f, "Leaving"),
	    _  => write!(f, "Unknown"),
	}
    }
}

/// Basic information about a node in the cluster. Contains [NodeId], and [NodeState]
pub struct Node
{
    nodeid: NodeId,
    state: NodeState
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "nodeid: {}, state: {:?}", self.nodeid, self.state)
    }
}

bitflags! {
/// Flags in the [NodeInfo] struct
    pub struct NodeInfoFlags: u32
    {
	const VOTEQUORUM_INFO_TWONODE             = 1;
	const VOTEQUORUM_INFO_QUORATE             = 2;
	const VOTEQUORUM_INFO_WAIT_FOR_ALL        = 4;
	const VOTEQUORUM_INFO_LAST_MAN_STANDING   = 8;
	const VOTEQUORUM_INFO_AUTO_TIE_BREAKER    = 16;
	const VOTEQUORUM_INFO_ALLOW_DOWNSCALE     = 32;
	const VOTEQUORUM_INFO_QDEVICE_REGISTERED  = 64;
	const VOTEQUORUM_INFO_QDEVICE_ALIVE       = 128;
	const VOTEQUORUM_INFO_QDEVICE_CAST_VOTE   = 256;
	const VOTEQUORUM_INFO_QDEVICE_MASTER_WINS = 512;
    }
}

/// Detailed information about a node in the cluster, returned from [get_info]
pub struct NodeInfo
{
    pub node_id: NodeId,
    pub node_state: NodeState,
    pub node_votes: u32,
    pub node_expected_votes: u32,
    pub highest_expected: u32,
    pub quorum: u32,
    pub flags: NodeInfoFlags,
    pub qdevice_votes: u32,
    pub qdevice_name: String,
}

// Turn a C nodeID list into a vec of NodeIds
fn list_to_vec(list_entries: u32, list: *const u32) -> Vec<NodeId>
{
    let mut r_member_list = Vec::<NodeId>::new();
    let temp_members: &[u32] = unsafe { slice::from_raw_parts(list, list_entries as usize) };
    for i in 0..list_entries as usize {
	r_member_list.push(NodeId::from(temp_members[i]));
    }
    r_member_list
}

// Called from votequorum callback function - munge params back to Rust from C
extern "C" fn rust_expectedvotes_notification_fn(
    handle: ffi::votequorum::votequorum_handle_t,
    context: u64,
    expected_votes: u32)
{
    match HANDLE_HASH.lock().unwrap().get(&handle) {
	Some(h) => {
	    match h.callbacks.expectedvotes_notification_fn {
		Some(cb) => (cb)(h,
				 context,
				 expected_votes),
		None => {}
	    }
	}
	None => {}
    }
}

// Called from votequorum callback function - munge params back to Rust from C
extern "C" fn rust_quorum_notification_fn(
    handle: ffi::votequorum::votequorum_handle_t,
    context: u64,
    quorate: u32,
    node_list_entries: u32,
    node_list: *mut ffi::votequorum::votequorum_node_t)
{
    match HANDLE_HASH.lock().unwrap().get(&handle) {
	Some(h) =>  {
	    let r_quorate = match quorate {
		0 => false,
		1 => true,
		_ => false,
	    };
	    let mut r_node_list = Vec::<Node>::new();
	    let temp_members: &[ffi::votequorum::votequorum_node_t] =
		unsafe { slice::from_raw_parts(node_list, node_list_entries as usize) };
		for i in 0..node_list_entries as usize {
		    r_node_list.push(Node{nodeid: NodeId::from(temp_members[i].nodeid),
					  state: NodeState::new(temp_members[i].state)} );
		}
	    match h.callbacks.quorum_notification_fn {
		Some (cb) => (cb)(h,
				  context,
				  r_quorate,
				  r_node_list),
		None => {}
	    }
	}
	None => {}
    }
}

// Called from votequorum callback function - munge params back to Rust from C
extern "C" fn rust_nodelist_notification_fn(
    handle: ffi::votequorum::votequorum_handle_t,
    context: u64,
    ring_id: ffi::votequorum::votequorum_ring_id_t,
    node_list_entries: u32,
    node_list: *mut u32)
{
    match HANDLE_HASH.lock().unwrap().get(&handle) {
	Some(h) =>  {
	    let r_ring_id = RingId{nodeid: NodeId::from(ring_id.nodeid),
				   seq: ring_id.seq};

	    let r_node_list = list_to_vec(node_list_entries, node_list);

	    match h.callbacks.nodelist_notification_fn {
		Some (cb) =>
		    (cb)(h,
			 context,
			 r_ring_id,
			 r_node_list),
		None => {}
	    }
	}
	None => {}
    }
}

/// Callbacks that can be called from votequorum, pass these in to [initialize]
#[derive(Copy, Clone)]
pub struct Callbacks {
    pub quorum_notification_fn: Option<fn(hande: &Handle,
					  context: u64,
					  quorate: bool,
					  node_list: Vec<Node>)>,
    pub nodelist_notification_fn: Option<fn(hande: &Handle,
					    context: u64,
					    ring_id: RingId,
					    node_list: Vec<NodeId>)>,
    pub expectedvotes_notification_fn: Option<fn(handle: &Handle,
						 context: u64,
						 expected_votes: u32)>,
}

/// A handle into the votequorum library. Returned from [initialize] and needed for all other calls
#[derive(Copy, Clone)]
pub struct Handle {
    votequorum_handle: u64,
    callbacks: Callbacks
}

/// Initialize a connection to the votequorum library. You must call this before doing anything
/// else and use the passed back [Handle].
/// Remember to free the handle using [finalize] when finished.
pub fn initialize(callbacks: &Callbacks) -> Result<Handle>
{
    let mut handle: ffi::votequorum::votequorum_handle_t = 0;

    let mut c_callbacks = ffi::votequorum::votequorum_callbacks_t {
	votequorum_quorum_notify_fn: Some(rust_quorum_notification_fn),
	votequorum_nodelist_notify_fn: Some(rust_nodelist_notification_fn),
	votequorum_expectedvotes_notify_fn: Some(rust_expectedvotes_notification_fn),
    };

    unsafe {
	let res = ffi::votequorum::votequorum_initialize(&mut handle,
							 &mut c_callbacks);
	if res == ffi::votequorum::CS_OK {
	    let rhandle = Handle{votequorum_handle: handle, callbacks: callbacks.clone()};
	    HANDLE_HASH.lock().unwrap().insert(handle, rhandle);
	    Ok(rhandle)
	} else {
	    Err(CsError::from_c(res))
	}
    }
}


/// Finish with a connection to corosync
pub fn finalize(handle: Handle) -> Result<()>
{
    let res =
	unsafe {
	    ffi::votequorum::votequorum_finalize(handle.votequorum_handle)
	};
    if res == ffi::votequorum::CS_OK {
	HANDLE_HASH.lock().unwrap().remove(&handle.votequorum_handle);
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}

// Not sure if an FD is the right thing to return here, but it will do for now.
/// Return a file descriptor to use for poll/select on the VOTEQUORUM handle
pub fn fd_get(handle: Handle) -> Result<i32>
{
    let c_fd: *mut c_int = &mut 0 as *mut _ as *mut c_int;
    let res =
	unsafe {
	    ffi::votequorum::votequorum_fd_get(handle.votequorum_handle, c_fd)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(c_fd as i32)
    } else {
	Err(CsError::from_c(res))
    }
}


const VOTEQUORUM_QDEVICE_MAX_NAME_LEN : usize = 255;

/// Returns detailed information about a node in a [NodeInfo] structure
pub fn get_info(handle: Handle, nodeid: NodeId) -> Result<NodeInfo>
{
    let mut c_info = ffi::votequorum::votequorum_info {
	node_id: 0,
	node_state:0,
	node_votes: 0,
	node_expected_votes:0,
	highest_expected:0,
	total_votes:0,
	quorum:0,
	flags:0,
	qdevice_votes:0,
	qdevice_name: [0; 255usize]
    };
    let res =
	unsafe {
	    ffi::votequorum::votequorum_getinfo(handle.votequorum_handle, u32::from(nodeid), &mut c_info)
	};

    if res == ffi::votequorum::CS_OK {
	let info = NodeInfo {
	    node_id : NodeId::from(c_info.node_id),
	    node_state : NodeState::new(c_info.node_state),
	    node_votes : c_info.node_votes,
	    node_expected_votes : c_info.node_expected_votes,
	    highest_expected : c_info.highest_expected,
	    quorum : c_info.quorum,
	    flags : NodeInfoFlags{bits: c_info.flags},
	    qdevice_votes : c_info.qdevice_votes,
	    qdevice_name : match string_from_bytes(c_info.qdevice_name.as_ptr(), VOTEQUORUM_QDEVICE_MAX_NAME_LEN) {
		Ok(s) => s,
		Err(_) => String::new()
	    },
	};
	Ok(info)
    } else {
	Err(CsError::from_c(res))
    }
}

/// Call any/all active votequorum callbacks for this [Handle]. see [DispatchFlags] for details
pub fn dispatch(handle: Handle, flags: DispatchFlags) -> Result<()>
{
    let res =
	unsafe {
	    ffi::votequorum::votequorum_dispatch(handle.votequorum_handle, flags as u32)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}

/// Track node and votequorum changes
pub fn trackstart(handle: Handle, context: u64, flags: TrackFlags) -> Result<()>
{
    let res =
	unsafe {
	    ffi::votequorum::votequorum_trackstart(handle.votequorum_handle, context, flags as u32)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}

/// Stop tracking node and votequorum changes
pub fn trackstop(handle: Handle) -> Result<()>
{
    let res =
	unsafe {
	    ffi::votequorum::votequorum_trackstop(handle.votequorum_handle)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}

/// Get the current 'context' value for this handle.
/// The context value is an arbitrary value that is always passed
/// back to callbacks to help identify the source
pub fn context_get(handle: Handle) -> Result<u64>
{
    let (res, context) =
	unsafe {
	    let mut c_context: *mut c_void = &mut 0u64 as *mut _ as *mut c_void;
	    let r = ffi::votequorum::votequorum_context_get(handle.votequorum_handle, &mut c_context);
	    let context: u64 = c_context as u64;
	    (r, context)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(context)
    } else {
	Err(CsError::from_c(res))
    }
}

/// Set the current 'context' value for this handle.
/// The context value is an arbitrary value that is always passed
/// back to callbacks to help identify the source.
/// Normally this is set in [trackstart], but this allows it to be changed
pub fn context_set(handle: Handle, context: u64) -> Result<()>
{
    let res =
	unsafe {
	    let c_context = context as *mut c_void;
	    ffi::votequorum::votequorum_context_set(handle.votequorum_handle, c_context)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}


/// Set the current expected_votes for the cluster, this value must
/// be valid and not result in an inquorate cluster.
pub fn set_expected(handle: Handle, expected_votes: u32) -> Result<()>
{
    let res =
	unsafe {
	    ffi::votequorum::votequorum_setexpected(handle.votequorum_handle, expected_votes)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}

/// Set the current votes for a node
pub fn set_votes(handle: Handle, nodeid: NodeId, votes: u32) -> Result<()>
{
    let res =
	unsafe {
	    ffi::votequorum::votequorum_setvotes(handle.votequorum_handle, u32::from(nodeid), votes)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}


/// Register a quorum device
pub fn qdevice_register(handle: Handle, name: &String) -> Result<()>
{
    let c_string = {
	match CString::new(name.as_str()) {
	    Ok(cs) => cs,
	    Err(_) => return Err(CsError::CsErrInvalidParam),
	}
    };

    let res =
	unsafe {
	    ffi::votequorum::votequorum_qdevice_register(handle.votequorum_handle, c_string. as_ptr())
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}


/// Unregister a quorum device
pub fn qdevice_unregister(handle: Handle, name: &String) -> Result<()>
{
    let c_string = {
	match CString::new(name.as_str()) {
	    Ok(cs) => cs,
	    Err(_) => return Err(CsError::CsErrInvalidParam),
	}
    };

    let res =
	unsafe {
	    ffi::votequorum::votequorum_qdevice_unregister(handle.votequorum_handle, c_string. as_ptr())
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}

/// Update the name of a quorum device
pub fn qdevice_update(handle: Handle, oldname: &String, newname: &String) -> Result<()>
{
    let on_string = {
	match CString::new(oldname.as_str()) {
	    Ok(cs) => cs,
	    Err(_) => return Err(CsError::CsErrInvalidParam),
	}
    };
    let nn_string = {
	match CString::new(newname.as_str()) {
	    Ok(cs) => cs,
	    Err(_) => return Err(CsError::CsErrInvalidParam),
	}
    };

    let res =
	unsafe {
	    ffi::votequorum::votequorum_qdevice_update(handle.votequorum_handle, on_string.as_ptr(), nn_string.as_ptr())
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}


/// Poll a quorum device
/// This must be done more often than the qdevice timeout (default 10s) while the device is active
/// and the [RingId] must match the current value returned from the callbacks for it to be accepted.
pub fn qdevice_poll(handle: Handle, name: &String, cast_vote: bool, ring_id: &RingId) -> Result<()>
{
    let c_string = {
	match CString::new(name.as_str()) {
	    Ok(cs) => cs,
	    Err(_) => return Err(CsError::CsErrInvalidParam),
	}
    };

    let c_cast_vote : u32 = if cast_vote {1} else {0};
    let c_ring_id = ffi::votequorum::votequorum_ring_id_t {
	nodeid: u32::from(ring_id.nodeid),
	seq: ring_id.seq};

    let res =
	unsafe {
	    ffi::votequorum::votequorum_qdevice_poll(handle.votequorum_handle, c_string.as_ptr(), c_cast_vote, c_ring_id)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}


/// Allow qdevice to tell votequorum if master_wins can be enabled or not
pub fn qdevice_master_wins(handle: Handle, name: &String, master_wins: bool) -> Result<()>
{
    let c_string = {
	match CString::new(name.as_str()) {
	    Ok(cs) => cs,
	    Err(_) => return Err(CsError::CsErrInvalidParam),
	}
    };

    let c_master_wins : u32 = if master_wins {1} else {0};

    let res =
	unsafe {
	    ffi::votequorum::votequorum_qdevice_master_wins(handle.votequorum_handle, c_string.as_ptr(), c_master_wins)
	};
    if res == ffi::votequorum::CS_OK {
	Ok(())
    } else {
	Err(CsError::from_c(res))
    }
}
