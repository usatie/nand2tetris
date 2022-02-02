use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug, PartialEq)]
pub enum VMCommandType {
    ARITHMETIC,
    PUSH,
    POP,
    LABEL,
    GOTO,
    IF,
    FUNCTION,
    CALL,
    RETURN,
}

pub struct Parser {
    pub current_command: String,
    index: usize,
    lines: Vec<String>,
}

impl Parser {
    pub fn new(file: &File) -> Self {
        let mut lines = Vec::<String>::new();
        let cursor = io::BufReader::new(file);
        for line in cursor.lines() {
            let mut line = line.unwrap();
            let tokens: Vec<&str> = line.split("//").collect();
            line = tokens[0].to_string();
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if !tokens.is_empty() {
                lines.push(tokens.join(" "));
            }
        }
        Self {
            current_command: "".to_string(),
            index: 0,
            lines,
        }
    }
    pub fn has_more_commands(&self) -> bool {
        self.index < self.lines.len()
    }
    pub fn advance(&mut self) {
        self.current_command = self.lines[self.index].to_string();
        self.index += 1;
    }
    pub fn command_type(&self) -> VMCommandType {
        let tokens: Vec<&str> = self.current_command.split_whitespace().collect();
        use VMCommandType::*;
        match tokens[0] {
            "add" | "sub" | "neg" | "eq" | "gt" | "lt" | "and" | "or" | "not" => return ARITHMETIC,
            "push" => return PUSH,
            "pop" => return POP,
            "label" => return LABEL,
            "goto" => return GOTO,
            "if-goto" => return IF,
            "function" => return FUNCTION,
            "call" => return CALL,
            "return" => return RETURN,
            _ => panic!("not impl"),
        }
    }
    pub fn arg1(&self) -> String {
        let tokens: Vec<&str> = self.current_command.split_whitespace().collect();
        use VMCommandType::*;
        match self.command_type() {
            ARITHMETIC => tokens[0].to_string(),
            PUSH | POP | LABEL | GOTO | IF | FUNCTION | CALL => tokens[1].to_string(),
            RETURN => panic!("arg1() shouldn't be called when command_type is RETURN."),
        }
    }
    pub fn arg2(&self) -> u16 {
        let tokens: Vec<&str> = self.current_command.split_whitespace().collect();
        use VMCommandType::*;
        match self.command_type() {
            PUSH | POP | FUNCTION | CALL => tokens[2].parse().unwrap(),
            _ => panic!(
                "arg2() shouldn't be called when command_type is not PUSH/POP/FUNCTION/CALL."
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let file = File::open("StackArithmetic/SimpleAdd/SimpleAdd.vm").expect("Can't open file!");
        let parser = Parser::new(&file);
        assert_eq!(parser.current_command, "");
        assert_eq!(parser.index, 0);
        assert_eq!(parser.lines.len(), 3);
        assert_eq!(parser.lines[0], "push constant 7");
        assert_eq!(parser.lines[1], "push constant 8");
        assert_eq!(parser.lines[2], "add");

        let file = File::open("tests/parser-test.vm").expect("Can't open file!");
        let parser = Parser::new(&file);
        assert_eq!(parser.current_command, "");
        assert_eq!(parser.index, 0);
        assert_eq!(parser.lines.len(), 3);
        assert_eq!(parser.lines[0], "push constant 7");
        assert_eq!(parser.lines[1], "push constant 8");
        assert_eq!(parser.lines[2], "add");
    }
    #[test]
    fn test_has_more_commands() {
        let parser = super::Parser {
            lines: [].to_vec(),
            index: 0,
            current_command: "".to_string(),
        };
        assert!(!parser.has_more_commands());

        let parser = super::Parser {
            lines: ["push constant 7".to_string()].to_vec(),
            index: 0,
            current_command: "".to_string(),
        };
        assert!(parser.has_more_commands());
    }
    #[test]
    fn test_advance() {
        let mut parser = Parser {
            lines: vec![
                String::from("push constant 7"),
                String::from("push constant 8"),
                String::from("add"),
            ],
            index: 0,
            current_command: String::new(),
        };
        assert_eq!(parser.current_command, "");
        parser.advance();
        assert_eq!(parser.current_command, "push constant 7");
        parser.advance();
        assert_eq!(parser.current_command, "push constant 8");
        parser.advance();
        assert_eq!(parser.current_command, "add");
    }
    #[test]
    fn test_command_type() {
        let mut parser = Parser {
            lines: vec![
                String::from("add"),
                String::from("sub"),
                String::from("neg"),
                String::from("eq"),
                String::from("gt"),
                String::from("lt"),
                String::from("and"),
                String::from("or"),
                String::from("not"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::ARITHMETIC);

        let mut parser = Parser {
            lines: vec![
                String::from("push constant 7"),
                String::from("pop local 0"),
                String::from("label LOOP"),
                String::from("goto END"),
                String::from("if-goto START"),
                String::from("function hoge 2"),
                String::from("call fuga 3"),
                String::from("return"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::PUSH);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::POP);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::LABEL);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::GOTO);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::IF);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::FUNCTION);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::CALL);
        parser.advance();
        assert_eq!(parser.command_type(), VMCommandType::RETURN);
    }
    #[test]
    fn test_arg1() {
        let mut parser = Parser {
            lines: vec![
                String::from("add"),
                String::from("push constant 7"),
                String::from("pop local 0"),
                String::from("label LOOP"),
                String::from("goto END"),
                String::from("if-goto START"),
                String::from("function hoge 2"),
                String::from("call fuga 3"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.arg1(), "add");
        parser.advance();
        assert_eq!(parser.arg1(), "constant");
        parser.advance();
        assert_eq!(parser.arg1(), "local");
        parser.advance();
        assert_eq!(parser.arg1(), "LOOP");
        parser.advance();
        assert_eq!(parser.arg1(), "END");
        parser.advance();
        assert_eq!(parser.arg1(), "START");
        parser.advance();
        assert_eq!(parser.arg1(), "hoge");
        parser.advance();
        assert_eq!(parser.arg1(), "fuga");
    }
    #[test]
    fn test_arg2() {
        let mut parser = Parser {
            lines: vec![
                String::from("push constant 7"),
                String::from("pop local 0"),
                String::from("function hoge 2"),
                String::from("call fuga 3"),
            ],
            index: 0,
            current_command: String::new(),
        };
        parser.advance();
        assert_eq!(parser.arg2(), 7);
        parser.advance();
        assert_eq!(parser.arg2(), 0);
        parser.advance();
        assert_eq!(parser.arg2(), 2);
        parser.advance();
        assert_eq!(parser.arg2(), 3);
    }
}
