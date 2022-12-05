use std::fs;
use std::iter::zip;

#[derive(Debug, Copy, Clone)]
enum Signs {
    Rock,
    Paper,
    Scissors,
}

impl Signs {
    fn new(sign: &str) -> Self {
        Self::convert_sign(sign)
    }
    fn convert_sign(sign: &str) -> Self {
        match sign {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "Y" => Self::Paper,
            "X" => Self::Rock,
            "Z" => Self::Scissors,
            _ => panic!(),
        }
    }
    fn sign_val(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum RoundResult {
    Win,
    Loss,
    Draw,
}

impl RoundResult {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Loss => 0,
            Self::Draw => 3,
        }
    }
}

struct Round {
    choices: Vec<Signs>,
}

impl From<&str> for Round {
    fn from(input: &str) -> Self {
        let choices: Vec<Signs> = input.split_whitespace().map(Signs::new).collect();
        Self { choices }
    }
}

impl Round {
    fn score_round(&self) -> i32 {
        self.score_sign_vals() + self.score_round_result()
    }

    fn score_sign_vals(&self) -> i32 {
        self.choices[1].sign_val()
    }

    fn score_round_result(&self) -> i32 {
        let own = self.choices[1];
        let enemy = self.choices[0];
        match (own, enemy) {
            (Signs::Rock, Signs::Rock) => RoundResult::Draw,
            (Signs::Rock, Signs::Paper) => RoundResult::Loss,
            (Signs::Rock, Signs::Scissors) => RoundResult::Win,
            (Signs::Paper, Signs::Paper) => RoundResult::Draw,
            (Signs::Paper, Signs::Scissors) => RoundResult::Loss,
            (Signs::Paper, Signs::Rock) => RoundResult::Win,
            (Signs::Scissors, Signs::Scissors) => RoundResult::Draw,
            (Signs::Scissors, Signs::Rock) => RoundResult::Loss,
            (Signs::Scissors, Signs::Paper) => RoundResult::Win,
        }
        .score()
    }
}

fn first_pass(rounds: &[Round]) -> i32 {
    let score = rounds.iter().fold(0, |mut accum, iround| {
        accum += iround.score_round();
        accum
    });
    score
}

fn second_pass(rounds: &[Round]) -> i32 {
    let forecasted_results: Vec<RoundResult> = rounds
        .iter()
        .map(|iround| match iround.choices[1] {
            Signs::Rock => RoundResult::Loss,
            Signs::Paper => RoundResult::Draw,
            Signs::Scissors => RoundResult::Win,
        })
        .collect();
    let enemy_choices: Vec<Signs> = rounds.iter().map(|iround| iround.choices[0]).collect();
    let combination = zip(&enemy_choices, forecasted_results);
    let new_choices = combination.map(|item| match item {
        (Signs::Paper, RoundResult::Draw) => Signs::Paper,
        (Signs::Scissors, RoundResult::Draw) => Signs::Scissors,
        (Signs::Rock, RoundResult::Draw) => Signs::Rock,
        (Signs::Paper, RoundResult::Win) => Signs::Scissors,
        (Signs::Scissors, RoundResult::Win) => Signs::Rock,
        (Signs::Rock, RoundResult::Win) => Signs::Paper,
        (Signs::Paper, RoundResult::Loss) => Signs::Rock,
        (Signs::Scissors, RoundResult::Loss) => Signs::Paper,
        (Signs::Rock, RoundResult::Loss) => Signs::Scissors,
    });
    let enemy_choices: Vec<Signs> = rounds.iter().map(|iround| iround.choices[0]).collect();
    zip(enemy_choices, new_choices)
        .map(|(enemy_choice, own_choice)| Round {
            choices: vec![enemy_choice, own_choice],
        })
        .fold(0, |mut accum, iround| {
            accum += iround.score_round();
            accum
        })
}

fn main() {
    const SRC: &str = "input.txt";
    let contents = fs::read_to_string(SRC).expect("Couldn't read file.");
    let lines = contents.lines();
    let rounds: Vec<Round> = lines.map(Round::from).collect();
    let score_first_pass = first_pass(&rounds);
    println!(
        "Your total score in the first pass is: {:?}",
        score_first_pass
    );
    let score_second_pass = second_pass(&rounds);
    println!(
        "Your total score in the second pass is: {:?}",
        score_second_pass
    );
}
