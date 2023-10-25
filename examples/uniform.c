#include <stdio.h>
#include <stdlib.h>
#include "../distributed_random.h"

void print_mathematical_expectation(const char *name, uniform_rand_gen* generator) {
    const int count = 10000000;
    double sum = 0;
    for (int i = 0; i < count; i++) {
        sum += generate_next(generator);
    }
    printf("%s: %f\n", name, sum / count);
}

double uniform() {
    return (double)rand() / RAND_MAX;
}

int main() {
    printf("getting the mathematical expectation of uniform_rand_gen:\n");
    {
        uniform_rand_gen* generator = new_multiplicative_generator();
        print_mathematical_expectation("miltiplicative", generator);
        free_uniform_generator(generator);
    }
    {
        uniform_rand_gen* generator = new_custom_uniform_generator(uniform);
        print_mathematical_expectation("stdlib", generator);
        free_uniform_generator(generator);
    }
}