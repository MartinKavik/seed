use seed::prelude::*;
use seed::fetch;
use futures::Future;

use shared;

fn get_request_url() -> String {
    "/api/send-message".into()
}

// Model

#[derive(Default)]
pub struct Model {
    pub new_message: String,
    pub response: Option<fetch::ResponseResult<shared::ResponseExampleA>>,
}

// Update

#[derive(Clone)]
pub enum Msg {
    NewMessageChanged(String),
    SendRequest,
    Fetched(fetch::FetchObject<shared::ResponseExampleA>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::NewMessageChanged(message) => {
            model.new_message = message;
        }
        Msg::SendRequest => {
            orders
                .skip()
                .perform_cmd(send_request(model.new_message.clone()));
        }

        Msg::Fetched(fetch_object) => {
            model.response = Some(fetch_object.response());
        }
    }
}

fn send_request(new_message: String) -> impl Future<Item=Msg, Error=Msg> {
    fetch::Request::new(get_request_url())
        .method(fetch::Method::Post)
        .send_json(&shared::RequestExampleA { text: new_message })
        .fetch_json(Msg::Fetched)
}

// View

pub fn view(model: &Model) -> impl ElContainer<Msg> {

    let message = match &model.response {
        Some(response) => {
            match response {
                Ok(response) => {
                    div![
                        format!(r#"{}. message: "{}""#, response.data.ordinal_number, response.data.text)
                    ]
                }
                Err(fail_reason) => {
                    log!("Example_A error:", fail_reason);
                    //@TODO: [BUG] it cannot be `seed::empty()` because of order bug (rewrite after fix)
                    div![]
                }
            }
        }
        //@TODO: [BUG] div![] cannot be `seed::empty()` because of order bug (rewrite after fix)
        None => div![]
    };

    vec![
        message,
        input![
            input_ev(Ev::Input, Msg::NewMessageChanged),
            attrs!{At::Value => model.new_message}
        ],
        button![
            simple_ev(Ev::Click, Msg::SendRequest),
            "Send message"
        ],
    ]
}