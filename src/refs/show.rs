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
    for works in table.values_mut() {
        works.sort();
    }
}

pub fn factorial(n: usize) -> usize {
    (1..=n).product()
}

#[test]
fn test_factorial(){
    assert_eq!(factorial(5), 120)
}
