extern crate libc;
use libc::{_SC_PAGESIZE, sysconf};

// TODO: This only works for *nix systems
fn get_page_size() -> u32 {
    unsafe {
        sysconf(_SC_PAGESIZE) as u32
    }
}

fn main() {
    println!("{:?}", get_page_size());
}

