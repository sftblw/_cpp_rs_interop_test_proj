#include "library.h"

#include <iostream>
#include <cstdint>
extern "C" {
    void hello() {
        std::cout << "Hello, World!" << std::endl;
        std::cout << "such debugging" << std::endl;
    }

    uint32_t meaning_of_life() {
        int seven = 7;
        int six = 6;
        int meaning = seven * six;
        return meaning;
    }

    void die_die_die() {
        throw "나죽는다";
    }
}

