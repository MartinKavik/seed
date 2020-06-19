use seed::{prelude::*, *};
use serde::Deserialize;

// ------ ------
//     Init
// ------ ------

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let user_session = UserSession {
        token: "xxx".to_owned(),
    };
    orders.perform_cmd(async { Msg::FetchAllUsers(fetch_all_users(user_session).await) });
    Model
}

async fn fetch_all_users(session: UserSession) -> fetch::Result<Vec<Entry<User>>> {
    Request::new("http://localhost:8000/api/users")
        .header(Header::authorization(session.token))
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

// ------ ------
//     Model
// ------ ------

struct Model;

struct UserSession {
    token: String,
}

#[derive(Debug, Deserialize)]
struct Entry<T> {
    value: T,
}

#[derive(Debug, Deserialize)]
struct User;

// ------ ------
//    Update
// ------ ------

enum Msg {
    FetchAllUsers(fetch::Result<Vec<Entry<User>>>),
}

fn update(msg: Msg, _: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchAllUsers(Ok(users)) => log!(users),
        Msg::FetchAllUsers(Err(fetch_error)) => log!(fetch_error),
    }
}

// ------ ------
//     View
// ------ ------

fn view(_: &Model) -> Node<Msg> {
    div!["Placeholder"]
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}
