
pub mod stacks {
    pub struct Stacks {
        pub(crate) stack_a: Vec<i32>,
        // stack_b: Vec<u32>
    }
    pub fn sa(stacks: &mut Stacks) {
        if stacks.stack_a.len() >= 2 {
            let f: i32 = stacks.stack_a[0];
            let s: i32 = stacks.stack_a[1];
            stacks.stack_a.remove(0);
            stacks.stack_a.remove(0);
            stacks.stack_a.insert(0, f);
            stacks.stack_a.insert(0, s);
        }
        // println!("{:?}", stacks.stack_a);
    }



}
