use std::{fs::File, io::Write};

use crate::parser::VMCommandType;

pub struct CodeWriter {
    file: File,
    asm: String,
    cnt: u16,
    file_name: String,
    function_name: String,
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
            file_name: String::new(),
            function_name: String::new(),
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
    fn push_d(&mut self) {
        self.asm += "@SP\n"; // A = @SP
        self.asm += "A=M\n"; // A = M[@SP] = sp
        self.asm += "M=D\n"; // M[sp] = argp + index
        self.asm += "@SP\n"; // A = @SP
        self.asm += "M=M+1\n"; // M[@SP] = sp + 1
    }
    fn push_symbol_plus_index(&mut self, symbol: &str, index: u16) {
        self.asm += format!("{}\n", symbol).as_str(); // A = @ARG
        self.asm += "D=M\n"; // D = M[@ARG] = argp
        self.asm += format!("@{}\n", index).as_str(); // A = index
        self.asm += "A=D+A\n"; // A = argp + index
        self.asm += "D=M\n"; // D = M[argp + index]
        self.push_d()
    }
    fn pop_symbol_plus_index(&mut self, symbol: &str, index: u16) {
        // D = argp + index
        self.asm += format!("{}\n", symbol).as_str(); // A = @ARG
        self.asm += "D=M\n"; // D = M[@ARG] = argp
        self.asm += format!("@{}\n", index).as_str(); // A = index
        self.asm += "D=D+A\n"; // D = argp + index

        // argp + indexを一時的にR13に格納
        self.asm += "@R13\n"; // A = @R13
        self.asm += "M=D\n"; // M[@R13] = argp + index

        // Dにpop
        self.pop_to_d();

        // M[argp+index] = popしてきた値
        self.asm += "@R13\n"; // A = @R13
        self.asm += "A=M\n"; // A = M[@R13] (= argp + index)
        self.asm += "M=D\n"; // M[argp+index] = D (= popしてきた値)
    }
    pub fn write_push_pop(&mut self, command: VMCommandType, segment: String, index: u16) {
        self.cnt += 1;
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
                "argument" => {
                    self.push_symbol_plus_index("@ARG", index);
                }
                "local" => {
                    self.push_symbol_plus_index("@LCL", index);
                }
                "this" => {
                    self.push_symbol_plus_index("@THIS", index);
                }
                "that" => {
                    self.push_symbol_plus_index("@THAT", index);
                }
                "temp" => {
                    self.asm += format!("@R{}\n", index + 5).as_str();
                    self.asm += "D=M\n";
                    self.push_d();
                }
                "static" => {
                    self.asm += format!("@{}.{}\n", self.file_name, index).as_str();
                    self.asm += "D=M\n"; // D = M[@ARG] = argp
                    self.push_d();
                }
                "pointer" => {
                    panic!("TODO!");
                }
                _ => {
                    panic!("TODO")
                }
            },
            POP => match segment.as_str() {
                "argument" => {
                    self.pop_symbol_plus_index("@ARG", index);
                }
                "local" => {
                    self.pop_symbol_plus_index("@LCL", index);
                }
                "this" => {
                    self.pop_symbol_plus_index("@THIS", index);
                }
                "that" => {
                    self.pop_symbol_plus_index("@THAT", index);
                }
                "temp" => {
                    self.pop_to_d();
                    self.asm += format!("@R{}\n", index + 5).as_str();
                    self.asm += "M=D\n";
                }
                "static" => {
                    panic!("TODO");
                }
                "pointer" => {
                    panic!("TODO!");
                }
                _ => {
                    panic!("TODO")
                }
            },
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
