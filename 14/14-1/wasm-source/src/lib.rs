use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let parsed: Vec<(i32, i32, i32, i32)> = String::from_utf8(file).unwrap()
        .lines()
        .map(|line| {
            let numbers = extract_numbers(line);
            (numbers[0],
                numbers[1],
                numbers[2],
                numbers[3])
        }).collect();
    
    let height = 103;
    let width = 101;

    let mut q1: u32 = 0;
    let mut q2: u32 = 0;
    let mut q3: u32 = 0;
    let mut q4: u32 = 0;

    for (x0, y0, v_x, v_y) in parsed {
        let x = (((x0 + v_x * 100) % width) + width) % width;
        let y = (((y0 + v_y * 100) % height) + height) % height;

        if x < width / 2 && y < height / 2 {
            q1 += 1;
        } else if x > width / 2 && y < height / 2 {
            q2 += 1;
        } else if x < width / 2 && y > height / 2 {
            q3 += 1;
        } else if x > width / 2 && y > height / 2 {
            q4 += 1;
        }
    }

    q1 * q2 * q3 * q4

}

fn extract_numbers(input: &str) -> Vec<i32> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for c in input.chars() {
        if c.is_digit(10) || c == '-' {
            current_number.push(c);
        } else {
            if !current_number.is_empty() {
                if let Ok(num) = current_number.parse::<i32>() {
                    numbers.push(num);
                }
                current_number.clear();
            }
        }
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<i32>() {
            numbers.push(num);
        }
    }

    numbers
}