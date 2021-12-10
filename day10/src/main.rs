fn main() {
    let nav = include_str!("../nav.txt");
    let autocomplete_iter = nav.lines().map(|line| autocomplete(line));

    let corrupt_score: u64 = autocomplete_iter
        .clone()
        .filter_map(|a| a.err())
        .map(corrupt_score)
        .sum();

    let mut autocomplete_scores: Vec<u64> = autocomplete_iter
        .filter_map(|a| a.ok())
        .map(|a| autocomplete_score(&a))
        .collect();

    autocomplete_scores.sort();

    let median = autocomplete_scores[autocomplete_scores.len() / 2];

    println!("Corrupt Score: {}", corrupt_score);
    println!(
        "Autocomplete Score: {:?}, median: {}",
        autocomplete_scores, median
    );
}

fn corrupt_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("What"),
    }
}

fn autocomplete_score(auto_line: &str) -> u64 {
    auto_line
        .chars()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            c => panic!("Wrong char {}", c),
        })
        .fold(0, |score, p| score * 5 + p)
}

/// Autocomplete a line, or return a char in case of a syntax error
fn autocomplete(line: &str) -> Result<String, char> {
    let mut openers = Vec::new();

    let corrupt = line.chars().find(|cur| {
        if cur.is_open() {
            openers.push(*cur);
            false
        } else if cur.is_close() {
            // good closer, no corruption
            if cur.closes(&openers.pop().unwrap_or(';')) {
                false
            } else {
                true
            }
        } else {
            panic!("very corrupt!: '{}'", cur);
        }
    });

    if let Some(corrupt) = corrupt {
        Err(corrupt)
    } else {
        // Create closing sequence from openers that weren't closed
        Ok(openers.iter().rev().filter_map(|c| c.closing()).collect())
    }
}

trait Chunk {
    fn is_open(&self) -> bool;

    fn closes(&self, open: &char) -> bool;

    /// Get the closing character
    fn closing(&self) -> Option<char>;

    fn is_close(&self) -> bool;
}

impl Chunk for char {
    fn is_open(&self) -> bool {
        match self {
            '{' | '(' | '[' | '<' => true,
            _ => false,
        }
    }

    fn closes(&self, other: &char) -> bool {
        match (other, self) {
            ('{', '}') => true,
            ('(', ')') => true,
            ('[', ']') => true,
            ('<', '>') => true,
            _ => false,
        }
    }

    fn is_close(&self) -> bool {
        match self {
            ']' | ')' | '}' | '>' => true,
            _ => false,
        }
    }

    fn closing(&self) -> Option<char> {
        match self {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '<' => Some('>'),
            _ => None,
        }
    }
}
