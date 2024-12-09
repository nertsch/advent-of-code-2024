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

pub fn part_b() -> u64 {
    let mut disk_contents = read_disk_contents();

    let mut tail = disk_contents.len()-1;
    'main_loop: loop {
        while disk_contents[tail].is_none() {tail-=1;}
        let file_end = tail;
        let file_id = disk_contents[tail].unwrap();
        while disk_contents[tail] == Some(file_id) {
            if tail == 0 {break 'main_loop;}
            tail-=1;
        }
        let file_range = tail+1..file_end+1;

        let mut free_block_start = 0usize;
        loop {
            while disk_contents[free_block_start].is_some() {free_block_start+=1;}
            if(free_block_start > file_range.start) {break}
            let mut free_block_end = free_block_start;
            while disk_contents[free_block_end].is_none() {free_block_end+=1;}
            if free_block_end-free_block_start >= file_range.len() {
                let (left, right) = disk_contents.split_at_mut(file_range.start);
                left[free_block_start..free_block_start+file_range.len()].swap_with_slice(&mut right[0..file_range.len()]);
                break;
            }
            free_block_start = free_block_end;
        }
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