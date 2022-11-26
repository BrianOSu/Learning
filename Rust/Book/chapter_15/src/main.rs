enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Drop has been called with existing data {}", self.data);
    }
}

fn main() {
    let b = Box::new(5);
    println!("Hello, world! {}", b);

    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));

    let custom = CustomSmartPointer {
        data: String::from("custom data"),
    };
    println!("Custom pointer created with data {}", custom.data);

    let other_custom = CustomSmartPointer {
        data: String::from("other data"),
    };
    println!("Custom pointer created with data {}", other_custom.data);
    drop(custom);
    drop(other_custom);

    println!("count before creating c = {}", Rc::strong_count(&a));
    let c = Cons(3, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
    {
        let d = Cons(4, Rc::clone(&a));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }
    println!("count after d goes out of scope= {}", Rc::strong_count(&a));
}


pub trait Messenger{
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger>{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a,T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T>{
        LimitTracker{
            messenger,
            value: 0,
            max,
        }
    }
    
    pub fn set_value(&mut self, value: usize){
        self.value = value;
        let pcnt = self.value as f64 / self.max as f64;
        if pcnt > 0.9 {
            self.messenger.send("The percentage value is too high");
        } else {
            self.messenger.send("The percentage is fine");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger{
        fn new() -> MockMessenger{
            MockMessenger{
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, message: &str){
            // Example showing how multiple borrows would break
            //let mut borrow_one = self.sent_messages.borrow_mut();
            //let mut borrow_two = self.sent_messages.borrow_mut();

            //borrow_one.push(String::from(message));
            //borrow_two.push(String::from(message));
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}