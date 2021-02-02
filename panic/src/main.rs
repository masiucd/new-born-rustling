enum MoodState {
    Happy,
    Sad,
}

enum MoodAction {
    Smile,
    Cry,
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

fn main() {
    take_action(MoodState::Happy, MoodAction::Smile);
    take_action(MoodState::Sad, MoodAction::Smile);
}
