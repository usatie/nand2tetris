use std::{fmt::format, fs::File, io::Write};

use crate::parser::VMCommandType;

pub struct CodeWriter {
    file: File,
    pub asm: String,
    cnt: u16,
    file_name: String,
    function_name: String,
}

// Assembly Parts
impl CodeWriter {
    fn push_d(&mut self) {
        self.asm += "@SP\n"; // A = @SP
        self.asm += "A=M\n"; // A = M[@SP] = sp
        self.asm += "M=D\n"; // M[sp] = argp + index
        self.asm += "@SP\n"; // A = @SP
        self.asm += "M=M+1\n"; // M[@SP] = sp + 1
    }
    fn pop_to_d(&mut self) {
        self.asm += "@SP\n";
        self.asm += "M=M-1\n";
        self.asm += "A=M\n";
        self.asm += "D=M\n";
        self.asm += "M=0\n";
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

    fn push_comparison_of_pop_and_d(&mut self, asm_ope: &str) {
        self.asm += "@SP\n";
        self.asm += "A=M-1\n";
        self.asm += "D=M-D\n";
        self.asm += "M=-1\n";
        self.asm += format!("@{}{}\n", asm_ope, self.cnt).as_str();
        self.asm += format!("D;{}\n", asm_ope).as_str();
        self.asm += "@SP\n";
        self.asm += "A=M-1\n";
        self.asm += "M=0\n";
        self.asm += format!("({}{})\n", asm_ope, self.cnt).as_str();
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
        writer.write_init();
        return writer;
    }
    pub fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
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
                self.push_comparison_of_pop_and_d("JEQ");
            }
            "gt" => {
                self.pop_to_d();
                self.push_comparison_of_pop_and_d("JGT");
            }
            "lt" => {
                self.pop_to_d();
                self.push_comparison_of_pop_and_d("JLT");
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
        self.cnt += 1;
        use VMCommandType::*;
        match command {
            PUSH => match segment.as_str() {
                "constant" => {
                    self.asm += format!("@{}\n", index).as_str();
                    self.asm += "D=A\n";
                    self.push_d();
                }
                "argument" => self.push_symbol_plus_index("@ARG", index),
                "local" => self.push_symbol_plus_index("@LCL", index),
                "this" => self.push_symbol_plus_index("@THIS", index),
                "that" => self.push_symbol_plus_index("@THAT", index),
                "temp" => {
                    self.asm += format!("@R{}\n", index + 5).as_str();
                    self.asm += "D=M\n";
                    self.push_d();
                }
                "static" => {
                    self.asm += format!("@{}.{}\n", self.file_name, index).as_str();
                    self.asm += "D=M\n"; // D = M[@Xxx.index] = staticp_xxx_index
                    self.push_d();
                }
                "pointer" => match index {
                    0 => {
                        self.asm += "@THIS\n";
                        self.asm += "D=M\n";
                        self.push_d();
                    }
                    1 => {
                        self.asm += "@THAT\n";
                        self.asm += "D=M\n";
                        self.push_d();
                    }
                    _ => panic!("'pointer' segment can only take 0 or 1 as index"),
                },
                _ => {
                    panic!("Uexpected segment");
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
                    self.pop_to_d();
                    self.asm += format!("@{}.{}\n", self.file_name, index).as_str();
                    self.asm += "M=D\n"; // M[@Xxx.index] = popされた値
                }
                "pointer" => match index {
                    0 => {
                        self.pop_to_d();
                        self.asm += "@THIS\n";
                        self.asm += "M=D\n";
                    }
                    1 => {
                        self.pop_to_d();
                        self.asm += "@THAT\n";
                        self.asm += "M=D\n";
                    }
                    _ => panic!("'pointer' segment can only take 0 or 1 as index"),
                },
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

impl CodeWriter {
    pub fn write_init(&mut self) {
        // @SP=256
        self.asm += "@256\n";
        self.asm += "D=A\n";
        self.asm += "@SP\n";
        self.asm += "M=D\n";
    }
    pub fn write_label(&mut self, label: String) {
        // TODO: check label name rule
        // Available: alphabet, number, under_score, dot, semi colon
        // The first character should not be a number
        self.asm += format!("({})\n", label).as_str();
    }
    pub fn write_goto(&mut self, label: String) {
        self.asm += format!("@{}\n", label).as_str();
        self.asm += "0;JMP\n";
    }
    pub fn write_if(&mut self, label: String) {
        self.pop_to_d();
        self.asm += format!("@{}\n", label).as_str();
        self.asm += "D;JNE\n";
    }
    pub fn write_call(&mut self, function_name: String, num_args: u16) {
        self.cnt += 1;
        // push return-address
        self.asm += format!("@return-address-of-{}\n", self.cnt).as_str();
        self.asm += "D=A\n";
        self.push_d();
        // push LCL
        self.asm += "@LCL\n";
        self.asm += "D=M\n";
        self.push_d();
        // push ARG
        self.asm += "@ARG\n";
        self.asm += "D=M\n";
        self.push_d();
        // push THIS
        self.asm += "@THIS\n";
        self.asm += "D=M\n";
        self.push_d();
        // push THAT
        self.asm += "@THAT\n";
        self.asm += "D=M\n";
        self.push_d();
        // ARG = SP-n-5
        self.asm += "@SP\n";
        self.asm += "D=M\n";
        self.asm += format!("@{}\n", num_args + 5).as_str();
        self.asm += "D=D-A\n";
        self.asm += "@ARG\n";
        self.asm += "M=D\n";
        // LCL=SP
        self.asm += "@SP\n";
        self.asm += "D=M\n";
        self.asm += "@LCL\n";
        self.asm += "M=D\n";
        // goto f
        self.asm += format!("@{}\n", function_name).as_str();
        self.asm += "0;JMP\n";
        // (return-address)
        self.asm += format!("(return-address-of-{})\n", self.cnt).as_str();
    }
    pub fn write_return(&mut self) {
        // M[@R13] = FRAME
        self.asm += "@LCL\n";
        self.asm += "D=M\n";
        self.asm += "@R13\n";
        self.asm += "M=D\n";

        // M[@14] = RET = *(FRAME-5)
        self.asm += "@5\n";
        self.asm += "A=D-A\n";
        self.asm += "D=M\n";
        self.asm += "@R14\n";
        self.asm += "M=D\n";

        // *ARG = pop()
        self.pop_to_d();
        self.asm += "@ARG\n";
        self.asm += "A=M\n";
        self.asm += "M=D\n";

        // SP = ARG+1
        self.asm += "@ARG\n";
        self.asm += "D=M+1\n";
        self.asm += "@SP\n";
        self.asm += "M=D\n";

        // THAT = *(FRAME-1))
        self.asm += "@R13\n";
        self.asm += "M=M-1\n";
        self.asm += "A=M\n";
        self.asm += "D=M\n";
        self.asm += "@THAT\n";
        self.asm += "M=D\n";

        // THIS = *(FRAME-2)

        self.asm += "@R13\n";
        self.asm += "M=M-1\n";
        self.asm += "A=M\n";
        self.asm += "D=M\n";
        self.asm += "@THIS\n";
        self.asm += "M=D\n";
        // ARG = *(FRAME-3)

        self.asm += "@R13\n";
        self.asm += "M=M-1\n";
        self.asm += "A=M\n";
        self.asm += "D=M\n";
        self.asm += "@ARG\n";
        self.asm += "M=D\n";
        // LCL = *(FRAME-4)
        self.asm += "@R13\n";
        self.asm += "M=M-1\n";
        self.asm += "A=M\n";
        self.asm += "D=M\n";
        self.asm += "@LCL\n";
        self.asm += "M=D\n";

        // goto RET
        self.asm += "@R14\n";
        self.asm += "A=M\n";
        self.asm += "0;JMP\n";
    }
    pub fn write_function(&mut self, function_name: String, num_locals: u16) {
        self.asm += format!("({})\n", function_name).as_str();
        // 何やったらいいんだっけ？
        let mut n = 0;
        while n < num_locals {
            n += 1;
            self.asm += "D=0\n";
            self.push_d();
        }
    }
}

// 全般的に今回は何をテストしたらいいかわからん。
// ちゃんとテストしようと思ったら生成されたアセンブリコードを実行して、
// それが要求されるVM命令の結果になっているかどうかを確認しないといけないけど、それは結合テストでやっている
// ユニットテストではどういうのテストしたらいいんだろうなー。
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
        // 何をテストしたらいいかわからん
        assert!(true);
    }
    #[test]
    fn test_close() {
        // 何をテストしたらいいかわからん
        assert!(true);
    }
}
