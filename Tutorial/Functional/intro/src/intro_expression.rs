#[derive(Debug)]
pub enum Term {
    TermVal { value: String },
    TermVar { symbol: String },
    TermApp { f: Box<Term>, x: Box<Term> }, //TODO: What is Box?
    TermAbs { arg: String, body: Box<Term> }
}

pub fn term() -> String {
    let mut t = Term::TermVar {
        symbol: "X".to_string()
    };

    match t {
        Term::TermVal { value: v1 } => v1,
        Term::TermVar { symbol: v1 } => v1,
        Term::TermApp { f: ref v1, x: ref v2 } =>
            "TermApp(?,?)".to_string(),
        Term::TermAbs { arg: ref mut v1, body: ref mut v2 } =>
            "TermAbs(?,?)".to_string()
    }
}