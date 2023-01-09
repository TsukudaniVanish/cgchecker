use super::symbol::Symbol;

#[derive(Debug)]
pub struct NonTerminal {
    pub meta_string: String
}

impl Symbol for NonTerminal {
    fn to_string(&self) -> String {
        self.meta_string.clone()
    }
}
