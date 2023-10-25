use distributed_random::uniform::UniformRandomGenerator;

use crate::uniform::wrapper::UniformRandomGeneratorWrapper;

pub struct CustomUniformGenerator {
    generate: unsafe extern "C" fn() -> f64
}

impl UniformRandomGenerator for CustomUniformGenerator {
    fn next(&mut self) -> f64 {
        unsafe { (self.generate)() }
    }
}

#[no_mangle]
pub extern "C" fn new_custom_uniform_generator(generate: unsafe extern "C" fn() -> f64) -> *mut UniformRandomGeneratorWrapper {
    Box::into_raw(
        Box::new(
            UniformRandomGeneratorWrapper { 
                generator: Box::new(CustomUniformGenerator {
                    generate
                })
            }
        )
    )
}
