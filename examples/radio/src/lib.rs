mod radio;

use seed::{prelude::*, *};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

// ------ ------
//     Model
// ------ ------

struct Model {
    radios: Vec<(Os, radio::State)>,
    selected_os: Os,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            radios: Os::iter().map(|os| (os, radio::State::default())).collect(),
            selected_os: Os::Windows,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, EnumIter, Display)]
enum Os {
    Windows,
    Linux,
    Redox,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone, Copy)]
enum Msg {
    RadioMsg(Os, radio::Msg),
    RadioClicked(Os),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::RadioMsg(selected_os, radio_msg) => {
            let (_, state) = model
                .radios
                .iter_mut()
                .find(|(os, _)| *os == selected_os)
                .unwrap();
            radio::update(radio_msg, state);
        }
        Msg::RadioClicked(selected_os) => model.selected_os = selected_os,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> impl View<Msg> {
    div![
        h3!["Your favorite OS:"],
        model
            .radios
            .iter()
            .map(|(os, state)| { view_radio(*os, state, model.selected_os) }),
        h4![model.selected_os.to_string()]
    ]
}

fn view_radio(os: Os, radio_state: &radio::State, selected_os: Os) -> Node<Msg> {
    radio::view(
        radio_state,
        radio::Config {
            label: os.to_string(),
            checked: selected_os == os,
            on_click: move || Msg::RadioClicked(os),
            style: radio::Style::Medium,
            disabled: false,
            map_msg: move |radio_msg| Msg::RadioMsg(os, radio_msg),
        },
    )
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
