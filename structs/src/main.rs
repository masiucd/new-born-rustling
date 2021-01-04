#[derive(Debug)]
enum FootBallPlayerPosition {
    GaolKeeper,
    Defender,
    Midfielder,
    Striker,
}

struct FootBallPlayer {
    name: String,
    number: u8,
    position: FootBallPlayerPosition,
    goals_scored: u8,
}

#[derive(Debug)]
struct Triangle(u32, u32, u32);

fn is_all_side(t: &Triangle) -> bool {
    t.0 == t.1 && t.1 == t.2
}

fn main() {
    let buffon = FootBallPlayer {
        name: String::from("Buffon"),
        number: 1,
        position: FootBallPlayerPosition::GaolKeeper,
        goals_scored: 0,
    };

    let triangele = Triangle(33, 23, 32);
    let res = is_all_side(&triangele);
    if res == true {
        println!("yeaah");
    } else {
        println!("Booo");
    }

    println!(
        "{} is a {:?} and has scored {} goals,has number {}",
        buffon.name, buffon.position, buffon.goals_scored, buffon.number
    );

    println!("{:?}", triangele);
}
