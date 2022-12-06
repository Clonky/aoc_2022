use std::collections::HashSet;

fn get_marker(n: usize, signal: &[char]) -> usize {
    let mut is_start = signal.windows(n).map(|window| {
        let set_length = window.iter().copied().collect::<HashSet<char>>().len();
        set_length == n
    });
    is_start
        .position(|item| item)
        .expect("No start signature found.")
}

fn main() {
    let content = include_str!("../input.txt");
    let chars = content.chars().collect::<Vec<char>>();
    let packet_marker = get_marker(4, &chars);
    println!(
        "Packet starts at {} and ends at {}",
        packet_marker,
        packet_marker + 4
    );
    let message_marker = get_marker(14, &chars);
    println!(
        "Message starts at {} and ends at {}",
        message_marker,
        message_marker + 14
    );
}
