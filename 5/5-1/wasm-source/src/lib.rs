use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn solve(file: Vec<u8>) -> u32
{
    let string = String::from_utf8(file).unwrap();

    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut parsing_rules = true;

    for line in string.lines() {
        if parsing_rules {
            if line.is_empty() {
                parsing_rules = false;
            } else if let Some((col1, col2)) = line.split_once('|') {
                if let (Ok(col1), Ok(col2)) = (col1.parse::<u32>(),
                                               col2.parse::<u32>()) {
                    rules.push((col1, col2));
                }
            }
        } else {
            let update: Vec<u32> = line
                .split(',')
                .map(|number| number.parse::<u32>().unwrap())
                .collect();
            updates.push(update);
        }
    }

    let mut sum = 0;

    for update in updates {
        let update_correct = (0..update.len() - 1).all(|i_page| {
            let page = update[i_page];
            rules.iter().all(|rule| {
                rule.1 != page || !update[i_page + 1..].contains(&rule.0)
            })
        });
    
        if update_correct {
            sum += update[update.len() / 2];
        }
    }

    return sum;
}

