//! A simple, cliché example demonstrating structure and syntax.
//! Inspired by [Elm example](https://guide.elm-lang.org/architecture/buttons.html).

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ ------

type Model = i32;

// ------ ------
//    Update
// ------ ------

#[derive(Debug)]
enum Msg {
    Increment,
    Decrement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => *model += 1,
        Msg::Decrement => *model -= 1,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    let mut nodes: Vec<Node<Msg>> = raw!(
        r#"<button>-</button><span></span><button>+</button>"#
    );
    if let [decrement, counter, increment] = nodes.as_mut_slice() {
        decrement.add_event_handler(ev(Ev::Click, |_| Msg::Decrement));
        counter.replace_text(model.to_string());
        increment.add_event_handler(ev(Ev::Click, |_| Msg::Increment));
    }
    log!(nodes);
    nodes
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
