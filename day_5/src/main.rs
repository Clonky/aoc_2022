#[derive(Debug, Copy, Clone)]
struct Crate(char);

#[derive(Debug, Clone)]
struct Stack(Vec<Crate>);

#[derive(Debug, Clone)]
struct Ship(Vec<Stack>);

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
        let mut stacks: Vec<Stack> = vec![];
        dbg!(&prelude);
        dbg!(&idx_stack);
        for idx in idx_stack {
            let mut currstack = vec![];
            prelude.iter().for_each(|row| {
                if row[idx] != ' ' {
                    currstack.push(row[idx])
                }
            });
            let currstack: Vec<Crate> = currstack.iter().map(|icrate| Crate(*icrate)).collect();
            stacks.push(Stack(currstack));
        }
        Self(stacks)
    }
}

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
            .nth(3)
            .unwrap()
            .parse::<usize>()
            .expect("src not a number?");
        let dest = stream
            .nth(5)
            .unwrap()
            .parse::<usize>()
            .expect("dest not a number?");
        Self { amount, src, dest }
    }
}

fn main() {
    let content = include_str!("../input.test");
    let ship = Ship::from(content);
    dbg!(ship);
}
