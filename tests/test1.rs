// From AoC 2022 day 20
// https://adventofcode.com/2022/day/20
//
use std::fs::File;
use std::io::{self, BufRead};

extern crate associative_positional_list;
use associative_positional_list::AssociativePositionalList;

const DEBUG: bool = false;

struct Problem {
    orders: AssociativePositionalList<usize>,
    order_to_value: Vec<isize>,
    size: usize,
}

fn load(filename: &str) -> Problem {
    let file = File::open(filename).unwrap();
    let mut p = Problem {
        orders: AssociativePositionalList::new(),
        order_to_value: Vec::new(),
        size: 0,
    };
    for line in io::BufReader::new(file).lines() {
        if let Ok(line_string) = line {
            let value: isize = line_string.parse().expect("n");
            let order: usize = p.order_to_value.len();
            p.orders.insert(order, order);
            p.order_to_value.push(value);
        }
    }
    p.size = p.order_to_value.len();
    return p;
}

fn mix(p: &mut Problem) {
    for order in 0 .. p.size {
        // Find the index of the number to be moved
        let old_index: usize = p.orders.find(&order).unwrap();
        if DEBUG {
            assert!(old_index < p.size);
            assert_eq!(*p.orders.get(old_index).unwrap(), order);
        }

        // Remove
        p.orders.remove(old_index);

        // Value of number
        let value: isize = *p.order_to_value.get(order).unwrap();

        // What's the new position?
        let mut pos: isize = (value + (old_index as isize)) % ((p.size as isize) - 1);

        // Insert in new position
        if pos < 0 {
            pos += (p.size as isize) - 1;
        }
        if DEBUG {
            println!("order {} old index {} value {} pos {}", order, old_index, value, pos);
        }
        p.orders.insert(pos as usize, order);

        if DEBUG {
            assert_eq!(*p.orders.get(pos as usize).unwrap(), order);
            for index in 0 .. p.size {
                let order: usize = *p.orders.get(index).unwrap();
                let value: isize = *p.order_to_value.get(order).unwrap();
                print!("{} ", value);
            }
            println!();
        }
    }
}

fn get_coords(p: &Problem) -> isize {
    // Find zero
    let mut zero_index: usize = usize::MAX;
    for order in 0 .. p.size {
        let value: isize = *p.order_to_value.get(order).unwrap();
        if value == 0 {
            zero_index = p.orders.find(&order).unwrap();
            break;
        }
    }
    assert!(zero_index < p.size);
    let a = *p.order_to_value.get(*p.orders.get((zero_index + 1000) % p.size).unwrap()).unwrap();
    let b = *p.order_to_value.get(*p.orders.get((zero_index + 2000) % p.size).unwrap()).unwrap();
    let c = *p.order_to_value.get(*p.orders.get((zero_index + 3000) % p.size).unwrap()).unwrap();
    return a + b + c;
}

fn part1(filename: &str) -> isize {
    let mut p = load(filename);
    mix(&mut p);
    return get_coords(&p);
}

fn part2(filename: &str) -> isize {
    let mut p = load(filename);
    for order in 0 .. p.size {
        *p.order_to_value.get_mut(order).unwrap() *= 811589153;
    }
    for _ in 0 .. 10 {
        mix(&mut p);
    }
    return get_coords(&p);
}

#[test]
fn test_part1_example() {
    assert_eq!(part1(&"tests/test1.example"), 3);
}

#[test]
fn test_part1_problem() {
    assert_eq!(part1(&"tests/test1.problem"), 2215);
}

#[test]
fn test_part2_example() {
    assert_eq!(part2(&"tests/test1.example"), 1623178306);
}

#[test]
fn test_part2_problem() {
    assert_eq!(part2(&"tests/test1.problem"), 8927480683);
}

