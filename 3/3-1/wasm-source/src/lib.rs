use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let mut sum: u32 = 0;
    let mut idx = 0;
    while idx < (file.len() - 4) {
        let pref = file.get(idx..idx+4).unwrap();
        let pref_str = std::str::from_utf8(pref).unwrap();
        if pref_str == "mul(" {
            idx += 4;
        } else {
            idx += 1;
            continue;
        }
        let mut idx_numbers = idx;
        let a_mayb = take_number(&file, &mut idx_numbers);
        if let Some(a) = a_mayb {
            idx = idx_numbers;
            if idx < file.len() && file[idx] == b',' {
                idx += 1;
                idx_numbers = idx;
                let b_mayb = take_number(&file, &mut idx_numbers);
                if let Some(b) = b_mayb {
                    idx = idx_numbers;
                    if idx < file.len() && file[idx] == b')' {
                        idx += 1;
                        sum += a * b;
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }
        } else {
            continue;
        }
    }
    return sum;
}

fn take_number(file: &Vec<u8>, idx: &mut usize) -> Option<u32>
{
    if *idx >= file.len()
        || file[*idx] < b'0'
        || file[*idx] > b'9' {
        return None;
    }

    let mut out: u32 = 0;
    
    while *idx < file.len()
            && file[*idx] >= b'0'
            && file[*idx] <= b'9' {
        out = 10 * out + ((file[*idx] - b'0') as u32);
        *idx += 1;
    }
    return Some(out);
}
