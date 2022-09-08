use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Order {
    deadline: usize,
    bonus: usize,
}

impl Order {
    fn new(deadline: usize, bonus: usize) -> Self {
        Self {
            deadline,
            bonus,
        }
    }
}

fn count_bounus(vec: &mut Vec<Order>) -> usize {
    let mut sum = 0;
    let mut used_days = vec![false; 100000000];

    for order in vec {
        let mut k = order.deadline;
        while k > 0 && used_days[k] {
            k -= 1;
        }
        if k == 0 {
            continue;
        }
        used_days[k] = true;
        sum += order.bonus
    }

    sum
}

fn task(filename: &str) -> Result<(), Box<dyn Error>> {
    println!("{filename}");
    let file = File::open(filename)?;
    let mut vec = Vec::new();
    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        if let Ok(data) = line {
            if i == 0 {
                println!("Кол-во строк в файле {data}");
            } else {
                let numbers: Vec<&str> = data.trim().split(" ").collect();
                let deadline: usize = numbers[0].trim().parse().expect("Не корректные данные(");
                let bonus: usize = numbers[1].trim().parse().expect("Не корректные данные(");
                vec.push(Order::new(deadline, bonus));
            }
        }
    }

    vec.sort_by(|a, b| b.bonus.cmp(&a.bonus));
    println!("{}", count_bounus(&mut vec));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tasks::task_2::task;

    #[test]
    fn it_works() {
        let _re = task("schedule2.in");
    }
}