fn main() {
    let mut i = 0;
    let inc = || {
        i += 1;
        println!("{}", i);
    };

    call_twice(inc);
}

fn call_twice<F>(mut fun: F) where F: FnMut() {
    fun();
    fun();
}