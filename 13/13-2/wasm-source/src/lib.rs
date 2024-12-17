use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> i64
{
    let parsed: Vec<(i64, i64, i64, i64, i64, i64)> = String::from_utf8(file).unwrap()
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

fn cheapest(dx1: i64,
    dy1: i64,
    dx2: i64,
    dy2: i64,
    mut x: i64,
    mut y: i64) -> Option<i64>
{
    x += 10000000000000;
    y += 10000000000000;

    if (dx1*dx2 + dy1*dy2) * (dx1*dx2 + dy1*dy2) // squared dot product
    == (dx1*dx1 + dy1*dy1) * (dx2*dx2 + dy1*dy1)  // product of squared lengths
    {
        if x % dx2 == 0 && y % dy2 == 0 {
            let tokens_1 = 3 * (x / dx1);
            let tokens_2 = x / dx2;
            if tokens_1 < tokens_2 {
                return Some(tokens_1);
            } else {
                return Some(tokens_2);
            }
        } else {
            return None;
        }
    }

    let m = (x*dy2 - y*dx2) / (dx1*dy2 - dx2*dy1);
    let n = (y*dx1 - x*dy1) / (dx1*dy2 - dx2*dy1);

    let x1 = dx1*m + dx2*n;
    let y1 = dy1*m + dy2*n;

    if x1 == x && y1 == y {
        return Some(3*m + n);
    } else {
        return None;
    }
}

fn extract_numbers(input: &str) -> Vec<i64> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<i64>() {
                    numbers.push(num);
                }
                current_number.clear();
            }
        }
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<i64>() {
            numbers.push(num);
        }
    }

    numbers
}