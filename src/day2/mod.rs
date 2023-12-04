pub fn part1() {
    let input = include_str!("../../inputs/real/day2.txt");

    let target_round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut total: u32 = 0;
    for line in input.lines() {
        let game = Game::parse(line).expect("invalid game");
        let max = game.max_possible_pulls();
        let possible = game.is_game_possible(&target_round);

        if possible {
            total += game.game_id.0 as u32;
            println!("new total: {total}");
        }

        println!("game: {game:?}");
        println!("max: {max:?}");
        println!("possible: {possible}");
    }
    println!("total: {total}");
}

pub fn part2() {
    let input = include_str!("../../inputs/real/day2.txt");

    let mut total: u32 = 0;
    for line in input.lines() {
        let game = Game::parse(line).expect("invalid game");
        let max = game.max_possible_pulls();
        let power = game.power();

        println!("game: {game:?}");
        println!("max: {max:?}");
        println!("power: {power}");

        total += power;
    }
    println!("total: {total}");
}

#[derive(Debug)]
struct Game {
    game_id: GameId,
    rounds: Vec<Round>,
}

impl Game {
    fn parse(s: &str) -> Result<Self, &str> {
        let parts = s.split_once(':');
        if let None = parts {
            return Err(s);
        }
        let (game, roundlist) = parts.expect("None case handled above");

        let game_id = GameId::parse(game)?;
        let rounds = roundlist
            .split(';')
            .map(|s| Round::parse(s))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { game_id, rounds })
    }

    fn power(&self) -> u32 {
        let max = self.max_possible_pulls();
        max.red as u32 * max.green as u32 * max.blue as u32
    }

    fn max_possible_pulls(&self) -> Round {
        self.rounds
            .iter()
            .fold(Round::empty(), |acc, round| acc.pick_max(round))
    }

    fn is_game_possible(&self, target_round: &Round) -> bool {
        let max = self.max_possible_pulls();
        target_round.other_round_fits(&max)
    }
}

#[derive(Debug)]
struct GameId(u8);

impl GameId {
    fn parse(s: &str) -> Result<Self, &str> {
        match s.strip_prefix("Game ") {
            None => Err(s),
            Some(id) => {
                let id = id.parse().map_err(|_| s)?;
                Ok(GameId(id))
            }
        }
    }
}

#[derive(Debug)]
struct Round {
    red: u8,
    green: u8,
    blue: u8,
}

impl Round {
    fn pick_max(&self, other: &Round) -> Round {
        let red = self.red.max(other.red);
        let green = self.green.max(other.green);
        let blue = self.blue.max(other.blue);
        Round { red, green, blue }
    }

    fn other_round_fits(&self, other: &Round) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn empty() -> Round {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn parse(s: &str) -> Result<Self, &str> {
        let pulls = s
            .split(',')
            .map(|s| Pull::parse(s))
            .collect::<Result<Vec<_>, _>>()?;

        let mut green = 0;
        let mut blue = 0;
        let mut red = 0;

        for pull in pulls {
            match pull {
                Pull::Red(n) => red += n,
                Pull::Green(n) => green += n,
                Pull::Blue(n) => blue += n,
            }
        }

        Ok(Round { red, green, blue })
    }
}

#[derive(Debug)]
enum Pull {
    Red(u8),
    Green(u8),
    Blue(u8),
}

impl Pull {
    fn parse(s: &str) -> Result<Self, &str> {
        let parts = s.trim().split_once(' ');
        if let None = parts {
            return Err(s);
        }
        let (n, color) = parts.unwrap();

        let n = n.parse().map_err(|_| s)?;
        let pull = match color {
            "red" => Pull::Red(n),
            "green" => Pull::Green(n),
            "blue" => Pull::Blue(n),
            _ => return Err(s),
        };
        Ok(pull)
    }
}
