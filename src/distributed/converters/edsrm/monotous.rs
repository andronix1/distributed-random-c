use std::ptr::null_mut;

use distributed_random::distributed::converters::edsrm::EdsrmMonotousDistributionConverter;

use crate::distributed::wrapper::DistributionConverterWrapper;

#[no_mangle]
pub extern "C" fn create_edsrm_monotous_converter(
    distribution: unsafe extern "C" fn(f64) -> f64,
    start: f64, end: f64,
    majorant_size: usize
) -> *mut DistributionConverterWrapper {
    if let Ok(converter) = EdsrmMonotousDistributionConverter::new(
            unsafe {
                std::mem::transmute(distribution)
            },
            start, end,
            majorant_size
        ) {
            Box::into_raw(Box::new(DistributionConverterWrapper { converter: Box::new(converter) }))
        } else {
            null_mut()
        }
}