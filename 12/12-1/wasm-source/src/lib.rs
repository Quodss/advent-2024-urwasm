use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(mut file: Vec<u8>) -> u32
{
    let width = file.iter().position(|&x| x == b'\n').unwrap();
    let height = file.len() / (width + 1);

    let mut sum = 0;

    for x in 0..width {
        for y in 0..height {
            let idx = (width + 1) * y + x;
            if (file[idx] >> 7) == 0 {
                let (area, perimeter) =
                    measure(x, y, &mut file, height, width);
                sum += area * perimeter;
            }
        }
    }

    return sum;
}

fn measure(x: usize,
    y: usize,
    file: &mut Vec<u8>,
    height: usize,
    width: usize) -> (u32, u32)
{
    let idx = (width + 1) * y + x;
    let letter: u8 = file[idx];
    file[idx] |=  1 << 7;
    let mut area = 1;
    let mut perimeter = 0;

    if x == 0 || (file[(width + 1) * y + (x - 1)] & !(1u8 << 7)) != letter {
        perimeter += 1;
    }

    if x == width - 1
        || (file[(width + 1) * y + (x + 1)] & !(1u8 << 7)) != letter {
        perimeter += 1;
    }

    if y == 0
        || (file[(width + 1) * (y - 1) + x] & !(1u8 << 7)) != letter {
        perimeter += 1;
    }

    if y == height - 1
        || (file[(width + 1) * (y + 1) + x] & !(1u8 << 7)) != letter {
        perimeter += 1;
    }
    
    if x > 0 {
        let idx_l = (width + 1) * y + (x - 1);
        if file[idx_l] == letter {
            let (area_l, perimeter_l) = measure(x-1, y, file, height, width);
            area += area_l;
            perimeter += perimeter_l;
        }
    }

    if x < width - 1 {
        let idx_r = (width + 1) * y + (x + 1);
        if file[idx_r] == letter {
            let (area_r, perimeter_r) = measure(x+1, y, file, height, width);
            area += area_r;
            perimeter += perimeter_r;
        }
    }

    if y > 0 {
        let idx_u = (width + 1) * (y - 1) + x;
        if file[idx_u] == letter {
            let (area_u, perimeter_u) = measure(x, y-1, file, height, width);
            area += area_u;
            perimeter += perimeter_u;
        }
    }

    if y < height - 1 {
        let idx_d = (width + 1) * (y + 1) + x;
        if file[idx_d] == letter {
            let (area_d, perimeter_d) = measure(x, y+1, file, height, width);
            area += area_d;
            perimeter += perimeter_d;
        }
    }

    return (area, perimeter);

}
