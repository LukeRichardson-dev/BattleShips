use serde::Deserialize;


#[derive(Debug, Deserialize, Clone, Copy, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct GameState {

    turn: u8,

}