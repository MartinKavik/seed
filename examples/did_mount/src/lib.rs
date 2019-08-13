#[macro_use]
extern crate seed;
use seed::prelude::*;
use web_sys;
use wasm_bindgen::JsCast;

type ErrorMessage = String;

// Model

#[derive(Default)]
struct Model {
    text: String,
    error_message: Option<ErrorMessage>,
}

// Update

enum Msg {
    TextChanged(String),
    TextIsInvalid(ErrorMessage),
    Submitted,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::TextChanged(text) => model.text = text,
        Msg::TextIsInvalid(error_message) => model.error_message = Some(error_message),
        Msg::Submitted => {
            model.error_message = None;
            log!("Submitted!")
        },
    }
}

// View

fn view(model: &Model) -> impl View<Msg> {
    vec![
        form![
            raw_ev(Ev::Submit, |event| {
                event.prevent_default();
                Msg::Submitted
            }),
            input![
                attrs! {
                    At::Value => model.text,
                    At::Required => true.as_at_value()
                },
                input_ev(Ev::Input, Msg::TextChanged),
                raw_ev(Ev::Invalid, |event| {
                    event.prevent_default();

                    let target = event.target().unwrap();
                    let html_input_element = target.dyn_ref::<web_sys::HtmlInputElement>().unwrap();
                    let error_message = html_input_element.validation_message().unwrap();

                    Msg::TextIsInvalid(error_message)
                }),
            ],
            button! [
                "Submit",
            ]
        ],
        match &model.error_message {
            Some(error_message) => {
                div! [
                    error_message
                ]
            },
            None => empty![]
        }
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();
}
