//!  Scull module in Rust.

use kernel::prelude::*;
use kernel::{file,
             miscdev,
            };
use kernel::io_buffer::{IoBufferWriter,
                        IoBufferReader};
//use kernel::sync::Ref;
use kernel::sync::{Arc,
                   ArcBorrow,
                   UniqueArc,
                   Mutex};

module! {
    type: Scull,
    name: "scull",
    author: "Rust for Linux Contributors",
    description: "Rust Scull sample",
    license: "GPL",
}

// latest version will have new pin macros
//#[pin_data]
struct Device {
    number: usize,
    // latest version will have new pin macros
    //#[pin]
    contents: Mutex<Vec<u8>>,
}

// latest version will have new pin macros style for initialization
impl Device {
    /*
       // Allocate a boxed `Device`.
	   // let d = Box::pin_init(Device::try_new())?;

       // will Arc have Arc::pin_init? => needs to check below
       // let d = Arc::pin_init(Device::try_new())?;

    fn try_new() -> impl PinInit<Self> {
        pin_init!(Self {
            number: 10,
            contents <- new_mutex!(Vec::new()),
        })
    */
    fn try_new() -> Result<Arc<Self>> {
        let mut dev = Pin::from(UniqueArc::try_new(Self {
            number: 0,
            // SAFETY: `mutex_init!` is called below.
            contents: unsafe {Mutex::new(Vec::new())},
        })?);

        // SAFETY: `connents' is pinned when `dev` is.
        let pinned = unsafe {dev.as_mut().map_unchecked_mut(|s| &mut s.contents) };
        kernel::mutex_init!(pinned, "Device::contents");

        Ok(dev.into())
    }
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
            writer: &mut impl IoBufferWriter,
            offset: u64,
    ) -> Result<usize> {
        // data.number deref coercion does not work on rust-analyzer
        //pr_info!("File for device {} was read\n", data.number);

        let offset = offset.try_into()?;
        let vec = data.contents.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&vec[offset..][..len])?;  // => is it the combination of &vec[offset..] and &vec[..len] ?
        //writer.write_slice(&vec[offset..len])?;  // => what is the difference between &vec[offset..][..len] and &vec[offset..len] ?
        pr_info!("File for device {} was read size: {}\n", data.number, len);

        Ok(len)
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
        //pr_info!("File for device {} was written\n", data.number);
        let copy = reader.read_all()?;

        let mut contents = data.contents.lock();
        *contents = copy;

        pr_info!("File for device {} was written size: {}\n", data.number, contents.len());

        Ok(contents.len())
    }

}

impl kernel::Module for Scull {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world!\n");
        // latest version will have new pin macros
        // such as ... Box::pin_init(miscdev::Registration::new(fmt!("scull"), ()))?;
        // new_pinned(name: fmt::Arguments<'_>, open_data: T::OpenData) -> Result<Pin<Box<Self>>>

        let dev = Device::try_new()?;
        let reg = miscdev::Registration::new_pinned(fmt!("scull"), dev)?;

        Ok(Self{_dev: reg})
    }
}
