

use othello::{create_board, load_board};
use zoon::{format, named_color::*, *,  };

// @TODO finish

type X = u32;
type Y = u32;

#[derive(Debug, Clone)]
struct Field {
    kind: State,
}

impl Field {
    fn new_empty(state:State) -> Self {
        Field {
            kind:state,
        }
    }
}


#[derive(Debug, Clone, Copy)]
enum State {
    Empty,
    Black,
    White,
}


mod othello;

use zoon::{named_color::*, *};
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

#[static_ref]
fn fields() -> &'static MutableVec<MutableVec<Field>> {
    MutableVec::new_with_values(hardcoded_fields())
}


//create_boardをする前は、fieldはすべてEmpty
fn hardcoded_fields() -> Vec<MutableVec<Field>> {
    let row = MutableVec::new_with_values(vec![
        Field::new_empty(State::Empty); 8 
    ]);
    vec![row; 8] 
}


fn judge_board() {
    if is_fields_empty(fields().lock_ref().to_vec()) {
        create_board(true);
    }else {
        // load_board(hardcoded_fields(), true);
    }
}

fn is_fields_empty(fields: Vec<MutableVec<Field>>) -> bool {
    fields.into_iter().all(|row| {
        row.lock_ref().iter().all(|field| matches!(field.kind, State::Empty))
    })
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::both(20))
        .s(Height::screen())
        .s(Width::percent(100))
        .s(Background::new().color(hsluv!(360, 100, 100)))
        .item(grid())
        .item(reset_button())
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
        .on_press(|| fields().lock_mut().replace_cloned(hardcoded_fields()))
}

fn grid() -> impl Element {
    let spacing = || Gap::both(0);
    Column::new()  
        .s(Align::center())
        .s(spacing())
        .s(Height::exact(800))
        .s(Width::exact(800)) 
        .items_signal_vec(
            fields()
                .signal_vec_cloned()
                .enumerate()
                .map(move |(y, fields)| {
                    Row::new()  
                        .s(spacing())
                        .items_signal_vec(
                            fields
                                .signal_vec_cloned()
                                .enumerate()
                                .map(move |(x, field)| {
                                    field_button(
                                        x.get().unwrap_throw() as X,
                                        y.get().unwrap_throw() as Y,
                                        field,
                                    )
                                }),
                        )
                }),
        )
}

fn field_button(x: X, y: Y, field: Field) -> impl Element {
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


fn stone(_x: X, _y: Y, field: Field) -> impl Element {
    El::new()
        .s(Align::center())
        .s(Width::exact(80))
        .s(Height::exact(80))
        .s(Background::new().color(match field.kind {
            State::Empty  => hsluv!(0, 0, 0, 0),
            State::Black => hsluv!(0, 0, 0),
            _ => hsluv!(0, 0, 100)
        }))
        .s(RoundedCorners::all(100))
        
}


fn main() {
    start_app("app", root);
}