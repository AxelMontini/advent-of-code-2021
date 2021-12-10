fn main() {
    let caves = include_str!("../caves.txt");
    // width of a line
    let width = caves.split_whitespace().next().unwrap().chars().count();
    println!("Width: {}", width);
    let caves: Vec<u8> = caves.chars().map(|c| c.to_digit(10).unwrap() as u8).collect();
    let sum = low_points_sum(&caves, width);
    println!("Sum of risk of low points: {}", sum);
}

fn low_points_sum(caves: &[u8], width: usize) -> u64 {
    caves.iter().enumerate().filter(|(idx, cell)| {
        let top = idx.checked_sub(width).and_then(|i| caves.get(i));
        let bottom = idx.checked_add(width).and_then(|i| caves.get(i));
        let right = idx.checked_sub(width).and_then(|i| caves.get(i));
    }).map(|(_i, v)| v).sum()
}
