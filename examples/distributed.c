#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include "../distributed_random.h"

void print_distribution(
    const char *name, 
    double range_start,
    double range_end, 
    distribution_converter* converter, 
    uniform_rand_gen* generator
) {
    const int graph_size = 100;
    unsigned int graph[graph_size];
    const int count = 10000000;

    memset(graph, 0, sizeof(graph));
    
    for (int i = 0; i < count; i++) {
        double result = generate_from_uniform(converter, generator);
        if (result < range_start || result > range_end) {
            printf("%s[ERROR]: new converted value is not in range %f<=x<=%f", name, range_start, range_end);
            exit(EXIT_FAILURE);
        }
        graph[(int)floor(result * (range_end - range_start) * graph_size)]++;
    }

    printf("%s: ", name);
    for (int i = 0; i < graph_size; i++) {
        printf("%i ", graph[i]);
    }
    putchar('\n');
}

double inverse_distribution_function(double x) {
    return sqrt(x);
}

double distribution(double x) {
    return 2.0 * x;
}

int main() {
    printf("getting converters distribution\n");
    printf("https://www.rapidtables.com/tools/line-graph.html to see plot\n");
    uniform_rand_gen* generator = new_multiplicative_generator();
    {
        distribution_converter *idfm = create_idfm_converter(inverse_distribution_function);
        print_distribution("idfm", 0, 1, idfm, generator);
        free_converter(idfm);
    }
    {
        distribution_converter *edsrm_monotous = create_edsrm_monotous_converter(
            distribution,
            0, 1,
            330
        );
        if (edsrm_monotous == NULL) { 
            printf("failed to create edsrm_monotous converter");
            return EXIT_FAILURE;
        }
        print_distribution("edsrm_monotous", 0, 1, edsrm_monotous, generator);
        free_converter(edsrm_monotous);
    }
    {
        #define ranges_length 5
        double ranges[ranges_length] = {0.0, 0.3, 0.6, 0.7, 1.0};
        distribution_converter *edsrm_universal = create_edsrm_universal_converter(
            ranges,
            ranges_length,
            distribution,
            330
        );
        if (edsrm_universal == NULL) { 
            printf("failed to create edsrm_universal converter");
            return EXIT_FAILURE;
        }
        print_distribution("edsrm_universal", 0, 1, edsrm_universal, generator);
        free_converter(edsrm_universal);
    }
    free_uniform_generator(generator);
    return EXIT_SUCCESS;
}