use std::io::Empty;

use othello::{create_board,  State};
use zoon::console::log;
use zoon::{format, named_color::*, *,  };
mod  othello;
mod connection;
use zoon::{named_color::*, *};
use crate::othello::{Board, Pos};
use crate::connection::{board, connection, search_room, send_board};

// @TODO finish

type X = u32;
type Y = u32;

/**
othelloの使い方
初期化
    let mut board = load_board(相手のjson,自分の石が黒か);
または
    let mut board = create_board(自分の石が黒か);

石の配置
    board.put(Pos{x,y}).unwrap();

盤面のダンプ
    board.dump() -> String
 */




fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::both(20))
        .s(Height::screen())
        .s(Width::percent(100))
        .s(Background::new().color(hsluv!(360, 100, 100)))
        .item(grid())
        .item(reset_button())
        .item(test_button())
}


fn reset_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::new().center_x())
        .s(Padding::new().x(20).y(10))
        .s(RoundedCorners::all(10))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| RED_5, || RED_6)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("Reset")
        .on_click(||{
            //
        })

}

fn test_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::new().center_x())
        .s(Padding::new().x(20).y(10))
        .s(RoundedCorners::all(10))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| RED_5, || RED_6)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("test")
        .on_click(|| {
            search_room();
        })

}



fn grid() -> impl Element {
    let spacing = || Gap::both(0);
    Column::new()  
        .s(Align::center())
        .s(spacing())
        .s(Height::exact(800))
        .s(Width::exact(800)) 
        .items(
            board().lock_mut().get_data().clone()
            .iter()
            .enumerate()
                .map(move |(y, row)| {
                    Row::new()  
                        .s(spacing())
                        .items(
                            row.clone().iter().enumerate()
                                .map(move |(x,col)| {
                                    field_button(
                                        x.try_into().unwrap(),
                                        y.try_into().unwrap(),
                                        col.clone(),
                                    )
                                }),
                        )
                }),
        )
}

fn field_button(x: X, y: Y, field: Mutable<State>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::center())
        .s(Padding::all(10))
        .s(Height::exact(100))
        .s(Width::exact(100))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| hsluv!(130, 100, 60), || hsluv!(130, 100, 53))))
        .s(Borders::all(Border::new().color(hsluv!(0, 0, 0)).width(2)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label(
            El::new().s(Height::fill()).child(
                Column::new()
                    .item(stone(x, y, field))
                    
            ),
        )
        .on_click(move|| {
            let i =board().lock_mut().put(Pos { x: x.try_into().unwrap(),  y: y.try_into().unwrap() });
            if i.is_ok() {
                send_board();
            }
            log(&*board().lock_mut().dump());
            log(&format!("x: {}, y: {}", x, y));
        })
        // @TODO refactor together with event handler API redesign
        .update_raw_el(|raw_el| {
            raw_el
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    move |event: events::ContextMenu| {
                        event.prevent_default();
                    },
                )
        })
}


fn stone(_x: X, _y: Y, state: Mutable<State>) -> impl Element {
    El::new()
        .s(Align::center())
        .s(Width::exact(80))
        .s(Height::exact(80))
        .s(Background::new().color_signal(state.signal().map(|state|{
            match state {
                State::Empty  => hsluv!(0, 0, 0, 0),
                State::Black => hsluv!(0, 0, 0),
                _ => hsluv!(0, 0, 100)
            }
        })))
        .s(RoundedCorners::all(100))
        
}


fn main() {
    start_app("app", root);
}