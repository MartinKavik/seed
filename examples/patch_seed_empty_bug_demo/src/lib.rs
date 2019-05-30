#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

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

fn view(model: &Model) -> impl ElContainer<Msg> {
    // it has to be vec! and in root to reproduce bug
    vec![
        if model.show_first {
            // it works with div![] or span![], etc
            seed::empty()
        } else {
            div!["Something"]
        },
        button![
            "Toggle element",
            simple_ev(Ev::Click, Msg::ToggleElement)
        ]
    ]
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model { show_first: true }, update, view)
        .finish()
        .run();
}
