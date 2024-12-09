use std::iter;
pub fn part_a() -> u64 {
    let mut disk_contents = read_disk_contents();

    let mut head = 0usize;
    let mut tail = disk_contents.len()-1;

    loop {
        while disk_contents[head].is_some() {head+=1;}
        while disk_contents[tail].is_none() {tail-=1;}
        if head >= tail {break;}
        disk_contents.swap(head, tail);
    }

    disk_contents.iter().enumerate().map(|(i,id)| (i as u64)*id.unwrap_or(0)).sum()
}

fn read_disk_contents() -> Vec<Option<u64>>{
    let input = include_str!("inputs/input09.txt");
    let mut blocks: Vec<Option<u64>> = Vec::new();

    for (i, space_size) in input.chars().enumerate()  {
        let i = i as u64;
        let space_size = space_size.to_digit(10).expect("input must just contain numbers") as usize;
        blocks.extend(iter::repeat(if i%2 == 0 {Some(i/2)} else { None }).take(space_size))
    }

    blocks
}