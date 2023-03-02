// From AoC 2020 day 23
// https://adventofcode.com/2020/day/23
// 
// This is not the best solution for the puzzle which can also be solved with a linked
// list: there is no need for the list index to be known at each iteration.
//
extern crate associative_positional_list;
use associative_positional_list::AssociativePositionalList;

type Cup = usize;

struct CrabGame {
    all_cups: AssociativePositionalList<Cup>,
    number_of_cups: usize,
}

impl CrabGame {
    
    fn new(initial_state: &str, number_of_cups: usize) -> Self {
        let mut cg = Self {
            all_cups: AssociativePositionalList::new(),
            number_of_cups,
        };

        let bytes = Vec::from_iter(initial_state.bytes());

        for i in 0 .. bytes.len() {
            cg.all_cups.insert(i, (bytes.get(i).unwrap() - b'0') as usize);
        }
        for i in bytes.len() + 1 .. number_of_cups + 1 {
            cg.all_cups.insert(i, i);
        }
        cg
    }

    fn play(&mut self, number_of_rounds: usize) {
        for _ in 0 .. number_of_rounds {
            let current_cup = *self.all_cups.get(0).unwrap();

            // remove cups 1, 2, 3
            let cup1 = *self.all_cups.get(1).unwrap();
            let cup2 = *self.all_cups.get(2).unwrap();
            let cup3 = *self.all_cups.get(3).unwrap();
            self.all_cups.remove(1);
            self.all_cups.remove(1);
            self.all_cups.remove(1);

            // determine destination
            let mut destination_cup = current_cup - 1;
            while (destination_cup == cup1) || (destination_cup == cup2)
                || (destination_cup == cup3) || (destination_cup == 0) {
                if destination_cup == 0 {
                    destination_cup = self.number_of_cups + 1;
                }
                destination_cup -= 1;
            }

            // insert at destination
            let mut destination_index = self.all_cups.find(&destination_cup).unwrap();
            destination_index = (destination_index + 1) % self.number_of_cups;
            self.all_cups.insert(destination_index, cup3);
            self.all_cups.insert(destination_index, cup2);
            self.all_cups.insert(destination_index, cup1);

            // rotate so that the next cup is at index 0
            self.all_cups.remove(0);
            self.all_cups.insert(self.number_of_cups - 1, current_cup);
        }
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let current_cup = self.all_cups.get(0).unwrap();
        let mut output = String::new();
        for i in 0 .. usize::min(self.number_of_cups, 10) {
            let v = self.all_cups.get(i).unwrap();
            if v == current_cup {
                output.push_str(" (");
                output.push_str(&v.to_string());
                output.push(')');
            } else {
                output.push(' ');
                output.push_str(&v.to_string());
            }
        }
        output
    }

    fn part_1_result(&self) -> String {
        let mut iter = self.all_cups.find(&1).unwrap();
        let mut output = String::new();
        for _ in 0 .. usize::min(self.number_of_cups, 8) {
            iter = (iter + 1) % self.number_of_cups;
            output.push_str(&self.all_cups.get(iter).unwrap().to_string());
        }
        output
    }

    fn part_2_result(&self) -> u64 {
        let mut iter = self.all_cups.find(&1).unwrap();
        iter = (iter + 1) % self.number_of_cups;
        let r1 = *self.all_cups.get(iter).unwrap() as u64;
        iter = (iter + 1) % self.number_of_cups;
        let r2 = *self.all_cups.get(iter).unwrap() as u64;
        r1 * r2  // <-- that's numberwang
    }
}

const TEST_INPUT: &str = "389125467";
const MY_INPUT: &str = "158937462";
const ORIGINAL_AOC_PROBLEM: bool = false;

#[test]
fn test_part1_example() {
    let mut cg = CrabGame::new(TEST_INPUT, 9);
    cg.play(10);
    assert_eq!(cg.part_1_result(), "92658374");
    cg.play(90);
    assert_eq!(cg.part_1_result(), "67384529");
}

#[test]
fn test_part2_example() {
    let mut cg = CrabGame::new(TEST_INPUT, 1e6 as usize);
    cg.play(1e3 as usize);
    assert_eq!(cg.part_2_result(), 12);
    assert_eq!(cg.part_1_result(), "3467251014");
    if ORIGINAL_AOC_PROBLEM {
        // On my PC this requires ~30s of CPU time in release mode, and more than 7 minutes
        // in debug mode. This is not the best solution to 2020 day 23!
        cg.play((10e6 - 1e3) as usize);
        assert_eq!(cg.part_2_result(), 149245887792);
    }
}

#[test]
fn test_part1_problem() {
    let mut cg = CrabGame::new(MY_INPUT, 9);
    cg.play(100);
    assert_eq!(cg.part_1_result(), "69473825");
}

#[test]
fn test_part2_problem() {
    let mut cg = CrabGame::new(MY_INPUT, 1e6 as usize);
    cg.play(1e3 as usize);
    assert_eq!(cg.part_2_result(), 28);
    assert_eq!(cg.part_1_result(), "74632101418");
    if ORIGINAL_AOC_PROBLEM {
        // Requires a lot of CPU time (see above)
        cg.play((10e6 - 1e3) as usize);
        assert_eq!(cg.part_2_result(), 96604396189);
    }
}
