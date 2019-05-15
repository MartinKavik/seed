use seed::prelude::*;
use seed::fetch;
use futures::Future;
use serde::Deserialize;

fn get_request_url() -> String {
    "/api/non-existent-endpoint".into()
}

// Model

#[derive(Default)]
pub struct Model {
    pub fetch_result: Option<fetch::FetchResult<ExpectedResponseData>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExpectedResponseData {
    something: String
}

// Update

#[derive(Clone)]
pub enum Msg {
    SendRequest,
    Fetched(fetch::FetchObject<ExpectedResponseData>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            orders
                .skip()
                .perform_cmd(send_request());
        }

        Msg::Fetched(fetch_object) => {
            model.fetch_result = Some(fetch_object.result);
        }
    }
}

fn send_request() -> impl Future<Item=Msg, Error=Msg> {
    fetch::Request::new(get_request_url())
        .method(fetch::Method::Get)
        .fetch_json(Msg::Fetched)
}

// View

pub fn view(model: &Model) -> impl ElContainer<Msg> {
    vec![
        match &model.fetch_result {
            Some(result) => {
                match result {
                    Ok(response_with_data_result) => {
                        div![
                            div![format!("Status code: {}", response_with_data_result.status.code())],
                            div![format!(r#"Status text: "{}""#, response_with_data_result.status.text())],
                            div![format!(r#"Data: "{:#?}""#, response_with_data_result.data)]
                        ]
                    }
                    Err(request_error) => {
                        log!("Example_B error:", request_error);
                        //@TODO: [BUG] it cannot be `seed::empty()` because of order bug (rewrite after fix)
                        div![]
                    }
                }
            }
            //@TODO: [BUG] it cannot be `seed::empty()` because of order bug (rewrite after fix)
            None => div![]
        },
        button![
            simple_ev(Ev::Click, Msg::SendRequest),
            "Fetch JSON from non-existent endpoint"
        ],
    ]
}