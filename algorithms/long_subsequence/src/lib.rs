use std::io;

fn check() {
    let mut text = String::from("");
    io::stdin().read_line(&mut text).unwrap();
    let n: usize = text.trim().parse().unwrap();

    let mut text = String::from("");
    io::stdin().read_line(&mut text).unwrap();
    let vec: Vec<isize> = text.trim().split(" ").map(|x| (x.parse().unwrap())).collect();

    let mut d: Vec<isize> = vec![isize::MIN; n + 1];
    d[0] = isize::MAX;

    let mut prev: Vec<isize> = vec![0; n];
    let mut pos: Vec<isize> = vec![-1; n + 1];

    let mut length = 0;

    for i in 0..n {
        let mut left = 0;
        let mut right = n;
        while right - left > 1 {
            let middle = (left + right) / 2;
            if d[middle] < vec[i] {
                right = middle;
            } else {
                left = middle;
            }
        }
        d[right] = vec[i];
        pos[right] = i as isize;
        prev[i] = pos[right - 1];
        length = usize::max(length, right);
    }

    let mut answer = Vec::new();
    println!("{}", length);

    let mut p = pos[length];
    while p != -1 {
        answer.push(p + 1);
        p = prev[p as usize];
    }
    answer.reverse();
    for i in answer {
        print!("{} ", i);
    }
}
//15
//7 2 1 3 8 4 9 1 2 6 5 9 3 8 1
//5 3 4 4 2 5 9

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        check();
    }
}
