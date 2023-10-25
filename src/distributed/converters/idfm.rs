use distributed_random::distributed::converters::IdfmDistributionConverter;

use crate::distributed::wrapper::DistributionConverterWrapper;

#[no_mangle]
pub extern "C" fn create_idfm_converter(inverse_distribution: unsafe extern "C" fn(f64) -> f64) -> *mut DistributionConverterWrapper {
    Box::into_raw(
        Box::new(
            DistributionConverterWrapper { 
                converter: Box::new(
                    IdfmDistributionConverter::new(unsafe {
                        std::mem::transmute(inverse_distribution)
                    })
                ),
            }
        )
    )
}