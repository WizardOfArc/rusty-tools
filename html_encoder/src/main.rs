use std::{fmt::Error, fs::File};
use std::io::{stdin, BufReader, BufRead, Write};

#[derive(Debug)]
enum Token { 
    Literal(String),
    LBrace,
    RBrace,
    LAngle,
    RAngle,
    DoubleQuote,
    SingleQuote,
    Ampersand,
    LineBreak,
    Semicolon,
    Space,
}

fn post_literal(tokens: &mut Vec<Token>, literal: &mut String) -> bool {
    if literal.len() > 0 {
        tokens.push(Token::Literal(literal.clone()));
        literal.clear();
        true
    } else {
        false
    }
}

fn process_file(file: &str) -> Result<Vec<Token>, Error> {
    let file_result = File::open(file);

    match file_result {
        Err(e) => {
            println!("Error opening file: {}", e);
            return Err(Error);
        },
        Ok(f) => {
            let reader = BufReader::new(f);
            let mut tokens: Vec<Token> = Vec::new();
            let mut literal: String = String::new();
            for line_result in reader.lines() {
                match line_result {
                    Err(e) => {
                        println!("Error reading line: {}", e);
                        return Err(Error);
                    },
                    Ok(line) => {
                        for c in line.chars() {
                            match c {
                                '{' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::LBrace);
                                },
                                '}' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::RBrace);
                                },
                                '<' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::LAngle);
                                },
                                '>' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::RAngle);
                                },
                                '"' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::DoubleQuote);
                                },
                                '\'' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::SingleQuote);
                                },
                                '&' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::Ampersand);
                                },
                                '\n' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::LineBreak);
                                },
                                ';' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::Semicolon);
                                },
                                ' ' => {
                                    post_literal(&mut tokens, &mut literal);
                                    tokens.push(Token::Space);
                                },
                                _ => literal.push(c),
                            }
                        }
                        post_literal(&mut tokens, &mut literal);
                        tokens.push(Token::LineBreak);
                    }
                }
            }
            Ok(tokens)
        }
    }
}

fn encode_html(tokens: Vec<Token>) -> String {
    let mut result: String = String::new();
    for token in tokens {
        match token {
            Token::Literal(s) => result.push_str(&s),
            Token::LBrace => result.push_str("&lbrace;"),
            Token::RBrace => result.push_str("&rbrace;"),
            Token::LAngle => result.push_str("&lt;"),
            Token::RAngle => result.push_str("&gt;"),
            Token::DoubleQuote => result.push_str("&quot;"),
            Token::SingleQuote => result.push_str("&apos;"),
            Token::Ampersand => result.push_str("&amp;"),
            Token::LineBreak => result.push_str("<br />"),
            Token::Semicolon => result.push_str("&semi;"),
            Token::Space => result.push_str(" "),
        }
    }
    result
}

fn main() {
    println!("Hello, there.");
    println!("What file would you like to encode? (I'll make a new file of course)");
    let mut file_name = String::new();
    stdin().read_line(&mut file_name).expect("Failed to read line");
    let new_file_name = format!("{}_encoded.html", file_name.trim());
    let result = process_file(file_name.trim());
    match result {
        Ok(tokens) => {
            let encoded = encode_html(tokens);
            let mut outfile = File::create(new_file_name).unwrap();
            outfile.write_all(encoded.as_bytes()).unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
