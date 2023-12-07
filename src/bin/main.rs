use aoc2023::day6::{Race, Record, Time};

fn main() {
    let race = Race::new(Time(2503), Record(21));
    let r1 = race.count_winners_constant();
    let r2 = race.count_winners_oracle();
    println!("{} == {}", r1, r2);
}
