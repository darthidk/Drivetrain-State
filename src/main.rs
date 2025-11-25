#[derive(Default)]
enum GlobalState {
    #[default]
    Drive(vel: i32, angle: i32),
    CrabWalk(vel: i32, angle: i32),
}

impl GlobalState {
    fn update(&mut self, magnitude: i32, angle: i32, swap_mode: bool) {
        if (swap_mode) {
            self = match self {
                GlobalState::Drive => {
                    GlobalState::CrabWalk(magnitude, angle);
                },
                GlobalState::CrabWalk => {
                    GlobalState::Drive(magnitude, angle);
                },
        } else {
            self = match self {
                GlobalState::Drive => {
                    GlobalState::Drive(magnitude, angle);
                },
                GlobalState::CrabWalk => {
                    GlobalState::CrabWalk(magnitude, angle);
            }
        }
    }
}

fn main() {
}
