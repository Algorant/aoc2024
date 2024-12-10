use std::fs::read_to_string;

fn parse_input(contents: &str) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();
    let mut is_file = true;
    let mut id = 0;

    // Parse through input
    for c in contents.chars() {
        if let Some(count) = c.to_digit(10) {
            let count = count as usize;
            if is_file {
                // Add file blocks
                for _ in 0..count {
                    blocks.push(Some(id));
                }
                id += 1;
            } else {
                // Add free spaces
                for _ in 0..count {
                    blocks.push(None);
                }
            }
            is_file = !is_file; //This toggles between true and false for is_file, good way to alternate between file and emtpy space
        }
    }
    blocks
}

fn compact(blocks: &mut Vec<Option<usize>>) {
    let (mut left, mut right) = (0, blocks.len() - 1);
    while left < right {
        // Find next empty space from left
        while left < blocks.len() && blocks[left].is_some() {
            left += 1;
        }
        // Find next file from right
        while right > 0 && blocks[right].is_none() {
            right -= 1;
        }
        // Swap if we found a valid pair
        if left < right {
            blocks.swap(left, right);
        }
        left += 1;
        right -= 1;
    }
}

fn defragment(blocks: &mut Vec<Option<usize>>) {
    // Find highest ID
    let mut max_id = 0;
    for block in blocks.iter() {
        if let Some(id) = block {
            max_id = max_id.max(*id);
        }
    }

    let mut current_id = max_id;
    while current_id > 0 {
        let mut right = blocks.len() - 1;

        // Find rightmost group of current ID
        while right > 0 {
            // Find rightmost occurrence of current ID
            while right > 0 && blocks[right] != Some(current_id) {
                right -= 1;
            }
            if blocks[right] != Some(current_id) {
                break;
            }

            // Count size of this group
            let mut group_size = 0;
            let mut n = right;
            while n > 0 && blocks[n] == Some(current_id) {
                group_size += 1;
                if n == 0 {
                    break;
                }
                n -= 1;
            }
            n += 1; // Adjust to start of group

            // Find leftmost empty space that can fit this group
            let mut space_start = 0;
            while space_start < n {
                // Skip non-empty spaces
                while space_start < n && blocks[space_start].is_some() {
                    space_start += 1;
                }
                if space_start >= n {
                    break;
                }

                // Count consecutive empty spaces
                let mut empty_size = 0;
                let mut space_pos = space_start;
                while space_pos < n && blocks[space_pos].is_none() {
                    empty_size += 1;
                    space_pos += 1;
                }

                // If we found enough space, move the group
                if empty_size >= group_size {
                    for i in 0..group_size {
                        blocks.swap(space_start + i, n + i);
                    }
                    break;
                }

                space_start = space_pos;
            }

            right = if n > 0 { n - 1 } else { 0 };
        }

        current_id -= 1;
    }
}

fn calculate_checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, &id)| id.map(|v| pos * v))
        .sum()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_to_string("input.txt").expect("Failed to read input.txt");
    //let example = "2333133121414131402";

    // Parse and create blocks
    let mut blocks = parse_input(&input);
    println!("Initial blocks: {:?}", &blocks[..blocks.len().min(50)]);

    // Compact the blocks
    compact(&mut blocks);
    println!("Compacted blocks: {:?}", &blocks[..blocks.len().min(50)]);

    // Calculate part 1 checksum after compact
    let p1_checksum = calculate_checksum(&blocks);
    println!("Part 1 Checksum: {}", p1_checksum);

    // Reset blocks and parse input again for part 2
    blocks = parse_input(&input);

    // Defrag the blocks
    defragment(&mut blocks);
    println!("Defragmented blocks: {:?}", &blocks[..blocks.len().min(50)]);

    // Calculate part 2 checksum
    let p2_checksum = calculate_checksum(&blocks);
    println!("Part 2 Checksum: {}", p2_checksum);

    Ok(())
}
