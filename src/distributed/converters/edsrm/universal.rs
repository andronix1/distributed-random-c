use std::{ptr::null_mut, slice};

use distributed_random::distributed::converters::edsrm::EdsrmUniversalDistributionConverter;

use crate::distributed::wrapper::DistributionConverterWrapper;

#[no_mangle]
pub extern "C" fn create_edsrm_universal_converter(
    ranges: *const f64,
    ranges_count: usize,
    distribution: unsafe extern "C" fn(f64) -> f64,
    majorant_size: usize
) -> *mut DistributionConverterWrapper {
    if let Ok(converter) = EdsrmUniversalDistributionConverter::new(
            unsafe { slice::from_raw_parts(ranges, ranges_count) }.to_vec(),
            unsafe {
                std::mem::transmute(distribution)
            },
            majorant_size
        ) {
            Box::into_raw(Box::new(DistributionConverterWrapper { converter: Box::new(converter) }))
        } else {
            null_mut()
        }
}