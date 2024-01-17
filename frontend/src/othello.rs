use serde::{Serialize, Deserialize};

enum State {
    Empty,
    Black,
    White
}

#[derive(Serialize, Deserialize, Debug)]
struct Board{
    data: [[State;8];8]
}

impl Board {
    pub fn dump(&self) -> String {
        return output_json(**self).unwrap();
    }
    pub fn load(&self, val: String) -> Board {
        let board = serde_json::from_str(&val).unwrap();
        return Board{data: board}
    }
}

fn output_json(board: Board) -> std::io::Result<(String)> {
    let serialized: String = serde_json::to_string(&board).unwrap();

    Ok(serialized)
}

pub fn create_board() -> Board {
    let data: [[State;8];8] = [[State::Empty;8];8];
    return Board{data };
}