use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Pair {
    left: usize,
    right: usize,
}

impl Pair {
    fn new(left: usize, right: usize) -> Self {
        Self {
            left,
            right,
        }
    }
}

fn count_friends(vec: Vec<Pair>) -> usize {
    let mut pairs = vec![0x00; 20];

    for i in 0..20 {
        pairs[i] = 1 << i;
    }

    for v in &vec {
        pairs[v.left - 1] |= 1 << (v.right - 1);
        pairs[v.right - 1] |= 1 << (v.left - 1);
    }

    let mut max = 0;
    for mask in 1..1 << 20 {
        let mut count = 0;
        for pair in &pairs {
            if (pair & mask) == mask {
                count += 1;
            }
        }
        let mut _count = 0;
        let mut _mask = mask;
        while _mask > 0 {
            _count += _mask & 1;
            _mask >>= 1;
        }

        if max < count && _count == count {
            max = count;
        }
    }
    max
}

fn task(filename: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut vec = Vec::new();
    for line in io::BufReader::new(file).lines() {
        if let Ok(data) = line {
            let numbers: Vec<&str> = data.trim().split(" ").collect();
            let left: usize = numbers[0].trim().parse().expect("Не корректные данные(");
            let right: usize = numbers[1].trim().parse().expect("Не корректные данные(");
            vec.push(Pair::new(left, right));
        }
    }

    println!("{}", count_friends(vec));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tasks::task_6::task;

    #[test]
    fn it_works() {
        let _re = task("friends2.in");
    }
}