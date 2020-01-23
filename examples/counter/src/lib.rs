//! A simple, cliché example demonstrating structure and syntax.
//! Inspired by [Elm example](https://guide.elm-lang.org/architecture/buttons.html).

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

mod state_manager;

use seed::{prelude::*, *};
use enclose::enc;
use state_manager::{StateManager, use_state};

// ------ ------
//     Model
// ------ ------

#[derive(Default)]
struct Model {
    sm: StateManager,
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(_: Url, orders: &mut impl Orders<Msg>) -> AfterMount<Model> {
    orders.after_next_render(|_| Msg::Rendered);
    AfterMount::default()
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    Rendered,
    NoOp,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered => {
            model.sm.reset_id();
            orders.after_next_render(|_| Msg::Rendered).skip();
        }
        Msg::NoOp => {}
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    vec![
        view_title(model),
        hr![],
        view_counter(model),
        hr![],
        view_counter(model),
    ]
}

fn view_title(model: &Model) -> Node<Msg> {
    let title = use_state(&model.sm, "my title".to_owned());
    div![
        h2![title.get()],
        input![
            attrs!{
                At::Value => title.get(),
            },
            input_ev(Ev::Input, move |text| { title.update(|t| *t = text); Msg::NoOp } )
        ],
    ]
}

fn view_counter(model: &Model) -> Node<Msg> {
    let count = use_state(&model.sm, 3);
    div![
        button![ev(Ev::Click, enc!((count) move |_| { count.update(|v| *v -= 1); Msg::NoOp } )), "-"],
        div![count.get().to_string()],
        button![ev(Ev::Click, enc!((count) move |_| { count.update(|v| *v += 1); Msg::NoOp } )), "+"],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).after_mount(after_mount).build_and_start();
}
