#[derive(Debug, PartialEq)]
pub enum TokenType {
    KEYWORD,
    SYMBOL,
    IDENTIFIER,
    INT,
    STRING,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    CLASS,
    METHOD,
    FUNCTION,
    CONSTRUCTION,
    INT,
    BOOLEAN,
    CHAR,
    VOID,
    VAR,
    STATIC,
    FIELD,
    LET,
    DO,
    IF,
    ELSE,
    WHILE,
    RETURN,
    TRUE,
    FALSE,
    NULL,
    THIS,
}

#[derive(Default)]
pub struct Tokenizer {
    text: String,
}

impl Tokenizer {
    pub fn has_more_tokens(&self) -> bool {
        return true;
    }

    pub fn advance(&self) {}

    pub fn token_type(&self) -> TokenType {
        return TokenType::KEYWORD;
    }
    pub fn keyword(&self) -> Keyword {
        return Keyword::CLASS;
    }
    pub fn symbol(&self) -> char {
        return 'a';
    }
    pub fn identifier(&self) -> String {
        return "".to_string();
    }
    pub fn int_val(&self) -> u16 {
        return 0;
    }
    pub fn string_val(&self) -> String {
        return "".to_string();
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        let tokenizer = Self::default();
        return tokenizer;
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
    fn test_has_more_tokens() {
        let tokenizer = Tokenizer::new();
        assert!(tokenizer.has_more_tokens());
    }
    #[test]
    fn test_advance() {
        let tokenizer = Tokenizer::new();
        tokenizer.advance();
        assert!(true);
    }
    #[test]
    fn test_token_type() {
        let tokenizer = Tokenizer::new();
        assert_eq!(tokenizer.token_type(), TokenType::KEYWORD);
    }
    #[test]
    fn test_keyword() {
        let tokenizer = Tokenizer::new();
        assert_eq!(tokenizer.keyword(), Keyword::CLASS);
    }
    #[test]
    fn test_symbol() {
        let tokenizer = Tokenizer::new();
        assert_eq!(tokenizer.symbol(), 'a');
    }
    #[test]
    fn test_identifier() {
        let tokenizer = Tokenizer::new();
        assert_eq!(tokenizer.identifier(), "");
    }
    #[test]
    fn test_int_val() {
        let tokenizer = Tokenizer::new();
        assert_eq!(tokenizer.int_val(), 0);
    }
    #[test]
    fn test_string_val() {
        let tokenizer = Tokenizer::new();
        assert_eq!(tokenizer.string_val(), "");
    }
}
