fn main() {
    variables();
}

// 1. Variables are immutable by default.
// 2. Constants can be declared in any scope including the global scope, and can't be set to the result of a value that could only be computed at runtime, and must be type annotated, and its rust naming convention is to use all uppercase with underscores between words.

fn variables() {
    let mut x: isize = 5; // Architecture size
                          // The primary situation in which you'd use isize or usize is when indexing some sort of collection
    println!("{}", x); // This works.
    x = 6;
    println!("{x}"); // And this also works.
                     //const Y: i32 = 6 * x; This doesn't work

    // Shadowing:
    let n: u8 = 5;
    let n: u8 = n + 1;
    {
        let n: u8 = n * 2;
        println!("{n}");
    }
    println!("{n}");

    // Int Literals
    let a = 57u8;
    let b = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byte = b'A'; // u8 only
    println!("{a}, {b}, {hex}, {oct}, {bin}, {byte}");

    // // Integer Overflow
    // let temp: u16 = 256;
    // let c: u8 = temp as u8; // When you're compiling in debug mode, Rust includes checks for integer overflow that cause you program to panic at runtime, but if you're compiling in release mode with the --release flag (i.e: cargo build --release), Rust does not include checks for integer overflow that cause panics, instead Rust performs two's complement wrapping, however, the compiler still performs other checks, such as range checks for literals assigned to fixed-size integer types like 'u8', relying on integer overflow's wrapping behavior is considered an error for more information go to page 38
    // println!("{c}");

    // Floating-point numbers are represented according to the IEEE-754 standard (which I honestly don't know anything about it).

    /*
        To understand it you will need to understand significant figures or digits:

        Significant digits, also known as significant figures, are the digits in a number that carry meaning contributing to its precision. They are crucial for expressing the accuracy and precision of a measurement or calculation.

        Here are some rules to determine significant digits:

        1. All nonzero digits are considered significant. For example, in the number 123.45, all digits (1, 2, 3, 4, 5) are significant.
        2. Zeros between nonzero digits are significant. For example, in the number 5003, all digits (5, 0, 0, 3) are significant.
        3. Leading zeros (zeros to the left of the first nonzero digit) are not significant. For example, in the number 0.0056, only the digits 5 and 6 are significant.
        4. Trailing zeros (zeros to the right of the last nonzero digit) are significant if they are after the decimal point. For example, in the number 4.00, all digits (4, 0, 0) are significant.
        5. Trailing zeros that are simply placeholders (without a decimal point) are not considered significant unless otherwise indicated. For example, in the number 1200, if it's meant to convey that there are four significant digits, it should be written as 1200.
    */

    /*
        The 3 basic components of the IEEE-754 Standard:
            1. The Sign of Mantissa: 0 represents a positive number while 1 represents a negative number.
            2. The Biased exponent: In floating-point representation, numbers are stored in a format that consists of three parts: the sign bit, the exponent, and the fraction (also called the mantissa or significand). The exponent field represents the power of the number, which can be positive or negative.

            To represent both positive and negative exponents in a fixed number of bits, a bias is added to the actual exponent before storing it. This bias shifts the range of representable exponents so that they are centered around zero.

            For example, let's say we have a floating-point format with 8 bits for the exponent field. If we use a bias of 127, then an exponent of 0 would be represented as 127 (because 0 + 127 = 127), an exponent of 1 would be represented as 128 (because 1 + 127 = 128), and so on. Similarly, a negative exponent like -1 would be represented as 126 (because -1 + 127 = 126).

            This biasing technique allows us to store both positive and negative exponents in a fixed number of bits, making floating-point representation more versatile and efficient.
            3. The Normalised Mantissa: The mantissa is part of a number in scientific notation or a floating-point number, consisting of its significant digits. Here we have only 2 digits, i.e. 0 and 1. So a normalised mantissa is one with only one 1 to the left of the decimal.

            The formula to Calculate it is (-1)^sign x(1 + significand (23 or 52 bit)) X 2^exponent-bias
    */
    let num: f32 = 4.36; // Change this to the number you want to convert

    let bits: u32 = num.to_bits();

    println!("{:032b}", bits); // Print the binary representation

    println!("{}, {}", -5 / 3, -5 % 3); // truncated

    // let b: bool = false OR true;

    // let c: char = '😀'; It must be '' => unicode scalar value
    // Unicode Scalar Value. Any Unicode code point except high-surrogate and low-surrogate code points. In other words, the ranges of integers 0 to D7FF16 and E00016 to 10FFFF16 inclusive.
    // U+xxxx is called the code point
    // The Unicode codespace is the range of integers from 0 to 10FFFF (hexadecimal notation)
    // Not all possible values within this range correspond to an encoded character. Some values are reserved for future use, while others might be used for special purposes (like control characters).
    // Encoded Characters: Specific code points that have been assigned a particular character, like 'A' or '你'.
    // A Unicode encoding form assigns each Unicode scalar value to a unique code unit sequence.
    // For historical reasons, the Unicode encoding forms are also referred to as Unicode (or UCS) transformation formats (UTF). That term is actually ambiguous between its usage for encoding forms and encoding schemes.
    // Encoding Forms: To store and manipulate the code points, Unicode provides different encoding forms, the most common being UTF-8, UTF-16, and UTF-32. Each of these encoding forms represents code points as sequences of one or more "code units".

    // Code Units:

    // UTF-8: Uses 8-bit code units. Each code point can be represented by one to four 8-bit code units.
    // UTF-16: Uses 16-bit code units. Each code point can be represented by one or two 16-bit code units (the latter using a pair of "surrogate" code units for characters outside the Basic Multilingual Plane).
    // UTF-32: Uses 32-bit code units. Each code point is represented by a single 32-bit code unit.
    // The Basic Multilingual Plane (BMP) is the range of Unicode code points from U+0000 to U+FFFF. This plane includes the most commonly used characters.

    // UTF-16 can represent code points that fall within the BMP directly using a single 16-bit code unit. However, for code points outside the BMP (i.e., those from U+10000 to U+10FFFF), UTF-16 uses surrogate pairs. This means that two 16-bit code units are used together to represent a single character.
    // UTF-16 reserves two specific ranges within the 16-bit space for surrogates:
    // High Surrogates (Lead Surrogates): U+D800 to U+DBFF
    // Low Surrogates (Trail Surrogates): U+DC00 to U+DFFF
    // These ranges are reserved solely for constructing surrogate pairs and are not used to directly encode characters from the BMP.

    // Encoding Non-BMP Characters: When encoding a character outside the BMP:
    // Subtract 0x10000 from the code point to get a 20-bit value.
    // Split this 20-bit value into two 10-bit halves.
    // Add 0xD800 to the high 10 bits to get the high surrogate. (1)
    // Add 0xDC00 to the low 10 bits to get the low surrogate. (2)
    // 1, 2 are the character

    // UTF-16LE (UTF-16 Little Endian) and UTF-16BE (UTF-16 Big Endian) are two encoding schemes within the UTF-16 Unicode encoding format, differing primarily in byte order. In UTF-16LE, the least significant byte (LSB) of each 16-bit code unit comes first, followed by the most significant byte (MSB), while in UTF-16BE, this order is reversed, with the most significant byte preceding the least significant byte. This distinction in byte order is crucial for interoperability between systems with different endianness preferences. UTF-16LE is commonly used in modern computing environments, especially on little-endian systems like those based on Intel x86 architecture, while UTF-16BE is less common but necessary for big-endian systems. Despite this difference in byte order, both encoding schemes represent the same Unicode characters, ensuring compatibility across platforms through standardized encoding conventions.

    // Control characters are special non-printing characters in the Unicode and ASCII character sets used to control the interpretation or display of text, rather than to represent printable information. They were originally designed for communication control in text terminals and telecommunication systems.

    // UTF-8
    // ASCII characters (U+0000 to U+007F): Represented as single bytes with the same value as their ASCII code.
    // Characters in the range U+0080 to U+07FF: Represented as two bytes. 11-bits at most
    // Represented as: 110xxxxx 10xxxxxx
    // Characters in the range U+0800 to U+FFFF: Represented as three bytes.
    // Represented as: 1110xxxx 10xxxxxx 10xxxxxx
    // Characters in the range U+10000 to U+10FFFF: Represented as four bytes.
    // Represented as: 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx

    // Characters in rust are encoded in UTF-8 by default.
    // encoding_rs: This crate offers encoding and decoding functionalities with a focus on UTF-16 (both little-endian and big-endian) alongside UTF-8.
    // For more granular control, you can work with raw byte slices (&[u8]) and handle the encoding yourself. This approach requires a deeper understanding of character encoding and is generally less recommended due to potential for errors.
}
