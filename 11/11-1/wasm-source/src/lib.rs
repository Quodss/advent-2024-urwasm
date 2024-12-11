use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let mut stones: Vec<u64> = String::from_utf8(file).unwrap()
        .split(" ")
        .filter_map(|column| {
            column.parse::<u64>().ok()
        })
        .collect();
    
    
    for _ in 0..25 {
        let mut stones_new: Vec<u64> = Vec::new();
        for stone in stones {
            if stone == 0 {
                stones_new.push(1);
            } else if even_digits(stone) {
                let (a, b) = split_digits(stone);
                stones_new.push(a);
                stones_new.push(b);
            } else {
                stones_new.push(2024*stone)
            }
        }
        stones = stones_new;
    }

    return stones.len() as u32;
}

fn even_digits(mut a: u64) -> bool
{
    loop {
        if a < 10 {
            return false;
        }
        if a < 100 {
            return true;
        }
        a /= 100;
    }
}

fn split_digits (mut a: u64) -> (u64, u64)
{
    let mut ceil = 1;

    let mut bottom = 0;

    while a > ceil {
        bottom += ceil * (a % 10);
        a /= 10;
        ceil *= 10;
    }
    return (a, bottom);
}