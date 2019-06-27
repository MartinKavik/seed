use crate::vdom::{Effect, ShouldRender};
use futures::Future;
use std::collections::VecDeque;
use std::rc::Rc;

// ------ Orders ------

pub trait Orders<Ms, GMs = ()> {
    type RootMs: 'static;

    /// Automatically map message type. It allows you to pass `Orders` into child module.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///Msg::Child(child_msg) => {
    ///    child::update(child_msg, &mut model.child, &mut orders.proxy(Msg::Child));
    ///}
    /// ```
    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl Fn(ChildMs) -> Ms + 'static,
    ) -> OrdersProxy<ChildMs, Self::RootMs, GMs>;

    /// Schedule web page rerender after model update. It's the default behaviour.
    fn render(&mut self) -> &mut Self;

    /// Force web page to rerender immediately after model update.
    fn force_render_now(&mut self) -> &mut Self;

    /// Don't rerender web page after model update.
    fn skip(&mut self) -> &mut Self;

    /// Call function `update` with the given `msg` after model update.
    /// You can call this function more times - messages will be sent in the same order.
    fn send_msg(&mut self, msg: Ms) -> &mut Self;

    /// Schedule given future `cmd` to be executed after model update.
    /// You can call this function more times - futures will be scheduled in the same order.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///fn write_emoticon_after_delay() -> impl Future<Item=Msg, Error=Msg> {
    ///    TimeoutFuture::new(2_000)
    ///        .map(|_| Msg::WriteEmoticon)
    ///        .map_err(|_| Msg::TimeoutError)
    ///}
    ///orders.perform_cmd(write_emoticon_after_delay());
    /// ```
    fn perform_cmd<C>(&mut self, cmd: C) -> &mut Self
    where
        C: Future<Item = Ms, Error = Ms> + 'static;

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self;

    fn perform_g_cmd<C>(&mut self, g_cmd: C) -> &mut Self
    where
        C: Future<Item = GMs, Error = GMs> + 'static;
}

// ------ OrdersContainer ------

#[allow(clippy::module_name_repetitions)]
pub struct OrdersContainer<Ms, GMs = ()> {
    pub(crate) should_render: ShouldRender,
    pub(crate) effects: VecDeque<Effect<Ms, GMs>>,
}

impl<Ms, GMs> Default for OrdersContainer<Ms, GMs> {
    fn default() -> Self {
        Self {
            should_render: ShouldRender::Render,
            effects: VecDeque::new(),
        }
    }
}

impl<Ms: 'static, GMs> Orders<Ms, GMs> for OrdersContainer<Ms, GMs> {
    type RootMs = Ms;

    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl Fn(ChildMs) -> Ms + 'static,
    ) -> OrdersProxy<ChildMs, Ms, GMs> {
        OrdersProxy::new(self, f)
    }

    fn render(&mut self) -> &mut Self {
        self.should_render = ShouldRender::Render;
        self
    }

    fn force_render_now(&mut self) -> &mut Self {
        self.should_render = ShouldRender::ForceRenderNow;
        self
    }

    fn skip(&mut self) -> &mut Self {
        self.should_render = ShouldRender::Skip;
        self
    }

    fn send_msg(&mut self, msg: Ms) -> &mut Self {
        self.effects.push_back(msg.into());
        self
    }

    fn perform_cmd<C>(&mut self, cmd: C) -> &mut Self
    where
        C: Future<Item = Ms, Error = Ms> + 'static,
    {
        let effect = Effect::Cmd(Box::new(cmd));
        self.effects.push_back(effect);
        self
    }

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self {
        let effect = Effect::GMsg(g_msg);
        self.effects.push_back(effect);
        self
    }

    fn perform_g_cmd<C>(&mut self, g_cmd: C) -> &mut Self
    where
        C: Future<Item = GMs, Error = GMs> + 'static,
    {
        let effect = Effect::GCmd(Box::new(g_cmd));
        self.effects.push_back(effect);
        self
    }
}

// ------ OrdersProxy ------

#[allow(clippy::module_name_repetitions)]
pub struct OrdersProxy<'a, Ms, RootMs: 'static, GMs: 'static = ()> {
    orders_container: &'a mut OrdersContainer<RootMs, GMs>,
    f: Rc<Fn(Ms) -> RootMs>,
}

impl<'a, Ms: 'static, RootMs: 'static, GMs> OrdersProxy<'a, Ms, RootMs, GMs> {
    pub fn new(
        orders_container: &'a mut OrdersContainer<RootMs, GMs>,
        f: impl Fn(Ms) -> RootMs + 'static,
    ) -> Self {
        OrdersProxy {
            orders_container,
            f: Rc::new(f),
        }
    }
}

impl<'a, Ms: 'static, RootMs: 'static, GMs> Orders<Ms, GMs> for OrdersProxy<'a, Ms, RootMs, GMs> {
    type RootMs = RootMs;

    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl Fn(ChildMs) -> Ms + 'static,
    ) -> OrdersProxy<ChildMs, RootMs, GMs> {
        let previous_f = self.f.clone();
        OrdersProxy {
            orders_container: self.orders_container,
            f: Rc::new(move |child_ms| previous_f(f(child_ms))),
        }
    }

    fn render(&mut self) -> &mut Self {
        self.orders_container.render();
        self
    }

    fn force_render_now(&mut self) -> &mut Self {
        self.orders_container.force_render_now();
        self
    }

    fn skip(&mut self) -> &mut Self {
        self.orders_container.skip();
        self
    }

    #[allow(clippy::redundant_closure)]
    fn send_msg(&mut self, msg: Ms) -> &mut Self {
        let f = self.f.clone();
        self.orders_container
            .effects
            .push_back(Effect::Msg(msg).map_message_with_fn_once(move |ms| f(ms)));
        self
    }

    #[allow(clippy::redundant_closure)]
    fn perform_cmd<C>(&mut self, cmd: C) -> &mut Self
    where
        C: Future<Item = Ms, Error = Ms> + 'static,
    {
        let f = self.f.clone();
        let effect = Effect::Cmd(Box::new(cmd)).map_message_with_fn_once(move |ms| f(ms));
        self.orders_container.effects.push_back(effect);
        self
    }

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self {
        let effect = Effect::GMsg(g_msg);
        self.orders_container.effects.push_back(effect);
        self
    }

    fn perform_g_cmd<C>(&mut self, g_cmd: C) -> &mut Self
    where
        C: Future<Item = GMs, Error = GMs> + 'static,
    {
        let effect = Effect::GCmd(Box::new(g_cmd));
        self.orders_container.effects.push_back(effect);
        self
    }
}
