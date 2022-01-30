use std::{
    env::args,
    fmt::format,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use bitvec::prelude::*;
use parser::HackCommandType;

mod code;
mod parser;
mod symbol_table;

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        panic!("assembler requires path as an argument.");
    }
    let mut path = PathBuf::from(args[1].to_string());
    //println!("{}", path.to_string_lossy());
    if !path
        .extension()
        .expect("target file must have extension.")
        .to_string_lossy()
        .ends_with("asm")
    {
        panic!("target file's extension must be .asm");
    }

    // Read and assemble .asm file
    let binary: BitVec<u16, Lsb0> = assemble(path.to_str().expect("").to_string());

    // もっと簡単にbitvecをbinaryにformatするやり方はないものか。[]がはいってしまうのが邪魔。
    let lines: Vec<String> = binary
        .chunks(16)
        .map(|x| format!("{:b}", x)) // [1111000011110000]
        .map(|x| x[1..(x.len() - 1)].to_string()) // 1111000011110000
        .map(|x| x + "\n") // 1111000011110000\n
        .collect();
    let output = lines.join("");

    // Save .hack
    path.set_extension("hack");
    let output_file_name = path.file_name().expect("");
    fs::write(output_file_name, output).expect("Couldn't write to output destination.");
}

fn assemble(path: String) -> BitVec<u16, Lsb0> {
    let mut assemblies: Vec<String> = Vec::<String>::new();
    let mut orders = bitvec![u16, Lsb0;0;0];
    let file = File::open(path).expect("Couldn't open the file");
    let mut parser = parser::Parser::new(file);
    while parser.has_more_commands() {
        parser.advance();
        assemblies.push(parser.current_command.to_string());
        match parser.command_type() {
            HackCommandType::A => {
                let value_bits_u16: u16 = parser.symbol().parse().unwrap();
                let mut bits = bitarr![u16, Msb0; 0; 16];
                bits[1..16].store(value_bits_u16);
                orders.append(&mut bits.to_bitvec());
            }
            HackCommandType::C => {
                let comp_bits_u16: u16 = code::comp(parser.comp()).load();
                let dest_bits_u16: u16 = code::dest(parser.dest()).load();
                let jump_bits_u16: u16 = code::jump(parser.jump()).load();

                let mut bits = bitarr![u16, Lsb0; 0; 16];
                bits[0..3].store(0b111u16);
                bits[3..10].store(comp_bits_u16);
                bits[10..13].store(dest_bits_u16);
                bits[13..16].store(jump_bits_u16);
                orders.append(&mut bits.to_bitvec());
            }
            HackCommandType::L => {
                //println!("L_COMMAND: {}", parser.current_command);
            }
        }
    }

    //for (i, order) in orders.chunks(16).enumerate() {
    //    println!("{:10} => {:b}", assemblies[i], order);
    //}
    return orders;
}
