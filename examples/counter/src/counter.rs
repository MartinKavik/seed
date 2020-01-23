use seed::{prelude::*, *};
use std::rc::Rc;

pub enum Msg {
    Increment,
    Decrement,
    Log(&'static str),
}

pub struct Counter<Ms> {
    msg_mapper: Rc<dyn Fn(Msg) -> Ms>,
    count: i32,
}

impl<Ms: 'static> Counter<Ms> {
    pub fn new(msg_mapper: impl FnOnce(Msg) -> Ms + 'static + Clone) -> Self {
        Self {
            msg_mapper: Rc::new(move |msg| (msg_mapper.clone())(msg)),
            count: 0
        }
    }

    pub fn update<GMsg: 'static>(&mut self, msg: Msg, orders: &mut impl Orders<Ms, GMsg>) {
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        let mut orders = orders.proxy(move |msg| (msg_mapper.clone())(msg));

        match msg {
            Msg::Increment => {
                self.count += 1;
                orders.send_msg(Msg::Log("incremented!"));
            },
            Msg::Decrement => {
                self.count -= 1;
                orders.send_msg(Msg::Log("decremented!"));
            },
            Msg::Log(text) => log!(text),
        }
    }

    pub fn render(&self) -> Node<Ms> {
        let msg_mapper = Rc::clone(&self.msg_mapper.clone());
        self.view().map_msg(move |msg| (msg_mapper.clone())(msg))
    }

    pub fn view(&self) -> Node<Msg> {
        div![
            button![ev(Ev::Click, |_| Msg::Decrement), "-"],
            div![self.count.to_string()],
            button![ev(Ev::Click, |_| Msg::Increment), "+"],
        ]
    }
}

