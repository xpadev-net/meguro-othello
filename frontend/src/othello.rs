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
    fn dump(&self) -> String {
        return output_json().unwrap(); 
    }
    fn load(&self, val: String) {
        let board = serde_json::from_str(&val).unwrap();
        self.data = board;
    }
}

fn output_json(board: Board) -> std::io::Result<(String)> {
    let serialized: String = serde_json::to_string(&board).unwrap();

    Ok((serialized))
}