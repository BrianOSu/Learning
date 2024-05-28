use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum K {
    Atom(i64),
    List(Vec<i64>),
    Error,
}

impl fmt::Display for K {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            K::Atom(a) => write!(f, "{}", a),
            K::List(a) => write!(f, "{:?}", a),
            K::Error => write!(f, "Err"),
        }
    }
}