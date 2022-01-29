use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
    pub fn add_entry(&mut self, symbol: String, address: u16) {
        self.table.insert(symbol, address);
    }
    pub fn contains(&self, symbol: String) -> bool {
        self.table.contains_key(symbol.as_str())
    }
    pub fn get_address(&self, symbol: String) -> u16 {
        return self.table[symbol.as_str()];
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::symbol_table::SymbolTable;

    #[test]
    fn test_add_entry() {
        let table: HashMap<String, u16> = HashMap::new();
        let mut symbol_table = SymbolTable { table };
        assert!(!symbol_table.table.contains_key("sum"));
        symbol_table.add_entry(String::from("sum"), 0);
        assert!(symbol_table.table.contains_key("sum"));
        assert_eq!(symbol_table.table["sum"], 0u16);

        assert!(!symbol_table.table.contains_key("LUUP"));
        symbol_table.add_entry(String::from("LUUP"), 1);
        assert!(symbol_table.table.contains_key("LUUP"));
        assert_eq!(symbol_table.table["LUUP"], 1u16);
    }
    #[test]
    fn test_contains() {
        let mut table: HashMap<String, u16> = HashMap::new();
        table.insert(String::from("sum"), 0);
        table.insert(String::from("LUUP"), 1);
        let symbol_table = SymbolTable { table };
        assert!(symbol_table.contains(String::from("sum")));
        assert!(symbol_table.contains(String::from("LUUP")));
        assert!(!symbol_table.contains(String::from("i")));
    }
    #[test]
    fn test_get_address() {
        let mut table: HashMap<String, u16> = HashMap::new();
        table.insert(String::from("sum"), 0);
        table.insert(String::from("LUUP"), 1);
        table.insert(String::from("END"), 2);
        let symbol_table = SymbolTable { table };
        assert_eq!(symbol_table.get_address(String::from("sum")), 0);
        assert_eq!(symbol_table.get_address(String::from("LUUP")), 1);
        assert_eq!(symbol_table.get_address(String::from("END")), 2);
    }
}
