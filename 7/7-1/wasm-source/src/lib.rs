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
            let can_be_true = (0..(1 << n_operators)).any(|permutation| {
                result == operands[1..].iter()
                    .fold((permutation, operands[0]), |acc, &x| {
                        if acc.0 % 2 == 0 {
                            (acc.0 / 2, acc.1 + x)
                        } else {
                            (acc.0 / 2, acc.1 * x)
                        }
                    }).1
            });
            if can_be_true {Some(result)} else {None}
        }).sum()
    

}