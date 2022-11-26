pub trait Summary{
    fn summarize(&self) -> String{
        String::from("this is a test")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String, 
}

impl Summary for NewsArticle{
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn largest<T: PartialOrd>(nums: &[T]) -> &T{
    let mut largest = &nums[0];

    for num in nums.iter() {
        if largest < num {
            largest = num;
        }
    }
    largest
}

fn main() {
    println!("Hello, world!");

    let article = NewsArticle{
        headline: String::from("this is the headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("this is all the content"),
    };

    println!("{}", article.summarize());
    notify(&article);

    let v = vec!(1, 2, 6, 4, 9);
    let num = largest(&v);
    println!("{:?}, max = {}", v, num);

    let v = vec!(1.0, 2.0, 6.0, 4.0, 9.1);
    let num = largest(&v);
    println!("{:?}, max = {}", v, num);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let num = largest(&char_list);
    println!("{:?}, max = {}", char_list, num);
}
