// RUST_LOG=info cargo run
// RUST_LOG=debug cargo run
// cargo test
use log::{debug, info, warn};
use std::fmt::Display;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn debug() {
    info!("In the debug, running lib!");
    let file_name = "config_1";
    let config_file = file_name.clone().to_owned() + ".txt";
    let expected_file = file_name.clone().to_owned() + "_set.txt";
    let config = open_config_file(&config_file);

    let mut config_writer = ConfigWriter::new(config.clone());
    let config_writer_result = config_writer.write_configs();
    println!("{config_writer_result}");
    let expected = open_config_file(&expected_file);
    println!(
        "Should be:\n
{expected}"
    );
    assert_eq!(config_writer_result, expected);
}

// Open a configuration file and return the content as a String:
pub fn open_config_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match fs::File::open(&path) {
        Err(error) => panic!("could not open {}: {}", display, error),
        Ok(file) => file,
    };

    let mut string = String::new();

    match file.read_to_string(&mut string) {
        Err(error) => panic!("could not read {}: {}", display, error),
        Ok(_) => {
            return string;
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
enum Token {
    Identifier(String),
    Eof,
    LeftSquirly,
    RightSquirly,
    Semicolon,
    Unkown(String),
    Start,
    LeftBracket,
    RightBracket,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Unkown(x) => write!(f, "Unkown({:#?})", x),
            Token::Identifier(x) => write!(f, "Identifier(\"{}\")", x),
            Token::Eof => write!(f, "Eof"),
            Token::LeftSquirly => write!(f, "LeftSquirly"),
            Token::RightSquirly => write!(f, "RightSquirly"),
            Token::Semicolon => write!(f, "SemiColon"),
            Token::Start => write!(f, "Start"),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
        };
    }
}

struct Lexer {
    source: Vec<char>,
    position: usize,
    read_position: usize,
    character: char,
    end: usize,
}

impl Lexer {
    fn new(source: String) -> Lexer {
        let characters: Vec<char> = source.chars().collect();
        let end = characters.len();
        let mut lex = Lexer {
            source: characters,
            position: 0,
            read_position: 0,
            character: '0',
            end: end,
        };

        lex.read_char();
        return lex;
    }
    fn read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.character = '0';
        } else {
            self.character = self.source[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position == self.end {
            return Token::Eof;
        }
        let token = match self.character {
            '{' => Token::LeftSquirly,
            '}' => Token::RightSquirly,
            ';' => Token::Semicolon,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            'a'..='z' | 'A'..='Z' | '"' | '-' | '0'..='9' | '^' | '<' | '*' | '>' => {
                let statement = self.read_identifier();
                match statement.as_str() {
                    "placeholder" => Token::Identifier(String::from("placeholder")),
                    _ => Token::Identifier(statement),
                }
            }
            _ => Token::Unkown(String::from(self.character as char)),
        };

        self.read_char();
        return token;
    }
    fn skip_whitespace(&mut self) {
        while self.character == ' ' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        // deal with idenfiers that are enclosed with a comma:
        if self.character == '"' {
            self.read_char(); // read the first comma
            while self.character != '"' {
                self.read_char();
            }
        }

        while !self.character.is_whitespace() {
            self.read_char();
        }
        let word: String = String::from_iter(&self.source[position..self.position]);
        return word;
    }
}

struct ConfigWriter {
    tokens: Vec<Token>,
    token: Token,
    position: usize,
    read_position: usize,
    output: Vec<String>,
}

impl ConfigWriter {
    fn new(config: String) -> ConfigWriter {
        let mut lexer = Lexer::new(config);
        let mut tokens = Vec::new();

        {
            // fill tokens:
            let mut n = 0;
            let mut token = lexer.next_token();
            tokens.push(token.clone());
            debug!("{token}");
            while token != Token::Eof {
                n = n + 1;
                token = lexer.next_token();
                info!("{token}");
                tokens.push(token.clone());
            }
        }
        debug!("Tokenization done!!\n\n\n");

        let output: Vec<String> = Vec::new();
        return ConfigWriter {
            tokens: tokens,
            token: Token::Start,
            position: 0,
            read_position: 0,
            output: output,
        };
    }

