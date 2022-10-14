use std::collections::HashMap;

pub type Table = HashMap<String, Vec<String>>;

pub fn showing(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

pub fn sort_works(table: &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

pub fn factorial(n: usize) -> usize {
    (1..n+1).product()
}

