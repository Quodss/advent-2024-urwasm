use std::collections::HashMap;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u64
{
    let stones: Vec<u64> = String::from_utf8(file).unwrap()
        .split(" ")
        .filter_map(|column| {
            column.parse::<u64>().ok()
        })
        .collect();
    
    let mut memo: HashMap<(u32, u64), u64> = HashMap::new();
    let mut n_stones: u64 = 0;
    for stone in stones {
        n_stones += count_stones(75, stone, &mut memo);
    }
    return n_stones;
}

fn count_stones(blinks: u32, stone: u64, memo: &mut HashMap<(u32, u64), u64>) -> u64
{
    if let Some(&count) = memo.get(&(blinks, stone)) {
        return count;
    } else if blinks == 0 {
        return 1;
    } else if stone == 0 {
        let count = count_stones(blinks-1, 1, memo);
        memo.insert((blinks, stone), count);
        return count;
    } else if even_digits(stone) {
        let (a, b) = split_digits(stone);
        let count_a = count_stones(blinks-1, a, memo);
        let count_b = count_stones(blinks-1, b, memo);
        let count = count_a + count_b;
        memo.insert((blinks, stone), count);
        return count;
    } else {
        let count = count_stones(blinks-1, stone*2024, memo);
        memo.insert((blinks, stone), count);
        return count;
    }
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