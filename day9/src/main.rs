fn main() {
    let caves = include_str!("../caves.txt");
    // width of a line
    let width = caves.split_whitespace().next().unwrap().chars().count();
    println!("Width: {}", width);
    let caves: Vec<u8> = caves
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect();
    let sum = low_points_sum(&caves, width);
    println!("Sum of risk of low points: {}", sum);

    let mut basins = basins(&caves, width);
    basins.sort();
    println!("Basins: {:?}", basins);
    println!(
        "Biggest three multiplied: {}",
        basins.into_iter().rev().take(3).product::<usize>()
    )
}

fn low_points_sum(caves: &[u8], width: usize) -> u64 {
    caves
        .iter()
        .enumerate()
        .filter(move |&(idx, &cell)| {
            let col = idx % width;

            let top = idx.checked_sub(width).and_then(|i| caves.get(i)).copied();
            let bottom = idx.checked_add(width).and_then(|i| caves.get(i)).copied();

            let left = (col > 0).then(|| caves.get(idx - 1)).flatten().copied();
            let right = (col + 1 < width)
                .then(|| caves.get(idx + 1))
                .flatten()
                .copied();

            // figure out whether it's lower than all of these.
            cell < top.unwrap_or(u8::MAX)
                && cell < bottom.unwrap_or(u8::MAX)
                && cell < left.unwrap_or(u8::MAX)
                && cell < right.unwrap_or(u8::MAX)
        })
        .map(|(_i, v)| (*v + 1) as u64) // risk factor is height + 1
        .sum()
}

fn basins(caves: &[u8], width: usize) -> Vec<usize> {
    // use the caves array as a graph. Then perform DFS on points until the entire graph has been discovered.
    // Each node (cell != 9) has edges top, bottom, left and right to close nodes (if they're 9, then there's no node to be connected to).
    //

    let mut sums = Vec::new();
    let mut discovered = vec![false; caves.len()];
    let mut stack = Vec::with_capacity(30); // hold the DFS stack
    let mut start_idx = 0; // start at this index

    // choose a random node that hasn't been discovered yet and DFS
    // the entire cave

    // start DFS
    while start_idx < caves.len() {
        if discovered[start_idx] || caves[start_idx] == 9 {
            start_idx += 1;
            continue;
        }

        sums.push(0);
        let sum = sums.last_mut().unwrap();

        // start DFS at start_idx
        stack.push(start_idx);

        while let Some(node) = stack.pop() {
            if !discovered[node] {
                discovered[node] = true;
                *sum += 1; // increase current basin sum

                let col = node % width;

                let adjacent = [
                    node.checked_sub(width),
                    node.checked_add(width).filter(|&i| i < caves.len()),
                    (col > 0).then(|| node - 1),
                    (col + 1 < width).then(|| node + 1),
                ];

                adjacent.iter().for_each(|maybe_adj| {
                    if let Some(adj) = maybe_adj {
                        if !discovered[*adj] && caves[*adj] != 9 {
                            stack.push(*adj);
                        }
                    }
                })
            }
        }
    }

    sums
}
