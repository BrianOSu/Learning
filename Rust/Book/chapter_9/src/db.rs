use std::collections::HashMap;

pub enum Command {
    Add {name: String, department: String},
    List(String),
    All,
    Quit,
}

impl Command{
    pub fn match_input(input: String) -> Option<Self> {
        let words: Vec<&str>  = input.split_whitespace().collect();

        match words.as_slice(){
            ["Add",name,"to",department] => Some(Command::Add{name: name.to_string(), department: department.to_string()}),
            ["All"] => Some(Command::All),
            ["List", department] => Some(Command::List(department.to_string())),
            ["Quit"] => Some(Command::Quit),
            _ => None,
        }
    }
}

pub fn add_to_db(db: &mut HashMap<String, Vec<String>>, department: String, name: String){
    db.entry(department).or_default().push(name);
}

pub fn list_department(db: &mut HashMap<String, Vec<String>>, department: String){
    match db.get_mut(&department) {
        Some(names) => { names.sort(); println!("{:?}", names) },
        None => ()
    }
}

pub fn list_all(db: &mut HashMap<String, Vec<String>>){
    for (department, names) in db{
        names.sort();
        println!("{} {:?}", department, names);
    }
}