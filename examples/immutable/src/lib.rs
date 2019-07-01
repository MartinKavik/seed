#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

struct Model {
    text: &'static str,
}

impl Default for Model {
    fn default() -> Self {
        Self { text: "Click me!" }
    }
}

// Update

#[derive(Clone, Copy)]
enum Msg {
    UpdateText,
}

fn update(msg: Msg, model: &mut Option<Model>, _: &mut impl Orders<Msg>) {
    let mut md = model.take().unwrap();
    match msg {
        Msg::UpdateText => {
            // `md` isn't reference, I can do whatever I want with it
            md.text = "Hello!";
        }
    }
    *model = Some(md);
}

// View

fn view(model: &Option<Model>) -> impl ElContainer<Msg> {
    let model = model.as_ref().unwrap();
    div![
        model.text,
        simple_ev(Ev::Click, Msg::UpdateText),
        style! { "cursor" => "pointer" }
    ]
}

#[wasm_bindgen]
pub fn start() {
    seed::App::build(|_, _| Some(Model::default()), update, view)
        .finish()
        .run();
}
