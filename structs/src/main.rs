#[path = "./math/math.rs"]
mod math;
use math::math::arithmetic;
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

impl FootBallPlayer {
    fn new(name: String, number: u8, position: FootBallPlayerPosition, goals_scored: u8) -> Self {
        FootBallPlayer {
            name,
            number,
            position,
            goals_scored,
        }
    }
    fn show_player(&self) {
        if self.name == "Mio" {
            println!("Yooo I am {}", self.name);
        } else {
            println!(
                "hello my name is {} and I am a {:?} ",
                self.name, self.position
            );
        }
    }
}

#[derive(Debug)]
struct Triangle(u32, u32, u32);

fn is_all_side(t: &Triangle) -> bool {
    t.0 == t.1 && t.1 == t.2
}

fn calc_area(shape: (u8, u8)) -> u8 {
    shape.0 * shape.1
}

fn main() {
    let mut buffon = FootBallPlayer::new(
        String::from("Buffon"),
        1,
        FootBallPlayerPosition::GaolKeeper,
        0,
    );
    let son_of_buffon = FootBallPlayer {
        name: String::from("Mio"),
        position: FootBallPlayerPosition::Midfielder,
        ..buffon
    };

    son_of_buffon.show_player();

    // let triangle = Triangle(33, 23, 32);
    // let shape = (10, 20);

    // let shape_area = calc_area(shape);

    // let res = is_all_side(&triangle);
    // if res == true {
    //     println!("yeaah");
    // } else {
    //     println!("Booo");
    // }

    // println!("shape area is {}", shape_area);

    // println!(
    //     "{} is a {:?} and has scored {} goals,has number {}",
    //     buffon.name, buffon.position, buffon.goals_scored, buffon.number
    // );

    // println!("{:?}", triangle);

    // let twenty_two = arithmetic::add(10, 12);
    // println!("calculating through a module {}", twenty_two);
}
