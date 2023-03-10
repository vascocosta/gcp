use std::collections::HashMap;

mod tests;

fn display_float(value: f32) -> String {
    if value == value.floor() {
        value.to_string()
    } else {
        format!("{:.2}", value)
    }
}

fn mean(numbers: &Vec<i32>) -> Option<f32> {
    if numbers.len() == 0 {
        return None;
    }

    let mut sum = 0;

    for number in numbers {
        sum += number;
    }

    Some(sum as f32 / numbers.len() as f32)
}

fn median(numbers: &Vec<i32>) -> Option<f32> {
    if numbers.len() == 0 {
        return None;
    }

    let mut numbers = numbers.clone();

    numbers.sort();

    if numbers.len() % 2 == 0 {
        Some((numbers[numbers.len() / 2] + numbers[(numbers.len() / 2) - 1]) as f32 / 2.0)
    } else {
        Some(numbers[numbers.len() / 2] as f32)
    }
}

fn mode(numbers: &Vec<i32>) -> Option<Vec<i32>> {
    let mut mode: Option<Vec<i32>> = None;
    let mut modes: Vec<i32> = Vec::new();
    let mut frequencies: HashMap<i32, i32> = HashMap::new();

    for number in numbers {
        let count = frequencies.entry(*number).or_insert(0);

        *count += 1;
    }

    let max_frequency = *frequencies.values().max().unwrap_or(&0);

    for (number, count) in frequencies {
        if count == max_frequency && max_frequency > 1 {
            modes.push(number);
        }
    }

    if modes.len() > 0 {
        modes.sort();
        mode = Some(modes);
    }

    mode
}

fn p1(numbers: &Vec<i32>) {
    match mean(&numbers) {
        None => print!("Mean: None, "),
        Some(mean) => print!("Mean: {}, ", display_float(mean)),
    }

    match median(&numbers) {
        None => print!("Median: None, "),
        Some(median) => print!("Median: {}, ", display_float(median)),
    }

    match mode(&numbers) {
        None => println!("Mode: None"),
        Some(modes) => println!("Mode: {:?}", modes),
    }
}

fn main() {
    p1(&vec![1, 2, 3]);
    p1(&vec![4, 11, 2, 6, 10, 20]);
    p1(&vec![3, 6, 2, 6, 1, 6, 1]);
    p1(&vec![1, 1, 2, 2, 2, 3, 3, 3, 4]);
    p1(&vec![1]);
    p1(&vec![]);
}