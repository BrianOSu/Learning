// slow solution
pub fn daily_temperatures1(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans:Vec<i32> = vec![];
    for i in 0..temperatures.len(){
        if i==temperatures.len()-1 {
            ans.push(0);
            break;
        }

        let mut x = 1;

        for j in i+1..temperatures.len() {
            if temperatures[j] > temperatures[i] {
                ans.push(x);
                break;
            } else if j==temperatures.len()-1 {
                ans.push(0);
            } else {
                x+=1;
            }
        }
    }
    ans
}

// Stack based
pub fn daily_temperatures2(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans:Vec<i32> = vec![0;temperatures.len()];
    let mut stack:Vec<(usize, i32)> = vec![];
    for (index,value) in temperatures.iter().enumerate() {
        while !stack.is_empty() & stack.last().map_or(false, |x| x.1<temperatures[index]){
            let x = stack.pop().unwrap();
            ans[x.0] = (index - x.0) as i32;
        }
        stack.push((index, value.clone()));
    }
    ans
}

pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut ans:Vec<i32> = vec![0;temperatures.len()];
    let mut stack:Vec<usize> = vec![];
    for index in 0..temperatures.len() {
        while !stack.is_empty() & stack.last().map_or(false, |&x| temperatures[x]<temperatures[index]){
            let x = stack.pop().unwrap();
            ans[x] = (index - x) as i32;
        }
        stack.push(index);
    }
    ans
}

fn main() {
    println!("temperatures = [73,74,75,71,69,72,76,73] = {:?}", daily_temperatures(vec![73,74,75,71,69,72,76,73]));
    println!("temperatures = [30,40,50,60] = {:?}", daily_temperatures(vec![30,40,50,60]));
    println!("temperatures = [30,60,90] = {:?}", daily_temperatures(vec![30,60,90]));
}