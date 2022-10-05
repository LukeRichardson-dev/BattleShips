use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Commands {
    Sync,
    Strike {
        x: u32,
        y: u32,
        user: u8,
    },
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GameState {

    turn: u8,

}

impl GameState {
    
    

}