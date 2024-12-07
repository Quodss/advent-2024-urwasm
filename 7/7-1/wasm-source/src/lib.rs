use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u64
{
    String::from_utf8(file).unwrap()
        .lines()
        .filter_map(|line| {
            let (target_str, operands_str) = line.split_once(": ")?;
            let target: u64 = target_str.parse::<u64>().ok()?;
            let operands: Vec<u64> = operands_str.split(' ')
                .map(|number| number.parse::<u64>().unwrap())
                .collect();

            let n_operators = operands.len() - 1;

            let can_be_true = (0..(1 << n_operators)).any(|permutation| {
                let result = operands[1..]
                    .iter()
                    .fold(
                        (permutation, operands[0]),
                        |(perm, res), &x| match perm % 2 {
                            0 => (perm / 2, res + x),
                            _ => (perm / 2, res * x),
                        }
                ).1;

                result == target
            });
            if can_be_true {Some(target)} else {None}
        }).sum()
    

}