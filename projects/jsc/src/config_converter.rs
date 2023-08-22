// RUST_LOG=info cargo run
// cargo test
use log::{debug, info, warn};
use std::fmt::Display;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

pub fn debug() {
    info!("In the debug, running lib!");
    let config = open_config_file("config_1.txt");
    debug!("lexing the following:\n{config}");
    let config_length = config.clone().into_bytes().len();
    let mut lexer = Lexer::new(config.clone());
    // while loop for now:
    let mut n = 1;

    let mut token = lexer.next_token();
    debug!("{token}");
    while token != Token::Eof {
        n = n + 1;
        token = lexer.next_token();
        debug!("{token}");
    }

    let mut config_writer = ConfigWriter::new(config.clone());
    config_writer.write_configs();

    info!("finished the debug function");

    //let stupid_token = Token::Identifier("as-path".to_string());
    //match stupid_token {
    //    Token::Identifier(s) => println!("identifier \"{}\".", s),
    //    _ => println!("FK"),
    //};
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
            'a'..='z' | 'A'..='Z' | '"' | '-' | '0'..='9' | '^' | '-' => {
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
    end: usize,
    stack: Vec<Token>,
    stack_pointer: usize,
    cur_line: Vec<String>,
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
        info!("Tokenization done!!\n\n\n");
        let end = tokens.len();
        let mut stack: Vec<Token> = Vec::new();
        let mut cur_line: Vec<String> = Vec::new();
        let mut output: Vec<String> = Vec::new();
        return ConfigWriter {
            tokens: tokens,
            token: Token::Start,
            position: 0,
            read_position: 0,
            end: end,
            stack: stack,
            stack_pointer: 0,
            cur_line: cur_line,
            output: output,
        };
    }

    fn write_configs(&mut self) -> String {
        self.read_token();
        while self.token != Token::Eof {
            debug!("{} {}", self.read_position, self.token);
            match &self.token {
                Token::LeftSquirly => {
                    self.stack_pointer = self.cur_line.len();
                    debug!("LeftSquirly self.cur_line.len() {}", self.cur_line.len(),);
                    let joined = self.cur_line.join(" ");
                    debug!("LeftSquirly joined:{joined}");
                }
                Token::RightSquirly => {
                    let joined = self.cur_line.join(" ");

                    self.stack_pointer = self.stack_pointer - 1;

                    //debug!(
                    //    "before pop: {joined} {} {} ",
                    //    self.cur_line.len(),
                    //    self.stack_pointer
                    //);

                    //while self.cur_line.len() <= self.stack_pointer {
                    //    self.cur_line.pop();
                    //    debug!(
                    //        "while pop cur_line len:{} pointer {}",
                    //        self.cur_line.len(),
                    //        self.stack_pointer
                    //    );
                    //}
                    //let joined = self.cur_line.join(" ");
                    //debug!("after pop: {joined}");
                    self.shrink_line_to_stack_pointer()
                }
                Token::Identifier(string) => {
                    let mut statement = string;
                    if statement.ends_with(';') {
                        let result_addition = self.create_addition_statement(statement.clone());
                        self.output.push(String::from(result_addition));
                        self.shrink_line_to_stack_pointer()
                    } else {
                        &self.cur_line.push(statement.to_owned());

                        let joined = self.cur_line.join(" ");
                        debug!("cur_line {joined}");
                    }
                }
                _ => debug!("hit default case for {}", self.token),
            }
            self.read_token();
        }
        return self.output.join("\n");
    }

    // prepare the string so that it is formatted as a configuration statement.
    fn create_addition_statement(&mut self, statement: String) -> String {
        let mut addition_statement = statement.clone();
        addition_statement.pop();
        let joined = self.cur_line.join(" ");
        let mut result_addition =
            "set ".to_string() + &joined + &" ".to_string() + &addition_statement;
        info!("{result_addition}");
        return result_addition;
    }

    // shrink the config line that is to be written to the size of the
    // current stack pointer.
    fn shrink_line_to_stack_pointer(&mut self) {
        while self.cur_line.len() != self.stack_pointer {
            self.cur_line.pop();
        }
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

#[cfg(test)]
mod test {

    use super::ConfigWriter;
    use super::{open_config_file, Lexer, Token};
    #[test]
    fn instantiate_lexer() {
        let input = String::from("{ example }");
        let mut lexer = Lexer::new(input);
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
    fn basic_config_convert() {
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
    fn config_convert() {
        let files = vec!["config_1", "config_3"];
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
