// SPDX-License-Identifier: MIT

mod color;
mod color_picker;

use color::Color;
use color_picker::color_picker;
use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

struct Model {
    color1: Color,
    color2: Color,
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(_: Url, _: &mut impl Orders<Msg>) -> AfterMount<Model> {
    AfterMount::new(Model {
        color1: Color {
            red: 50,
            green: 200,
            blue: 100,
        },
        color2: Color {
            red: 0,
            green: 255,
            blue: 255,
        },
    })
}

// ------ ------
//    Update
// ------ ------

#[derive(Debug, Clone, Copy)]
enum Msg {
    Color1Changed(Color),
    Color2Changed(Color),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Color1Changed(color) => model.color1 = color,
        Msg::Color2Changed(color) => model.color2 = color,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    div![
        h1![
            style! {St::Color => model.color1.to_css()},
            "Color Picker 1",
        ],
        color_picker(model.color1, Msg::Color1Changed),
        hr![],
        h1![
            style! {St::Color => model.color2.to_css()},
            "Color Picker 2",
        ],
        color_picker(model.color2, Msg::Color2Changed),
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view)
        .after_mount(after_mount)
        .build_and_start();
}
