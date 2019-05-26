use seed::prelude::*;
use seed::fetch;
use futures::Future;

const TIMEOUT: u32 = 2000;

fn get_request_url() -> String {
    let response_delay_ms: u32 = 2500;
    format!("/api/delayed-response/{}", response_delay_ms)
}

// Model

#[derive(Default)]
pub struct Model {
    pub response: Option<fetch::ResponseResult<()>>,
    pub request_controller: Option<fetch::RequestController>,
    pub status: Status,
}

pub enum TimeoutStatus {
    Enabled,
    Disabled,
}

pub enum Status {
    ReadyToSendRequest,
    WaitingForResponse(TimeoutStatus),
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
    DisableTimeout,
    Fetched(fetch::FetchObject<()>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut Orders<Msg>) {
    match msg {
        Msg::SendRequest => {
            model.status = Status::WaitingForResponse(TimeoutStatus::Enabled);
            model.response = None;
            orders
                .perform_cmd(send_request(&mut model.request_controller));
        }

        Msg::DisableTimeout => {
            model.request_controller.take()
                .ok_or("Msg:DisableTimeout: request controller cannot be None")
                .and_then(|controller| controller.disable_timeout())
                .unwrap_or_else(|err| log!(err));
            model.status = Status::WaitingForResponse(TimeoutStatus::Disabled)
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
        .timeout(TIMEOUT)
        .fetch(Msg::Fetched)
}

// View

pub fn view(model: &Model) -> impl ElContainer<Msg> {
    match &model.response {
        Some(response) => {
            match response {
                Ok(response) => {
                    vec![
                        div![format!("Server returned {}", response.status.text())],
                        view_button(&model.status)
                    ]
                }
                Err(fail_reason) => view_fail_reason(fail_reason, &model.status)
            }
        }
        None => {
            vec![
                //@TODO: [BUG] if you comment out div with info (only button remains),
                //       button disappears when request ends
                if let Status::WaitingForResponse(_) = model.status {
                    div!["Waiting for response..."]
                } else {
                    //@TODO: [BUG] it cannot be `seed::empty()` because of order bug (rewrite after fix)
                    div![]
                },
                view_button(&model.status)
            ]
        }
    }
}

fn view_fail_reason(fail_reason: &fetch::FailReason, status: &Status) -> Vec<El<Msg>> {
    if let fetch::FailReason::RequestError(
        fetch::RequestError::DomException(dom_exception)
    ) = fail_reason {
        if dom_exception.name() == "AbortError" {
            return
                vec![
                    div!["Request aborted."],
                    view_button(status)
                ];
        }
    }
    log!("Example_D error:", fail_reason);
    //@TODO: [BUG] it cannot be `seed::empty()` because of order bug (rewrite after fix)
    vec![div![]]
}

pub fn view_button(status: &Status) -> El<Msg> {
    if let Status::WaitingForResponse(timeout_status) = status {
        return match timeout_status {
            TimeoutStatus::Enabled => {
                button![
                    simple_ev(Ev::Click, Msg::DisableTimeout),
                    "Disable timeout"
                ]
            }
            TimeoutStatus::Disabled => {
                button![
                    attrs!{"disabled" => true},
                    "Timeout disabled"
                ]
            }
        };
    }
    button![
        simple_ev(Ev::Click, Msg::SendRequest),
        "Send request"
    ]
}








