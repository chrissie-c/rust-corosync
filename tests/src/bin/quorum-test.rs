// Test the QUORUM library. Requires that corosync is running and that we are root.

extern crate rust_corosync as corosync;
use corosync::quorum;
//use std::str;

fn quorum_fn(_handle: u64,
	     quorate: bool,
	     ring_id: quorum::RingId,
	     member_list: Vec<u32>)
{
    println!("TEST quorum_fn called. quorate = {}", quorate);
    println!("  ring_id: {}/{}", ring_id.nodeid, ring_id.seq);
    println!("  members: {:?}", member_list);
}


fn nodelist_fn(_handle: u64,
	       ring_id: quorum::RingId,
	       member_list: Vec<u32>,
	       joined_list: Vec<u32>,
	       left_list: Vec<u32>)
{
    println!("TEST nodelist_fn called for {}/{}", ring_id.nodeid, ring_id.seq);
    println!("  members: {:?}", member_list);
    println!("  joined: {:?}", joined_list);
    println!("  left: {:?}", left_list);
}



fn main() {

    // Initialise the model data
    let md = quorum::ModelData::ModelV1 (
	quorum::Model1Data {
	    flags: quorum::Model1Flags::None,
	    quorum_notification_fn: quorum_fn,
	    nodelist_notification_fn: nodelist_fn,
	}
    );

    let handle =
	match quorum::initialize(&md, 99 as u64) {
	    Ok((h, t)) => {
		println!("Quorum initialized; type = {}", t as u32);
		h
	    }
	    Err(e) => {
		println!("Error in QUORUM init: {}", e);
		return;
	    }
	};

    // Test context APIs
    let set_context: u64=0xabcdbeefcafe;
    match quorum::context_set(handle, set_context) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in QUORUM context_set: {}", e);
	    return;
	}
    }

    // NOTE This will fail on 32 bit systems because void* is not u64
    match quorum::context_get(handle) {
	Ok(c) => {
	    if c != set_context {
		println!("Error: context_get() returned {:x}, context should be {:x}", c, set_context);
	    }
	},
	Err(e) => {
	    println!("Error in QUORUM context_get: {}", e);
	}
    }


    match quorum::trackstart(handle, corosync::TrackFlags::Changes) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in QUORUM trackstart: {}", e);
	    return;
	}
    }

    // Wait for events
    loop {
	quorum::dispatch(handle, corosync::DispatchFlags::All).unwrap();
    }
}