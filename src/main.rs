use cgchecker::domain::nonterminal::NonTerminal;


fn main() {
    let list:NonTerminal = NonTerminal{ meta_string: "list".to_string()};
    println!("{:?}", list);
    println!("Hello, world!");
}
