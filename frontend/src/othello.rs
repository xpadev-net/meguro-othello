use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Copy, Clone,Serialize,Deserialize, Debug)]
pub enum State {
    Empty,
    Black,
    White,
    Placeable,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board{
    data: [[State;10];10]
}

impl Board {
    pub fn dump(&self) -> String {
        return serde_json::to_string(&self.data).unwrap();
    }
}

pub fn create_board() -> Board {
    let data: [[State;10];10] = [[State::Empty;10];10];
    return Board{ data };
}

pub fn load_board(val: String) -> Board {
    let board = serde_json::from_str(&val).unwrap();
    return Board{data: board}
}