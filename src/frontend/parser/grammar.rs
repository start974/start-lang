use super::super::lexer::Rule as LexerRule;
use std::collections::{BTreeSet, HashMap};
use std::fmt;
use std::rc::Rc;

type Symbol = Rc<str>;
type Pattern = Vec<Symbol>;

#[derive(Debug)]
pub struct Grammar {
    terminals: BTreeSet<Symbol>,
    // map non terminal symbol to parttern
    rules: HashMap<Symbol, Vec<Pattern>>,
}

impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            terminals: BTreeSet::new(),
            rules: HashMap::new(),
        }
    }

    // add a terminal symbol to the grammar
    pub fn new_terminal(&mut self, symbol_str: &str) -> Symbol {
        let symbol = Rc::from(symbol_str);
        assert!(!self.rules.contains_key(&symbol));
        // check not contain in not terminal
        self.terminals.insert(symbol.clone());
        symbol
    }

    // add a terminal symbol from a rule(use is name) to the grammar
    pub fn from_lexrule(&mut self, rule: &LexerRule) -> Symbol {
        self.new_terminal(rule.name())
    }

    // add a nonterminal symbol to the grammar
    pub fn new_nonterminal(&mut self, symbol_str: &str) -> Symbol {
        let symbol = Rc::from(symbol_str);
        assert!(!self.terminals.contains(&symbol));
        // check not contain in not terminal
        if !self.rules.contains_key(&symbol) {
            self.rules.insert(symbol.clone(), vec![]);
        }
        symbol
    }

    /// remove terminal symbol
    pub fn remove_symbol(&mut self, symbol: &Symbol) -> bool {
        self.terminals.remove(symbol) || self.rules.remove_entry(symbol).is_some()
    }

    /// true if symbol is terminal
    pub fn is_terminal(&self, symbol: &Symbol) -> bool {
        self.terminals.contains(symbol)
    }

    /// true if symbol is nonterminal
    pub fn is_nonterminal(&self, symbol: &Symbol) -> bool {
        self.rules.contains_key(symbol)
    }

    /// if symbol exist
    pub fn has_symbol(&self, symbol: &Symbol) -> bool {
        self.is_terminal(symbol) || self.is_nonterminal(symbol)
    }

    /// add rule to grammar
    pub fn add_rule(&mut self, symbol: &Symbol, pattern: Pattern) {
        // check symbol is nonterminal and declare
        assert!(
            !self.is_terminal(symbol),
            "symbol '{symbol}' is terminal (please use a nonterminal symbol)"
        );
        assert!(
            self.is_nonterminal(symbol),
            "non terminal symbol '{symbol}' is undeclare"
        );
        // check if all symbols in pattern exist
        let unknow_symbols: Vec<Symbol> = pattern
            .iter()
            .filter(|symb| !self.has_symbol(symb))
            .map(Rc::clone)
            .collect();
        assert!(unknow_symbols.is_empty(), "unknow symbols: {unknow_symbols:?}, please declare with `new_terminal` or `new_nonterminal`");

        // push pattern
        self.rules.get_mut(symbol).unwrap().push(pattern);
    }
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Symbols Terminal: {:?}", self.terminals)?;
        writeln!(f, "Symbols Nonterminal: {:?}", 
            self.rules.keys().cloned().collect::<BTreeSet<_>>())?;
        writeln!(f, "")?;
        writeln!(f, "Rules:")?;
        for (symbol, patterns) in &self.rules {
            for pattern in patterns {
                let string_pattern = pattern
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                writeln!(f, "{symbol} -> {string_pattern}")?;
            }
        }
        Ok(())
    }
}
