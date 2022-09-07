#[derive(Debug)]
struct Brace {
    vec: Vec<char>,
    index: i32,
    stack: Vec<char>,
}

impl Brace {
    fn new(n: usize) -> Self {
        Self {
            vec: vec![')'; n],
            index: 0,
            stack: Vec::new(),
        }
    }
}

fn task() {
    let mut brace = Brace::new(6);
    recursion(&mut brace, 0);
}


fn recursion(brace: &mut Brace, idx: usize) {
    if idx == brace.vec.len() {
        brace.index += 1;
        // if brace.index == 8644 {
        println!(" {0}, {1:?}", brace.index, brace.vec);
        return;
    }

    brace.vec[idx] = '(';
    brace.stack.push('(');
    recursion(brace, idx + 1);

    let bracket = brace.stack.pop().unwrap();
    if bracket == '(' {
        brace.vec[idx] = ')';
        recursion(brace, idx + 1);
    } else { brace.stack.push(bracket) }

    brace.vec[idx] = '[';
    brace.stack.push('[');
    recursion(brace, idx + 1);
    let bracket = brace.stack.pop().unwrap();
    if bracket == '(' {
        brace.vec[idx] = ']';
        recursion(brace, idx + 1);
    } else { brace.stack.push(bracket) }
}


#[cfg(test)]
mod tests {
    use crate::tasks::task_7::task;

    #[test]
    fn it_works() {
        task();
    }
}
