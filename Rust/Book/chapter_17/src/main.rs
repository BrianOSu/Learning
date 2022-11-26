use chapter_17::Post;

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw()
        }
    }
}


pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button{
    fn draw(&self) {
        // code for a button
    }
}


struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}


fn main() {
    println!("Hello, world!");

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 70,
                height: 60,
                options: vec![String::from("Yes"),
                              String::from("Maybe"),
                              String::from("No")],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();


    let mut post = Post::new();
    
    post.add_text("I ate a salad today");
    println!("test 1 {}", post.content());
    assert_eq!("", post.content());

    post.request_review();
    println!("test 2 {}", post.content());
    assert_eq!("", post.content());

    post.approve();
    println!("test 3 {}", post.content());
    assert_eq!("I ate a salad today", post.content());
}
