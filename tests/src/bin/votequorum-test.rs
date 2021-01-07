// Test the VOTEQUORUM library. Requires that corosync is running and that we are root.

extern crate rust_corosync as corosync;
use corosync::votequorum;

fn quorum_fn(_handle: u64,
	     _context: u64,
	     quorate: bool,
	     member_list: Vec<votequorum::Node>)
{
    println!("TEST votequorum_quorum_fn called. quorate = {}", quorate);
    println!("  members: {:?}", member_list);
}


fn nodelist_fn(_handle: u64,
	       _context: u64,
	       ring_id: votequorum::RingId,
	       member_list: Vec<u32>)
{
    println!("TEST nodelist_fn called for {}/{}", ring_id.nodeid, ring_id.seq);
    println!("  members: {:?}", member_list);
}

fn expectedvotes_fn(_handle: u64,
		    _context: u64,
		    expected_votes: u32)
{
    println!("TEST expected_votes_fn called: value is {}", expected_votes);
}



fn main() {

    // Initialise the model data
    let cb = votequorum::Callbacks {
	quorum_notification_fn: quorum_fn,
	nodelist_notification_fn: nodelist_fn,
	expectedvotes_notification_fn: expectedvotes_fn,
    };

    let handle =
	match votequorum::initialize(&cb) {
	    Ok(h) => {
		println!("Votequorum initialized.");
		h
	    }
	    Err(e) => {
		println!("Error in VOTEQUORUM init: {}", e);
		return;
	    }
	};

    // Test context APIs
    let set_context: u64=0xabcdbeefcafe;
    match votequorum::context_set(handle, set_context) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in VOTEQUORUM context_set: {}", e);
	    return;
	}
    }

    // NOTE This will fail on 32 bit systems because void* is not u64
    match votequorum::context_get(handle) {
	Ok(c) => {
	    if c != set_context {
		println!("Error: context_get() returned {:x}, context should be {:x}", c, set_context);
	    }
	},
	Err(e) => {
	    println!("Error in VOTEQUORUM context_get: {}", e);
	}
    }

    const QDEVICE_NAME : &str = "RustQdevice";

    match votequorum::qdevice_register(handle, &QDEVICE_NAME.to_string()) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in VOTEQUORUM qdevice_register: {}", e);
	}
    }

    match votequorum::get_info(handle, 1 as u32) {
	Ok(i) => {
	    println!("Node info for nodeid 1");
	    println!("  nodeid: {}", i.node_id);
	    println!("  node_state: {:?}", i.node_state);
	    println!("  node_votes: {}", i.node_votes);
	    println!("  node_expected: {}", i.node_expected_votes);
	    println!("  highest_expected: {}", i.highest_expected);
	    println!("  quorum: {}", i.quorum);
	    println!("  flags: {:x}", i.flags);
	    println!("  qdevice_votes: {}", i.qdevice_votes);
	    println!("  qdevice_name: {}", i.qdevice_name);

	    if i.qdevice_name != QDEVICE_NAME {
		println!("qdevice names do not match: s/b: \"{}\"  is: \"{}\"", QDEVICE_NAME, i.qdevice_name);
	    }
	},
	Err(e) => {
	    println!("Error in VOTEQUORUM get_info: {} (check nodeid 1 has been online)", e);
	}
    }

    match votequorum::qdevice_unregister(handle, &QDEVICE_NAME.to_string()) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in VOTEQUORUM qdevice_unregister: {}", e);
	}
    }

    match votequorum::trackstart(handle, 99 as u64, corosync::TrackFlags::Changes) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in VOTEQUORUM trackstart: {}", e);
	    return;
	}
    }

    // Wait for events
    loop {
	votequorum::dispatch(handle, corosync::DispatchFlags::All).unwrap();
    }
}
