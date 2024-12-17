use wasm_bindgen::prelude::wasm_bindgen;
use std::collections::HashMap;

struct Cell {
    area_walked: bool,
    group_idx: u16,
    byte: u8
}

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let mut file_cell : Vec<Cell>
        = file.iter()
            .map(|&x| {
                Cell {
                    area_walked: false,
                    group_idx: 0,
                    byte: x}
            })
            .collect();
    let width = file.iter().position(|&x| x == b'\n').unwrap();
    let height = file.len() / (width + 1);
    let mut areas = Vec::new();


    let mut group_idx: u16 = 0;

    for x in 0..width {
        for y in 0..height {
            let idx = (width + 1) * y + x;
            if !file_cell[idx].area_walked {
                let area
                    = walk_area(x,
                        y,
                        group_idx,
                        &mut file_cell,
                        height,
                        width);
                group_idx += 1;
                areas.push(area);
            }
        }
    }

    let mut groups_sides: HashMap<u16, u32> = HashMap::new();

    for x in 0..width {
        count_sides_vertical(x,
            &file_cell,
            &mut groups_sides,
            height,
            width);
    }

    for y in 0..height {
        count_sides_horizontal(y,
            &file_cell, 
            &mut groups_sides,
            height,
            width);
    }

    areas.iter()
        .zip(0..)
        .filter_map(|(area, key)| {Some(area * groups_sides.get(&key)?)})
        .sum()
}

fn count_sides_vertical(x: usize,
    file: &Vec<Cell>,
    groups_sides: &mut HashMap<u16, u32>,
    height: usize,
    width: usize) -> ()
{
    let mut y = 0;
    while y < height {
        let group = file[(width + 1) * y + x].group_idx;
        if x == 0
        || file[(width + 1) * y + (x - 1)].group_idx != group {
            *groups_sides.entry(group).or_insert(0) += 1;
            while y < height
            && file[(width + 1) * y + x].group_idx == group
            && (x == 0
                || file[(width + 1) * y + (x - 1)].group_idx != group)
            {
                y += 1;
            }
        } else {
            y += 1;
        }
    }

    y = 0;
    while y < height {
        let group = file[(width + 1) * y + x].group_idx;
        if x == width - 1
        || file[(width + 1) * y + (x + 1)].group_idx != group {
            *groups_sides.entry(group).or_insert(0) += 1;
            while y < height
            && file[(width + 1) * y + x].group_idx == group
            && (x == width - 1
                || file[(width + 1) * y + (x + 1)].group_idx != group)
            {
                y += 1;
            }
        } else {
            y += 1;
        }
    }
}


fn count_sides_horizontal(y: usize,
    file: &Vec<Cell>,
    groups_sides: &mut HashMap<u16, u32>,
    height: usize,
    width: usize) -> ()
{
    let mut x = 0;
    while x < width {
        let group = file[(width + 1) * y + x].group_idx;
        if y == 0
        || file[(width + 1) * (y - 1) + x].group_idx != group {
            *groups_sides.entry(group).or_insert(0) += 1;
            while x < width
            && file[(width + 1) * y + x].group_idx == group
            && (y == 0
                || file[(width + 1) * (y - 1) + x].group_idx != group)
            {
                x += 1;
            }
        } else {
            x += 1;
        }
    }

    x = 0;
    while x < width {
        let group = file[(width + 1) * y + x].group_idx;
        if y == height - 1
        || file[(width + 1) * (y + 1) + x].group_idx != group {
            *groups_sides.entry(group).or_insert(0) += 1;
            while x < width
            && file[(width + 1) * y + x].group_idx == group
            && (y == height - 1
                || file[(width + 1) * (y + 1) + x].group_idx != group)
            {
                x += 1;
            }
        } else {
            x += 1;
        }
    }
}

fn walk_area(x: usize,
    y: usize,
    group_idx: u16,
    file: &mut Vec<Cell>,
    height: usize,
    width: usize) -> u32
{
    let idx = (width + 1) * y + x;
    let letter = file[idx].byte;
    file[idx].area_walked = true;
    file[idx].group_idx = group_idx;
    let mut area = 1;
    
    if x > 0 {
        let idx_l = (width + 1) * y + (x - 1);
        if file[idx_l].byte == letter && !file[idx_l].area_walked {
            area += walk_area(x-1, y, group_idx, file, height, width);
        }
    }

    if x < width - 1 {
        let idx_r = (width + 1) * y + (x + 1);
        if file[idx_r].byte == letter && !file[idx_r].area_walked {
            area += walk_area(x+1, y, group_idx, file, height, width);
        }
    }

    if y > 0 {
        let idx_u = (width + 1) * (y - 1) + x;
        if file[idx_u].byte == letter && !file[idx_u].area_walked {
            area += walk_area(x, y-1, group_idx, file, height, width);
        }
    }

    if y < height - 1 {
        let idx_d = (width + 1) * (y + 1) + x;
        if file[idx_d].byte == letter && !file[idx_d].area_walked {
            area += walk_area(x, y+1, group_idx, file, height, width);
        }
    }

    return area;

}


