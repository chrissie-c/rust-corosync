// Test the CPG library. Requires that corosync is running and that we are root.

extern crate rust_corosync as corosync;
use corosync::{cpg, NodeId};
use std::str;

fn deliver_fn(_handle: &cpg::Handle,
	      group_name: String,
	      nodeid: NodeId,
	      pid: u32,
	      msg: &[u8],
	      msg_len: usize)
{
    println!("TEST deliver_fn called for {}, from nodeid/pid {}/{}. len={}", group_name, nodeid, pid, msg_len);

    // Print as text if it's valid UTF8
    match  str::from_utf8(msg) {
	Ok(s) => println!("  {}", s),
	Err(_) => {
	    for i in 0..msg_len {
		print!("{:02x} ", msg[i]);
	    }
	    println!("");
	}
    }
}

fn confchg_fn(_handle: &cpg::Handle,
	      group_name: &str,
	      member_list: Vec<cpg::Address>,
	      left_list: Vec<cpg::Address>,
	      joined_list: Vec<cpg::Address>)
{
    println!("TEST confchg_fn called for {}", group_name);
    println!("  members: {:?}", member_list);
    println!("  left:    {:?}", left_list);
    println!("  joined:  {:?}", joined_list);
}


fn totem_confchg_fn(_handle: &cpg::Handle,
		    ring_id: cpg::RingId,
		    member_list: Vec<NodeId>)
{
    println!("TEST totem_confchg_fn called for {}/{}", ring_id.nodeid, ring_id.seq);
    println!("  members: {:?}", member_list);
}



fn main() {

    // Initialise the model data
    let md = cpg::ModelData::ModelV1 (
	cpg::Model1Data {
	    flags: cpg::Model1Flags::None,
	    deliver_fn,
	    confchg_fn,
	    totem_confchg_fn,
	}
	);

    let handle =
	match cpg::initialize(&md, 99 as u64) {
	    Ok(h) => h,
	    Err(e) => {
		println!("Error in CPG init: {}", e);
		return;
	    }
	};


    match cpg::join(handle, &"TEST".to_owned()) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in CPG join: {}", e);
	    return;
	}
    }

    match cpg::local_get(handle) {
	Ok(n) => {
	    println!("Local nodeid is {}", n);
	},
	Err(e) => {
	    println!("Error in CPG local_get: {}", e);
	}
    }

    // Test membership_get()
    match cpg::membership_get(handle, &"TEST".to_owned()) {
	Ok(m) => {
	    println!("  members: {:?}", m);
	    println!("");
	},
	Err(e) => {
	    println!("Error in CPG membership_get: {}", e);
	}
    }

    // Test context APIs
    let set_context: u64=0xabcdbeefcafe;
    match cpg::context_set(handle, set_context) {
	Ok(_) => {},
	Err(e) => {
	    println!("Error in CPG context_set: {}", e);
	    return;
	}
    }

    // NOTE This will fail on 32 bit systems because void* is not u64
    match cpg::context_get(handle) {
	Ok(c) => {
	    if c != set_context {
		println!("Error: context_get() returned {:x}, context should be {:x}", c, set_context);
	    }
	},
	Err(e) => {
	    println!("Error in CPG context_get: {}", e);
	}
    }

    // Test iterator
    let cpg_iter = cpg::CpgIterStart::new(handle, &"".to_owned(), cpg::CpgIterType::All).unwrap();
    for i in cpg_iter {
	println!("ITER: {:?}", i);
    }
    println!("");

    // We should receive our own message (at least) in the event loop
    match cpg::mcast_joined(handle, cpg::Guarantee::TypeAgreed,
			    &"This is a test".to_string().into_bytes()) {
	Ok(_) => {}
	Err(e) => {
	    println!("Error in CPG mcast_joined: {}", e);
	}
    }

    // Wait for events
    loop {
	match cpg::dispatch(handle, corosync::DispatchFlags::One) {
	    Ok(_) => {}
	    Err(_) => break,
	}
    }
    println!("ERROR: Corosync quit");
}
