pub fn part_a() -> u32 {
    let my_str = include_str!("inputs/input01.txt");

    let mut list1 = vec![];
    let mut list2 = vec![];

    for line in my_str.lines() {
        if let [ns1, ns2] = line.split_whitespace().collect::<Vec<_>>()[..] {
            if let [Ok(n1), Ok(n2)] = [ns1.parse::<i32>(), ns2.parse::<i32>()] {
                list1.insert(list1.binary_search(&n1).unwrap_or_else(|i| i), n1);
                list2.insert(list2.binary_search(&n2).unwrap_or_else(|i| i), n2);
            } else {
                panic!("Row elements must be numbers")
            }
        } else {
            panic!("Row must contain 2 Elements")
        }
    }

    list1
        .into_iter()
        .zip(list2.into_iter())
        .map(|r| r.0.abs_diff(r.1))
        .sum()
}
