use crate::color::{Color, ColorValue};
use seed::{prelude::*, *};
use std::str::FromStr;

// ------ ------
//     View
// ------ ------

pub fn color_picker<Ms>(
    color: Color,
    on_change: impl FnOnce(Color) -> Ms + 'static + Clone,
) -> Node<Ms> {
    div![
        slider("Red", color.red, {
            let on_change = on_change.clone(); // or use `enc!` to reduce boilerplate (search in the repo)
            move |red| on_change(color.map_red(|_| red))
        },),
        slider("Green", color.green, {
            let on_change = on_change.clone();
            move |green| on_change(color.map_green(|_| green))
        },),
        slider("Blue", color.blue, move |blue| on_change(
            color.map_blue(|_| blue)
        )),
        div![style! {
            St::Width => px(100),
            St::Height => px(100),
            St::BackgroundColor => color.to_css()
        },],
    ]
}

fn slider<Ms>(
    name: &str,
    color_channel_value: ColorValue,
    on_change: impl FnOnce(ColorValue) -> Ms + 'static + Clone,
) -> Node<Ms> {
    div![
        p![name],
        input![
            attrs![At::Type => "range",
                   At::Name => format!("color-{}", name.to_lowercase()),
                   At::Min => 0,
                   At::Max => 255,
                   At::Value => color_channel_value,
            ],
            // We can use `.expect("ColorValue from slider value")` instead of `unwrap_or_default()`
            // to crash the app because something in slider is really bad.
            input_ev(Ev::Input, |value| on_change(
                ColorValue::from_str(&value).unwrap_or_default()
            )),
        ],
        span![color_channel_value.to_string(),],
    ]
}
