#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]
mod state;

use seed::{prelude::*, *};
use state::State;

// ------ ------
//     View
// ------ ------

fn view(_: &()) -> Node<()> {
    static COUNTER: State<i32> = State::new(0);
    div![
        button![ev(Ev::Click, |_| { COUNTER.update(|c| *c -= 1) }), "-"],
        div![COUNTER.get().to_string()],
        button![ev(Ev::Click, |_| { COUNTER.update(|c| *c += 1) }), "+"],
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
fn update(_: (), _: &mut (), _: &mut impl Orders<()>) {}
