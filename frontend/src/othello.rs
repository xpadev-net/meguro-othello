use serde::{Serialize, Deserialize};
use serde_json;
use zoon::console::log;
use crate::othello::State::{Black, Empty, White};

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
    data: [[State;10];10]
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
    fn update(&mut self, x: i8, y: i8,is_black: bool){
        self.data[y as usize][x as usize] = if is_black { Black } else { White };
    }

    /**
    座標からマスの状態を取得する
    */
    fn get(&self, x: i8, y: i8) -> State{
        self.data[y as usize][x as usize]
    }

    /**
    石を置けるか確認する
    */
    fn is_placeable(&self, current_x:i8, current_y: i8, is_black: bool) -> bool{
        let current = self.get(current_x, current_y);
        if current != Empty {
            return false;
        }
        for offset_x in -1..2 {
            for offset_y in -1..2{
                let mut x = current_x + offset_x;
                let mut y = current_y + offset_y;
                if (offset_x ==0&& offset_y ==0) || !is_in_board(x,y) {
                    continue;
                }
                let target = self.get(x,y);
                if target == Empty || (target == Black && is_black) || (target == White && !is_black) {
                    continue;
                }
                while is_in_board(x,y) {
                    let target = self.get(x,y);
                    if target == Empty {
                        break;
                    }
                    if (target == Black && !is_black) || (target == White && is_black) {
                        return true;
                    }
                    x += offset_x;
                    y += offset_y;
                }
            }
        }
        false
    }

    /**
    石を置くメゾットの内部処理
    置いた際のひっくり返す処理もここ
    */
    fn _put(&mut self, current_x:i8, current_y: i8, is_black: bool){
        let current = self.get(current_x, current_y);
        if current != Empty {
            return;
        }
        self.update(current_x,current_y,is_black);
        for offset_x in -1..2 {
            for offset_y in -1..2{
                let mut x = current_x + offset_x;
                let mut y = current_y + offset_y;
                if (offset_x ==0&& offset_y ==0) || !is_in_board(x,y) {
                    continue;
                }
                let target = self.get(x,y);
                if target == Empty || (target == Black && is_black) || (target == White && !is_black) {
                    continue;
                }
                while is_in_board(x,y) {
                    let target = self.get(x,y);
                    if target == Empty {
                        break;
                    }
                    log(&*format!("search,{},{}", x, y));
                    if (target == Black && is_black) || (target == White && !is_black) {
                        let mut _x = current_x + offset_x;
                        let mut _y = current_y + offset_y;
                        while is_in_board(_x,_y) {
                            log(&*format!("update, {},{}, {},{}", _x, _y,x,y));
                            self.update(_x,_y,is_black);
                            _x += offset_x;
                            _y += offset_y;
                            if _x == x {
                                break;
                            }
                        }
                        break;
                    }
                    x += offset_x;
                    y += offset_y;
                }
            }
        }
    }

    /**
    石を置くメゾット
    */
    pub fn put(&mut self, x: i8, y: i8, is_black: bool) -> Result<i8,String> {
        if !self.is_placeable(x,y,is_black) {
            return Err("slot is not placeable".to_owned());
        }
        self._put(x,y,is_black);
        Ok(0)
    }
}

/**
座標がボード上にあるか
*/
fn is_in_board(x: i8, y: i8) -> bool{
    0 <= x && x <= 9 && 0 <= y && y <= 9
}

/**
空のボードデータを作成
*/
pub fn create_board() -> Board {
    let mut data: [[State;10];10] = [[Empty;10];10];
    data[4][4] = White;
    data[5][5] = White;
    data[4][5] = Black;
    data[5][4] = Black;
    return Board{ data };
}

/**
受け取ったjsonからボードデータを読み込みstructを生成
*/
pub fn load_board(val: String) -> Board {
    let board = serde_json::from_str(&val).unwrap();
    return Board{data: board}
}