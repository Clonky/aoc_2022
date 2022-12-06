use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};

#[derive(Debug, Clone)]
struct CleaningPair(Schedule, Schedule);

#[derive(Debug, Copy, Clone)]
struct Schedule {
    start: u32,
    stop: u32,
}

impl From<Schedule> for RangeInclusive<u32> {
    fn from(schedule: Schedule) -> Self {
        RangeInclusive::new(schedule.start, schedule.stop)
    }
}

impl Schedule {
    fn into_hashset(self) -> HashSet<u32> {
        RangeInclusive::from(self).collect::<HashSet<u32>>()
    }
}

impl From<&str> for CleaningPair {
    fn from(line: &str) -> Self {
        let schedules: Vec<Schedule> = line
            .split(',')
            .map(|schedule| {
                let start_and_stop = schedule
                    .split('-')
                    .map(|startstop| startstop.parse::<u32>().expect("Couldn't parse number"))
                    .collect::<Vec<u32>>();
                Schedule {
                    start: start_and_stop[0],
                    stop: start_and_stop[1],
                }
            })
            .collect();
        Self(schedules[0], schedules[1])
    }
}

impl CleaningPair {
    fn is_complete_overlap(&self) -> bool {
        let first_elf: HashSet<u32> = self.0.into_hashset();
        let second_elf: HashSet<u32> = self.1.into_hashset();
        println!(
            "{}..{} zu {}..{}",
            self.0.start, self.0.stop, self.1.start, self.1.stop
        );
        let is_overlap = first_elf.intersection(&second_elf).count() > 0;
        println!("Is overlap: {is_overlap:?}");
        is_overlap
    }
}

fn main() {
    let lines = include_str!("../input.txt").lines();
    let schedules = lines
        .map(CleaningPair::from)
        .map(|item| item.is_complete_overlap())
        .filter(|item| *item)
        .count();
    println!("{schedules:?}");
}
