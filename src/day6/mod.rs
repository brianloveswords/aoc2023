#![allow(unused)]

pub fn part1(races: &[Race]) -> usize {
    races.iter().map(|r| r.count_winners()).product()
}

pub fn part2(race: &Race) -> usize {
    race.count_winners()
}

const EXAMPLE: [Race; 3] = [
    Race {
        time: Time(7),
        record: Record(9),
    },
    Race {
        time: Time(15),
        record: Record(40),
    },
    Race {
        time: Time(30),
        record: Record(200),
    },
];

pub const REAL: [Race; 4] = [
    Race {
        time: Time(41),
        record: Record(214),
    },
    Race {
        time: Time(96),
        record: Record(1789),
    },
    Race {
        time: Time(88),
        record: Record(1127),
    },
    Race {
        time: Time(94),
        record: Record(1055),
    },
];

pub const EXAMPLE_PART2: Race = Race {
    time: Time(71530),
    record: Record(940200),
};

pub const REAL_PART2: Race = Race {
    time: Time(41968894),
    record: Record(214178911271055),
};

#[derive(Debug, Clone, Copy)]
pub struct Time(pub usize);

#[derive(Debug, Clone, Copy)]
pub struct Record(pub usize);

#[derive(Debug)]
pub struct Race {
    time: Time,
    record: Record,
}

impl Race {
    pub fn new(time: Time, record: Record) -> Self {
        Self { time, record }
    }

    pub fn count_winners(&self) -> usize {
        self.count_winners_constant()
    }

    pub fn count_winners_oracle(&self) -> usize {
        let mut count = 0;
        let mut time = self.time.0;
        let mut record = self.record.0;

        let mut count = 0;
        for charge in 1..time {
            let speed = charge;
            let duration = (time - charge);
            let distance = duration * speed;
            if distance > record {
                count += 1;
            }
        }
        count
    }

    pub fn calculate_distance(&self, charge: usize) -> usize {
        let duration = (self.time.0 - charge);
        let distance = duration * charge;
        distance
    }

    pub fn is_possible_winner(&self, charge: usize) -> bool {
        self.time.0 % 2 != charge % 2
    }

    pub fn count_winners_constant(&self) -> usize {
        let time = self.time.0;
        let record = self.record.0;
        let speed = time / 2;
        let remaining = time - speed;
        let distance = speed * remaining;

        if distance <= record {
            return 0;
        }

        let diff = distance.saturating_sub(record);

        // the `4*a*c` part of the quadratic formula
        let approx = (-4.0 * -1.0 * diff as f64).sqrt().floor() as usize;

        let distance = self.calculate_distance(speed + (approx / 2));
        if distance == record {
            return approx - 1;
        }

        // at this point the approx is usually within 1 of the correct answer
        todo!("figure out how to refine the approximation");
    }

    pub fn count_winners_fast(&self) -> usize {
        let mut count = 0;
        let mut time = self.time.0;
        let mut record = self.record.0;

        let mut midpoint = time / 2;

        // offset if time is odd
        let start = midpoint + (time % 2);

        let mut count: usize = 0;

        for charge in start..time {
            let speed = charge;
            let duration = (time - charge);
            let distance = duration * speed;
            if distance <= record {
                break;
            }
            count += 1;
        }

        let adjustment = if time % 2 == 0 { 1 } else { 0 };
        (count * 2).saturating_sub(adjustment)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn part1_example() {
        let result = part1(&EXAMPLE);
        assert_eq!(result, 288);
    }

    #[test]
    fn part1_real() {
        let result = part1(&REAL);
        assert_eq!(result, 4811940);
    }

    #[test]
    fn part2_example() {
        let result = part2(&EXAMPLE_PART2);
        assert_eq!(result, 71503);
    }

    #[test]
    fn part2_real() {
        let result = part2(&REAL_PART2);
        assert_eq!(result, 30077773);
    }

    // proptest! {
    //     #[test]
    //     fn race_count_record_beaters(time in 7..30, record in 9..200) {
    //         let race = Race::new(Time(time as usize), Record(record as usize));
    //         assert_eq!(
    //             race.count_winners(),
    //             race.count_winners_oracle(),
    //         )
    //     }
    // }

    // #[test]
    // fn race_count_record_beaters_oracle() {
    //     let race = Race::new(Time(7), Record(9));
    //     let winners = race.count_winners_oracle();
    //     assert_eq!(winners, 4);

    //     let race = Race::new(Time(15), Record(40));
    //     let winners = race.count_winners_oracle();
    //     assert_eq!(winners, 8);

    //     let race = Race::new(Time(30), Record(200));
    //     let winners = race.count_winners_oracle();
    //     assert_eq!(winners, 9);
    // }
}
