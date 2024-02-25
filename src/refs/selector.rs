struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use num::pow;

impl<T> Deref for Selector<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.elements[self.current]
    }
}

impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.elements[self.current]
    }
}

#[test]
fn test_derefs() {
    let mut s = Selector {
        elements: vec!['x', 'y', 'z'],
        current: 2,
    };

    assert_eq!(*s, 'z');

    assert!(s.is_alphabetic());

    *s = 'w';

    assert_eq!(s.elements, ['x', 'y', 'w']);
}

fn show_it(thing: &str) {
    println!("{}", thing)
}

fn show_it_generic<T: Display>(thing: T) {
    println!("{}", thing)
}

#[test]
fn test_show_it() {
    let s = Selector { elements: vec!["good", "bad", "ugly"], current: 2 };
    show_it(&s);

    show_it_generic(&*s);
    show_it_generic(&s as &str)
}

#[test]
fn test_default() {
    use std::collections::HashSet;

    let squares = [4, 9, 16, 25, 36, 49, 64];
    let (powers_of_2, impure): (HashSet<i32>, HashSet<i32>) = squares.iter().partition(|&n| n & (n - 1) == 0);

    assert_eq!(powers_of_2.len(), 3);
    assert_eq!(impure.len(), 4)
}

struct City {
    name: String,
    population: i64,
    country: String,
}

fn city_population_descending(city: &City) -> i64 {
    -city.population
}

fn sort_cities(cities: &mut Vec<City>) {
    cities.sort_by_key(city_population_descending)
    //cities.sort_by_key(|city| -city.population)
}

use std::thread;

fn start_sorting_thread(mut cities: Vec<City>) {}

fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn has_montster_attacks(city: &City) -> bool {
    true
}