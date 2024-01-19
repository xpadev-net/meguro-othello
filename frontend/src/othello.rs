use serde::{Serialize, Deserialize};
use serde_json;
use zoon::console::log;
use crate::othello::State::{Black, Empty, White};

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    x: i8,
    y: i8
}

impl Pos {
    fn apply_offset(&mut self, x: i8, y:i8){
        self.x += x;
        self.y += y;
    }
    fn new_offset(&self, x: i8, y:i8) -> Pos{
        Pos{ x: self.x + x, y: self.y + y}
    }
}

/**
マス目の状態のenum
*/
#[derive(Copy, Clone,Serialize,Deserialize, Debug,Eq, PartialEq)]
pub enum State {
    Empty,
    Black,
    White,
}

/**
盤面データのstruct
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Board{
    data: [[State;8];8],
    is_black: bool
}

impl Board {
    /**
    盤面データを転送する用
    */
    pub fn dump(&self) -> String {
        return serde_json::to_string(&self.data).unwrap();
    }

    /**
    座標からマスの状態を更新する
    */
    fn update(&mut self, pos: Pos){
        self.data[pos.y as usize][pos.x as usize] = if self.is_black { Black } else { White };
    }

    /**
    座標からマスの状態を取得する
    */
    fn get(&self, pos: Pos) -> State{
        self.data[pos.y as usize][pos.x as usize]
    }

    /**
    石を置けるか確認する
    */
    fn is_placeable(&self, current_pos: Pos) -> bool{
        let current = self.get(current_pos);
        if current != Empty {
            return false;
        }
        for offset_x in -1..2 {
            for offset_y in -1..2{
                let mut pos = current_pos.new_offset(offset_x,offset_y);
                if (offset_x ==0&& offset_y ==0) || !is_in_board(pos) {
                    continue;
                }
                let target = self.get(pos);
                if target == Empty || (target == Black && self.is_black) || (target == White && !self.is_black) {
                    continue;
                }
                while is_in_board(pos) {
                    let target = self.get(pos);
                    if target == Empty {
                        break;
                    }
                    if (target == Black && self.is_black) || (target == White && !self.is_black) {
                        return true;
                    }
                    pos.apply_offset(offset_x,offset_y);
                }
            }
        }
        false
    }

    /**
    石を置くメゾットの内部処理
    置いた際のひっくり返す処理もここ
    */
    fn _put(&mut self, current_pos: Pos){
        let current = self.get(current_pos);
        if current != Empty {
            return;
        }
        self.update(current_pos);
        for offset_x in -1..2 {
            for offset_y in -1..2{
                let mut pos = current_pos.new_offset(offset_x,offset_y);
                if (offset_x ==0&& offset_y ==0) || !is_in_board(pos) {
                    continue;
                }
                let target = self.get(pos);
                if target == Empty || (target == Black && self.is_black) || (target == White && !self.is_black) {
                    continue;
                }
                while is_in_board(pos) {
                    let target = self.get(pos);
                    if target == Empty {
                        break;
                    }
                    log(&format!("search,{:?}", pos));
                    if (target == Black && self.is_black) || (target == White && !self.is_black) {
                        let mut _pos = pos.new_offset(offset_x,offset_y);
                        while is_in_board(_pos) {
                            self.update(_pos);
                            _pos.apply_offset(offset_x,offset_y);
                            if _pos.x == pos.x {
                                break;
                            }
                        }
                        break;
                    }
                    pos.apply_offset(offset_x,offset_y);
                }
            }
        }
    }

    /**
    石を置くメゾット
    */
    pub fn put(&mut self, pos: Pos) -> Result<i8,String> {
        if !self.is_placeable(pos) {
            return Err("slot is not placeable".to_owned());
        }
        self._put(pos);
        Ok(0)
    }

    pub fn get_placeable_pos(&self) -> Vec<Pos> {
        let mut result: Vec<Pos> = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                let pos = Pos{x,y};
                if !self.is_placeable(pos) {
                    continue;
                }
                result.push(pos);
            }
        }
        result
    }
}

/**
座標がボード上にあるか
*/
fn is_in_board(pos: Pos) -> bool{
    0 <= pos.x && pos.x <= 9 && 0 <= pos.y && pos.y <= 9
}

/**
空のボードデータを作成
*/
pub fn create_board(is_black: bool) -> Board {
    println!("ボードを制作しました");
    let mut data: [[State;8];8] = [[Empty;8];8];
    data[3][3] = White;
    data[4][4] = White;
    data[3][4] = Black;
    data[4][3] = Black;
    return Board{ data, is_black };
}

/**
受け取ったjsonからボードデータを読み込みstructを生成
*/
pub fn load_board(val: String, is_black: bool) -> Board {
    let board = serde_json::from_str(&val).unwrap();
    return Board{data: board, is_black}
}