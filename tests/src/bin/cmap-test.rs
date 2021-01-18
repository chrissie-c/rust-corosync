// Test the CMAP library. Requires that corosync is running and that we are root.

extern crate rust_corosync as corosync;
use corosync::{cmap};

fn track_notify_fn(_handle: &cmap::Handle,
		   _track_handle: &cmap::TrackHandle,
		   event: cmap::TrackType,
		   key_name: &String,
		   old_value: &cmap::Data,
		   new_value: &cmap::Data,
		   user_data: u64)
{
    println!("Track notify callback");
    println!("Key: {}, event: {}, user_data: {}", key_name, event, user_data);
    println!("   Old value: {}", old_value);
    println!("   New value: {}", new_value);
}


fn main()
{
    let handle =
	match cmap::initialize(cmap::Map::Icmap) {
	    Ok(h) => {
		println!("cmap initialized.");
		h
	    }
	    Err(e) => {
		println!("Error in CMAP (Icmap) init: {}", e);
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

    let test_d = cmap::Data::UInt64(0xdeadbeefcafebabe);
    match cmap::set(handle, "test.test_data".to_string(), &test_d)
    {
	Ok(_) => {}
	Err(e) => {
	    println!("Error in CMAP set_data: {}", e);
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
    match cmap::get(handle, "test.test_int16".to_string())
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

    match cmap::get(handle, "test.test_data".to_string())
    {
	Ok(v) => {
	    println!("GOT data value {:}", v);
	}

	Err(e) => {
	    println!("Error in CMAP get: {}", e);
	    return;
	}
    };

    // Test an iterator
    match cmap::CmapIterStart::new(handle, &"totem.".to_string()) {
	Ok(cmap_iter) => {
	    for i in cmap_iter {
		println!("ITER: {:?}", i);
	    }
	    println!("");
	}
	Err(e) => {
	    println!("Error in CMAP iter start: {}", e);
	}
    }

    // Close this handle
    match cmap::finalize(handle)
    {
	Ok(_) => {}
	Err(e) => {
	    println!("Error in CMAP get: {}", e);
	    return;
	}
    };


    // Test notifications on the stats map
    let handle =
	match cmap::initialize(cmap::Map::Stats) {
	    Ok(h) => h,
	    Err(e) => {
		println!("Error in CMAP (Stats) init: {}", e);
		return;
	    }
	};

    let cb = cmap::NotifyCallback{notify_fn: track_notify_fn};
    let _track_handle =
	match cmap::track_add(handle, &"stats.srp.memb_merge_detect_tx".to_string(),
			      cmap::TrackType::MODIFY |cmap::TrackType::ADD | cmap::TrackType::DELETE,
			      &cb,
			      997u64) {
	    Ok(th) => th,
	    Err(e) => {
		println!("Error in CMAP track_add {}", e);
		return;
	    }
	};

    // Wait for events
    let mut event_num = 0;
    loop {
	match cmap::dispatch(handle, corosync::DispatchFlags::One) {
	    Ok(_) => {}
	    Err(e) => println!("Error from CMAP dispatch: {}", e)
	}
	// Just do 5
	event_num += 1;
	if event_num > 5 {
	    break;
	}
    }
}
