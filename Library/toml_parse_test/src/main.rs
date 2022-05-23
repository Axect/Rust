use toml_parse::{parse_it, SyntaxNodeExtTrait};

fn main() {
    let file = 
r#"[deps]
alpha = "beta"
number = 1234
array = [ true, false, true ]
inline-table = { date = 1988-02-03T10:32:10, }
"#;

    let parsed = parse_it(file).expect("parse_failed");
    let root = parsed.syntax();
    println!("{}", root.token_text());
    println!("{}", root.children_with_tokens().into_iter().for_each(|x| println!("{:?}", x))); 
}
