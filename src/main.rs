pub mod generator;
pub mod parser;
pub mod tokenizer;

use std::{env, error::Error, fs, path::Path, process::Command};

use generator::Generator;
use parser::Parser;
use tokenizer::{Token, Tokenizer};

fn main() -> Result<(), Box<dyn Error>> {
    // Collect command line arguments into a vector.
    let args: Vec<String> = env::args().collect();

    // Check for the correct number of arguments.
    if args.len() < 2 {
        eprintln!("Incorrect usage, provide *.hy file:"); // Use eprintln! for errors
        eprintln!("shpiller <input.hy>");
        return Err("Missing input file".into());
    }

    // Read the content of the provided file.
    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path)?;

    // Check if the file content is empty.
    if file_content.is_empty() {
        return Err("Input file is empty".into());
    }

    // Tokenize the file content.
    let tokenizer = Tokenizer::new(file_content);
    let tokens = tokenizer.tokenize();

    let mut parser = Parser::new(tokens);

    let tree = parser.parse();

    if tree.is_none() {
        panic!("No exit statement found");
    }

    let generator = Generator::new(tree.unwrap());

    let out_path = Path::new(file_path).with_extension("asm");
    fs::write(&out_path, generator.generate())?;

    assemble_and_link(&out_path)?;

    Ok(())
}

/// Assembles and links the generated assembly file using `nasm` and `ld`.
///
/// # Arguments
///
/// * `asm_path` - Path to the generated assembly file.
///
/// # Returns
///
/// * `Result<()>` - Ok if the assembly and linking processes succeed, Err otherwise.
fn assemble_and_link(asm_path: &Path) -> Result<(), Box<dyn Error>> {
    // Assemble the file using NASM.
    let status_nasm = Command::new("nasm").arg("-felf64").arg(asm_path).status()?;

    if !status_nasm.success() {
        return Err("Failed to assemble the code with NASM.".into());
    }

    // Prepare paths for linking.
    let o_path = asm_path.with_extension("o");
    let exec_path = format!("{}", asm_path.with_extension("").display());

    // Link the object file using ld.
    let status_ld = Command::new("ld")
        .arg(&o_path)
        .arg("-o")
        .arg(&exec_path)
        .status()?;

    if !status_ld.success() {
        return Err("Failed to link the object file.".into());
    }

    Ok(())
}
