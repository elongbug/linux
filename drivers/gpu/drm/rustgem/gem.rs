// SPDX-License-Identifier: MIT

use kernel::{
    drm::{gem, gem::shmem},
    prelude::*,
};

use crate::file::DrmFile;
use crate::{VgemDevice, VgemDriver};

/// Represents the inner data of a GEM object for this driver.
#[pin_data]
pub(crate) struct DriverObject {}

/// Type alias for the shmem GEM object type for this driver.
pub(crate) type Object = shmem::Object<DriverObject>;

impl gem::BaseDriverObject<Object> for DriverObject {
    type Initializer = impl PinInit<Self, Error>;

    /// Callback to create the inner data of a GEM object
    fn new(_dev: &VgemDevice, _size: usize) -> Self::Initializer {
        pr_info!("DriverObject::new size = {}\n", _size);
        try_pin_init!(DriverObject {
        })
    }

    /// Callback to drop all mappings for a GEM object owned by a given `File`
    fn close(_obj: &Object, _file: &DrmFile) {}
}

impl shmem::DriverObject for DriverObject {
    type Driver = VgemDriver;
}
