use std::{fs::File, io::Write};

use crate::parser::VMCommandType;

pub struct CodeWriter {
    file: File,
}

impl CodeWriter {
    pub fn new(file: File) -> Self {
        Self { file }
    }
    pub fn set_file_name(&mut self, file_name: String) {
        self.file = File::create(file_name).expect("Couldn't create a file.");
    }
    pub fn write_arithmetic(&mut self, command: String) {
        match command.as_str() {
            "add" => {
                let asm: &str = "\
                    // pop y -> D\n\
                    @SP\n\
                    M=M-1\n\
                    A=M\n\
                    D=M\n\
                    // don't have to destroy?\n\
                    // M=0\n\
\n\
                    // pop x + D -> D\n\
                    // push D\n\
                    A=A-1\n\
                    M=D+M\n\
                    ";
                write!(self.file, "{}", asm);
            }
            "sub" => {}
            "neg" => {}
            "eq" => {}
            "gt" => {}
            "lt" => {}
            "and" => {}
            "or" => {}
            "not" => {}
            _ => panic!("Non Arithmetic command is passed"),
        }
    }
    pub fn write_push_pop(&mut self, command: VMCommandType, segment: String, index: u16) {
        use VMCommandType::*;
        match command {
            PUSH => match segment.as_str() {
                "constant" => {
                    let asm: String = format!(
                        "\
                    @{}\n\
                    D=A\n\
                    @SP\n\
                    A=M\n\
                    M=D\n\
                    D=A+1\n\
                    @SP\n\
                    M=D\n\
                    ",
                        index
                    );
                    write!(self.file, "{}", asm);
                }
                _ => {
                    panic!("TODO")
                }
            },
            POP => {}
            _ => {
                panic!("write_push_pop() is only available for PUSH or POP command type.")
            }
        }
    }
    pub fn close(&self) {
        panic!("TODO");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_new() {
        // 何をテストしたらいいかわからん
        assert!(true);
    }
    #[test]
    fn test_set_file_name() {
        // 何をテストしたらいいかわからん
        assert!(true);
    }
    #[test]
    fn test_write_arithmetic() {
        let file = File::create("test.asm").expect("Couldn't create a file.");
        let mut cw = CodeWriter::new(file);
        cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 7);
        cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 8);
        cw.write_arithmetic("add".to_string());
        assert!(true);
    }
    #[test]
    fn test_write_push_pop() {
        assert!(true);
    }
    #[test]
    fn test_close() {
        assert!(true);
    }
}
