use std::env;
use std::process::exit;

pub mod operations;

// pub struct Stack {
//     stack_a: Vec<i32>,
//     // stack_b: Vec<u32>
// }

use operations::stacks;

fn check_sorted(stacks: &stacks::Stacks) -> bool {
    let mut i = 0;
    for _x in &stacks.stack_a {
        if stacks.stack_a[i] == *stacks.stack_a.last().unwrap() {
            return true;
        }
        if stacks.stack_a[i] > stacks.stack_a[i + 1] {
            return false;
        }
        i += 1;
    }
    return true;
}

pub fn main() {
    let mut stacks = stacks::Stacks { stack_a: vec![] /*, stack_b: vec![]*/ };
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    for x in &args {
        stacks.stack_a.push(x.parse().unwrap());
    }
    if check_sorted(&stacks) == true {
        println!("The stack is already sorted!");
        exit(127);
    }
    println!("{:?}", stacks.stack_a);
    stacks::sa(&mut stacks);
}
