#![allow(clippy::non_ascii_literal)]

#[macro_use]
extern crate seed;

use gloo_timers::future::TimeoutFuture;
use seed::prelude::*;

// Model

#[derive(Default)]
struct Model {
    title: String,
    greet_clicked: bool,
}

// Update

#[derive(Clone)]
enum Msg {
    Greet,
    WriteHello,
    WriteName(String),
    WriteExclamationMarks,
    WriteEmoticon(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Greet => {
            model.greet_clicked = true;
            orders
                .skip()
                .send_msg(Msg::WriteHello)
                .send_msg(Msg::WriteName("World".into()))
                .perform_cmd(write_exclamation_marks_after_delay())
                .perform_cmd(write_emoticon_after_delay("🙂".into()));
        }
        Msg::WriteHello => model.title.push_str("Hello "),
        Msg::WriteName(name) => model.title.push_str(&name),
        Msg::WriteExclamationMarks => model.title.push_str("!!! "),
        Msg::WriteEmoticon(emoticon) => model.title.push_str(&emoticon),
    }
}

async fn write_exclamation_marks_after_delay() -> Result<Msg, Msg> {
    TimeoutFuture::new(1_000).await;
    Ok(Msg::WriteExclamationMarks)
}

async fn write_emoticon_after_delay(emoticon: String) -> Result<Msg, Msg> {
    TimeoutFuture::new(2_000).await;
    Ok(Msg::WriteEmoticon(emoticon))
}

// View

fn view(model: &Model) -> impl View<Msg> {
    div![
        style![
            St::Display => "flex",
            St::JustifyContent => "center",
            St::AlignItems => "center",
            St::FontSize => vmin(5),
            St::FontFamily => "sans-serif",
            St::Height => vmin(50),
        ],
        if model.greet_clicked {
            h1![model.title]
        } else {
            div![
                style![
                    St::BackgroundColor => "lightgreen",
                    St::Padding => vmin(3),
                    St::BorderRadius => vmin(3),
                    St::Cursor => "pointer",
                    St::BoxShadow => [vmin(0), vmin(0.5), vmin(0.5), "green".into()].join(" "),
                ],
                simple_ev(Ev::Click, Msg::Greet),
                "Greet!"
            ]
        }
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    seed::App::builder(update, view).build_and_start();
}
