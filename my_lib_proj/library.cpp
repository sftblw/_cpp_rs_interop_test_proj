#include "library.h"

#include <iostream>
#include <cstdint>
#include "Calculator.h"


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

    Calculator* calculator_new() {
        return new Calculator();
    }
    int32_t calculator_add(Calculator* self, int32_t a) {
        return reinterpret_cast<Calculator*>(self)->add_1(a);
    }
    void calculator_delete(Calculator* self) {
        delete reinterpret_cast<Calculator*>(self);
    }
}

