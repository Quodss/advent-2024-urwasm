use wasm_bindgen::prelude::wasm_bindgen;
use std::cmp::Ordering;
use std::collections::HashSet;

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

    let mut orderings: HashSet<(u32, u32)> = HashSet::new();
    let mut sum = 0;
    for mut update in updates {
        let update_correct = (0..update.len() - 1).all(|i_page| {
            let page = update[i_page];
            rules.iter().all(|rule| {
                rule.1 != page || !update[i_page + 1..].contains(&rule.0)
            })
        });
    
        if !update_correct {
            //  assume that rules form DAG
            //
            update.sort_unstable_by(|&a, &b| {order_pages(a, b, &rules, &mut orderings)});
            sum += update[update.len() / 2];
        }
    }

    return sum;
}

fn order_pages(a: u32,
    b: u32,
    rules: &Vec<(u32, u32)>,
    orderings: &mut HashSet<(u32, u32)>) -> Ordering
{
    if a == b                      {return Ordering::Equal;}
    if orderings.contains(&(a, b)) {return Ordering::Greater;}
    if orderings.contains(&(b, a)) {return Ordering::Less;}

    for rule in rules {
        if (a, b) == *rule {
            orderings.insert((b, a));
            return Ordering::Less;
        }
        if (b, a) == *rule {
            orderings.insert((a, b));
            return Ordering::Greater;
        }
    }

    for rule in rules {
        if a == rule.0
            && order_pages(rule.1, b, rules, orderings) == Ordering::Less {
            orderings.insert((b, a));
            return Ordering::Less;
        }
    }

    for rule in rules {
        if a == rule.1
            && order_pages(rule.0, b, rules, orderings) == Ordering::Greater {
            orderings.insert((a, b));
            return Ordering::Greater;
        }
    }
    return Ordering::Equal // couldn't compare, the order shouldn't matter
}