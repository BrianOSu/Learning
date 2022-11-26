struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn square(side: u32) -> Rectangle {
        Rectangle{ width: side, height: side }
    }

    fn area(&self) -> u32 {
        self.width*self.height
    }

    fn perim(&self) -> u32 {
        self.width + self.height
    }

    fn contains(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle{ width: 30, height: 50 };
    println!("The area is: {}", rect.area());
    println!("The perim is: {}", rect.perim());

    let rect1 = Rectangle::square(20);
    println!("The area is: {}", rect1.area());
    println!("The perim is: {}", rect1.perim());


    let rect2 = Rectangle{ width: 60, height: 100 };
    println!("rect contains rect1? {}", rect.contains(&rect1));
    println!("rect contains rect2? {}", rect.contains(&rect2));
}



