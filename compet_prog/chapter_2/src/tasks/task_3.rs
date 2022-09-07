#[derive(Debug)]
struct Brace {
    vec: Vec<char>,
    index: i32,
}

impl Brace {
    fn new(n: usize) -> Self {
        Self {
            vec: vec![')'; n],
            index: 0,
        }
    }
}

fn task() {
    let mut brace = Brace::new(20);
    let sum = 0;
    let idx = 0;
    recursion(&mut brace, sum, idx);
}


fn recursion(brace: &mut Brace, sum: i32, idx: usize) {
    if sum == ((brace.vec.len() / 2) as i32) {
        if is_valid(&brace.vec) {
            brace.index += 1;
            if brace.index == 8644 {
                println!(" {0}, {1:?}", brace.index, brace.vec);
            }
        }
        return;
    }
    if idx == brace.vec.len() {
        return;
    }
    brace.vec[idx] = '(';
    recursion(brace, sum + 1, idx + 1);
    brace.vec[idx] = ')';
    recursion(brace, sum, idx + 1);
}

fn is_valid(vec: &Vec<char>) -> bool {
    let mut count = 0;
    for v in vec {
        if *v == '(' {
            count += 1;
        }
        if *v == ')' {
            count -= 1;
            if count < 0 {
                return false;
            }
        }
    }
    return count == 0;
}


#[cfg(test)]
mod tests {
    use crate::tasks::task_3::task;

    #[test]
    fn it_works() {
        task();
    }
}
