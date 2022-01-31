use std::{fs::File, io::Write};

use crate::parser::VMCommandType;

pub struct CodeWriter {
    file: File,
}

impl CodeWriter {
    fn initialize_stack(&mut self) {
        write!(
            self.file,
            "@256\n\
             D=A\n\
             @SP\n\
             M=D\n"
        );
    }
    pub fn new(file: File) -> Self {
        let mut s = Self { file };
        s.initialize_stack();
        return s;
    }
    pub fn set_file_name(&mut self, file_name: String) {
        self.file = File::create(file_name).expect("Couldn't create a file.");
        self.initialize_stack();
    }
    pub fn write_arithmetic(&mut self, command: String) {
        let pop_to_d_asm: &str = "\
                    // pop y -> D\n\
                    @SP\n\
                    M=M-1\n\
                    A=M\n\
                    D=M\n\
                    // スタックにゴミ残しておいていいのか？\n\
                    M=0\n\
                    ";

        match command.as_str() {
            "add" => {
                let asm: &str = "\
                    // push x + D\n\
                    A=A-1\n\
                    M=M+D\n\
                    ";
                write!(self.file, "{}{}", pop_to_d_asm, asm);
            }
            "sub" => {
                let asm: &str = "\
                    // push x - D\n\
                    A=A-1\n\
                    M=M-D\n\
                    ";
                write!(self.file, "{}{}", pop_to_d_asm, asm);
            }
            "neg" => {
                let asm: &str = "\
                    // push -D\n\
                    M=-D\n\
                    @SP
                    M=M+1
                    ";
                write!(self.file, "{}{}", pop_to_d_asm, asm);
            }
            "eq" => {}
            "gt" => {}
            "lt" => {}
            "and" => {
                let asm: &str = "\
                    // push x & D\n\
                    A=A-1\n\
                    M=M&D\n\
                    ";
                write!(self.file, "{}{}", pop_to_d_asm, asm);
            }
            "or" => {
                let asm: &str = "\
                    // push x & D\n\
                    A=A-1\n\
                    M=M|D\n\
                    ";
                write!(self.file, "{}{}", pop_to_d_asm, asm);
            }
            "not" => {
                let asm: &str = "\
                    // push !D\n\
                    M=!D\n\
                    @SP
                    M=M+1
                    ";
                write!(self.file, "{}{}", pop_to_d_asm, asm);
            }
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
        //cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 12);
        //cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 8);
        //cw.write_arithmetic("sub".to_string());
        //cw.write_arithmetic("neg".to_string());
        //cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 0x0ff0);
        //cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 0x00ff);
        //cw.write_arithmetic("and".to_string());
        //cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 0x0ff0);
        //cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 0x00ff);
        //cw.write_arithmetic("or".to_string());
        cw.write_push_pop(VMCommandType::PUSH, "constant".to_string(), 0x0ff0);
        cw.write_arithmetic("not".to_string());
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
