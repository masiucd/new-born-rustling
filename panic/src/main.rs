enum MoodState {
    Happy,
    Sad,
}

enum MoodAction {
    Smile,
    Cry,
}

struct TweetRecord {
    id: i64,
    tweet: String,
    created_at: std::time::Instant,
}

fn take_action(current_state: MoodState, action: MoodAction) {
    match (current_state, action) {
        (MoodState::Happy, MoodAction::Smile) => {
            println!("I am happy and I smile")
        }
        (MoodState::Sad, MoodAction::Cry) => {
            println!("I am sad and I am crying")
        }
        _ => unreachable!(),
    }
}

fn save_tweet(tweet: &str) -> Result<i64, &'static str> {
    if tweet.len() > 200 {
        return Err("tweet is to long");
    }
    let record = save_to_db(tweet)?;

    Ok(record.id)
}

fn save_to_db(text: &str) -> Result<TweetRecord, &'static str> {
    Err("db unavailable")
}

fn add_ten(n: &str) -> Result<i32, std::num::ParseIntError> {
    let new_num: i32 = n.parse()?;
    Ok(new_num + 10)
}

fn double(n: &str) -> Result<i32, std::num::ParseIntError> {
    let res: i32 = n.parse()?;
    Ok(res * 2)
}
// unwrap our value from Some(n)
fn main() {
    let x = "10";
    let res = double(&x);
    println!("{:?}", res);
    let res = match res {
        Ok(num) => num,
        Err(err) => panic!(err),
    };
    println!("{:?}", res);

    // take_action(MoodState::Happy, MoodAction::Smile);
    // take_action(MoodState::Sad, MoodAction::Smile);
}
