use std::str::from_utf8;

fn display(data: &mut Vec<u8>, vec: &mut Vec<usize>, mas: &mut Vec<u8>, idx: usize, count: &mut usize) {
    if idx == vec.len() {
        *count += 1;
        if *count == 5151 {
            for (i, v) in vec.iter_mut().enumerate() {
                data[*v] = mas[i];
            }
            println!("{} {:?}", *count, from_utf8(data).unwrap());
        }
        return;
    }

    for i in b'a'..=b'e' {
        mas[idx] = i;
        display(data, vec, mas, idx + 1, count);
    }
}

fn task() {
    let mut data = String::from("?be?bdcb?dcb?debcd??bdad?ee").into_bytes();
    let mut vec = Vec::new();

    for (i, char) in data.iter().enumerate() {
        if *char == b'?' {
            vec.push(i);
        }
    }

    let mut mas = vec![b'a'; vec.len()];
    let mut count = 0;
    display(&mut data, &mut vec, &mut mas, 0, &mut count);
}

#[cfg(test)]
mod tests {
    use crate::tasks::task_1::task;

    #[test]
    fn it_works() {
        task();
    }
}