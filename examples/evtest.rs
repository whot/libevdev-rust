extern crate libevdev;

use libevdev::*;
use std::fs::File;

fn usage() {
    println!("Usage: evtest /path/to/device");
}

fn print_info() {
}

fn main() {
    let mut args = std::env::args();

    if args.len() != 2 {
        usage();
        std::process::exit(1);
    }

    let path = &args.nth(1).unwrap();
    let f = File::open(path).unwrap();

    let mut d = Device::new();
    d.set_fd(f).unwrap();
}
