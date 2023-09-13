//!  Scull module in Rust.

use kernel::prelude::*;
use kernel::{file,
             miscdev,
            };
use kernel::io_buffer::IoBufferWriter;

module! {
    type: Scull,
    name: "scull",
    author: "Rust for Linux Contributors",
    description: "Rust Scull sample",
    license: "GPL",
}

// latest version will have new pin macros
//#[pin_data]
struct Scull {
    // latest version will have new pin macros
    //#[pin]
    _dev: Pin<Box<miscdev::Registration<Scull>>>,
}

#[vtable]
impl file::Operations for Scull {
    //fn open(context: &Self::OpenData, file: &File) -> Result<Self::Data>;
    fn open(_context: &(), _file: &file::File) -> Result {
        pr_info!("File was opened\n");
        Ok(())
    }

    /*
    fn read(
        _data: <Self::Data as ForeignOwnable>::Borrowed<'_>,
        _file: &File,
        _writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        Err(EINVAL)
    }

    // need to check below type
    // <Self::Data as ForeignOwnable>::Borrowed<'_>
    // type Borrowed<'a>;
    */
    fn read(_data: (),
            _file: &file::File,
            _writer: &mut impl IoBufferWriter,
            _offset: u64,
    ) -> Result<usize> {
        pr_info!("File was read\n");
        Ok(0)
    }
}

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world!\n");
        // latest version will have new pin macros
        // such as ... Box::pin_init(miscdev::Registration::new(fmt!("scull"), ()))?;
        // new_pinned(name: fmt::Arguments<'_>, open_data: T::OpenData) -> Result<Pin<Box<Self>>>
        let reg = miscdev::Registration::new_pinned(fmt!("scull"), ())?;
        Ok(Self{_dev: reg})
    }
}
