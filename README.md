
# TernLSB

TernLSB is an esoteric programming language that uses steganography to store its programs within images.

## How to Run

### Prerequisites

Ensure you have Rust and Cargo installed on your system. If not, you can install them from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

### Build

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/ternLSB-rs.git
cd ternLSB-rs
```

### Usage

#### Run TernLSB Program

To interpret a TernLSB program from an image file, use:

`cargo run <filename>`

Replace `<filename>` with the path to the image file containing the TernLSB program.

#### Encode Brainfuck Program into Image

To encode a Brainfuck program into an image file for TernLSB, use:

`cargo run <inputfn> <brainfuckfn> <outputfn>`

Replace `<inputfn>` with the image file, `<brainfuckfn>` with the path to the Brainfuck program file, and `<outputfn>` with the desired output image file.

