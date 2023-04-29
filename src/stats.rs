#[derive(Debug, PartialEq, Eq)]
pub struct StatsConfig {
    pub time: bool,
    pub percentage: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CollectedStats {
    pub times_secs: Option<Vec<u64>>,
    pub pos_negs: Option<Vec<bool>>,
}

pub struct PosNeg {
    pub positive: u8,
    pub negative: u8,
}

pub fn calculate_average_time_secs(times_secs: Vec<u64>) -> u64 {
    let mut full: u64 = 0;
    let mut prev: u64 = 0;
    let length = times_secs.len() as u64;
    for time in times_secs {
        full = full + (time - prev);
        prev = time;
    }
    prev / length
}

pub fn calculate_total_pos_neg(pos_negs: Vec<bool>) -> PosNeg {
    let mut pos: u8 = 0;
    let mut neg: u8 = 0;

    for item in pos_negs {
        if item {
            pos += 1;
        } else {
            neg += 1;
        }
    }

    PosNeg {
        positive: pos,
        negative: neg,
    }
}
