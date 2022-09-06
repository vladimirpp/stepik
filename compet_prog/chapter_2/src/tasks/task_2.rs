fn task() {
    let mut count = 0;
    rec(0, &mut count);
    println!("{count}");
}

fn rec(idx: usize, count: &mut i32) {
    if idx == 6 {
        *count += 1;
        return;
    }

    for _i in 1..=4 {
        rec(idx + 1, count);
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