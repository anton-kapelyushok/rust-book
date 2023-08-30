use std::collections::HashMap;

fn main() {
    println!("{:?}", median_and_mode(&vec![5, 4, 1, 2, 3, 5]));
    println!("{:?}", median_and_mode(&vec![2]));
    println!("{:?}", median_and_mode(&vec![2, 4]));
}

#[allow(dead_code)]
#[derive(Debug)]
struct MedianAndMode {
    median: f64,
    mode: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
enum MedianAndModeError {
    InputIsEmpty,
}

fn median_and_mode(in_vec: &[i32]) 
    -> Result<MedianAndMode, MedianAndModeError> {
    
    if in_vec.len() == 0 {
        return Err(MedianAndModeError::InputIsEmpty);
    }

    let mut in_vec = in_vec.to_vec();
    in_vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut counts = HashMap::new();
    let mut max_count = 0;
    let mut mode = 0;

    for &i in &in_vec {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = i;
        }
    }


    let mid = in_vec.len() / 2;
    let median: f64 = if in_vec.len() % 2 == 1 {
        f64::from(in_vec[mid])
    } else {
        f64::from(in_vec[mid - 1] + in_vec[mid]) / 2.0
    };

    return Ok(MedianAndMode { median, mode });
}
