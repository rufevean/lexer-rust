// Title: lexer
//
use std::fs;
#[derive(Debug, PartialEq)]
enum Token {
    Identifier(String),
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Bang,
    Equal,
    LessThan,
    GreaterThan,
    LParen,
    RParen,
    Print,
    EOF,
    StringLiteral(String),
}

struct Lexer {
    input: String,
    position: usize,
}

impl Lexer {
    fn consume(&mut self) {
        self.position += 1;
    }
    fn quit_whitespace(&mut self) {
        while self.input.as_bytes()[self.position] == b' ' {
            self.consume();
        }
    }
    fn next_token(&mut self) -> Token {
        self.quit_whitespace();

        if let Some(c) = self.current_char() {
            match c {
                // handling operators
                b'+' => {
                    self.consume();
                    Token::Plus
                }
                b'-' => {
                    self.consume();
                    Token::Minus
                }
                b'*' => {
                    self.consume();
                    Token::Asterisk
                }

                b'!' => {
                    self.consume();
                    Token::Bang
                }
                b'=' => {
                    self.consume();
                    Token::Equal
                }
                b'<' => {
                    self.consume();
                    Token::LessThan
                }
                b'>' => {
                    self.consume();
                    Token::GreaterThan
                }
                b'(' => {
                    self.consume();
                    Token::LParen
                }
                b')' => {
                    self.consume();
                    Token::RParen
                }
                b'/' => {
                    self.consume();
                    Token::Slash
                }
                // handling print statement
                b'p' => {
                    if self.input[self.position..].starts_with("print") {
                        self.position += 5;
                        Token::Print
                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }
                // handling single line comments
                b'#' => {
                    while let Some(c) = self.current_char() {
                        if c == b'\n' {
                            break;
                        }
                        self.consume();
                    }

                    self.next_token()
                }
                // handling new line
                b'\n' => {
                    self.consume();
                    self.next_token()
                }
                //escape characters
                b'\\' => {
                    self.consume();
                    self.next_token()
                } 

                // handling multi line comments
                b'"' => {
                    if self.input[self.position..].starts_with("\"\"\"") {
                        self.consume();
                        self.position += 3;
                        while let Some(c) = self.current_char() {
                            if c == b'"' {
                                if self.input[self.position..].starts_with("\"\"\"") {
                                    self.position += 3;
                                    break;
                                }
                            }
                            self.consume();
                        }
                        self.next_token()
                    } else if self.input[self.position..].starts_with("\"") {
                        let start = self.position;
                        let end = self.input[self.position + 1..].find("\"").unwrap();
                        while let Some(c) = self.current_char() {
                            if c == b'"' {
                                self.consume();
                                break;
                            }
                            self.consume();
                        }
                        self.position += end;
                        Token::StringLiteral(self.input[start..self.position].to_string())

                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }

                //handing variables
                _ => {
                    if c.is_ascii_digit() {
                        Token::Number(self.read_number())
                    } else if c.is_ascii_alphabetic() {
                        Token::Identifier(self.read_identifier())
                    } else {
                        panic!("Unknown token: {}", c as char);
                    }
                }
            }
        } else {
            Token::EOF
        }
    }
    fn current_char(&self) -> Option<u8> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input.as_bytes()[self.position])
        }
    }
    fn read_number(&mut self) -> i32 {
        let mut number = String::new();
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                number.push(c as char);
                self.consume();
            } else {
                break;
            }
        }
        number.parse().unwrap()
    }
    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if c.is_ascii_alphabetic() {
                identifier.push(c as char);
                self.consume();
            } else {
                break;
            }
        }
        identifier
    }
}
fn main() {
    let input = read_file("test_files/main.py");
    println!("{}", input);

    let mut lexer = Lexer {
        input: input.to_string(),
        position: 0,
    };

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
