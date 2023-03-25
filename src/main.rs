use std::io;
use std::collections::HashMap;
use std::collections::BTreeMap;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let w = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    let _count_x = parse_input!(inputs[2], i32);
    let _count_y = parse_input!(inputs[3], i32);
    let mut inputs = String::new();

    let mut width_values = Vec::<i32>::new();
    io::stdin().read_line(&mut inputs).unwrap();
    for i in inputs.split_whitespace() {
        let x = parse_input!(i, i32);
        width_values.push(x);
    }

    let mut height_values = Vec::<i32>::new();
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).unwrap();
    for i in inputs.split_whitespace() {
        let y = parse_input!(i, i32);
        height_values.push(y);
    }

    width_values = calculate_all_values_and_sort(
        width_values,
        w
    );

    height_values = calculate_all_values_and_sort(
        height_values,
        h
    );

    let mut width_map = HashMap::<i32, i32>::new();
    let mut height_map = HashMap::<i32, i32>::new();

    for x in width_values {
        *width_map.entry(x).or_default() += 1;
    }

    for y in height_values {
        *height_map.entry(y).or_default() += 1;
    }

    let mut matches = 0;
    for (k, v) in width_map {
        match height_map.get(&k) {
            None => {}
            Some(s) => {
                matches += s * v;
            }
        };
    }

    println!("{matches}");

}

fn calculate_all_values_and_sort(
    mut values: Vec::<i32>,
    total: i32
) -> Vec::<i32> {
    values.sort();
    let mut append_vec = Vec::<i32>::new();

    for (i, outter) in values.iter().enumerate() {
        for j in (i + 1 as usize)..values.len() {
            append_vec.push(values[j]-outter);
        }
        append_vec.push(total - outter);
    }

    values.extend(append_vec);
    values.push(total);
    values.sort();

    values
}