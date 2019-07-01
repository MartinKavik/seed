#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

#[derive(Default)]
struct Model {
    // it's reliable if you don't use variables with interior mutability (Cell, RefCell) in State
    undo_stack: Vec<State>,
    redo_stack: Vec<State>,
    state: State,
}

#[derive(Default, Clone, Debug)]
struct State {
    text: String,
}

// Update

#[derive(Clone)]
enum Msg {
    Undo,
    Redo,
    UpdateText(String),
}

fn update_state(model: &mut Model, f: impl FnOnce(&mut State)) {
    model.undo_stack.push(model.state.clone());
    model.redo_stack.clear();
    f(&mut model.state);
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Undo => {
            if let Some(state) = model.undo_stack.pop() {
                model.redo_stack.push(model.state.clone());
                model.state = state;
            }
        }
        Msg::Redo => {
            if let Some(state) = model.redo_stack.pop() {
                model.undo_stack.push(model.state.clone());
                model.state = state;
            }
        }
        Msg::UpdateText(text) => {
            update_state(model, |s| s.text = text);
        }
    }
    log!("_____________");
    log!("undo_stack: ", model.undo_stack);
    log!("redo_stack: ", model.redo_stack);
    log!("_____________");
}

// View

fn view(model: &Model) -> impl ElContainer<Msg> {
    vec![
        button![
            style! { "width" => px(120) },
            "Undo",
            if model.undo_stack.is_empty() {
                " (empty)"
            } else {
                ""
            },
            simple_ev(Ev::Click, Msg::Undo),
        ],
        input![
            model.state.text,
            input_ev(Ev::Input, Msg::UpdateText),
            attrs! { At::Value => model.state.text }
        ],
        button![
            style! { "width" => px(120) },
            "Redo",
            if model.redo_stack.is_empty() {
                " (empty)"
            } else {
                ""
            },
            simple_ev(Ev::Click, Msg::Redo),
        ],
    ]
}

#[wasm_bindgen]
pub fn start() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();
}
