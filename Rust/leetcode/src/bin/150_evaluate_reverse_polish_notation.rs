pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack:Vec<i32> = Vec::new();

    for i in tokens{
        match &*i {
            "+" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a+b);
            },
            "-" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b-a);
            },
            "/" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(b/a);
            },
            "*" => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(a*b);
            },
            x => stack.push(x.parse().unwrap())
        }
    }

    stack.pop().unwrap()
}

fn main() {
    println!("nums = ['2','1','+','3','*'] = {:?}", eval_rpn(vec!["2".to_string(),"1".to_string(),"+".to_string(),"3".to_string(),"*".to_string()]));
    println!("nums = ['4','13','5','/','+'] = {:?}", eval_rpn(vec!["4".to_string(),"13".to_string(),"5".to_string(),"/".to_string(),"+".to_string()]));
    println!("nums = ['10','6','9','3','+','-11','*','/','*','17','+','5','+'] = {:?}", eval_rpn(vec!["10".to_string(),"6".to_string(),"9".to_string(),"3".to_string(),"+".to_string(),"-11".to_string(),"*".to_string(),"/".to_string(),"*".to_string(),"17".to_string(),"+".to_string(),"5".to_string(),"+".to_string()]));
}