#![allow(clippy::enum_variant_names, clippy::large_enum_variant)]

#[macro_use]
extern crate seed;

use seed::prelude::*;

mod example_a;
mod example_b;
mod example_c;
mod example_d;

// Model

#[derive(Default)]
struct Model {
    example_a: example_a::Model,
    example_b: example_b::Model,
    example_c: example_c::Model,
    example_d: example_d::Model,
}

// Update

#[derive(Clone)]
enum Msg {
    ExampleA(example_a::Msg),
    ExampleB(example_b::Msg),
    ExampleC(example_c::Msg),
    ExampleD(example_d::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ExampleA(msg) => {
            example_a::update(msg, &mut model.example_a, &mut orders.proxy(Msg::ExampleA));
        }
        Msg::ExampleB(msg) => {
            example_b::update(msg, &mut model.example_b, &mut orders.proxy(Msg::ExampleB));
        }
        Msg::ExampleC(msg) => {
            example_c::update(msg, &mut model.example_c, &mut orders.proxy(Msg::ExampleC));
        }
        Msg::ExampleD(msg) => {
            example_d::update(msg, &mut model.example_d, &mut orders.proxy(Msg::ExampleD));
        }
    }
}

// View

fn view(model: &Model) -> impl ElContainer<Msg> {
    let examples = vec![
        // example_a
        view_example_introduction(example_a::TITLE, example_a::DESCRIPTION),
        example_a::view(&model.example_a)
            .els()
            .map_message(Msg::ExampleA),
        // example_b
        view_example_introduction(example_b::TITLE, example_b::DESCRIPTION),
        example_b::view(&model.example_b)
            .els()
            .map_message(Msg::ExampleB),
        // example_c
        view_example_introduction(example_c::TITLE, example_c::DESCRIPTION),
        example_c::view(&model.example_c)
            .els()
            .map_message(Msg::ExampleC),
        // example_d
        view_example_introduction(example_d::TITLE, example_d::DESCRIPTION),
        example_d::view(&model.example_d)
            .els()
            .map_message(Msg::ExampleD),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<El<Msg>>>();

    div![
        style! {
            "font-family" => "sans-serif";
            "max-width" => px(460);
            "margin" => "auto";
        },
        examples
    ]
}

fn view_example_introduction(title: &str, description: &str) -> Vec<El<Msg>> {
    vec![
        hr![],
        h2![title],
        div![style! {"margin-bottom" => px(15);}, description],
    ]
}

#[wasm_bindgen]
pub fn start() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();
}
