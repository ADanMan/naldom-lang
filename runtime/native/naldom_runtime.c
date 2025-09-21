// runtime/native/naldom_runtime.c

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <time.h>

// Forward declaration for the Rust function we will link against.
// The actual implementation is in the `naldom-runtime` crate.
void naldom_async_sleep(uint64_t ms);


// --- Existing Runtime Code (unchanged) ---

// A simple struct to act as a "fat pointer" for our arrays,
// containing both the data and its size.
typedef struct {
    double* data;
    int64_t size;
} NaldomArray;

// This function is called from our compiled code.
NaldomArray* create_random_array(int64_t size) {
    printf("Runtime: Creating an array of %lld random numbers...\n", size);

    // Seed the random number generator
    srand(time(NULL));

    // Allocate memory for our array struct
    NaldomArray* array_struct = (NaldomArray*)malloc(sizeof(NaldomArray));
    if (!array_struct) return NULL;

    // Allocate memory for the actual double values
    array_struct->data = (double*)malloc(size * sizeof(double));
    if (!array_struct->data) {
        free(array_struct);
        return NULL;
    }
    array_struct->size = size;

    // Fill the array with random doubles between 0.0 and 100.0
    for (int i = 0; i < size; ++i) {
        array_struct->data[i] = (double)rand() / RAND_MAX * 100.0;
    }

    return array_struct;
}

// A comparison function for qsort
int compare_doubles_asc(const void* a, const void* b) {
    double val1 = *(const double*)a;
    double val2 = *(const double*)b;
    if (val1 < val2) return -1;
    if (val1 > val2) return 1;
    return 0;
}

int compare_doubles_desc(const void* a, const void* b) {
    return compare_doubles_asc(b, a); // Reverse order
}

void sort_array(NaldomArray* arr, int64_t order) {
    if (!arr || !arr->data) return;
    printf("Runtime: Sorting the array...\n");
    
    if (order == 1) { // 1 for descending
         qsort(arr->data, arr->size, sizeof(double), compare_doubles_desc);
    } else { // 0 for ascending
         qsort(arr->data, arr->size, sizeof(double), compare_doubles_asc);
    }
}

void print_array(NaldomArray* arr) {
    if (!arr || !arr->data) return;
    
    printf("\n--- Naldom Native Output ---\n[");
    for (int i = 0; i < arr->size; ++i) {
        printf("%.2f%s", arr->data[i], (i == arr->size - 1) ? "" : ", ");
    }
    printf("]\n--------------------------\n\n");
    
    fflush(stdout);
}