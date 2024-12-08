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

            let x_dif: i32 = (x1 as i32) - (x2 as i32);
            let y_dif: i32 = (y1 as i32) - (y2 as i32);
            
            let mayb_anti1_x: i32 = (x1 as i32) + x_dif;
            let mayb_anti1_y: i32 = (y1 as i32) + y_dif;
            let mayb_anti2_x: i32 = (x2 as i32) - x_dif;
            let mayb_anti2_y: i32 = (y2 as i32) - y_dif;

            let anti1_inside = mayb_anti1_x >= 0
                && mayb_anti1_x < width as i32
                && mayb_anti1_y >= 0
                && mayb_anti1_y < height as i32;
            
            let anti2_inside = mayb_anti2_x >= 0
                && mayb_anti2_x < width as i32
                && mayb_anti2_y >= 0
                && mayb_anti2_y < height as i32;
            
            if anti1_inside {
                positions.insert(
                    (mayb_anti1_x as usize, mayb_anti1_y as usize)
                );
            }

            if anti2_inside {
                positions.insert(
                    (mayb_anti2_x as usize, mayb_anti2_y as usize)
                );
            }
            
        }
    }
    return positions.len() as u32;
}