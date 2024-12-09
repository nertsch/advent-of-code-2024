use std::iter;
use std::ops::Range;

pub fn part_a() -> u64 {
    let input = include_str!("inputs/input09.txt");
    let mut disk_contents: Vec<Option<u64>> = Vec::new();

    for (i, space_size) in input.chars().enumerate()  {
        let i = i as u64;
        let space_size = space_size.to_digit(10).expect("input must just contain numbers") as usize;
        disk_contents.extend(iter::repeat(if i%2 == 0 {Some(i/2)} else { None }).take(space_size))
    }

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

pub fn part_b() -> u64 {
    let input = include_str!("inputs/input09.txt");

    let mut file_list: Vec<(Range<usize>, u64)> = Vec::new();
    let mut free_list: Vec<Range<usize>> = Vec::new();
    let mut current_index = 0;

    for (i, space_size) in input.chars().enumerate() {
        let space_size = space_size.to_digit(10).expect("input must just contain numbers") as usize;
        let space_range = current_index..current_index + space_size;
        current_index = space_range.end;
        if (i % 2 == 0) {
            file_list.push((space_range, i as u64 / 2));
        } else {
            free_list.push(space_range);
        }
    }

    for file in file_list.iter_mut().rev() {
        for free in free_list.iter_mut().filter(|f| f.end <= file.0.start) {
            if (free.len() >= file.0.len()) {
                let free_start = free.start;
                *free = free_start + file.0.len()..free.end;
                *file = (free_start..free_start + file.0.len(), file.1);
                break;
            }
        }
    }

    file_list.into_iter().map(|f| f.0.sum::<usize>() as u64 * f.1).sum()
}
