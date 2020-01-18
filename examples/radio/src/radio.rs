use enclose::enc;
use seed::{prelude::*, *};

// ------ ------
//    Config
// ------ ------

pub struct Config<ParentMs, OnClick, MapMsg>
where
    OnClick: FnOnce() -> ParentMs + Clone,
    MapMsg: FnOnce(Msg) -> ParentMs + Clone,
{
    pub label: String,
    pub checked: bool,
    pub on_click: OnClick,
    pub style: Style,
    pub disabled: bool,
    pub map_msg: MapMsg,
}

#[allow(dead_code)]
pub enum Style {
    Small,
    Medium,
    Large,
}

// ------ ------
//     State
// ------ ------

#[derive(Default)]
pub struct State {
    mouse_over: bool,
    focus: bool,
}

// ------ ------
//    Update
// ------ ------

#[derive(Clone, Copy)]
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
}

pub fn update(msg: Msg, state: &mut State) {
    match msg {
        Msg::MouseEnter => state.mouse_over = true,
        Msg::MouseLeave => state.mouse_over = false,
        Msg::Focus => state.focus = true,
        Msg::Blur => state.focus = false,
    }
}

// ------ ------
//     View
// ------ ------

pub fn view<ParentMs, OnClick, MapMsg>(
    state: &State,
    config: Config<ParentMs, OnClick, MapMsg>,
) -> Node<ParentMs>
where
    OnClick: FnOnce() -> ParentMs + Clone + 'static,
    MapMsg: FnOnce(Msg) -> ParentMs + Clone + 'static,
{
    let on_click = config.on_click;
    let map_msg = config.map_msg;
    div![
        style! {
            St::Color => if state.mouse_over { Some("blue") } else { None },
            St::Background => if state.focus { Some("yellow") } else { None },
        },
        attrs! {
            At::TabIndex => 0,
        },
        ev(Ev::Click, move |_| on_click()),
        ev(
            Ev::MouseEnter,
            enc!((map_msg) move |_| map_msg(Msg::MouseEnter))
        ),
        ev(
            Ev::MouseLeave,
            enc!((map_msg) move |_| map_msg(Msg::MouseLeave))
        ),
        ev(Ev::Focus, enc!((map_msg) move |_| map_msg(Msg::Focus))),
        ev(Ev::Blur, enc!((map_msg) move |_|map_msg(Msg::Blur))),
        input![attrs! {
            At::Type => "radio",
            At::Checked => config.checked.as_at_value(),
        }],
        label![config.label,]
    ]
}
