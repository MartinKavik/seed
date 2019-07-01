#[macro_use]
extern crate seed;
use seed::prelude::*;

// Model

enum Model {
    Loading,
    Home(UserData),
    Guide(UserData),
    About(UserData),
}

impl Default for Model {
    fn default() -> Self {
        Model::Home(UserData::default())
    }
}

impl Model {
    fn take(&mut self) -> Self {
        std::mem::replace(self, Model::Loading)
    }

    fn into_user_data(self) -> Option<UserData> {
        use Model::*;
        match self {
            Loading => None,
            Home(user_data) | Guide(user_data) | About(user_data) => Some(user_data),
        }
    }
}

struct UserData {
    name: String,
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            name: "John".into(),
        }
    }
}

// Update

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
enum Msg {
    ShowHome,
    ShowGuide,
    ShowAbout,
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    if let Some(user_data) = model.take().into_user_data() {
        match msg {
            Msg::ShowHome => {
                *model = Model::Home(user_data);
            }
            Msg::ShowGuide => {
                *model = Model::Guide(user_data);
            }
            Msg::ShowAbout => {
                *model = Model::About(user_data);
            }
        }
    }
}

// View

fn view(model: &Model) -> impl ElContainer<Msg> {
    let head = div![
        button!["Home", simple_ev(Ev::Click, Msg::ShowHome)],
        button!["Guide", simple_ev(Ev::Click, Msg::ShowGuide)],
        button!["About", simple_ev(Ev::Click, Msg::ShowAbout)],
    ];
    let content = match model {
        Model::Loading => empty![],
        Model::Home(user_data) => div![format!("Welcome home {}!", user_data.name)],
        Model::Guide(user_data) => div![format!("Guide for {}.", user_data.name)],
        Model::About(user_data) => div![format!("It's only about me, {}.", user_data.name)],
    };
    vec![head, content]
}

#[wasm_bindgen]
pub fn start() {
    seed::App::build(|_, _| Model::default(), update, view)
        .finish()
        .run();
}
