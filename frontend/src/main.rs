use std::fmt::format;

use zoon::{named_color::*, *};
use yew::prelude::*;

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

fn root() -> impl Element {
    Column::new().s(Align::center())
    .s(Gap::new().y(2))
    .items( (0..8).map(|y| grid(y)))
    .item(reset_button())
}

fn reset_button() -> impl Element {
    let(hovered , hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
    .s(Padding::all(5))
    //.s(Background::new().color(RED_6))
    .s(Background::new().color_signal(hovered_signal.map_bool(|| RED_1, || RED_9)))
    .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    .label("Reset")
}
fn grid(y:usize)-> impl Element{
    Row::new()
    .s(Gap::new().x(2))
    .items((0..8).map(|x| field(x,y)))

}
fn field(x: usize, y:usize)-> impl Element{
    let(hovered , hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
    .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_3, || GRAY_6)))
    .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    .s(RoundedCorners::all(3))
    .s(Width::exact(50))
    .s(Height::exact(50))
    .label(
        Label::new()
        .s(Align::center())
        .label(format!("{x} , {y}"))
    )

}

fn main() {
    start_app("app", root);
}
