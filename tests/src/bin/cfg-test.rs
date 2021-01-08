// Test the CFG library. Requires that corosync is running and that we are root.

extern crate rust_corosync as corosync;
use corosync::cfg;

use std::thread::spawn;

fn dispatch_thread(handle: u64)
{
    loop {
	println!("Calling dispatch");
	match cfg::dispatch(handle, corosync::DispatchFlags::One) {
	    Ok(_) => {}
	    Err(_) => return,
	}
	println!("Called dispatch");
    }
}


// Test the shutdown callback
fn shutdown_check_fn(handle:u64, _flags: u32)
{
    println!("in shutdown callback");

    // DON'T shutdown corosync - we're just testing
    match cfg::reply_to_shutdown(handle, cfg::ShutdownReply::No) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in CFG replyto_shutdown: {}", e);
	}
    }
}


fn main()
{
    // Initialise the callbacks data
    let cb = cfg::Callbacks {
	corosync_cfg_shutdown_callback_fn: shutdown_check_fn,
    };

    let handle =
	match cfg::initialize(&cb) {
	    Ok(h) => {
		println!("cfg initialized.");
		h
	    }
	    Err(e) => {
		println!("Error in CFG init: {}", e);
		return;
	    }
	};

    // Open two handles to CFG so that the second one can refuse shutdown
    let handle2 =
	match cfg::initialize(&cb) {
	    Ok(h) => {
		println!("cfg2 initialized.");
		h
	    }
	    Err(e) => {
		println!("Error in CFG init: {}", e);
		return;
	    }
	};
    // Run handle2 dispatch in its own thread
    spawn(move || dispatch_thread(handle2));

    let our_nodeid = {
	match cfg::local_get(handle) {
	    Ok(n) => {
		println!("Local nodeid is {}", n);
		n
	    },
	    Err(e) => {
		println!("Error in CFG local_get: {}", e);
		0
	    }
	}
    };

    // Get node info for us
    if our_nodeid != 0 {
	match cfg::node_status_get(handle, our_nodeid, cfg::NodeStatusVersion::V1) {
	    Ok(ns) => {
		println!("Node Status for nodeid {}", ns.nodeid);
		println!("   reachable: {}", ns.reachable);
		println!("   remote: {}", ns.remote);
		println!("   onwire_min: {}", ns.onwire_min);
		println!("   onwire_max: {}", ns.onwire_max);
		println!("   onwire_ver: {}", ns.onwire_ver);
		let mut ls_num = 0;
		for ls in ns.link_status {
		    ls_num += 1;
		    if ls.enabled {
			println!("   Link {}", ls_num);
			println!("      connected: {}", ls.connected);
			println!("      mtu: {}", ls.mtu);
			println!("      src: {}", ls.src_ipaddr);
			println!("      dst: {}", ls.dst_ipaddr);
		    }
		}
	    }
	    Err(e) => {
		println!("Error in CFG node_status get: {}", e);
	    }
	}
    }

// -- corosync_cfg_tryshutdown() is currently broken in the daemon (since 2012)
// 815375411e80131f31b172d7c43625769ee8b53d
    
// This should not shutdown corosync because the callback on handle2 will refuse it.
//    match cfg::try_shutdown(handle, cfg::ShutdownFlags::Request) {
//	Ok(_) => {},
//	    Err(e) => {
//		println!("Error in CFG try_shutdown: {}", e);
//	    }
//  }

    // Wait for events
    loop {
	match cfg::dispatch(handle, corosync::DispatchFlags::One) {
	    Ok(_) => {}
	    Err(_) => break,
	}
    }
    println!("Corosync quit");
}
