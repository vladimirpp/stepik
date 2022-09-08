use std::cmp::Ordering;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufRead;

#[derive(Debug, Eq, PartialEq, Ord)]
struct Meeting {
    left: usize,
    right: usize,
}

impl PartialOrd for Meeting {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // if self.left == other.left {
        //     return Some(self.right.cmp(&other.right));
        // }
        Some(self.left.cmp(&other.left))
    }
}

impl Meeting {
    fn new(left: usize, right: usize) -> Self {
        Self {
            left,
            right,
        }
    }
}

fn count_rooms(vec: &mut Vec<Meeting>) -> usize {
    let mut count = 0;
    let mut used_meetings = vec![false; vec.len()];

    for i in 0..vec.len() {
        let mut last = i;
        if !used_meetings[i] {
            for j in i + 1..vec.len() {
                if !used_meetings[j] && vec[j].left >= vec[last].right {
                    used_meetings[j] = true;
                    last = j;
                }
            }
            used_meetings[i] = true;
            count += 1;
        }
    }

    // for room in used_meetings.iter_mut() {
    //     if *room {
    //         count += 1;
    //     }
    // }

    count
}

fn task(filename: &str) -> Result<(), Box<dyn Error>> {
    println!("{filename}");
    let file = File::open(filename)?;
    let mut vec = Vec::new();
    let mut _j = 0;
    for (i, line) in io::BufReader::new(file).lines().enumerate() {
        if let Ok(data) = line {
            if i == 0 {} else {
                // _j += 1;
                // if _j == 20 {
                //     break;
                // }
                let numbers: Vec<&str> = data.trim().split(" ").collect();
                let left: usize = numbers[0].trim().parse().expect("Не корректные данные(");
                let right: usize = numbers[1].trim().parse().expect("Не корректные данные(");
                vec.push(Meeting::new(left, right));
            }
        }
    }

    vec.sort();

    // println!("{:?}", vec);
    println!("{}", count_rooms(&mut vec));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tasks::task_6::task;

    #[test]
    fn it_works() {
        let _re = task("request2.in");
    }
}