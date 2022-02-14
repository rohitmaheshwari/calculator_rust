use std::env::{args, Args};

// fn nth(&mut self, n: usize) -> Option<String> {
// 	// asume n = 0
// 	self.inner.next();
// 	//calling next again in result to get second element
// 	return self.inner.next();
// }

/* first way with if else
fn calc(first_num: f32, operator: char, second_num: f32) -> f32 {
    if operator == '+' {
        return first_num + second_num;
    }
    else if operator == '-' {
        return first_num - second_num;
    }
    else if operator == '*' {
        return first_num * second_num;
    }
    else if operator == '/' {
        return first_num / second_num;
    }
    else {
        println!("Wrong operator!");
        return 0.0;
    }
}
*/

fn calc(first_num: f32, operator: char, second_num: f32) -> f32 {
    match operator {
        '+' => first_num + second_num,
        '-' => first_num - second_num,
        '*' => first_num * second_num,
        '/' => first_num / second_num,
        // _ => 0.0
        _ => panic!("Wrong operator!"),
    }
}

fn output(first_num: f32, operator: char, second_num: f32, result: f32) {
    println!("{} {} {} = {}", first_num, operator, second_num, result);
}

fn main() {
    let mut args: Args = args();
    // initally starts with 0 index have main, and when you input 1 arg, you can access it by nth(1)
    // but when we access 1st arg, it jumps to the next element and considers it as the 0th index element
    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    let first_num: f32 = first.parse().unwrap(); // one way to parse
    let second_num = second.parse::<f32>().unwrap(); // second way to parse

    let result: f32 = calc(first_num, operator, second_num);

    println!("{:?} {:?} {:?}", first_num, operator, second);
    println!("");
    output(first_num, operator, second_num, result);
}
