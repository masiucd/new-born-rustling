#[derive(Debug)]
enum FootBallPosition {
    Goalie,
    Defender,
    Midfielder,
    Attacker,
}

#[derive(Debug)]
struct FootBallPlayer {
    name: String,
    number: u8,
    position: FootBallPosition,
}

impl FootBallPlayer {
    fn new(name: String, number: u8, position: FootBallPosition) -> FootBallPlayer {
        FootBallPlayer {
            name,
            number,
            position,
        }
    }

    fn score(&self, time_left_remain: u16) {
        if time_left_remain < 5 {
            println!("miss")
        } else {
            match self.position {
                FootBallPosition::Midfielder => println!(
                    "{} scored and only {:?} can score",
                    self.name, self.position
                ),
                _ => println!("miss"),
            }
        }
    }
}

#[derive(Debug)]
struct Triangle(u32, u32, u32);

fn main() {
    let buffon = FootBallPlayer {
        name: String::from("Buffon"),
        number: 1,
        position: FootBallPosition::Goalie,
    };

    let ronaldo = FootBallPlayer {
        name: String::from("Ronaldo"),
        number: 7,
        position: FootBallPosition::Midfielder,
    };

    let zlatan = FootBallPlayer::new(String::from("Zlatan"), 11, FootBallPosition::Attacker);

    let football_players = vec![&buffon, &ronaldo];

    for player in football_players {
        println!(
            "players name {}, plays with number {} and is an {:?}",
            player.name, player.number, player.position
        );
    }

    let check_has_scored = &ronaldo.score(50);
    let check_has_scored2 = &buffon.score(50);
}
