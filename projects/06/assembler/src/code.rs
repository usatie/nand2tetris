use bitvec::prelude::*;

pub fn comp(mnemonic: String) -> BitArray {
    match mnemonic.as_str() {
        "0" => bitarr![0, 1, 0, 1, 0, 1, 0],
        "1" => bitarr![0, 1, 1, 1, 1, 1, 1],
        "-1" => bitarr![0, 1, 1, 1, 0, 1, 0],
        "D" => bitarr![0, 0, 0, 1, 1, 0, 0],
        "A" => bitarr![0, 1, 1, 0, 0, 0, 0],
        "!D" => bitarr![0, 0, 0, 1, 1, 0, 1],
        "!A" => bitarr![0, 1, 1, 0, 0, 0, 1],
        "-D" => bitarr![0, 0, 0, 1, 1, 1, 1],
        "-A" => bitarr![0, 1, 1, 0, 0, 1, 1],
        "D+1" => bitarr![0, 0, 1, 1, 1, 1, 1],
        "A+1" => bitarr![0, 1, 1, 0, 1, 1, 1],
        "D-1" => bitarr![0, 0, 0, 1, 1, 1, 0],
        "A-1" => bitarr![0, 1, 1, 0, 0, 1, 0],
        "D+A" => bitarr![0, 0, 0, 0, 0, 1, 0],
        "D-A" => bitarr![0, 0, 1, 0, 0, 1, 1],
        "A-D" => bitarr![0, 0, 0, 0, 1, 1, 1],
        "DA" => bitarr![0, 0, 0, 0, 0, 0, 0],
        "D|A" => bitarr![0, 0, 1, 0, 1, 0, 1],
        "M" => bitarr![1, 1, 1, 0, 0, 0, 0],
        "!M" => bitarr![1, 1, 1, 0, 0, 0, 1],
        "-M" => bitarr![1, 1, 1, 0, 0, 1, 1],
        "M+1" => bitarr![1, 1, 1, 0, 1, 1, 1],
        "M-1" => bitarr![1, 1, 1, 0, 0, 1, 0],
        "D+M" => bitarr![1, 0, 0, 0, 0, 1, 0],
        "D-M" => bitarr![1, 0, 1, 0, 0, 1, 1],
        "M-D" => bitarr![1, 0, 0, 0, 1, 1, 1],
        "DM" => bitarr![1, 0, 0, 0, 0, 0, 0],
        "D|M" => bitarr![1, 0, 1, 0, 1, 0, 1],
        _ => panic!("unexpected mnemonic for code::comp(): {}", mnemonic),
    }
}
pub fn dest(mnemonic: String) -> BitArray {
    match mnemonic.as_str() {
        "" => bitarr![0, 0, 0],
        "M" => bitarr![0, 0, 1],
        "D" => bitarr![0, 1, 0],
        "MD" => bitarr![0, 1, 1],
        "A" => bitarr![1, 0, 0],
        "AM" => bitarr![1, 0, 1],
        "AD" => bitarr![1, 1, 0],
        "AMD" => bitarr![1, 1, 1],
        _ => panic!("unexpected mnemonic for code::dest(): {}", mnemonic),
    }
}
pub fn jump(mnemonic: String) -> BitArray {
    match mnemonic.as_str() {
        "" => bitarr![0, 0, 0],
        "JGT" => bitarr![0, 0, 1],
        "JEQ" => bitarr![0, 1, 0],
        "JGE" => bitarr![0, 1, 1],
        "JLT" => bitarr![1, 0, 0],
        "JNE" => bitarr![1, 0, 1],
        "JLE" => bitarr![1, 1, 0],
        "JMP" => bitarr![1, 1, 1],
        _ => panic!("unexpected mnemonic for code::jump(): {}", mnemonic),
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;

    use crate::code::{comp, dest, jump};

    #[test]
    fn test_comp() {
        //a-bit=0
        assert_eq!(comp(String::from("0")), bitarr![0, 1, 0, 1, 0, 1, 0]);
        assert_eq!(comp(String::from("1")), bitarr![0, 1, 1, 1, 1, 1, 1]);
        assert_eq!(comp(String::from("-1")), bitarr![0, 1, 1, 1, 0, 1, 0]);
        assert_eq!(comp(String::from("D")), bitarr![0, 0, 0, 1, 1, 0, 0]);
        assert_eq!(comp(String::from("A")), bitarr![0, 1, 1, 0, 0, 0, 0]);
        assert_eq!(comp(String::from("!D")), bitarr![0, 0, 0, 1, 1, 0, 1]);
        assert_eq!(comp(String::from("!A")), bitarr![0, 1, 1, 0, 0, 0, 1]);
        assert_eq!(comp(String::from("-D")), bitarr![0, 0, 0, 1, 1, 1, 1]);
        assert_eq!(comp(String::from("-A")), bitarr![0, 1, 1, 0, 0, 1, 1]);
        assert_eq!(comp(String::from("D+1")), bitarr![0, 0, 1, 1, 1, 1, 1]);
        assert_eq!(comp(String::from("A+1")), bitarr![0, 1, 1, 0, 1, 1, 1]);
        assert_eq!(comp(String::from("D-1")), bitarr![0, 0, 0, 1, 1, 1, 0]);
        assert_eq!(comp(String::from("A-1")), bitarr![0, 1, 1, 0, 0, 1, 0]);
        assert_eq!(comp(String::from("D+A")), bitarr![0, 0, 0, 0, 0, 1, 0]);
        assert_eq!(comp(String::from("D-A")), bitarr![0, 0, 1, 0, 0, 1, 1]);
        assert_eq!(comp(String::from("A-D")), bitarr![0, 0, 0, 0, 1, 1, 1]);
        assert_eq!(comp(String::from("DA")), bitarr![0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(comp(String::from("D|A")), bitarr![0, 0, 1, 0, 1, 0, 1]);
        //a-bit=1
        //assert_eq!(comp(String::from("")), bitarr![1, 1, 0, 1, 0, 1, 0]);
        //assert_eq!(comp(String::from("")), bitarr![1, 1, 1, 1, 1, 1, 1]);
        //assert_eq!(comp(String::from("")), bitarr![1, 1, 1, 1, 0, 1, 0]);
        //assert_eq!(comp(String::from("")), bitarr![1, 0, 0, 1, 1, 0, 0]);
        assert_eq!(comp(String::from("M")), bitarr![1, 1, 1, 0, 0, 0, 0]);
        //assert_eq!(comp(String::from("")), bitarr![1, 0, 0, 1, 1, 0, 1]);
        assert_eq!(comp(String::from("!M")), bitarr![1, 1, 1, 0, 0, 0, 1]);
        //assert_eq!(comp(String::from("")), bitarr![1, 0, 0, 1, 1, 1, 1]);
        assert_eq!(comp(String::from("-M")), bitarr![1, 1, 1, 0, 0, 1, 1]);
        //assert_eq!(comp(String::from("")), bitarr![1, 0, 1, 1, 1, 1, 1]);
        assert_eq!(comp(String::from("M+1")), bitarr![1, 1, 1, 0, 1, 1, 1]);
        //assert_eq!(comp(String::from("")), bitarr![1, 0, 0, 1, 1, 1, 0]);
        assert_eq!(comp(String::from("M-1")), bitarr![1, 1, 1, 0, 0, 1, 0]);
        assert_eq!(comp(String::from("D+M")), bitarr![1, 0, 0, 0, 0, 1, 0]);
        assert_eq!(comp(String::from("D-M")), bitarr![1, 0, 1, 0, 0, 1, 1]);
        assert_eq!(comp(String::from("M-D")), bitarr![1, 0, 0, 0, 1, 1, 1]);
        assert_eq!(comp(String::from("DM")), bitarr![1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(comp(String::from("D|M")), bitarr![1, 0, 1, 0, 1, 0, 1]);
    }

    #[test]
    fn test_dest() {
        assert_eq!(dest(String::from("")), bitarr![0, 0, 0]);
        assert_eq!(dest(String::from("M")), bitarr![0, 0, 1]);
        assert_eq!(dest(String::from("D")), bitarr![0, 1, 0]);
        assert_eq!(dest(String::from("MD")), bitarr![0, 1, 1]);
        assert_eq!(dest(String::from("A")), bitarr![1, 0, 0]);
        assert_eq!(dest(String::from("AM")), bitarr![1, 0, 1]);
        assert_eq!(dest(String::from("AD")), bitarr![1, 1, 0]);
        assert_eq!(dest(String::from("AMD")), bitarr![1, 1, 1]);
    }
    #[test]
    fn test_jump() {
        assert_eq!(jump(String::from("")), bitarr![0, 0, 0]);
        assert_eq!(jump(String::from("JGT")), bitarr![0, 0, 1]);
        assert_eq!(jump(String::from("JEQ")), bitarr![0, 1, 0]);
        assert_eq!(jump(String::from("JGE")), bitarr![0, 1, 1]);
        assert_eq!(jump(String::from("JLT")), bitarr![1, 0, 0]);
        assert_eq!(jump(String::from("JNE")), bitarr![1, 0, 1]);
        assert_eq!(jump(String::from("JLE")), bitarr![1, 1, 0]);
        assert_eq!(jump(String::from("JMP")), bitarr![1, 1, 1]);
    }
}
