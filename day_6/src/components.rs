#[derive(Debug)]
pub struct Race {
    duration: u32,
    record_distance: u32,
    pub ways_to_win: Vec<u32>
}

impl Race {

    pub fn new(duration: u32, record: u32) -> Race {
        Race {
            duration: duration,
            record_distance: record,
            ways_to_win: Vec::new(),
        }
    }

    pub fn calculate_ways_to_win(&mut self) {
        self.ways_to_win.clear();

        for push_time in 1..self.duration {
            let distance = push_time * (self.duration - push_time);

            if distance > self.record_distance {
                self.ways_to_win.push(push_time);
            }
        }
    }
}