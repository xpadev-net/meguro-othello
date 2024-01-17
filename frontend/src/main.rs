use std::{ops::Div, collections::btree_set::Union, };

use zoon::{format, named_color::*, *, dominator::{animation::Percentage, window_size}, web_sys::HtmlScriptElement};

// @TODO finish

type X = u32;
type Y = u32;

#[derive(Debug, Clone)]
struct Field {
    kind: FieldKind,
}

impl Field {
    fn new_empty(mines: u16) -> Self {
        Field {
            kind: FieldKind::Empty { mines }
        }
    }
}

    

#[derive(Debug, Clone, Copy)]
enum FieldKind {
    Mine,
    Empty { mines: u16 },
}



#[static_ref]
fn fields() -> &'static MutableVec<MutableVec<Field>> {
    MutableVec::new_with_values(hardcoded_fields())
}

fn hardcoded_fields() -> Vec<MutableVec<Field>> {
    vec![
        MutableVec::new_with_values(vec![
            Field::new_empty(1),
            Field::new_empty(1),
            Field::new_empty(1),
            Field::new_empty(1),
            Field::new_empty(1),
            Field::new_empty(1),
            Field::new_empty(1),
            Field::new_empty(1),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
        ]),
        MutableVec::new_with_values(vec![
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
            Field::new_empty(0),
        ]),
    ]
}

// fn flagged_count() -> impl Signal<Item = usize> {
//     fields()
//         .signal_vec_cloned()
//         .map_signal(|fields| {
//             fields
//                 .signal_vec_cloned()
//                 .filter_signal_cloned(|field| {
//                     field
//                         .state
//                         .signal_ref(|state| matches!(state, FieldState::Flagged))
//                 })
//                 .len()
//         })
//         .sum()
// }





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
                    // .item(El::new().child(format!("[{x}, {y}]")))
                    // .item(
                    //     El::new()
                    //         .child_signal(field.state.signal_ref(|state| format!("{state:#?}"))),
                    // )
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


fn stone(x: X, y: Y, field: Field) -> impl Element {
    El::new()
        .s(Align::center())
        .s(Width::exact(80))
        .s(Height::exact(80))
        .s(Background::new().color(hsluv!(0, 0, 0)))
        .s(RoundedCorners::all(100))

}


fn main() {
    start_app("app", root);
}