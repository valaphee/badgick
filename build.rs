use std::{env, fs, io, path};

fn main() -> io::Result<()> {
    let out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out_dir.display());

    //fs::write(out_dir.join("device.x"), include_bytes!("device.x"))?;
    println!("cargo:rerun-if-changed=device.x");
    fs::write(out_dir.join("memory.x"), include_bytes!("memory.x"))?;
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rustc-link-arg=-Tlink.x");

    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
