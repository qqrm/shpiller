use std::{env, error::Error, fs};

#[derive(Debug, PartialEq)]
enum TokenType {
    r#return,
    int_literal(i32),
    semicolon,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Incorrect usage, provide *.hy file:");
        println!("    shpiller <input.hy>");
        return Err("Missing input file".into());
    }

    let file_path = &args[1];
    let file_content = fs::read_to_string(file_path)?;

    if file_content.is_empty() {
        return Err("Input file is empty".into());
    }

    let tokens = tokenize(file_content);
    dbg!(&tokens);

    println!("let's compile");

    Ok(())
}

fn tokenize(input: String) -> Vec<TokenType> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();

    for ch in input.chars() {
        match ch {
            ' ' | ';' => {
                if !current_token.is_empty() {
                    if current_token == "return" {
                        tokens.push(TokenType::r#return);
                    } else if let Ok(value) = current_token.parse::<i32>() {
                        tokens.push(TokenType::int_literal(value));
                    }

                    current_token.clear();
                }

                if ch == ';' {
                    tokens.push(TokenType::semicolon);
                }
            }
            _ => current_token.push(ch),
        }
    }

    if !current_token.is_empty() {
        if current_token == "return" {
            tokens.push(TokenType::r#return);
        } else if let Ok(value) = current_token.parse::<i32>() {
            tokens.push(TokenType::int_literal(value));
        }
    }

    tokens
}
