use std::{fs::File, io::Write};

use crate::parser::VMCommandType;

pub struct CodeWriter {
    file: File,
    asm: String,
    cnt: u16,
}

// Assembly Parts
impl CodeWriter {
    fn initialize_asm(&mut self) {
        self.asm += "@256\n";
        self.asm += "D=A\n";
        self.asm += "@SP\n";
        self.asm += "M=D\n";
    }
    fn pop_to_d(&mut self) {
        self.asm += "@SP\n";
        self.asm += "M=M-1\n";
        self.asm += "A=M\n";
        self.asm += "D=M\n";
        self.asm += "M=0\n";
    }
}

impl CodeWriter {
    pub fn new(file: File) -> Self {
        let mut writer = Self {
            file,
            asm: String::new(),
            cnt: 0,
        };
        writer.initialize_asm();
        return writer;
    }
    pub fn set_file_name(&mut self, file_name: String) {
        self.file = File::create(file_name).expect("Couldn't create a file.");
        self.initialize_asm();
    }
    pub fn write_arithmetic(&mut self, command: String) {
        self.cnt += 1;
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
                self.asm += pop_to_d_asm;
                self.asm += asm;
            }
            "sub" => {
                let asm: &str = "\
                    // push x - D\n\
                    A=A-1\n\
                    M=M-D\n\
                    ";
                self.asm += pop_to_d_asm;
                self.asm += asm;
            }
            "neg" => {
                let asm: &str = "\
                    // push -D\n\
                    M=-D\n\
                    @SP
                    M=M+1
                    ";
                self.asm += pop_to_d_asm;
                self.asm += asm;
            }
            "eq" => {
                let asm: String = format!(
                    "\
                    // D = x - D\n\
                    A=A-1\n\
                    D=M-D\n\
                    M=-1\n\
                    @eq{0}\n\
                    D;JEQ\n\
                    @SP\n\
                    A=M-1\n\
                    M=0\n\
                    (eq{0})\n\
                    ",
                    self.cnt
                )
                .to_string();
                self.asm += pop_to_d_asm;
                self.asm += asm.as_str();
            }
            "gt" => {
                let asm: String = format!(
                    "\
                    // D = x - D\n\
                    A=A-1\n\
                    D=M-D\n\
                    M=-1\n\
                    @gt{0}\n\
                    D;JGT\n\
                    @SP
                    A=M-1\n\
                    M=0\n\
                    (gt{0})\n\
                    ",
                    self.cnt
                )
                .to_string();
                self.asm += pop_to_d_asm;
                self.asm += asm.as_str();
            }
            "lt" => {
                let asm: String = format!(
                    "\
                    // D = x - D\n\
                    A=A-1\n\
                    D=M-D\n\
                    M=-1\n\
                    @lt{0}\n\
                    D;JLT\n\
                    @SP\n\
                    A=M-1\n\
                    M=0\n\
                    (lt{0})\n\
                    ",
                    self.cnt
                )
                .to_string();
                self.asm += pop_to_d_asm;
                self.asm += asm.as_str();
            }
            "and" => {
                let asm: &str = "\
                    // push x & D\n\
                    A=A-1\n\
                    M=M&D\n\
                    ";
                self.asm += pop_to_d_asm;
                self.asm += asm;
            }
            "or" => {
                let asm: &str = "\
                    // push x & D\n\
                    A=A-1\n\
                    M=M|D\n\
                    ";
                self.asm += pop_to_d_asm;
                self.asm += asm;
            }
            "not" => {
                let asm: &str = "\
                    // push !D\n\
                    M=!D\n\
                    @SP
                    M=M+1
                    ";
                self.asm += pop_to_d_asm;
                self.asm += asm;
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
                    self.asm += asm.as_str();
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
    pub fn close(&mut self) {
        write!(self.file, "{}", self.asm).expect("Couldn't write file.");
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
        // 何をテストしたらいいかわからん
        assert!(true);
    }
}
