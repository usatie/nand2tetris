use std::collections::HashMap;

pub struct SymbolTable {
    table: HashMap<String, u16>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: [
                ("SP".to_string(), 0x0000u16),
                ("LCL".to_string(), 0x0001u16),
                ("ARG".to_string(), 0x0002u16),
                ("THIS".to_string(), 0x0003u16),
                ("THAT".to_string(), 0x0004u16),
                ("R0".to_string(), 0x0000u16),
                ("R1".to_string(), 0x0001u16),
                ("R2".to_string(), 0x0002u16),
                ("R3".to_string(), 0x0003u16),
                ("R4".to_string(), 0x0004u16),
                ("R5".to_string(), 0x0005u16),
                ("R6".to_string(), 0x0006u16),
                ("R7".to_string(), 0x0007u16),
                ("R8".to_string(), 0x0008u16),
                ("R9".to_string(), 0x0009u16),
                ("R10".to_string(), 0x000au16),
                ("R11".to_string(), 0x000bu16),
                ("R12".to_string(), 0x000cu16),
                ("R13".to_string(), 0x000du16),
                ("R14".to_string(), 0x000eu16),
                ("R15".to_string(), 0x000fu16),
                ("SCREEN".to_string(), 0x4000u16),
                ("KBD".to_string(), 0x6000u16),
            ]
            .into_iter()
            .collect(),
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
    use std::{collections::HashMap, os::unix::fs::symlink};

    use crate::symbol_table::{self, SymbolTable};

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

        let symbol_table = SymbolTable::new();
        assert!(symbol_table.contains(String::from("SP")));
        assert!(symbol_table.contains(String::from("LCL")));
        assert!(symbol_table.contains(String::from("ARG")));
        assert!(symbol_table.contains(String::from("THIS")));
        assert!(symbol_table.contains(String::from("THAT")));
        assert!(symbol_table.contains(String::from("R0")));
        assert!(symbol_table.contains(String::from("R15")));
        assert!(symbol_table.contains(String::from("SCREEN")));
        assert!(symbol_table.contains(String::from("KBD")));
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

        let symbol_table = SymbolTable::new();
        assert_eq!(symbol_table.get_address(String::from("SP")), 0);
        assert_eq!(symbol_table.get_address(String::from("LCL")), 1);
        assert_eq!(symbol_table.get_address(String::from("ARG")), 2);
        assert_eq!(symbol_table.get_address(String::from("THIS")), 3);
        assert_eq!(symbol_table.get_address(String::from("THAT")), 4);
        assert_eq!(symbol_table.get_address(String::from("R0")), 0);
        assert_eq!(symbol_table.get_address(String::from("R15")), 15);
        assert_eq!(symbol_table.get_address(String::from("SCREEN")), 16384);
        assert_eq!(symbol_table.get_address(String::from("KBD")), 24576);
    }
}
