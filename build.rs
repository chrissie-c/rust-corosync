fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS") == Ok("macos".to_string()) {
        return;
    }

    println!("cargo:rustc-link-lib=cpg");
    println!("cargo:rustc-link-lib=cfg");
    println!("cargo:rustc-link-lib=cmap");
    println!("cargo:rustc-link-lib=quorum");
    println!("cargo:rustc-link-lib=votequorum");
}
