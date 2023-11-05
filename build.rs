//! Build script.



#![deny(warnings)]



use std::fs::{ self, File };
use std::io::{ Write };
use std::path::{ PathBuf };



fn main() {
    // Get output directory.
    let out = PathBuf::from( std::env::var("OUT_DIR").expect("Could not get output directory.") );

    // Get current directory.
    let current = std::env::current_dir().expect("Could not get current directory.");

    // Relocate the linker.
    linker(&out, &current);

    // Indicate to cargo when to rebuild this crate.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");

    // Indicate to cargo where to search for the linker.
    println!("cargo:rustc-link-search={}", out.display());
}



fn linker(out: &PathBuf, current: &PathBuf) {
    // Create the path of the output file.
    let path = out.join("memory.x");

    // Create the output file.
    let mut file = File::create(path).expect("Could not create linker output file");

    // Look for the linker file.
    let input = current.join("memory.x");

    // Copy the linker.
    file.write_all(&fs::read(input).expect("Could not read content of 'memory.x'")).expect("Failed to write 'memory.x' to output file");

    println!("cargo:rustc-link-search={}", out.display());
}
