use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug)]
struct Product {
    weight: usize,
    profit: usize,
}

impl Product {
    fn new(weight: usize, cost: usize) -> Self {
        Self {
            weight,
            profit: cost / weight,
        }
    }
}

fn count_bounus(vec: &mut Vec<Product>, global_weight: usize) -> usize {
    let mut sum = 0;
    let mut total_weight = 0;

    for product in vec {
        while product.weight > 0 && global_weight - total_weight > 0 {
            sum += product.profit;
            total_weight += 1;
            product.weight -= 1;
        }
    }

    sum
}

fn task(filename: &str) -> Result<(), Box<dyn Error>> {
    println!("{filename}");
    let file = File::open(filename)?;
    let mut vec = Vec::new();
    let mut global_weigth: usize = 0;
    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        if let Ok(data) = line {
            if i == 0 {
                let numbers: Vec<&str> = data.trim().split(" ").collect();
                global_weigth = numbers[1].trim().parse().expect("Не корректные данные(");
                println!("Total weight is {}", global_weigth);
            } else {
                let numbers: Vec<&str> = data.trim().split(" ").collect();
                let weight: usize = numbers[0].trim().parse().expect("Не корректные данные(");
                let cost: usize = numbers[1].trim().parse().expect("Не корректные данные(");
                vec.push(Product::new(weight, cost));
            }
        }
    }

    vec.sort_by(|a, b| b.profit.cmp(&a.profit));
    // println!("{:?}", vec);
    println!("{}", count_bounus(&mut vec, global_weigth));
    // println!("{:?}", vec);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tasks::task_4::task;

    #[test]
    fn it_works() {
        let _re = task("cont2.in");
    }
}