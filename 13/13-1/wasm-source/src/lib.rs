use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let parsed: Vec<(u32, u32, u32, u32, u32, u32)> = String::from_utf8(file).unwrap()
        .split("\n\n")
        .map(|block| {
            let numbers = extract_numbers(block);
            (numbers[0],
                numbers[1],
                numbers[2],
                numbers[3],
                numbers[4],
                numbers[5])
        }).collect();
    
    parsed.iter()
        .filter_map(|(dx1, dy1, dx2, dy2, x, y)| {
            cheapest(*dx1, *dy1, *dx2, *dy2, *x, *y)
        })
        .sum()
}

fn cheapest(dx1: u32,
    dy1: u32,
    dx2: u32,
    dy2: u32,
    x: u32,
    y: u32) -> Option<u32>
{
    let mut min = None;
    for m in 0..100 {
        if (x - m * dx1) % dx2 == 0
        && (y - m * dy1) % dy2 == 0
        && (x - m * dx1) / dx2 == (y - m * dy1) / dy2
        {
            let a = 3 * m + (y - m * dy1) / dy2;
            min = if let Some(b) = min {
                Some(if a < b {a} else {b})
            } else {
                Some(a)
            };
        }
    }
    return min;
}

fn extract_numbers(input: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<u32>() {
                    numbers.push(num);
                }
                current_number.clear();
            }
        }
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<u32>() {
            numbers.push(num);
        }
    }

    numbers
}