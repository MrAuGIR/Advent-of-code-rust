use memoize::memoize;

pub fn get_diff(values: &[u64]) -> Vec<u64> {
    return values.windows(2)
    .map(|pair| (pair[1] as i64 - pair[0] as i64).abs() as u64)
    .collect()
}

pub fn is_between_1_and_3(values: &[u64] ) -> bool {
    return values.iter().all(|&value| (1..=3).contains(&value));
}

pub fn has_single_direction(values: &[u64]) -> bool {
    let directions = values.windows(2).map(|pair| pair[1] as i64 - pair[0] as i64);

    let mut last_sign = 0;

    for diff in directions {
        let sign = diff.signum();

        if sign != 0 {
            if last_sign != 0 && sign != last_sign {
                return false;
            }
            last_sign = sign;
        }
    }
    true
}

pub fn with_remove_single_one(values: &[u64]) -> bool {
    let mut valid_count = 0;
    
    for i in 0..values.len() {
        let mut new_vec = values.to_vec();
        new_vec.remove(i);

        if check_condition(new_vec) {
            return true;
        }
    }

    return false;
}


pub fn check_condition(values: Vec<u64>) -> bool {

    let diff = get_diff(&values);

    is_between_1_and_3(&diff) && has_single_direction(&values)
}