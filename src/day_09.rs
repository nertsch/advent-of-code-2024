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

    let mut file_start_search_position = disk_contents.len();
    'main_loop: loop {
        let Some(mut file_end) = disk_contents[0..file_start_search_position].iter().rposition(|&i| i.is_some()).and_then(|i| Some(i+1)) else {break;};

        let file_id = disk_contents[file_end-1].unwrap();
        let Some(mut file_start) = disk_contents[0..file_end].iter().rposition(|&i| i != Some(file_id)).and_then(|i| Some(i+1)) else {break;};

        let file_range = file_start..file_end;

        let mut block_search_start_position = 0usize;
        loop {
            let free_block_start;
            match disk_contents[block_search_start_position..].iter().position(|&i| i.is_none()){
                Some(i) if block_search_start_position + i > file_range.start => break,
                Some(i) => free_block_start = block_search_start_position + i,
                None => break,
            }
            let Some(free_block_length) = disk_contents[free_block_start..].iter().position(|i| i.is_some()) else { break; };

            if free_block_length >= file_range.len() {
                let (left, right) = disk_contents.split_at_mut(file_range.start);
                left[free_block_start..free_block_start+file_range.len()].swap_with_slice(&mut right[0..file_range.len()]);
                break;
            }
            block_search_start_position = free_block_start + free_block_length;
        }
        file_start_search_position = file_start;
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