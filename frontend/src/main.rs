use std::fmt::format;

use yew::prelude::*;
use zoon::{named_color::*, *};

// #[static_ref]
// fn counter() -> &'static Mutable<u32> {
//     Mutable::new(0)
// }

// fn increment() {
//     counter().update(|counter| counter + 1)
// }

// fn root() -> impl Element {
//     Row::new()
//         .s(Align::center())
//         .s(Transform::new().move_up(20))
//         .s(Gap::both(20))
//         .s(Font::new().color(GRAY_0).size(30))
//         .item(increment_button())
//         .item_signal(counter().signal())
// }

// fn increment_button() -> impl Element {
//     let (hovered, hovered_signal) = Mutable::new_and_signal(false);
//     Button::new()
//         .s(Padding::new().x(12).y(7))
//         .s(RoundedCorners::all(10))
//         .s(Background::new().color_signal(hovered_signal.map_bool(|| GREEN_7, || GREEN_8)))
//         .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
//         .label("Arttır")
//         .on_press(increment)
// }
#[derive(Clone, Debug)]
struct Field {
    mine: FieldMine,
    state: Mutable<FieldState>,
}
#[derive(Clone, Debug)]
enum FieldMine {
    Mine,
    Empty(u8),
}
#[derive(Clone, Debug)]
enum FieldState {
    Covered,
    Uncovered,
    Flagged,
}
impl Default for Field {
    fn default() -> Self {
        Self {
            mine: FieldMine::Empty(0),
            state: Mutable::new(FieldState::Covered),
        }
    }
}

#[static_ref]
fn fields() -> &'static MutableVec<MutableVec<Field>> {
    let mut v_fields = vec![];
    for _ in 0..8 {
        let mut fields = vec![];
        for _ in 0..8 {
            fields.push(Field::default())
        }
        v_fields.push(MutableVec::new_with_values(fields))
    }
    MutableVec::new_with_values(v_fields)
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::new().y(2))
        .items_signal_vec(fields().signal_vec_cloned().map(|fields| grid(fields)))
        .item(reset_button())
}

fn reset_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(5))
        //.s(Background::new().color(RED_6))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| RED_1, || RED_9)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label("Reset")
}
fn grid(fields: MutableVec<Field>) -> impl Element {
    Row::new()
        .s(Gap::new().x(2))
        .items_signal_vec(fields.signal_vec_cloned().map(|field| mine(field)))
}
fn mine(field: Field) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_3, || GRAY_6)))
        .s(RoundedCorners::all(3))
        .s(Width::exact(50))
        .s(Height::exact(50))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label_signal(field_label(&field).signal_ref(|label| format!("{label}")))
        .update_raw_el(|raw_el| {
            raw_el
                .event_handler(move |event: events::MouseDown| match event.button() {
                    events::MouseButton::Left => (),
                    events::MouseButton::Right => decrease_state(&field),
                    _ => (),
                })
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    move |event: events::ContextMenu| {
                        event.prevent_default();
                    },
                )
        })
}
#[static_ref]
fn field_label(field: &Field) -> &'static Mutable<String> {
    let state = field.state.lock_ref();
    match *state {
        FieldState::Covered => Mutable::new("C".to_string()),
        _ => Mutable::new("A".to_string()),
    }
}
fn decrease_state(field: &Field) {
    let mut state = field.state.lock_mut();
    match *state {
        FieldState::Covered => *state = FieldState::Flagged,
        _ => (),
    }
}

fn main() {
    start_app("app", root);
}
