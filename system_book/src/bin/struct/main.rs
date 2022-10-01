use std::ops::Add;
use crate::BinaryTree::{Empty, NonEmpty};

fn main() {
    struct Queue<T> {
        older: Vec<T>,
        younger: Vec<T>,
    }

    impl<T> Queue<T> {
        fn new() -> Self {
            Self {
                older: Vec::new(),
                younger: Vec::new(),
            }
        }
        pub fn push(&mut self, c: T) {
            self.younger.push(c);
        }


        pub fn pop(&mut self) -> Option<T> {
            if self.older.is_empty() {
                if self.younger.is_empty() {
                    return None;
                }

                use std::mem::swap;
                swap(&mut self.older, &mut self.younger);
                println!("{}", self.younger.capacity());
                self.older.reverse();
            }
            self.older.pop()
        }
    }

    let mut queue = Queue::<char>::new();

    queue.push('c');
    queue.push('s');
    queue.push('d');


    println!("{:?}", queue.pop());

    println!("{}", TimeUnit::Days > TimeUnit::Years);


    let mut tree = BinaryTree::Empty;
    tree.add("mercury");
    tree.add("mars");
    tree.add("earth");

    println!("{:?}", tree);

    let result = rough_time_to_english(RoughTime::InTheFuture(TimeUnit::Seconds, 3));


    println!("{}", result);
}


struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

#[derive(PartialOrd, PartialEq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "Hello",
            _ => "World"
        }
    }
}

#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

enum RoughTime {
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InTheFuture(TimeUnit::Seconds, 2) => "hello".to_owned(),
        RoughTime::InTheFuture(units, count) =>
            format!("{} {} ago", count, units.plural()),
        _ => "Bue".to_owned(),
    }
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            Empty => {
                *self = NonEmpty(Box::new(TreeNode {
                    value,
                    left: Empty,
                    right: Empty,
                }))
            }
            NonEmpty(ref mut node) => {
                if value <= node.value {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
            }
        }
    }
}

struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Add for Complex<T>
    where T: Add<Output=T> {
    type Output = ();

    fn add(self, rhs: Self) -> Self::Output {
        ()
    }
}

