const NUM_MOTOR: usize = 4;

enum GlobalState {
    Drive {vel: i32, angle: i32},
    CrabWalk {vel: i32, angle: i32},
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState::Drive{vel: 0, angle: 0}
    }
}

#[derive(Default)]
struct MotorState {
    vel: i32,
    angle: i32,
}


impl GlobalState {
    fn update(&mut self, motors: &mut [MotorState; NUM_MOTOR], magnitude: i32, angle: i32, swap_mode: bool) {
        if swap_mode {
            let new_state = match self {
                GlobalState::Drive{vel: _, angle: _} => {
                    GlobalState::CrabWalk{vel: magnitude, angle: angle}
                },
                GlobalState::CrabWalk{vel: _, angle: _} => {
                    GlobalState::Drive{vel: magnitude, angle: angle}
                },
            };
            *self = new_state;
        } else {
            *self = match self {
                GlobalState::Drive{vel: _, angle: _} => {
                    GlobalState::Drive{vel: magnitude, angle: angle}
                },
                GlobalState::CrabWalk{vel: _, angle: _} => {
                    GlobalState::CrabWalk{vel: magnitude, angle: angle}
                },
            };
            match self {
                GlobalState::Drive{vel: _, angle: _} => {
                    for i in 0..2 {
                        motors[i] = MotorState{vel: magnitude, angle: angle};
                    }
                    for i in 2..4 {
                        motors[i] = MotorState{vel: magnitude, angle: 0};
                    }
                }
                GlobalState::CrabWalk{vel: _, angle:_} => {
                    for i in 0..NUM_MOTOR {
                        motors[i] = MotorState{vel: magnitude, angle: angle};
                    }
                }
            }
        }
    }
}


fn main() {

}
