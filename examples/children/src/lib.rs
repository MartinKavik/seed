#![allow(clippy::redundant_closure)]

use seed::{dom_types::Style, prelude::*, *};

mod children;

// Model

#[derive(Default)]
struct Model {
    mutate_me: children::Data,
}

// Update

#[derive(Debug, Clone)]
enum Msg {
    WhenPressedDoThis(children::Data),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::WhenPressedDoThis(data) => {
            log!(data);
            model.mutate_me = data;
        }
    }
}

// View

fn view(model: &Model) -> impl View<Msg> {
    div![
        id!("app"),
        app_style(),
        pre![model.mutate_me,],
        children::view(|data| Msg::WhenPressedDoThis(data)),
        children::view(Msg::WhenPressedDoThis),
    ]
}

fn app_style() -> Style {
    style! {
        St::FontFamily => "Avenir, Helvetica, Arial, sans-serif",
        St::TextAlign => "center",
        St::Color => "#2c3e50",
        St::MarginTop => px(60),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
