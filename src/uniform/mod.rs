pub mod wrapper;
pub mod generators;

use libc::c_double;
use self::wrapper::UniformRandomGeneratorWrapper;

#[no_mangle]
pub extern "C" fn generate_next(wrapper: *mut UniformRandomGeneratorWrapper) -> c_double {
    unsafe { wrapper.as_mut() }.unwrap().generator.next()
}