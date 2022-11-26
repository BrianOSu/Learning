pub fn pig_latin(s: &mut String) -> String{
    let mut letter = s.chars();

    let first_char = match letter.next() {
        Some(c) => c,
        None => return String::new(),
    };

    match first_char{
        'a'|'e'|'i'|'o'|'u' => format!("{}-hay", s),
        _ => format!("{}-{}ay", s, first_char)
    }
}