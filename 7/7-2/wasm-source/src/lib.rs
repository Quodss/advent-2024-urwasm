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

            let mut permutations = 0..(3u64.pow(n_operators as u32));

            let can_be_true = permutations.any(|permutation| {
                let result = operands[1..]
                    .iter()
                    .fold(
                        (permutation, operands[0]),
                        |(perm, res), &x| match perm % 3 {
                            0 => (perm / 3, res + x),
                            1 => (perm / 3, res * x),
                            _ => (perm / 3, concat(res, x)),
                        }
                ).1;
                
                result == target
            });
            if can_be_true {Some(target)} else {None}
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