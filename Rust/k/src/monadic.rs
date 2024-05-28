use crate::K;

pub fn nyi(_: &K) -> K {
    K::Error
}

pub fn sub(x: &K) -> K {
    match x {
        &K::Atom(a) => K::Atom(-a),
        K::List(a) => K::List(a.iter().map(|x| -x).collect()),
        &K::Error => K::Error
    }
}

pub fn ind(x: &K) -> K {
    match x {
        &K::Atom(a) => K::List((0..a).collect::<Vec<_>>()),
        _ => K::Error
    }
}

pub fn cnt(x: &K) -> K {
    match x {
        &K::Atom(_) => K::Atom(1),
        K::List(a) => K::Atom(a.iter().count() as i64),
        &K::Error => K::Error
    }
}

pub fn cat(x: &K) -> K {
    match x {
        &K::Atom(a) => K::List(vec![a]),
        _ => K::Error
    }
}

pub fn at(x: &K) -> K {
    match x {
        &K::Atom(a) => K::Atom(a),
        K::List(a) => K::Atom(a[0]),
        _ => K::Error
    }
}

pub const MONADIC: [fn(&K)->K; 7] = [nyi, nyi, sub, ind, cnt, cat, at];