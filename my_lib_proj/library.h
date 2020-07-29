#ifndef CPPRSINTEROPTEST_LIBRARY_H
#define CPPRSINTEROPTEST_LIBRARY_H

#include <cstdint>

extern "C" {
    struct Calculator;

    void hello();
    uint32_t meaning_of_life();
    void die_die_die();

    Calculator* calculator_new();
    int32_t calculator_add(Calculator* self, int32_t a);
    void calculator_delete(Calculator* self);
};

#endif //CPPRSINTEROPTEST_LIBRARY_H
