
#[derive(Clone,Debug)]
pub struct Report {
    pub levels: Level
}

#[derive(Clone,Debug)]
pub struct Level {
    pub values: Vec<u64>,
    pub is_safe: bool
}

impl Report {

    pub fn new(levels: Level) -> Report {
        Report {
            levels
        }
    }
}

impl Level {

    pub fn calcul_is_safe(&mut self) {
        
        let diff: Vec<u64> = self.get_diff();

        let is_in_range = Level::is_between_1_and_3(
            diff.clone()
        );

        let has_one_direction = Level::has_single_direction(
            self.values.clone()
        );

        self.is_safe = is_in_range && has_one_direction;

       // println!("{:?}  is in range {} and has one direction {}",self.values,is_in_range,has_one_direction)

    }

    fn get_diff(&self) -> Vec<u64> {
        return self.values.windows(2)
        .map(|pair| (pair[1] as i64 - pair[0] as i64).abs() as u64)
        .collect()
    }

    fn is_between_1_and_3(values: Vec<u64> ) -> bool {
        return values.iter().all(|&value| (1..=3).contains(&value));
    }

    fn has_single_direction(values: Vec<u64>) -> bool {
        let mut directions = values.windows(2).map(|pair| pair[1] as i64 - pair[0] as i64);

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
}