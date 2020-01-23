
mod counter;

use seed::{prelude::*, *};
use counter::Counter;

// ------ ------
//     Model
// ------ ------

struct Model {
    counter_a: Counter<Msg>,
    counter_b: Counter<Msg>,
    title: String,
}

// ------ ------
//  After Mount
// ------ ------

fn after_mount(_: Url, _: &mut impl Orders<Msg>) -> AfterMount<Model> {
    AfterMount::new(Model {
        counter_a: Counter::new(Msg::CounterA),
        counter_b: Counter::new(Msg::CounterB),
        title: "My Title".to_owned()
    })
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    CounterA(counter::Msg),
    CounterB(counter::Msg),
    TitleChanged(String),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CounterA(msg) => model.counter_a.update(msg, orders),
        Msg::CounterB(msg) => model.counter_b.update(msg, orders),
        Msg::TitleChanged(title) => model.title = title,
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        h2![model.title],
        input![
            attrs!{
                At::Value => model.title,
            },
            input_ev(Ev::Input, |text| Msg::TitleChanged(text)),
        ],
        hr![],
        model.counter_a.render(),
        hr![],
        model.counter_b.render(),
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).after_mount(after_mount).build_and_start();
}
