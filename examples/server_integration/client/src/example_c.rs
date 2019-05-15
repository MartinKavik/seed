use seed::prelude::*;
use seed::fetch;
use futures::Future;

fn get_request_url() -> String {
    let response_delay_ms: u32 = 2000;
    format!("/api/delayed-response/{}", response_delay_ms)
}

// Model

#[derive(Default)]
pub struct Model {
    pub response: Option<fetch::ResponseResult<String>>,
    pub request_controller: Option<fetch::RequestController>,
    pub status: Status,
}

pub enum Status {
    ReadyToSendRequest,
    WaitingForResponse,
    RequestAborted,
}

impl Default for Status {
    fn default() -> Self {
        Status::ReadyToSendRequest
    }
}

// Update

#[derive(Clone)]
pub enum Msg {
    SendRequest,
    AbortRequest,
    Fetched(fetch::FetchObject<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            model.status = Status::WaitingForResponse;
            model.response = None;
            orders
                .perform_cmd(send_request(&mut model.request_controller));
        }

        Msg::AbortRequest => {
            model.request_controller.take().unwrap().abort();
            model.status = Status::RequestAborted;
        }

        Msg::Fetched(fetch_object) => {
            model.status = Status::ReadyToSendRequest;
            model.response = Some(fetch_object.response());
        }
    }
}

fn send_request(
    request_controller: &mut Option<fetch::RequestController>
) -> impl Future<Item=Msg, Error=Msg>
{
    fetch::Request::new(get_request_url())
        .method(fetch::Method::Get)
        .controller(|controller| *request_controller = Some(controller))
        .fetch_string(Msg::Fetched)
}

// View

pub fn view(model: &Model) -> impl ElContainer<Msg> {
    match model.status {
        Status::ReadyToSendRequest => {
            vec![
                view_response(&model.response),
                button![
                    simple_ev(Ev::Click, Msg::SendRequest),
                    "Send request"
                ]
            ]
        }
        Status::WaitingForResponse => {
            vec![
                div!["Waiting for response..."],
                button![
                    simple_ev(Ev::Click, Msg::AbortRequest),
                    "Abort request"
                ]
            ]
        }
        Status::RequestAborted => {
            vec![
                view_response(&model.response),
                button![
                    attrs!{At::Disabled => false},
                    "Request aborted"
                ]
            ]
        }
    }
}

fn view_response(response: &Option<fetch::ResponseResult<String>>) -> El<Msg> {
    match &response {
        Some(response) => {
            match response {
                Ok(response) => {
                    div![format!(r#"Response string body: "{}""#, response.data)]
                }
                Err(fail_reason) => view_fail_reason(fail_reason)
            }
        }
        //@TODO: [BUG] it cannot be `seed::empty()` because of order bug (rewrite after fix)
        None => div![]
    }
}

fn view_fail_reason(fail_reason: &fetch::FailReason) -> El<Msg> {
    if let fetch::FailReason::RequestError(
        fetch::RequestError::DomException(dom_exception)
    ) = fail_reason {
        if dom_exception.name() == "AbortError" {
            return
                div![
                    div![format!(r#"Error name: "{}""#, dom_exception.name())],
                    div![format!(r#"Error message: "{}""#, dom_exception.message())]
                ];
        }
    }
    log!("Example_C error:", fail_reason);
    //@TODO: [BUG] it cannot be `seed::empty()` because of order bug (rewrite after fix)
    div![]
}







