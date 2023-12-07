#![allow(unused)]

pub fn part1(races: &[Race]) -> usize {
    races
        .iter()
        .map(|r| r.count_record_beaters_oracle())
        .product()
}

pub fn part2(races: &[Race]) -> usize {
    todo!("Implement part 2")
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

const REAL: [Race; 4] = [
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

#[derive(Debug)]
struct Time(usize);

#[derive(Debug)]
struct Record(usize);

#[derive(Debug)]
pub struct Race {
    time: Time,
    record: Record,
}

impl Race {
    fn new(time: Time, record: Record) -> Self {
        Self { time, record }
    }

    fn count_record_beaters_oracle(&self) -> usize {
        let mut count = 0;
        let mut time = self.time.0;
        let mut record = self.record.0;

        let mut count = 0;
        for charge in 1..time {
            let speed = charge;
            if (time - charge) * speed > record {
                count += 1;
            }
        }
        count
    }

    pub fn count_record_beaters(&self) -> usize {
        let mut count = 0;
        let mut time = self.time.0;
        let mut record = self.record.0;

        let mut count = 0;
        for charge in 1..time {
            let speed = charge;
            if (time - charge) * speed > record {
                count += 1;
            }
        }
        count
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
    proptest! {
        #[test]
        fn race_count_record_beaters(time in 7..30, record in 9..200) {
            let race = Race::new(Time(time as usize), Record(record as usize));
            assert_eq!(
                race.count_record_beaters_oracle(),
                race.count_record_beaters()
            )
        }
    }

    #[test]
    fn race_count_record_beaters_oracle() {
        let race = Race::new(Time(7), Record(9));
        let winners = race.count_record_beaters_oracle();
        assert_eq!(winners, 4);

        let race = Race::new(Time(15), Record(40));
        let winners = race.count_record_beaters_oracle();
        assert_eq!(winners, 8);

        let race = Race::new(Time(30), Record(200));
        let winners = race.count_record_beaters_oracle();
        assert_eq!(winners, 9);
    }
}
