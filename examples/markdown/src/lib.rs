//! A simple, cliché example demonstrating structure and syntax.
//! Inspired by [Elm example](https://guide.elm-lang.org/architecture/buttons.html).

// Some Clippy linter rules are ignored for the sake of simplicity.
#![allow(clippy::needless_pass_by_value, clippy::trivially_copy_pass_by_ref)]

use seed::{prelude::*, *};

// ----- ------
//    Model
// ----- -----

type Model = ();

// ------ ------
//    Update
// ----- ------

type Msg = ();

fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {}

// ------ ------
//     View
// ------ ------

fn view(_model: &Model) -> Node<Msg> {
    div![
        md!("## Markdown Example

Intended as a demo of using [El::from_markdown()](https://docs.rs/seed/latest/seed/virtual_dom/node/el/struct.El.html#method.from_markdown) and ```md!()``` for markdown conversion.

---

```bash
cargo make start
```

Open [127.0.0.1](https://127.0.0.1:8000) in your browser."),
        Node::from_markdown(
            "## Additional Extensions (x=showcased)

* [ ] Tables
* [ ] Footnotes
* [x] ~~Strikethrough~~
* [x] Tasklists"
        )
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
