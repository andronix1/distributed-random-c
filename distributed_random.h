#pragma once

// uniform
typedef void uniform_rand_gen;
typedef double(*uniform_generate)();

extern uniform_rand_gen *new_multiplicative_generator();
extern uniform_rand_gen *new_custom_uniform_generator(uniform_generate generate_function);
extern double generate_next(uniform_rand_gen* generator);

extern void free_uniform_generator(uniform_rand_gen* generator);

// converters
typedef double(*simple_function)();
typedef void distribution_converter;
extern double generate_from_uniform(distribution_converter* converter, uniform_rand_gen* uniform);

extern distribution_converter* create_idfm_converter(simple_function* inverse_distribution);
extern distribution_converter* create_edsrm_monotous_converter(
    simple_function distribution,
    double start, 
    double end,
    unsigned int majorant_size
);
extern distribution_converter* create_edsrm_universal_converter(
    double* ranges,
    unsigned int ranges_count,
    simple_function distribution,
    unsigned int majorant_size_per_range
);

extern void free_converter(distribution_converter* converter);