extern crate fuse;
extern crate time;
extern crate libc;

mod filesystem;

use std::env;

fn main() {
     let mountpoint = env::args_os().nth(1).unwrap();
     let backing_file = env::args_os().nth(2).unwrap();

     let fs = filesystem::initialize(backing_file).unwrap();
    fuse::mount(fs, &mountpoint, &[]);
}
