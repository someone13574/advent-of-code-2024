use std::collections::VecDeque;

fn main() {
    let input = include_str!("../data/input.txt").trim();
    let input = if input.len() % 2 == 0 {
        &input[..input.len() - 1]
    } else {
        input
    };

    let (free_spaces, file_blocks) = get_blocks(input);
    println!(
        "Part 1: {}",
        fragmented_checksum(free_spaces.clone(), file_blocks.clone())
    );
    println!(
        "Part 2: {}",
        unfragmented_checksum(free_spaces, file_blocks)
    );
}

fn get_blocks(input: &str) -> (VecDeque<(usize, usize)>, Vec<(usize, usize, usize)>) {
    let (file_blocks, empty_blocks): (Vec<_>, Vec<_>) = input
        .char_indices()
        .scan(0, |offset, (idx, length)| {
            let length = length.to_digit(10).unwrap() as usize;
            let this_offset = *offset;
            *offset += length;
            Some((idx, this_offset, length))
        })
        .filter(|(_, _, length)| *length != 0)
        .partition(|(idx, _, _)| idx % 2 == 0);
    let empty_blocks = empty_blocks
        .into_iter()
        .map(|(_, offset, length)| (offset, length))
        .collect();
    let file_blocks = file_blocks
        .into_iter()
        .map(|(idx, offset, length)| (idx / 2, offset, length))
        .collect();
    (empty_blocks, file_blocks)
}

fn fragmented_checksum(
    mut free_space: VecDeque<(usize, usize)>,
    mut file_blocks: Vec<(usize, usize, usize)>,
) -> usize {
    let mut checksum = 0;

    while !free_space.is_empty() {
        let (file_idx, file_offset, file_length) = file_blocks.last_mut().unwrap();
        let (free_space_offset, free_space_length) = free_space.front_mut().unwrap();

        if *file_offset + *file_length - 1 < *free_space_offset {
            break;
        }

        checksum += *free_space_offset * *file_idx;

        *file_length -= 1;
        *free_space_length -= 1;
        *free_space_offset += 1;

        if *free_space_length == 0 {
            free_space.pop_front().unwrap();
        }
        if *file_length == 0 {
            file_blocks.pop().unwrap();
        }
    }

    for (file_idx, offset, length) in file_blocks {
        for i in offset..offset + length {
            checksum += file_idx * i;
        }
    }

    checksum
}

fn unfragmented_checksum(
    mut free_spaces: VecDeque<(usize, usize)>,
    mut file_blocks: Vec<(usize, usize, usize)>,
) -> usize {
    let mut checksum = 0;

    while let Some((file_idx, file_offset, file_length)) = file_blocks.pop() {
        let mut new_offset = file_offset;

        for (free_space_offset, free_space_length) in &mut free_spaces {
            if *free_space_length < file_length || *free_space_offset > file_offset {
                continue;
            }

            new_offset = *free_space_offset;

            *free_space_offset += file_length;
            *free_space_length -= file_length;

            break;
        }

        for i in new_offset..new_offset + file_length {
            checksum += i * file_idx;
        }
    }

    checksum
}
