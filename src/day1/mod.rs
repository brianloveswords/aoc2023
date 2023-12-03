fn parse_digit(s: &str) -> Option<char> {
    if s.contains("one") {
        return Some('1');
    }
    if s.contains("two") {
        return Some('2');
    }
    if s.contains("three") {
        return Some('3');
    }
    if s.contains("four") {
        return Some('4');
    }
    if s.contains("five") {
        return Some('5');
    }
    if s.contains("six") {
        return Some('6');
    }
    if s.contains("seven") {
        return Some('7');
    }
    if s.contains("eight") {
        return Some('8');
    }
    if s.contains("nine") {
        return Some('9');
    }
    None
}

fn parse(s: &str) -> Result<u32, &str> {
    let bytes = s.as_bytes();

    let mut first_digit = None;
    let mut offset = 0;

    let max = bytes.len();
    while offset < max {
        let c = char::from(bytes[offset]);

        if c.is_digit(10) {
            first_digit = Some(c);
            break;
        }

        if let Some(c) = parse_digit(&s[0..offset + 1]) {
            first_digit = Some(c);
            break;
        };

        offset += 1;
    }

    if offset == max {
        return Err(s);
    }

    let min = offset;

    let mut second_digit = None;

    let mut offset = max - 1;
    while offset > min {
        let c = char::from(bytes[offset]);

        if c.is_digit(10) {
            second_digit = Some(c);
            break;
        }

        if let Some(c) = parse_digit(&s[offset..max]) {
            second_digit = Some(c);
            break;
        };

        offset -= 1;
    }

    let first = first_digit.expect("must have first digit");
    let second = second_digit.unwrap_or(first);
    let value = format!("{}{}", first, second)
        .parse::<u32>()
        .expect("expected two digits");
    Ok(value)
}

pub fn part2() {
    const REAL_INPUT: &str = include_str!("../../inputs/real/day1.txt");

    let mut total = 0;

    for line in REAL_INPUT.lines() {
        let res = parse(line);
        println!("{line} -> {res:?}");

        match res {
            Ok(value) => total += value,
            Err(line) => panic!("failed to parse any digits: {line}"),
        }
    }

    println!("total: {}", total)
}
