#ifndef CPPRSINTEROPTEST_LIBRARY_H
#define CPPRSINTEROPTEST_LIBRARY_H

#include <cstdint>

extern "C" {
    void hello();
    uint32_t meaning_of_life();
    void die_die_die();
};
#endif //CPPRSINTEROPTEST_LIBRARY_H
