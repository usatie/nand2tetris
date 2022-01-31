pub enum VMCommandType {
    C_ARITHMETIC,
    C_PUSH,
    C_POP,
    C_LABEL,
    C_GOTO,
    C_IF,
    C_FUNCTION,
    C_RETURN,
    C_CALL,
}

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Self {}
    }
    pub fn has_more_commands(&self) -> bool {
        panic!("TODO");
    }
    pub fn advance(&mut self) {
        panic!("TODO");
    }
    pub fn command_type(&self) -> VMCommandType {
        panic!("TODO");
    }
    pub fn arg1(&self) -> String {
        panic!("TODO");
    }
    pub fn arg2(&self) -> u16 {
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
    fn test_advance() {
        assert!(true);
    }
    #[test]
    fn test_command_type() {
        assert!(true);
    }
    #[test]
    fn test_arg1() {
        assert!(true);
    }
    #[test]
    fn test_arg2() {
        assert!(true);
    }
}
