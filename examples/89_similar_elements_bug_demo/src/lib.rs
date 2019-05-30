#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

#[derive(Default)]
struct Model {
    show_first: bool
}

// Update

#[derive(Clone)]
enum Msg {
    ToggleElement,
}

fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
    match msg {
        Msg::ToggleElement => model.show_first = !model.show_first,
    }
}

// View

use wasm_bindgen::JsCast;

fn view(model: &Model) -> impl ElContainer<Msg> {
    vec![
        if model.show_first {
            button!(attrs!{At::Class => "button is-static"},
                i!(attrs!{At::Class => "fas fa-search"}),
            )
        } else {
            button!(attrs!{At::Class => "button is-static"},
                i!(attrs!{At::Class => "fas fa-filter"}),
            )
        },
        button![
            "Toggle element",
            simple_ev(Ev::Click, Msg::ToggleElement)
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .finish()
        .run();
}
