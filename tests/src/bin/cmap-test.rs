// Test the CMAP library. Requires that corosync is running and that we are root.

extern crate rust_corosync as corosync;
use corosync::{cmap};

fn main()
{
    let handle =
	match cmap::initialize(cmap::Map::Icmap) {
	    Ok(h) => {
		println!("cmap initialized.");
		h
	    }
	    Err(e) => {
		println!("Error in CMAP init: {}", e);
		return;
	    }
	};


    // Test some SETs
    match cmap::set_u32(handle, "test.test_uint32".to_string(), 456)
    {
	Ok(_) => {}
	Err(e) => {
	    println!("Error in CMAP set_u32: {}", e);
	    return;
	}
    };

    match cmap::set_i16(handle, "test.test_int16".to_string(), -789)
    {
	Ok(_) => {}
	Err(e) => {
	    println!("Error in CMAP set_i16: {}", e);
	    return;
	}
    };

    match cmap::set_string(handle, "test.test_string".to_string(), "Hello from Rust".to_string())
    {
	Ok(_) => {}
	Err(e) => {
	    println!("Error in CMAP set_string: {}", e);
	    return;
	}
    };

    // get them back again
    match cmap::get(handle, "test.test_uint32".to_string())
    {
	Ok(v) => {
	    println!("GOT value {}", v);
	}

	Err(e) => {
	    println!("Error in CMAP get: {}", e);
	    return;
	}
    };

    match cmap::get(handle, "test.test_string".to_string())
    {
	Ok(v) => {
	    println!("GOT value {}", v);
	}

	Err(e) => {
	    println!("Error in CMAP get: {}", e);
	    return;
	}
    };

    
    // Wait for events
    // loop {
    // 	match cmap::dispatch(handle, corosync::DispatchFlags::One) {
    // 	    Ok(_) => {}
    // 	    Err(_) => break,
    // 	}
    // }
    // println!("ERROR: Corosync quit");
}
