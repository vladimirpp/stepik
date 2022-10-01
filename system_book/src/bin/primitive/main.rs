use std::collections::HashMap;
use std::{io, result};
use std::error::Error;


type Table = HashMap<String, Vec<String>>;
pub type Result<T> = result::Result<T, dyn Error>;


fn main() {
    let mut table = Table::new();
    // let ref mut tab = table;
    // tab.insert("Gesualdo1".to_owned(), vec!["many madrigals".to_owned(), "Tenebrae Responsoria".to_owned()]);
    table.insert("Gesualdo2".to_owned(), vec!["many madrigals".to_owned(), "Tenebrae Responsoria".to_owned()]);
    table.insert("Garavaggio".to_owned(), vec!["The Musicians".to_owned(), "The calling of St. Mathew".to_owned()]);

    println!("{:?}", &table);

    show(table);

    let r = &factorial(6);

    println!("{}", r);

    let t = "43";
    let mut s;
    let mut u;
    {
        let q = "44";
        s = S { r: &t, f: &q };
        u = s.r;
    }
    println!("{:?}", u);


    let q = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];

    let mut i = 0;
    while i < 10 {
        println!("{}", q[i]);
        i += 1;
    }

    let q = if let Some(x) = Some(4) {
        4
    } else {
        5
    };

    println!("{}", q);
}

fn show(ref mut table: Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        works.pop();
        for work in works {
            println!(" {}", work);
        }
    }
}

fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[derive(Debug)]
struct S<'a, 'b, T> {
    r: &'a T,
    f: &'b T,
}



