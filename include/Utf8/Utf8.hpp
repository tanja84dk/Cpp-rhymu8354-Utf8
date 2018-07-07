#ifndef UTF8_HPP
#define UTF8_HPP

/**
 * @file Utf8.hpp
 *
 * This module declares the Utf8::Utf8 class.
 *
 * © 2018 by Richard Walters
 */

#include <memory>
#include <string>
#include <ostream>
#include <vector>

namespace Utf8 {

    /**
     * This class is used to encode or decode Unicode "code points",
     * or characters from many different international character sets,
     *  in order to store or transmit them across any interface
     * that accepts a sequence of bytes.
     */
    class Utf8 {
        // Lifecycle management
    public:
        ~Utf8();
        Utf8(const Utf8&) = delete;
        Utf8(Utf8&&) = delete;
        Utf8& operator=(const Utf8&) = delete;
        Utf8& operator=(Utf8&&) = delete;

        // Public methods
    public:
        /**
         * This is the default constructor.
         */
        Utf8();

        // Private properties
    private:
        /**
         * This is the type of structure that contains the private
         * properties of the instance.  It is defined in the implementation
         * and declared here to ensure that it is scoped inside the class.
         */
        struct Impl;

        /**
         * This contains the private properties of the instance.
         */
        std::unique_ptr< struct Impl > impl_;
    };

}

#endif /* UTF8_HPP */
