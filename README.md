
# TernLSB

TernLSB is an esoteric programming language that uses steganography to store its programs within images.
https://esolangs.org/wiki/TernLSB

this is a port of original python interpreter by User:None1

## How to Run

### Prerequisites

Ensure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Usage

#### Run TernLSB Program

To interpret a TernLSB program from an image file, use:

`cargo run <filename>`

Replace `<filename>` with the path to the image file containing the TernLSB program.

#### Encode Brainfuck Program into Image

To encode a Brainfuck program into an image file for TernLSB, use:

`python ternlsb.py <inputfn> <brainfuckfn> <outputfn>`

Replace `<inputfn>` with the image file, `<brainfuckfn>` with the path to the Brainfuck program file, and `<outputfn>` with the desired output image file.


currently encoding only works in original python with the imaging library, but the rust interpreter does run TernLSB files
