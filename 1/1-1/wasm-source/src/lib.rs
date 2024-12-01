use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> i32
{
    let parsed: Vec<(i32, i32)> = String::from_utf8(file).unwrap()
        .lines()
        .filter_map(|line| {
            let mut columns = line.split("   ");
            let col1 = columns.next()?.parse::<i32>().ok();
            let col2 = columns.next()?.parse::<i32>().ok();
            Some((col1?, col2?))
        })
        .collect();
    let mut left: Vec<i32> = parsed.iter().map(|(a, _)| *a).collect();
    let mut right: Vec<i32> = parsed.iter().map(|(_, b)| *b).collect();
    left.sort(); right.sort();
    let distance: i32 = left.iter()
                          .zip(right.iter())
                          .map(|(a, b)| (a - b).abs())
                          .sum();
    return distance;

}