use std::{alloc::System, env, error::Error, fs, path::Path, process::Command};

/// Represents the types of tokens that can be produced by the lexer.
#[derive(Debug, PartialEq)]
enum TokenType {
    /// Represents the 'return' keyword.
    Return,
    /// Represents an integer literal.
    IntLiteral(i32),
    /// Represents the ';' symbol.
    Semicolon,
}

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
    let tokens = tokenize(&file_content); // Pass a &str

    // Convert tokens to assembly language.
    let asm = tokens_to_asm(tokens);
    dbg!(&asm);

    let out_path = Path::new(file_path).with_extension("asm");
    // let output = fs::File::create()?;
    fs::write(out_path.clone(), asm)?;

    // Assemble the file using NASM
    let status_nasm = Command::new("nasm")
        .arg("-felf64")
        .arg(&out_path)
        .status()?; // Returns the exit status of the command

    if !status_nasm.success() {
        return Err("Failed to assemble the code with NASM.".into());
    }

    let o_path = out_path.with_extension("o");
    let path = format!("{}", out_path.with_extension("").display()); // Get the file path without any extension

    // Link the object file using ld
    let status_ld = Command::new("ld")
        .arg(&o_path)
        .arg("-o")
        .arg(&path)
        .status()?; // Returns the exit status of the command

    if !status_ld.success() {
        return Err("Failed to link the object file.".into());
    }

    Ok(())
}

/// Convert a list of tokens into assembly language representation.
///
/// # Arguments
///
/// * `tokens` - A vector of tokens to be converted to assembly.
///
/// # Returns
///
/// A string containing the assembly language representation of the tokens.
fn tokens_to_asm(tokens: Vec<TokenType>) -> String {
    tokens
        .into_iter()
        .map(|token| -> String {
            match token {
                TokenType::Return => "    mov rax, 60\n    mov rdi, ".to_string(),
                TokenType::IntLiteral(value) => value.to_string(),
                TokenType::Semicolon => "\n    syscall\n".to_string(),
            }
        })
        .fold(
            String::from("global _start\n_start:\n"),
            |out, token_asm| out + &token_asm,
        )
}

/// Tokenize the given input into a list of `TokenType` values.
///
/// # Arguments
///
/// * `input` - A string slice containing the source code to tokenize.
///
/// # Returns
///
/// A vector of tokens representing the source code.
fn tokenize(input: &str) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for ch in input.chars() {
        match ch {
            ' ' | ';' => {
                if !current_token.is_empty() {
                    if current_token == "return" {
                        tokens.push(TokenType::Return);
                    } else if let Ok(value) = current_token.parse::<i32>() {
                        tokens.push(TokenType::IntLiteral(value));
                    }

                    current_token.clear();
                }

                if ch == ';' {
                    tokens.push(TokenType::Semicolon);
                }
            }
            _ => current_token.push(ch),
        }
    }

    if !current_token.is_empty() {
        if current_token == "return" {
            tokens.push(TokenType::Return);
        } else if let Ok(value) = current_token.parse::<i32>() {
            tokens.push(TokenType::IntLiteral(value));
        }
    }

    tokens
}
