use wasm_bindgen::prelude::wasm_bindgen;
use std::collections::HashSet;
use itertools::iproduct;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let width = file.iter().position(|&x| x==b'\n').unwrap();
    let height = file.len() / width;

    let mut positions: HashSet<(usize, usize)> = HashSet::new();

    for (x1, y1) in iproduct!(0..width, 0..height) {
        for (x2, y2) in iproduct!(0..width, 0..height) {
            if x2 == x1 && y2 == y1 {
                continue;
            }
            let idx1 = (width + 1) * y1 + x1;
            let idx2 = (width + 1) * y2 + x2;

            if file[idx1] != file[idx2] || file[idx2] == b'.' {
                continue;
            }

            positions.insert((x1, y1));
            positions.insert((x2, y2));

            let x_dif: i32 = (x1 as i32) - (x2 as i32);
            let y_dif: i32 = (y1 as i32) - (y2 as i32);

            let mut x_pos: i32 = (x1 as i32) + x_dif;
            let mut y_pos: i32 = (y1 as i32) + y_dif;
            while x_pos >= 0 && x_pos < width as i32
                             && y_pos >= 0
                             && y_pos < height as i32 {
                positions.insert((x_pos as usize, y_pos as usize));
                x_pos += x_dif;
                y_pos += y_dif;
            }

            let mut x_neg: i32 = (x2 as i32) - x_dif;
            let mut y_neg: i32 = (y2 as i32) - y_dif;
            while x_neg >= 0 && x_neg < width as i32
                             && y_neg >= 0
                             && y_neg < height as i32 {
                positions.insert((x_neg as usize, y_neg as usize));
                x_neg -= x_dif;
                y_neg -= y_dif;
            }
            
        }
    }
    return positions.len() as u32;
}