use crate::{K, monadic};

pub fn nyi(_: &K, _: &K) -> K {
    K::Error
}

pub fn add(x: &K, y: &K) -> K {
    match x {
        &K::Atom(a) => match y {
            &K::Atom(b) => K::Atom(a + b),
            K::List(b) => K::List(b.into_iter().map(|x| x+a).collect()),
            &K::Error => K::Error,
        },
        K::List(a) => match y {
            &K::Atom(b) => K::List(a.into_iter().map(|x| x+b).collect()),
            K::List(b) => if a.len() == b.len() { K::List(a.iter().zip(b.iter()).map(|(x, y)| x+y).collect()) } else { K::Error },
            &K::Error => K::Error,
        },
        &K::Error => K::Error,
    }
}

pub fn sub(x: &K, y: &K) -> K {
    add(x, &monadic::sub(y))
}


pub fn ind(x: &K, y: &K) -> K {
    match x {
        &K::Atom(a) => match y {
            &K::Atom(b) => if a==0 {K::Error} else {K::Atom(b % a)},
            K::List(b) => if a==0 {K::Error} else {K::List(b.into_iter().map(|x| x % a).collect())},
            _  => K::Error,
        },
        _ => K::Error
    }
}

pub fn cnt(x: &K, y: &K) -> K {
    match x {
        &K::Atom(a) => match y {
            &K::Atom(b) => K::List(vec![a; b as usize]),
            K::List(b) => K::List((0..a as usize).map(|x| b[x % b.len()]).collect()),
            _ => K::Error,
        },
        _ => K::Error
    }
}

pub fn cat(x: &K, y: &K) -> K {
    match x {
        &K::Atom(a) => match y {
            &K::Atom(b) => K::List(vec![a].into_iter().chain(vec![b]).collect::<Vec<_>>()),
            K::List(b) => K::List(vec![a].into_iter().chain(b.clone()).collect::<Vec<_>>()),
            _  => K::Error,
        },
        K::List(a) => match y {
            &K::Atom(b) => K::List(a.clone().into_iter().chain(vec![b]).collect::<Vec<_>>()),
            K::List(b) => K::List(a.clone().into_iter().chain(b.clone()).collect::<Vec<_>>()),
            _ => K::Error,
        },
        _ => K::Error
    }
}

pub fn at(x: &K, y: &K) -> K {
    match x {
        &K::Atom(a) => match y {
            &K::Atom(b) => if b!=0 {K::Error} else {K::Atom(a)},
            _  => K::Error,
        },
        K::List(a) => match y {
            &K::Atom(b) => if b>a.iter().count() as i64 {K::Error} else {K::Atom(a[b as usize])},
            K::List(b) => K::List(a.iter().map(|&i| b[i as usize]).collect::<Vec<_>>()),
            _ => K::Error,
        },
        _ => K::Error
    }
}

pub const DYADIC: [fn(&K,&K)->K; 7] = [nyi, add, sub, ind, cnt, cat, at];