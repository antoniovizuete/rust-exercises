use std::collections::HashMap;

fn main() {
    let v = vec![4, 2, 1, 3, 5, 3, 10, 5, 5];
    let mean = mean(&v);
    let median = median(&v);
    let mode = mode(&v);

    println!("The mean of {:?} is {}", v, mean);
    println!("The median of {:?} is {}", v, median);
    println!("The mode of {:?} is {}", v, mode);
}

fn mean(v: &Vec<u32>) -> f32 {
    let mut sum = 0f32;
    let mut count = 0f32;
    for value in v.iter() {
        let f_value: f32 = *value as f32;
        sum += f_value;
        count += 1f32;
    }
    sum / count
}

fn median(v: &[u32]) -> f32 {
    if v.len() == 0 {
        return 0f32;
    }

    let mut vector: Vec<u32> = v.to_vec();
    vector.sort();
    let half = v.len() / 2;
    
    if v.len() % 2 == 0 {
        return match vector.get(half) {
            Some(i) => *i as f32,
            None =>  0f32
        }
    } else {
        return mean(&vector[half - 1..half].to_vec())
    }
}

fn mode (v: &[u32]) -> u32 {
    let mut map: HashMap<u32, u32> = HashMap::new();
    for el in v.iter() {
        let mut count: u32 = match map.get(el) {
            Some(i) => *i,
            None => 0
        };
        count = count + 1;
        map.insert(*el, count);
    }
    let mut result: (u32, u32) = (0, 0);
    for (k, v) in map.iter() {
        if v > &result.1 {
            result = (*k, *v);
        }
    }
    result.0
}