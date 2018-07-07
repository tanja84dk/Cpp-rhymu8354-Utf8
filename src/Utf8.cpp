/**
 * @file Utf8.cpp
 *
 * This module contains the implementation of the Utf8::Utf8 class.
 *
 * © 2018 by Richard Walters
 */

#include <Utf8/Utf8.hpp>

namespace Utf8 {

    /**
     * This contains the private properties of a Utf8 instance.
     */
    struct Utf8::Impl {
    };

    Utf8::~Utf8() = default;

    Utf8::Utf8()
        : impl_(new Impl)
    {
    }

}
