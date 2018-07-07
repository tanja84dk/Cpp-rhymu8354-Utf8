/**
 * @file Utf8Tests.cpp
 *
 * This module contains the unit tests of the
 * Utf8::Utf8 class.
 *
 * © 2018 by Richard Walters
 */

#include <gtest/gtest.h>
#include <stdint.h>
#include <Utf8/Utf8.hpp>
#include <vector>

TEST(Utf8Tests, AsciiToUnicode) {
    const std::vector< Utf8::UnicodeCodePoint > expectedCodePoints{ 0x48, 0x65, 0x6C, 0x6C, 0x6F };
    const auto actualCodePoints = Utf8::AsciiToUnicode("Hello");
    Utf8::Utf8 utf8;
}
