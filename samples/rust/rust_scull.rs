//!  Scull module in Rust.

use kernel::prelude::*;
use kernel::file;

module! {
    type: Scull,
    name: "scull",
    author: "Rust for Linux Contributors",
    description: "Rust Scull sample",
    license: "GPL",
}

struct Scull;

#[vtable]
impl file::Operations for Scull {
    //fn open(context: &Self::OpenData, file: &File) -> Result<Self::Data>;
    fn open(_context: &(), _file: &file::File) -> Result {
        pr_info!("File was opened\n");
        Ok(())
    }
}

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world!\n");
        Ok(Self)
    }
}
