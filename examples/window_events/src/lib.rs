//! A simple, cliché example demonstrating structure and syntax.

#[macro_use]
extern crate seed;
use seed::{events::Listener, prelude::*};

// Model

#[derive(Clone, Default)]
struct Model {
    counter: i32,
}

// Update

#[derive(Clone)]
enum Msg {
    Increment,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.counter += 1,
    }
}

// View

fn view(model: &Model) -> Node<Msg> {
    div![
        model.counter.to_string()
    ]
}

fn window_events(model: &Model) -> Vec<Listener<Msg>> {
    vec![
        mouse_ev(Ev::Click, |_| Msg::Increment),
        mouse_ev(Ev::Click, |_| Msg::Increment),
        mouse_ev(Ev::Click, |_| Msg::Increment),
        mouse_ev(Ev::Click, |_| Msg::Increment),
        mouse_ev(Ev::Click, |_| Msg::Increment),
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Init::new(Model::default()), update, view)
        .window_events(window_events)
        .build_and_start();
}
