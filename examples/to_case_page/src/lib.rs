use seed::{prelude::*, *};

mod page;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    Model {
        case_model: page::case::init(&mut orders.proxy(Msg::CaseMsg))
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    case_model: page::case::Model,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    HomeMsg(page::home::Msg),
    CaseMsg(page::case::Msg),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::HomeMsg(msg) => page::home::update(msg, &mut orders.proxy(Msg::HomeMsg)),
        Msg::CaseMsg(msg) => page::case::update(msg, &mut model.case_model),
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div![
        page::home::view().map_msg(Msg::HomeMsg),
        page::case::view(&model.case_model),
    ]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
