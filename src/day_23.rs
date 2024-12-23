pub fn part_a() -> u64 {
    for line in include_str!("inputs/input23.txt").lines() {
        let Some((computer_1, computer_2)) = line.split_once("-") else {
            panic!("Invalid line: [{}]", line)
        };

    }

    0
}
