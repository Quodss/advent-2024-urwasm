use wasm_bindgen::prelude::wasm_bindgen;

enum Command {
    Do,
    Dont,
    Mul(u32, u32),
}

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let mut idx = 0;
    let mut commands = Vec::<Command>::new();
    while idx < file.len() {
        if let Some(command) = take_do(&file, &mut idx) {
            commands.push(command);
            continue;
        }
        if let Some(command) = take_dont(&file, &mut idx) {
            commands.push(command);
            continue;
        }
        if let Some(command) = take_mul(&file, &mut idx) {
            commands.push(command);
            continue;
        }
        idx += 1;
    }
    
    let mut sum: u32 = 0;
    let mut flag: bool = true;
    for i in 0..commands.len() {
        match commands[i] {
            Command::Mul(a, b) => {
                if flag {sum += a * b}
            }
            Command::Do =>   {flag = true}
            Command::Dont => {flag = false}
        }
    }
    return sum;
}


fn take_mul(file: &Vec<u8>, idx: &mut usize) -> Option<Command>
{
    if *idx >= (file.len() - 4) {return None};

    let mut idx_local = *idx;

    let pref = file.get(idx_local..idx_local+4).unwrap();
    let pref_str = std::str::from_utf8(pref).unwrap();
    if pref_str != "mul(" {return None};
    idx_local += 4;
    let a: u32 = take_number(&file, &mut idx_local)?;
    if (idx_local >= file.len()) || (file[idx_local] != b',') {
        return None;
    }
    idx_local += 1;
    let b: u32 = take_number(&file, &mut idx_local)?;
    if (idx_local >= file.len()) || (file[idx_local] != b')') {
        return None;
    }
    *idx = idx_local;
    return Some(Command::Mul(a, b));
}

fn take_do(file: &Vec<u8>, idx: &mut usize) -> Option<Command>
{
    if *idx >= (file.len() - 4) {return None};
    let pref = file.get(*idx..*idx+4).unwrap();
    let pref_str = std::str::from_utf8(pref).unwrap();
    if pref_str != "do()" {return None};
    *idx += 4;
    return Some(Command::Do);
}

fn take_dont(file: &Vec<u8>, idx: &mut usize) -> Option<Command>
{
    if *idx >= (file.len() - 7) {return None};
    let pref = file.get(*idx..*idx+7).unwrap();
    let pref_str = std::str::from_utf8(pref).unwrap();
    if pref_str != "don't()" {return None};
    *idx += 7;
    return Some(Command::Dont);
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
