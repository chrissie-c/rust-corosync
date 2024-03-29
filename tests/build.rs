extern crate pkg_config;

fn main() {
    if let Err(e) = pkg_config::probe_library("libcpg") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
		"Pkg-config failed - usually this is because corosync development headers are not installed.\n\n\
                For Fedora users:\n# dnf install corosynclib-devel\n\n\
                For Debian/Ubuntu users:\n# apt-get install libcpg-dev\n\n\
                pkg_config details:\n{}",
		e
	    ),
            _ => panic!("{}", e),
        }
    }
    if let Err(e) = pkg_config::probe_library("libquorum") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
		"Pkg-config failed - usually this is because corosync development headers are not installed.\n\n\
                For Fedora users:\n# dnf install corosynclib-devel\n\n\
                For Debian/Ubuntu users:\n# apt-get install libquorum-dev\n\n\
                pkg_config details:\n{}",
		e
	    ),
            _ => panic!("{}", e),
        }
    }
    if let Err(e) = pkg_config::probe_library("libvotequorum") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
		"Pkg-config failed - usually this is because corosync development headers are not installed.\n\n\
                For Fedora users:\n# dnf install corosynclib-devel\n\n\
                For Debian/Ubuntu users:\n# apt-get install libvotequorum-dev\n\n\
                pkg_config details:\n{}",
		e
	    ),
            _ => panic!("{}", e),
        }
    }
    if let Err(e) = pkg_config::probe_library("libcfg") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
		"Pkg-config failed - usually this is because corosync development headers are not installed.\n\n\
                For Fedora users:\n# dnf install corosynclib-devel\n\n\
                For Debian/Ubuntu users:\n# apt-get install libcfg-dev\n\n\
                pkg_config details:\n{}",
		e
	    ),
            _ => panic!("{}", e),
        }
    }
    if let Err(e) = pkg_config::probe_library("libcmap") {
        match e {
            pkg_config::Error::Failure { .. } => panic! (
		"Pkg-config failed - usually this is because corosync development headers are not installed.\n\n\
                For Fedora users:\n# dnf install corosynclib-devel\n\n\
                For Debian/Ubuntu users:\n# apt-get install libcmap-dev\n\n\
                pkg_config details:\n{}",
		e
	    ),
            _ => panic!("{}", e),
        }
    }
    println!("cargo:rustc-link-lib=cpg");
    println!("cargo:rustc-link-lib=cfg");
    println!("cargo:rustc-link-lib=cmap");
    println!("cargo:rustc-link-lib=quorum");
    println!("cargo:rustc-link-lib=votequorum");
}
