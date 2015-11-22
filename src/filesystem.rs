use fuse::Filesystem;
use std::fs::File;
use std::io;
use std::path::Path;

pub struct SimpleFS {
    backing_file: File,
}

pub fn initialize<P: AsRef<Path>>(file: P) -> io::Result<SimpleFS> {
    let backing_file = try!(File::open(file));
    Ok(SimpleFS {
        backing_file: backing_file,
    })
}

impl Filesystem for SimpleFS {
}
