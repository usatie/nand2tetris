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
        match command.as_str() {
            "add" => {
                self.pop_to_d();
                self.asm += "A=A-1\n";
                self.asm += "M=M+D\n";
            }
            "sub" => {
                self.pop_to_d();
                self.asm += "A=A-1\n";
                self.asm += "M=M-D\n";
            }
            "neg" => {
                self.pop_to_d();
                self.asm += "M=-D\n";
                self.asm += "@SP\n";
                self.asm += "M=M+1\n";
            }
            "eq" => {
                self.pop_to_d();
                self.asm += "A=A-1\n";
                self.asm += "D=M-D\n";
                self.asm += "M=-1\n";
                self.asm += format!("@eq{0}\n", self.cnt).as_str();
                self.asm += "D;JEQ\n";
                self.asm += "@SP\n";
                self.asm += "A=M-1\n";
                self.asm += "M=0\n";
                self.asm += format!("(eq{0})\n", self.cnt).as_str();
            }
            "gt" => {
                self.pop_to_d();
                self.asm += "A=A-1\n";
                self.asm += "D=M-D\n";
                self.asm += "M=-1\n";
                self.asm += format!("@gt{0}\n", self.cnt).as_str();
                self.asm += "D;JGT\n";
                self.asm += "@SP\n";
                self.asm += "A=M-1\n";
                self.asm += "M=0\n";
                self.asm += format!("(gt{0})\n", self.cnt).as_str();
            }
            "lt" => {
                self.pop_to_d();
                self.asm += "A=A-1\n";
                self.asm += "D=M-D\n";
                self.asm += "M=-1\n";
                self.asm += format!("@lt{0}\n", self.cnt).as_str();
                self.asm += "D;JLT\n";
                self.asm += "@SP\n";
                self.asm += "A=M-1\n";
                self.asm += "M=0\n";
                self.asm += format!("(lt{0})\n", self.cnt).as_str();
            }
            "and" => {
                self.pop_to_d();
                self.asm += "A=A-1\n";
                self.asm += "M=M&D\n";
            }
            "or" => {
                self.pop_to_d();
                self.asm += "A=A-1\n";
                self.asm += "M=M|D\n";
            }
            "not" => {
                self.pop_to_d();
                self.asm += "M=!D\n";
                self.asm += "@SP\n";
                self.asm += "M=M+1\n";
            }
            _ => panic!("Non Arithmetic command is passed"),
        }
    }
    pub fn write_push_pop(&mut self, command: VMCommandType, segment: String, index: u16) {
        use VMCommandType::*;
        match command {
            PUSH => match segment.as_str() {
                "constant" => {
                    self.asm += format!("@{}\n", index).as_str();
                    self.asm += "D=A\n";
                    self.asm += "@SP\n";
                    self.asm += "A=M\n";
                    self.asm += "M=D\n";
                    self.asm += "D=A+1\n";
                    self.asm += "@SP\n";
                    self.asm += "M=D\n";
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
