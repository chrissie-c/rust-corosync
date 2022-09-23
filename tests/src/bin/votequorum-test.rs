// Test the VOTEQUORUM library. Requires that corosync is running and that we are root.

extern crate rust_corosync as corosync;
use corosync::votequorum;
use std::thread::spawn;

static EXAMPLE_NODEID: std::sync::Mutex<u32> = std::sync::Mutex::new(0);

fn quorum_fn(_handle: &votequorum::Handle,
	     context: u64,
	     quorate: bool,
	     member_list: Vec<votequorum::Node>)
{
    println!("TEST votequorum_quorum_fn called. quorate = {}", quorate);
    println!("  members: {:?}", member_list);
    if context != 99_u64 {
	println!("Context in quorum_fn is not 99, got {}", context);
	std::process::exit(2);
    }
}

fn nodelist_fn(_handle: &votequorum::Handle,
	       context: u64,
	       ring_id: votequorum::RingId,
	       member_list: Vec<corosync::NodeId>)
{
    println!("TEST nodelist_fn called for {}/{}", ring_id.nodeid, ring_id.seq);
    println!("  members: {:?}", member_list);
    if context != 99_u64 {
	println!("Context in nodelist_fn is not 99, got {}", context);
	std::process::exit(2);
    }
    *EXAMPLE_NODEID.lock().unwrap() = u32::from(member_list[0]);
}

fn expectedvotes_fn(_handle: &votequorum::Handle,
		    context: u64,
		    expected_votes: u32)
{
    println!("TEST expected_votes_fn called: value is {}", expected_votes);
    if context != 99_u64 {
	println!("Context in expectedvotes_fn is not 99, got {}", context);
	std::process::exit(2);
    }
}

fn dispatch_routine(handle: votequorum::Handle)
{
    // Wait for events
    loop {
	if votequorum::dispatch(handle, corosync::DispatchFlags::One).is_err() {
	    break;
	}
    }
}

fn main() {

    // Initialise the model data
    let cb = votequorum::Callbacks {
	quorum_notification_fn: Some(quorum_fn),
	nodelist_notification_fn: Some(nodelist_fn),
	expectedvotes_notification_fn: Some(expectedvotes_fn),
    };

    let handle =
	match votequorum::initialize(&cb) {
	    Ok(h) => {
		println!("Votequorum initialized.");
		h
	    }
	    Err(e) => {
		println!("Error in VOTEQUORUM init: {}", e);
		std::process::exit(1);
	    }
	};

    let handle_clone = handle.clone();
    let _dispatch_thread = spawn(move || {
	dispatch_routine(handle_clone)}
    );

    // Test context APIs
    let set_context: u64=0xabcdbeefcafe;
    if let Err(e) = votequorum::context_set(handle, set_context) {
	println!("Error in VOTEQUORUM context_set: {}", e);
	std::process::exit(1);
    }

    // NOTE This will fail on 32 bit systems because void* is not u64
    match votequorum::context_get(handle) {
	Ok(c) => {
	    if c != set_context {
		println!("Error: context_get() returned {:x}, context should be {:x}", c, set_context);
		std::process::exit(2);
	    }
	},
	Err(e) => {
	    println!("Error in VOTEQUORUM context_get: {}", e);
	    std::process::exit(1);
	}
    }

    if let Err(e) = votequorum::trackstart(handle, 99_u64, corosync::TrackFlags::Changes) {
	println!("Error in VOTEQUORUM trackstart: {}", e);
	std::process::exit(1);
    }

    const QDEVICE_NAME : &str = "RustQdevice";
    if let Err(e) = votequorum::qdevice_register(handle, QDEVICE_NAME) {
	println!("Error in VOTEQUORUM qdevice_register: {}", e);
	std::process::exit(1);
    }

    // Make sure we have a nodeid to get_info for
    let mut t = 0;
    while t == 0 { t = *EXAMPLE_NODEID.lock().unwrap();
    }

    let example_nodeid = { *EXAMPLE_NODEID.lock().unwrap()
    };

    match votequorum::get_info(handle, corosync::NodeId::from(example_nodeid)) {
	Ok(i) => {
	    println!("Node info for nodeid {}", example_nodeid);
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
		std::process::exit(2);
	    }
	    if u32::from(i.node_id) != example_nodeid {

		println!("Got wrong nodeid for get_info({}) request", example_nodeid);
		std::process::exit(2);
	    }
	},
	Err(e) => {
	    println!("Error in VOTEQUORUM get_info: {} (check nodeid {} has been online)", e,
		     example_nodeid);
	    std::process::exit(1);
	}
    }

    if let Err(e) = votequorum::qdevice_unregister(handle, QDEVICE_NAME) {
	println!("Error in VOTEQUORUM qdevice_unregister: {}", e);
	std::process::exit(1);
    }

    // Let it all finish
    std::thread::sleep(std::time::Duration::new(1,0));
}
