use std::collections::{BinaryHeap, HashMap};
use std::io;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
struct LeafNode {
    count: usize,
    ch: char,
    sym: String,
    left: Option<Rc<RefCell<Self>>>,
    right: Option<Rc<RefCell<Self>>>,
}

impl Ord for LeafNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.count.cmp(&self.count)
    }
}

impl PartialOrd for LeafNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl LeafNode {
    fn new(count: usize, ch: char) -> Self {
        Self {
            count,
            ch,
            sym: String::from(""),
            left: None,
            right: None,
        }
    }

    fn internal_node(left_node: Rc<RefCell<Self>>, right_node: Rc<RefCell<Self>>) -> Self {
        Self {
            count: left_node.borrow().count + right_node.borrow().count,
            ch: ' ',
            sym: String::from(""),
            left: Some(Rc::clone(&left_node)),
            right: Some(Rc::clone(&right_node)),
        }
    }

    fn build_haffman_code(&mut self, code: String) {
        self.sym = code.clone();

        if let Some(ref mut x) = self.left {
            x.borrow_mut().build_haffman_code(code.clone() + "1");
        }
        if let Some(ref mut x) = self.right {
            x.borrow_mut().build_haffman_code(code.clone() + "0");
        }
        if self.ch != ' ' {
            println!("{}: {}", self.ch, self.sym);
        }
    }
}


fn haffman_create() {
    let mut map = HashMap::new();

    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .expect("Failed to read line");

    let text = text.trim();
    for a in text.chars() {
        let count = map.entry(a).or_insert(0);
        *count += 1;
    }

    let mut heap = BinaryHeap::new();
    let mut map2 = HashMap::new();

    for entry in map.iter() {
        let leaf_node = Rc::new(RefCell::new(LeafNode::new(*entry.1, *entry.0)));
        heap.push(Rc::clone(&leaf_node));
        map2.insert(entry.0, Rc::clone(&leaf_node));
    }

    let mut sum = 0;
    while heap.len() > 1 {
        let right = heap.pop().unwrap();
        let left = heap.pop().unwrap();
        let internal_node = Rc::new(RefCell::new(LeafNode::internal_node(left, right)));
        sum += internal_node.borrow().count;
        heap.push(internal_node);
    }
    let root = heap.pop().unwrap();

    if map.len() == 1 {
        sum = *map.iter().next().unwrap().1;
        println!("{} {}", map.len(), sum);
        root.borrow_mut().build_haffman_code(String::from("0"));
    } else {
        println!("{} {}", map.len(), sum);
        root.borrow_mut().build_haffman_code(String::from(""));
    }

    for a in text.chars() {
        print!("{}", map2.get(&a).unwrap().borrow_mut().sym);
    }
}


fn haffman_recover() {
    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .unwrap();

    let numbers: Vec<&str> = text.trim().split(" ").collect();
    let mut letter_counts: usize = numbers[0].trim().parse().unwrap();

    let mut map = HashMap::new();
    while letter_counts > 0 {
        let mut text = String::new();
        io::stdin().read_line(&mut text)
            .unwrap();
        let numbers: Vec<&str> = text.trim().split(": ").collect();
        let letter_mask = String::from(numbers[1].trim());
        let letter = String::from(numbers[0].trim());
        map.insert(letter_mask, letter);
        letter_counts -= 1;
    }

    let mut text = String::new();
    io::stdin().read_line(&mut text)
        .unwrap();

    let mut mask = String::from("");
    for a in text.chars() {
        mask.push(a);
        if let Some(ch) = map.get(&mask) {
            print!("{}", ch);
            mask.clear();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn haffman_rec() {
        haffman_recover();
    }

    #[test]
    fn haffman_cr() {
        haffman_create();
    }
}
