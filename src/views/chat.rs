use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum UserStatus {
    Online,
    AwayFromKeyboard,
    DoNotDisturb,
    Offline,
}
