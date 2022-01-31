use std::fs::File;

use crate::parser::VMCommandType;

pub struct CodeWriter {
    // hey
}

impl CodeWriter {
    pub fn new(file: File) -> Self {
        Self {}
    }
    pub fn set_file_name(file_name: String) {
        panic!("TODO");
    }
    pub fn write_arithmetic(command: String) {
        panic!("TODO");
    }
    pub fn write_push_pop(command: VMCommandType, segment: String, index: u16) {
        panic!("TODO");
    }
    pub fn close() {
        panic!("TODO");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert!(true);
    }
    #[test]
    fn test_set_file_name() {
        assert!(true);
    }
    #[test]
    fn test_write_arithmetic() {
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