    fn write_configs(&mut self) -> String {
        self.read_token();
        let mut stanza_stack_record: Vec<Vec<String>> = Vec::new();
        let mut stanza_stack: Vec<String> = Vec::new();
        let mut stanza_pointer: usize = 0;
        let mut config_line_stack: Vec<String> = Vec::new();
        let mut inside_bracket_array: bool = false;
        while self.token != Token::Eof {
            debug!("write_configs: {} {}", self.read_position, self.token);
            match &self.token {
                Token::LeftSquirly => {
                    info!("LeftSquirly stack size increase ");
                    stanza_pointer += 1;
                    stanza_stack_record.push(stanza_stack.clone());
                    stanza_stack.clear();
                    config_line_stack.clear();
                }
                Token::LeftBracket => {
                    info!("LeftBracket");
                    inside_bracket_array = true;
                }
                Token::RightBracket => {
                    info!("RightBracket");
                    inside_bracket_array = false;
                    config_line_stack.clear();
                }
                Token::RightSquirly => {
                    info!("RightSquirly");
                    stanza_pointer -= 1;
                    stanza_stack.clear();
                    stanza_stack_record.pop();
                }
                Token::Identifier(string) => {
                    let statement = string;
                    if statement.ends_with(';') {
                        info!("stanza_stack_record {:#?}", stanza_stack_record);
                        info!("terminating statement {statement}");
                        config_line_stack.push(statement.clone().to_owned());
                        info!("config_line_stack {:#?}", config_line_stack);
                        let addition = build_string(&stanza_stack_record, &config_line_stack);
                        self.output.push(addition);
                        config_line_stack.clear();
                        stanza_stack.clear();
                    } else if inside_bracket_array {
                        // bracket array statements are build in this condition
                        // the config_line_stack is reset when the right bracket is encountered
                        config_line_stack.push(statement.clone().to_owned());
                        let addition = build_string(&stanza_stack_record, &config_line_stack);
                        self.output.push(addition);
                        config_line_stack.pop();
                    } else {
                        info!("non terminating statement {statement}");
                        stanza_stack.push(statement.clone().to_owned());
                        config_line_stack.push(statement.clone().to_owned());
                    }
                }
                _ => debug!("hit default case for {}", self.token),
            }

            self.read_token();
            debug!("stanza_stack {:#?}", stanza_stack);
            debug!("stanza_pointer {stanza_pointer}");
            debug!("stanza_stack_record {:#?}", stanza_stack_record);
            info!("config_line_stack {:#?}", config_line_stack);
        }
        return self.output.join("\n");
    }

    fn read_token(&mut self) {
        if self.read_position >= self.tokens.len() {
            self.token = Token::Eof;
        } else {
            self.token = self.tokens[self.read_position].clone();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }
}

// Take the stanza_stack_record and the config_line_stack to produce a syntactically valid Juniper
// set configuration command.
fn build_string(stanza_stack_record: &Vec<Vec<String>>, config_line_stack: &Vec<String>) -> String {
    let mut new_string = String::from("set");

    for vec in stanza_stack_record {
        for string in vec {
            new_string.push_str(" ");
            new_string.push_str(&string.to_string());
        }
    }
    for string in config_line_stack {
        new_string.push_str(" ");
        new_string.push_str(&string.to_string());
    }

    if new_string.ends_with(";") {
        new_string.pop();
    }
    info!("config_line:\n{new_string}");
    new_string
}

#[cfg(test)]
mod test {

    use super::ConfigWriter;
    use super::{build_string, open_config_file, Lexer, Token};

    #[test]
    fn test_build_string() {
        let input = String::from("{ example }");
        let stanza_stack_record = vec![
            vec![String::from("groups")],
            vec![String::from("BLOCK-V6")],
            vec![String::from("vlans")],
            vec![String::from("<*>")],
            vec![String::from("forwarding-options")],
            vec![String::from("filter")],
        ];
        let config_line_stack = vec![String::from("input"), String::from("BLOCK-IPv6;")];
        let result = build_string(&stanza_stack_record, &config_line_stack);
        let expected = String::from(
            "set groups BLOCK-V6 vlans <*> forwarding-options filter input BLOCK-IPv6",
        );
        assert_eq!(result, expected);
    }
    #[test]
    fn instantiate_lexer() {
        let input = String::from("{ example }");
        let _ = Lexer::new(input);
    }

    #[test]
    fn basic_identifiers_test() {
        let input = String::from("{ as-path test another-statement-123 } } }");

        let mut lexer = Lexer::new(input);

        let tokens: Vec<Token> = vec![
            Token::LeftSquirly,
            Token::Identifier("as-path".to_string()),
            Token::Identifier("test".to_string()),
            Token::Identifier("another-statement-123".to_string()),
            Token::RightSquirly,
            Token::RightSquirly,
            Token::RightSquirly,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }

    #[test]
    fn basic_config_convert_1() {
        let input = String::from(
            "system {
            host-name myrouter;
            services {
                ftp;
                ssh;
                telnet;
                netconf {
                    ssh;
                }
            }
        }",
        );

        let expected = String::from(
            "set system host-name myrouter
set system services ftp
set system services ssh
set system services telnet
set system services netconf ssh",
        );
        let mut config_writer = ConfigWriter::new(input.clone());
        let result = config_writer.write_configs();
        assert_eq!(result, expected);
    }

    #[test]
    fn basic_config_convert_2() {
        let input = String::from(
            "policy-options {
                policy-statement directs {
                    term Lo0 {
                        from {
                            protocol direct;
                            route-filter 192.168.100.0/24 orlonger;
                        }
                        then accept;
                    }
                }   
            }",
        );

        let expected = String::from(
            "set policy-options policy-statement directs term Lo0 from protocol direct
set policy-options policy-statement directs term Lo0 from route-filter 192.168.100.0/24 orlonger
set policy-options policy-statement directs term Lo0 then accept",
        );
        let mut config_writer = ConfigWriter::new(input.clone());
        let result = config_writer.write_configs();
        assert_eq!(result, expected);
    }

    #[test]
    fn config_convert_files() {
        let files = vec![
            "config_1",
            "config_2",
            "config_3",
            "config_4",
            "config_5",
            "config_6",
            "config_7",
            "config_10",
            "config_11",
        ];
        for filename in files {
            let filename_text = filename.clone().to_owned() + ".txt";
            let file_name_set = filename.clone().to_owned() + "_set.txt";
            let config = open_config_file(&filename_text);

            let expected = open_config_file(&file_name_set);

            let mut config_writer = ConfigWriter::new(config.clone());
            let result = config_writer.write_configs();
            assert_eq!(result, expected);
        }
    }
}
