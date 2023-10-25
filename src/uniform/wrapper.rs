use distributed_random::uniform::UniformRandomGenerator;

pub struct UniformRandomGeneratorWrapper {
    pub generator: Box<dyn UniformRandomGenerator>
}

impl UniformRandomGenerator for UniformRandomGeneratorWrapper {
    fn next(&mut self) -> f64 {
        self.generator.next()
    }
}

#[no_mangle]
pub extern "C" fn free_uniform_generator(generator: *mut UniformRandomGeneratorWrapper) {
    drop(unsafe { Box::from_raw(generator) })
}