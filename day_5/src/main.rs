use std::{fmt::Display, vec};

#[derive(Debug, Copy, Clone)]
struct Crate(char);

impl Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
struct Ship(Vec<Vec<Crate>>);

impl From<&str> for Ship {
    fn from(input: &str) -> Self {
        let prelude: Vec<Vec<char>> = input
            .lines()
            .take_while(|x| !x.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        let idx_stack = prelude
            .last()
            .unwrap()
            .iter()
            .enumerate()
            .filter(|(_idx, val)| matches!(val, '1'..='9'))
            .map(|(idx, _val)| idx)
            .collect::<Vec<usize>>();
        let mut stacks: Vec<Vec<Crate>> = vec![];
        for idx in idx_stack {
            let mut currstack = vec![];
            prelude.iter().for_each(|row| {
                if row[idx] != ' ' {
                    currstack.push(row[idx])
                }
            });
            let mut currstack: Vec<Crate> = currstack.iter().map(|icrate| Crate(*icrate)).collect();
            currstack.reverse();
            stacks.push(currstack);
        }
        Self(stacks)
    }
}

impl Ship {
    fn execute_move(&mut self, mv: Move) {
        for _ in 0..mv.amount {
            let moved_crate = self.0[mv.src - 1].pop().unwrap();
            self.0[mv.dest - 1].push(moved_crate);
        }
    }

    fn execute_move_multiples(&mut self, mv: Move) {
        let mut buffer = vec![];
        for _ in 0..mv.amount {
            buffer.push(self.0[mv.src - 1].pop().unwrap());
        }
        buffer.reverse();
        self.0[mv.dest - 1].append(&mut buffer);
    }

    fn get_top_crates(&self) -> String {
        let top_crates: Vec<Option<Crate>> =
            self.0.iter().map(|istack| istack.last().copied()).collect();
        format!("{:?}", top_crates)
    }
}

#[derive(Debug, Copy, Clone)]
struct Move {
    amount: u8,
    src: usize,
    dest: usize,
}

impl From<&str> for Move {
    fn from(input: &str) -> Self {
        let mut stream = input.split_whitespace();
        let amount = stream
            .nth(1)
            .unwrap()
            .parse::<u8>()
            .expect("Amount not a number?");
        let src = stream
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .expect("src not a number?");
        let dest = stream
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .expect("dest not a number?");
        Self { amount, src, dest }
    }
}

fn main() {
    let content = include_str!("../input.prod");
    let mut ship = Ship::from(content);
    let moves: Vec<Move> = content
        .lines()
        .filter(|iline| iline.starts_with("move"))
        .map(Move::from)
        .collect();
    moves
        .iter()
        .for_each(|imove| ship.execute_move_multiples(*imove));
    println!("{}", ship.get_top_crates());
}
