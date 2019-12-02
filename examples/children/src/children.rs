use seed::{prelude::*, *};

pub type Data = String;

pub fn view<Ms>(pressed: impl FnOnce(Data) -> Ms + 'static + Clone) -> Node<Ms> {
    button![
        raw_ev(Ev::Click, |_| pressed("any data".to_owned())),
        "Clicking me will emit 'pressed' message"
    ]
}
