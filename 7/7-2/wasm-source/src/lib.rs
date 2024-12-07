use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u64
{
    String::from_utf8(file).unwrap()
        .lines()
        .filter_map(|line| {
            let (result_str, operands_str) = line.split_once(": ")?;
            let result: u64 = result_str.parse::<u64>().ok()?;
            let operands: Vec<u64> = operands_str.split(' ')
                .map(|number| number.parse::<u64>().unwrap())
                .collect();
            let n_operators = operands.len() - 1;
            let can_be_true = (0..((3u64).pow(n_operators as u32)))
                .any(|permutation| {
                    result == operands[1..].iter()
                        .fold((permutation, operands[0]), |acc, &x| {
                            if acc.0 % 3 == 0 {
                                (acc.0 / 3, acc.1 + x)
                            } else if acc.0 % 3 == 1 {
                                (acc.0 / 3, acc.1 * x)
                            } else {
                                (acc.0 / 3, concat(acc.1, x))
                            }
                        }).1
                });

            if can_be_true {Some(result)} else {None}
        }).sum()
}

fn concat(a: u64, b: u64) -> u64
{
    let mut b_digits: u32 = 1;
    let mut b1 = b;
    while b1 > 9 {
        b1 /= 10;
        b_digits += 1;
    }
    a * (10u64).pow(b_digits) + b
}