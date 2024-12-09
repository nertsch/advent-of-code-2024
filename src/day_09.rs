use std::iter;
use std::ops::Range;

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

    let mut prev_block_contents = disk_contents[0];
    let mut last_delta_index = 0;
    let mut file_list: Vec<Range<usize>> = Vec::new();
    let mut free_list: Vec<Range<usize>> = Vec::new();

    for (index,block_contents) in disk_contents.iter().enumerate().skip(1){
        match (prev_block_contents, block_contents ){
            (None, Some(id)) => { free_list.push(last_delta_index..index); last_delta_index = index ; prev_block_contents = *block_contents;  }
            (Some(_), None) => { file_list.push(last_delta_index..index); last_delta_index = index; prev_block_contents = *block_contents; }
            (Some(a), Some(b)) if a != *b => { file_list.push(last_delta_index..index); last_delta_index = index; prev_block_contents = *block_contents; },
            _ => {}
        }
    }

    match(prev_block_contents){
        (Some(_)) => file_list.push(last_delta_index..disk_contents.len()),
        None => free_list.push(last_delta_index..disk_contents.len()),
    }


    for file in file_list.iter().rev()  {
        for free in free_list.iter_mut().filter(|f| f.end <= file.start){
            if(free.len() >= file.len()) {
                let (left, right) = disk_contents.split_at_mut(file.start);
                left[free.start..free.start+file.len()].swap_with_slice(&mut right[0..file.len()]);
                *free = free.start+file.len()..free.end;
                break;
            }
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