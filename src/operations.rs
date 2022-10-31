
pub mod stacks {
    pub struct Stacks {
        pub(crate) stack_a: Vec<i32>,
        // stack_b: Vec<u32>
    }
    fn sa(stacks: &mut Stacks) {
        if stacks.stack_a.len() >= 2 {
            let f: i32 = stacks.stack_a[0];
            let s: i32 = stacks.stack_a[1];
            stacks.stack_a.remove(0);
            stacks.stack_a.remove(0);
            stacks.stack_a.insert(0, f);
            stacks.stack_a.insert(0, s);
        }
        println!("sa");
    }

    fn rra(stacks: &mut Stacks) {
        let i: i32 = stacks.stack_a[stacks.stack_a.len() - 1];
        stacks.stack_a.pop();
        stacks.stack_a.insert(0, i);
        println!("rra");
    }

    fn ra(stacks: &mut Stacks) {
        let i: i32 = stacks.stack_a[0];
        stacks.stack_a.remove(0);
        stacks.stack_a.push(i);
        println!("ra");
    }

    fn do_three(stacks: &mut Stacks) {
        if stacks.stack_a[0] < stacks.stack_a[1] && stacks.stack_a[0] < stacks.stack_a[2] {
            rra(stacks);
            sa(stacks);
        } else if stacks.stack_a[1] < stacks.stack_a[0] && stacks.stack_a[1] < stacks.stack_a[2] {
            if stacks.stack_a[2] > stacks.stack_a[0] {
                sa(stacks);
            } else {
                ra(stacks);
            }
        } else if stacks.stack_a[2] < stacks.stack_a[0] && stacks.stack_a[2] < stacks.stack_a[0] {
            if stacks.stack_a[0] > stacks.stack_a[1] {
                sa(stacks);
            }
            rra(stacks);
        }
    }

    pub fn sort(stacks: &mut Stacks) {
        if stacks.stack_a.len() == 3 {
            do_three(stacks);
            println!("{:?}", stacks.stack_a);
        }
    }
}
