//!  Scull module in Rust.

use kernel::prelude::*;
use kernel::{file,
             miscdev,
            };
use kernel::io_buffer::{IoBufferWriter,
                        IoBufferReader};
//use kernel::sync::Ref;
use kernel::sync::{Arc,
                   ArcBorrow};

module! {
    type: Scull,
    name: "scull",
    author: "Rust for Linux Contributors",
    description: "Rust Scull sample",
    license: "GPL",
}


struct Device {
    number: usize,
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
    type OpenData = Arc<Device>;
    type Data = Arc<Device>;

    //fn open(context: &Self::OpenData, file: &File) -> Result<Self::Data>;
    fn open(context:&Arc<Device>, _file: &file::File) -> Result<Arc<Device>> {
        // context.number deref coercion does not work on rust-analyzer
        pr_info!("File for device {} was opened\n", context.number);
        Ok(context.clone())
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
    fn read(data: ArcBorrow<'_, Device>,
            _file: &file::File,
            _writer: &mut impl IoBufferWriter,
            _offset: u64,
    ) -> Result<usize> {
        // data.number deref coercion does not work on rust-analyzer
        pr_info!("File for device {} was read\n", data.number);
        Ok(0)
    }

    /*
    fn write(
        _data: <Self::Data as ForeignOwnable>::Borrowed<'_>,
        _file: &File,
        _reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        Err(EINVAL)
    }
    */
    fn write(data: ArcBorrow<'_, Device>,
             _file: &file::File,
             reader: &mut impl IoBufferReader,
             _offset: u64
    ) -> Result<usize> {
        pr_info!("File for device {} was written\n", data.number);
        Ok(reader.len())
    }

}

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world!\n");
        // latest version will have new pin macros
        // such as ... Box::pin_init(miscdev::Registration::new(fmt!("scull"), ()))?;
        // new_pinned(name: fmt::Arguments<'_>, open_data: T::OpenData) -> Result<Pin<Box<Self>>>
        let dev = Arc::try_new(Device {number: 0})?;
        let reg = miscdev::Registration::new_pinned(fmt!("scull"), dev)?;

        Ok(Self{_dev: reg})
    }
}
