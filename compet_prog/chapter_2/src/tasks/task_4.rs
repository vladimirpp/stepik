use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

#[derive(Debug)]
struct Path {
    vec: Vec<usize>,
    min_sum: usize,
    index: i32,
    used: Vec<bool>,
}

impl Path {
    fn new(n: usize) -> Self {
        Self {
            vec: vec![0; n],
            index: 0,
            min_sum: 1000,
            used: vec![false; n],
        }
    }
}

fn task(filename: &str) -> Result<(), Box<dyn Error>> {
    println!("{filename}");
    let file = File::open(filename)?;
    let mut n = 0;
    let mut vec = vec![vec![0; 1]; 1];
    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        if let Ok(data) = line {
            if i == 0 {
                println!("Кол-во строк в файле {data}");
                n = usize::from_str(&(data.trim())).expect("Не корректные данные(");
                vec = vec![vec![0; n]; n];
            } else {
                for (j, number) in data.trim().split(" ").enumerate() {
                    vec[i - 1][j] = usize::from_str(&(number.trim())).expect("Не корректные данные(");
                }
            }
        }
    }

    let mut path = Path::new(n + 1);
    let sum = 0;
    let idx = 1;
    rec(&mut vec, &mut path, sum, idx);

    Ok(())
}


fn rec(vec: &mut Vec<Vec<usize>>, path: &mut Path, mut sum: usize, idx: usize) {
    if idx == path.vec.len() - 1 {
        path.index += 1;
        sum += vec[path.vec[idx - 1]][0];
        if sum <= path.min_sum {
            path.min_sum = sum;
            println!(" {0}, {1:?}, {2}", path.index, path.vec, path.min_sum);
            for (i, _) in path.vec.iter().enumerate() {
                if i > path.vec.len() - 2 {
                    continue;
                }
                if i == path.vec.len() - 2 {
                    print!("{} = {} - is the path length", vec[path.vec[i]][0], sum);
                } else {
                    print!("{} + ", vec[path.vec[i]][path.vec[i + 1]]);
                }
            }
            println!();
        }
        return;
    }

    for i in 1..path.vec.len() - 1 as usize {
        if path.used[i] {
            continue;
        }
        sum += vec[path.vec[idx - 1]][i];
        if sum <= path.min_sum {
            path.used[i] = true;
            path.vec[idx] = i;

            rec(vec, path, sum, idx + 1);
            path.used[i] = false;
        }
        sum -= vec[path.vec[idx - 1]][i];
    }
}

#[cfg(test)]
mod tests {
    use crate::tasks::task_4::task;

    #[test]
    fn it_works() {
        let _re = task("salesman.in");
    }
}
