use super::symbol::Symbol;

pub type Terminal = String;

impl Symbol for Terminal {
    fn to_string(&self) -> String {
        self.clone()
    }
}