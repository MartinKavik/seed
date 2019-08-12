//! A simple, cliché example demonstrating structure and syntax.

#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

#[derive(Default)]
struct Model {
    text: String,
}

// Update

#[derive(Debug, Clone)]
enum Msg {
    TextChanged(String),
    InputMounted,
}

/// The sole source of updating the model
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TextChanged(text) => model.text = text,
        Msg::InputMounted => {
            log!("Mounted!");
        },
    }
}

// View

fn view(model: &Model) -> impl View<Msg> {
    let mut did_mount_data = did_mount(|_| ());
    did_mount_data.message = Some(Msg::InputMounted);

    div![
        input![
            attrs! {At::Value => model.text},
            input_ev(Ev::Input, Msg::TextChanged),
            did_mount_data
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();
}
