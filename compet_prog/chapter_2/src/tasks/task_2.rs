fn task() {
    let mut count = 0;
    recursion(0, &mut count);
    println!("{count}");
}

fn recursion(idx: usize, count: &mut i32) {
    if idx == 6 {
        *count += 1;
        return;
    }

    for _i in 1..=4 {
        recursion(idx + 1, count);
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::task_2::task;

    #[test]
    fn it_works() {
        task();
    }
}