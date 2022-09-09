#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut x = 0;
        for i in 0..5 {
            x |= 1 << i;
        }
        println!("{}", x);
    }
}