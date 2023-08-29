use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub enum UserStatus {
    Online,
    AwayFromKeyboard,
    DoNotDisturb,
    Offline,
}
