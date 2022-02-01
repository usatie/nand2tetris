use std::{env::args, fs::File, path::PathBuf};

use code_writer::CodeWriter;
use parser::Parser;

mod code_writer;
mod parser;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        panic!("vmtranslator requires dir or file_name as an argument.");
    }
    let path = PathBuf::from(args[1].to_string());
    if path.is_dir() {
        let mut output_file_path = path.to_owned();
        output_file_path.push(path.file_name().expect("Couln't get file name."));
        let mut writer = writer(&output_file_path);
        path.read_dir()
            .expect("read_dir call failed")
            .filter_map(Result::ok)
            .filter(|f| {
                if let Some(e) = f.path().extension() {
                    e == "vm"
                } else {
                    false
                }
            })
            .for_each(|f| {
                translate(&mut parser(&f.path()), &mut writer);
            });
    } else {
        let mut writer = writer(&path);
        let mut parser = parser(&path);
        translate(&mut parser, &mut writer);
    }
}

fn parser(path: &PathBuf) -> Parser {
    if !path
        .extension()
        .expect("target file must have extension.")
        .to_string_lossy()
        .ends_with("vm")
    {
        panic!("target file's extension must be .vm");
    }
    let file = File::open(path.clone()).expect("Couldn't open file.");
    return Parser::new(&file);
}

fn writer(path: &PathBuf) -> CodeWriter {
    println!("path: {:?}", path);
    let mut path = path.to_owned();
    path.set_extension("asm");
    println!("path.asm: {:?}", path);
    let output_file = File::create(path).expect("Couldn't create file.");
    return CodeWriter::new(output_file);
}

fn translate(parser: &mut Parser, writer: &mut CodeWriter) {
    while parser.has_more_commands() {
        parser.advance();
        use parser::VMCommandType::*;
        match parser.command_type() {
            ARITHMETIC => writer.write_arithmetic(parser.current_command.to_string()),
            PUSH => writer.write_push_pop(parser.command_type(), parser.arg1(), parser.arg2()),
            _ => panic!("TOOD"),
        }
    }
    writer.close();
}
