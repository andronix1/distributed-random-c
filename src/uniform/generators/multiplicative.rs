use distributed_random::uniform::MultiplicativeRandomGenerator;

use crate::uniform::wrapper::UniformRandomGeneratorWrapper;

#[no_mangle]
pub extern "C" fn new_multiplicative_generator() -> *mut UniformRandomGeneratorWrapper {
    Box::into_raw(
        Box::new(
            UniformRandomGeneratorWrapper { 
                generator: Box::new(MultiplicativeRandomGenerator::new())
            }
        )
    )
}
