use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let parsed: Vec<(u32, u32)> = String::from_utf8(file).unwrap()
        .lines()
        .filter_map(|line| {
            let mut columns = line.split("   ");
            let col1 = columns.next()?.parse::<u32>().ok();
            let col2 = columns.next()?.parse::<u32>().ok();
            Some((col1?, col2?))
        })
        .collect();
    let left: Vec<u32> = parsed.iter().map(|(a, _)| *a).collect();
    let right: Vec<u32> = parsed.iter().map(|(_, b)| *b).collect();
    let similarity: u32 = left.iter()
                          .map(|&a| (right.iter().filter(|&&b| b == a).count() as u32) * a)
                          .sum();
    return similarity;

}