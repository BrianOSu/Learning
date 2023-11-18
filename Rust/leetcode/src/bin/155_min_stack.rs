struct MinStack {
    s: Vec<i32>,
    min: Vec<i32>
}

impl MinStack {

    fn new() -> Self {
        MinStack{
            s: Vec::new(),
            min: Vec::new()
        }
    }

    fn push(&mut self, val: i32) {
        self.s.push(val);
        let current_min = std::cmp::min(self.min.last().copied().unwrap_or(val), val);
        self.min.push(current_min)
    }

    fn pop(&mut self) {
        self.s.pop();
        self.min.pop();
    }

    fn top(&self) -> i32 {
        self.s.last().unwrap().clone()
    }

    fn get_min(&self) -> i32 {
        self.min.last().unwrap().clone()
    }
}




fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    println!("{}", obj.get_min());
    obj.pop();
    println!("{}", obj.top());
    println!("{}", obj.get_min());
}
