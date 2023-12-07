use aoc2023::day6::Race;
use aoc2023::day6::Record;
use aoc2023::day6::Time;
use aoc2023::util;

fn main() {
    let time = Time(7950000);
    let record = Record(9400200000000);
    let race = Race::new(time, record);

    let time = time.0;
    let record = record.0;
    let speed = time / 2;
    let remaining = time - speed;
    let distance = speed * remaining;

    let diff = distance.saturating_sub(record);

    let low = (-4.0 * -1.0 * diff as f64).sqrt().floor() as u64;
    let high = low + 1;

    println!("{low} or {high}");

    if low * (low + 1) < diff as u64 {
        println!("low");
    } else {
        println!("high");
    }

    let result = race.count_record_beaters();
    println!("{:?}", result);
}
