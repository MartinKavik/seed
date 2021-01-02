use seed::{prelude::*, *};

// ------ ------
//    Action
// ------ ------

#[derive(Clone, Copy)]
pub struct DoNewCase;

// ------ ------
//     Init
// ------ ------

pub fn init(orders: &mut impl Orders<Msg>) -> Model {
    Model {
        case: Case::default(),
        _sub_handle: orders.subscribe_with_handle(|_: DoNewCase| Msg::NewCase) 
    }
}

// ------ ------
//     Model
// ------ ------

pub struct Model {
    case: Case,
    _sub_handle: SubHandle,
}

#[derive(Default)]
struct Case {
    id: u32,
}

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    NewCase,
}

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::NewCase => {
            log!("NEW");
            model.case.id += 1;
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<Ms: 'static>(model: &Model) -> Node<Ms> {
    div![
        style!{St::Background => "whitesmoke"},
        "I'm the Case view. Id: ",
        model.case.id,
    ]
}
