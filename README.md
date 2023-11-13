
```markdown
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

Build the project:

```bash
cargo build --release
```

### Usage

#### Run TernLSB Program

To interpret a TernLSB program from an image file, use:

```bash
cargo run --release -- <filename>
```

Replace `<filename>` with the path to the image file containing the TernLSB program.

#### Encode Brainfuck Program into Image

To encode a Brainfuck program into an image file for TernLSB, use:

```bash
cargo run --release -- <inputfn> <brainfuckfn> <outputfn>
```

Replace `<inputfn>` with the image file, `<brainfuckfn>` with the path to the Brainfuck program file, and `<outputfn>` with the desired output image file.

### Examples

#### Interpret TernLSB Program

```bash
cargo run --release -- examples/hello_world.png
```

#### Encode Brainfuck Program

```bash
cargo run --release -- examples/input_image.png examples/brainfuck_program.bf examples/output_image.png
```

### Notes

- TernLSB programs must be stored in a lossless compressed or uncompressed image type that uses RGB or RGBA color types (e.g., BMP and PNG).
