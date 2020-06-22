use seed::{prelude::*, *};

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model {
        number: 0,
        input_element: ElRef::new(),
    }
}

struct Model {
    number: i32,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

enum Msg {
    InputChanged(i32),
    ResetInputElement,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::InputChanged(number) => {
            log!(number);
            model.number = number;
        }
        Msg::ResetInputElement => {
            let element = model.input_element.get().expect("input element");
            element.set_value("");
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        input![
            el_ref(&model.input_element),
            attrs!{
                At::Type => "number",
            },
            ev(Ev::Input, |event| {
                let element = event.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>();
                IF!(element.report_validity() => Msg::InputChanged(element.value().parse().unwrap_or_default()))
            }),
        ],
        button![
            "Reset",
            ev(Ev::Click, |_| Msg::ResetInputElement),
        ]
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
