use seed::{prelude::*, *};

// ------ ------
//    Update
// ------ ------

pub enum Msg {
    AddCase,
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::AddCase => {
            log!("ADD");
            orders.notify(super::case::DoNewCase);
        }
    }
}

// ------ ------
//     View
// ------ ------

pub fn view() -> Node<Msg> {
    div![
        "I'm the Home view.",
        button![
            "Create a new case",
            ev(Ev::Click, |_| Msg::AddCase),
        ]
    ]
}
