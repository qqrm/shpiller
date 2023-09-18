# Shpiller Compiler

A minimalistic compiler that translates a custom programming language (`.hy` files) into assembly language, written in Rust.

## Inspiration

This project is inspired by the YouTube channel [Pixeled](https://www.youtube.com/playlist?list=PLUDlas_Zy_qC7c5tCgTMYq2idyyT241qs). While Pixeled's compiler is written in C++, this version seeks to explore the same concepts using the Rust programming language.

## Features

- **Tokenization**: Converts raw source code into a series of tokens.
- **Compilation**: Translates the tokenized source code into assembly language.
- **Integration**: Automatically assembles and links the generated assembly code.

## Getting Started

1. **Clone the Repository**:

    ```bash
    git clone https://github.com/qqrm/shpiller.git
    cd shpiller
    ```

2. **Compile the Code**:

    ```bash
    cargo build --release
    ```

3. **Run the Compiler**:

    ```bash
    target/release/shpiller your_source_code.hy
    ```

    This will produce an assembly file and an executable from your `.hy` source code.

## Dependencies

- Ensure you have the Rust toolchain installed. If not, get it from [rustup.rs](https://rustup.rs/).
- The system should have `nasm` and `ld` installed and available in the system's PATH.

## Contribution

Feel free to fork the project and submit pull requests. All contributions are welcome!

## License

This project is open source and available under the [MIT License](LICENSE).

## Acknowledgements

- Huge shoutout to [Pixeled](https://www.youtube.com/@pixeled-yt) for the inspiration and knowledge. If you're interested in programming, definitely check out his channel.
