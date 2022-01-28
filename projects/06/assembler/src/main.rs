use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq)]
enum CommandType {
    A_COMMAND,
    C_COMMAND,
    L_COMMAND,
}
struct Parser {
    lines: Vec<String>,
    index: usize,
    current_command: String,
}

impl Parser {
    pub fn new(file: File) -> Self {
        let mut lines = Vec::<String>::new();
        let cursor = io::BufReader::new(&file);
        for line in cursor.lines() {
            let line = line.unwrap();
            lines.push(line);
        }
        return Self {
            lines,
            index: 0,
            current_command: String::new(),
        };
    }
}

impl Parser {
    fn hasMoreCommands(&self) -> bool {
        self.index < self.lines.len()
    }

    fn advance(&mut self) {
        // read next command
        if self.hasMoreCommands() {
            self.current_command = self.lines.get(self.index).unwrap().to_string();
            self.index += 1;
        }
    }

    fn commandType(&self) -> CommandType {
        return CommandType::A_COMMAND;
    }

    fn symbol(&self) -> String {
        return String::from("");
    }

    fn dest(&self) -> String {
        return String::from("");
    }

    fn comp(&self) -> String {
        return String::from("");
    }

    fn jump(&self) -> String {
        return String::from("");
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod parser_tests {
    #[test]
    fn test_constructor() {
        let file = super::File::open("test1.txt").expect("Can't open file!");
        let parser = super::Parser::new(file);
        assert_eq!(parser.current_command, "");
        assert_eq!(parser.index, 0);
        assert_eq!(parser.lines.len(), 3);
    }

    #[test]
    fn test_hasMoreCommands() {
        let parser = super::Parser {
            lines: Vec::<String>::new(),
            index: 0,
            current_command: String::new(),
        };
        assert!(!parser.hasMoreCommands());

        let parser = super::Parser {
            lines: vec![String::from("@123"); 3],
            index: 0,
            current_command: String::new(),
        };
        assert!(parser.hasMoreCommands());
    }

    #[test]
    fn test_advance() {
        let mut parser = super::Parser {
            lines: vec![
                String::from("@123"),
                String::from("M=A"),
                String::from("@456"),
            ],
            index: 0,
            current_command: String::new(),
        };
        assert_eq!(parser.current_command, "");
        parser.advance();
        assert_eq!(parser.current_command, "@123");
        parser.advance();
        assert_eq!(parser.current_command, "M=A");
        parser.advance();
        assert_eq!(parser.current_command, "@456");
    }

    #[test]
    #[should_panic]
    fn test_commandType() {
        let mut parser = super::Parser {
            lines: vec![
                String::from("@sum"),
                String::from("M=0"),
                String::from("(LOOP)"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.commandType(), super::CommandType::A_COMMAND);
        parser.advance();
        assert_eq!(parser.commandType(), super::CommandType::C_COMMAND);
        parser.advance();
        assert_eq!(parser.commandType(), super::CommandType::L_COMMAND);
    }

    #[test]
    #[should_panic]
    fn test_symbol() {
        let mut parser = super::Parser {
            lines: vec![
                String::from("@sum"),
                String::from("@123"),
                String::from("(LOOP)"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.symbol(), "sum");
        parser.advance();
        assert_eq!(parser.symbol(), "123");
        parser.advance();
        assert_eq!(parser.symbol(), "LOOP");
    }

    #[test]
    #[should_panic]
    fn test_dest() {
        let mut parser = super::Parser {
            lines: vec![
                String::from("dest=comp;jump"),
                String::from("D=M"),
                String::from("M=D-A"),
                String::from("0;JMP"),
                String::from("D;JGT"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.dest(), "dest");
        parser.advance();
        assert_eq!(parser.dest(), "D");
        parser.advance();
        assert_eq!(parser.dest(), "M");
        parser.advance();
        assert_eq!(parser.dest(), "");
        parser.advance();
        assert_eq!(parser.dest(), "");
    }

    #[test]
    #[should_panic]
    fn test_comp() {
        let mut parser = super::Parser {
            lines: vec![
                String::from("dest=comp;jump"),
                String::from("D=M"),
                String::from("M=D-A"),
                String::from("0;JMP"),
                String::from("D;JGT"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.comp(), "comp");
        parser.advance();
        assert_eq!(parser.comp(), "M");
        parser.advance();
        assert_eq!(parser.comp(), "D-A");
        parser.advance();
        assert_eq!(parser.comp(), "0");
        parser.advance();
        assert_eq!(parser.comp(), "D");
    }

    #[test]
    #[should_panic]
    fn test_jump() {
        let mut parser = super::Parser {
            lines: vec![
                String::from("dest=comp;jump"),
                String::from("D=M"),
                String::from("M=D-A"),
                String::from("0;JMP"),
                String::from("D;JGT"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.jump(), "jump");
        parser.advance();
        assert_eq!(parser.jump(), "");
        parser.advance();
        assert_eq!(parser.jump(), "");
        parser.advance();
        assert_eq!(parser.jump(), "JMP");
        parser.advance();
        assert_eq!(parser.jump(), "JGT");
    }
}
