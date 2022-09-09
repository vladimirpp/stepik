#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!();
        let mut vec = vec![1361955, 207579012, 628145516, 376140462, 883515280, 186969585, 762888635, 326402539];

        for number in vec.iter_mut() {
            let mut bit_count = 0;
            while *number > 0 {
                bit_count += *number & 1;
                *number >>= 1;
            }

            print!("{} ", bit_count);
        }
    }
}