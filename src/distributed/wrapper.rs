use distributed_random::distributed::DistributionConverter;
use libc::c_double;

use crate::uniform::wrapper::UniformRandomGeneratorWrapper;

pub struct DistributionConverterWrapper {
    pub converter: Box<dyn DistributionConverter<UniformRandomGeneratorWrapper>>
}

#[no_mangle]
pub extern "C" fn generate_from_uniform(
    converter: *const DistributionConverterWrapper,
    uniform: *mut UniformRandomGeneratorWrapper
) -> c_double {
    unsafe { converter.as_ref() }.unwrap()
        .converter
        .generate_from_uniform(unsafe { uniform.as_mut() }.unwrap())
}

#[no_mangle]
pub extern "C" fn free_converter(converter: *mut DistributionConverterWrapper) {
    drop(unsafe { Box::from_raw(converter) })
}