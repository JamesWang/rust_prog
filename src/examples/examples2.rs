#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum MotorInput {
    Up { voltage: f64 },
    Down { voltage: f64 },
}

pub trait MotorForce {
    fn calculate_force(&self) -> f64;
}

impl MotorForce for MotorInput {
    fn calculate_force(&self) -> f64 {
        match *self {
            MotorInput::Up { voltage: v } => v * 8.0,
            MotorInput::Down { voltage: v } => v * -8.0,
        }
    }
}

pub trait MotorVoltage {
    fn voltage(&self) -> f64;
}

impl MotorVoltage for MotorInput {
    fn voltage(&self) -> f64 {
        match *self {
            MotorInput::Up { voltage: v } => v,
            MotorInput::Down { voltage: v } => -v,
        }
    }
}
