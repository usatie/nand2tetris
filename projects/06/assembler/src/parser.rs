use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq)]
pub enum HackCommandType {
    A,
    C,
    L,
}
pub struct Parser {
    lines: Vec<String>,
    index: usize,
    current_command: String,
}

impl Parser {
    pub fn new(file: File) -> Self {
        let mut lines = Vec::<String>::new();
        let cursor = io::BufReader::new(&file);
        for line in cursor.lines() {
            let line_without_whitespace: String = line.unwrap().split_whitespace().collect();
            if line_without_whitespace.starts_with("//") {
                continue;
            } else if line_without_whitespace.is_empty() {
                continue;
            } else {
                lines.push(line_without_whitespace);
            }
        }
        return Self {
            lines,
            index: 0,
            current_command: String::new(),
        };
    }
}

impl Parser {
    pub fn has_more_commands(&self) -> bool {
        self.index < self.lines.len()
    }

    pub fn advance(&mut self) {
        // read next command
        if self.has_more_commands() {
            self.current_command = self.lines.get(self.index).unwrap().to_string();
            self.index += 1;
        }
    }

    pub fn command_type(&self) -> HackCommandType {
        if self.current_command.starts_with("@") {
            return HackCommandType::A;
        } else if self.current_command.starts_with("(") & self.current_command.ends_with(")") {
            return HackCommandType::L;
        } else {
            return HackCommandType::C;
        }
    }

    pub fn symbol(&self) -> String {
        match self.command_type() {
            HackCommandType::A => {
                return self.current_command[1..self.current_command.len()].to_string()
            }
            HackCommandType::L => {
                return self.current_command[1..self.current_command.len() - 1].to_string()
            }
            _ => {
                panic!("symbol() should not be called if command type is neither A nor L.");
            }
        }
    }

    pub fn dest(&self) -> String {
        let mut dest = String::from("");
        if self.current_command.contains("=") {
            let tokens: Vec<&str> = self.current_command.split("=").collect();
            dest = tokens[0].to_string();
        }
        return dest;
    }

    pub fn comp(&self) -> String {
        let mut comp = self.current_command.clone();
        if comp.contains("=") {
            let tokens: Vec<&str> = comp.split("=").collect();
            comp = tokens[1].to_string();
        }
        if comp.contains(";") {
            let tokens: Vec<&str> = comp.split(";").collect();
            comp = tokens[0].to_string();
        }
        return comp;
    }

    pub fn jump(&self) -> String {
        let mut jump = String::from("");
        if self.current_command.contains(";") {
            let tokens: Vec<&str> = self.current_command.split(";").collect();
            jump = tokens[1].to_string();
        }
        return jump;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_constructor() {
        let file = super::File::open("test1.txt").expect("Can't open file!");
        let parser = super::Parser::new(file);
        assert_eq!(parser.current_command, "");
        assert_eq!(parser.index, 0);
        assert_eq!(parser.lines.len(), 5);
        assert_eq!(parser.lines[0], "@test");
        assert_eq!(parser.lines[1], "@12");
        assert_eq!(parser.lines[2], "(LOOP)");
        assert_eq!(parser.lines[3], "A=M");
        assert_eq!(parser.lines[4], "@sum");
    }

    #[test]
    fn test_has_more_commands() {
        let parser = super::Parser {
            lines: Vec::<String>::new(),
            index: 0,
            current_command: String::new(),
        };
        assert!(!parser.has_more_commands());

        let parser = super::Parser {
            lines: vec![String::from("@123"); 3],
            index: 0,
            current_command: String::new(),
        };
        assert!(parser.has_more_commands());
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
    fn test_command_type() {
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
        assert_eq!(parser.command_type(), super::HackCommandType::A);
        parser.advance();
        assert_eq!(parser.command_type(), super::HackCommandType::C);
        parser.advance();
        assert_eq!(parser.command_type(), super::HackCommandType::L);
    }

    #[test]
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
