use crate::dom_types::MessageMapper;
use crate::vdom::{Effect, ShouldRender};
use futures::Future;
use std::collections::VecDeque;

// ------ OrdersTrait ------

#[allow(clippy::module_name_repetitions)]
pub trait OrdersTrait<Ms, GMs = ()> {
    fn render(&mut self) -> &mut Self;

    fn force_render_now(&mut self) -> &mut Self;

    fn skip(&mut self) -> &mut Self;

    fn send_msg(&mut self, msg: Ms) -> &mut Self;

    fn perform_cmd<C>(&mut self, cmd: C) -> &mut Self
    where
        C: Future<Item = Ms, Error = Ms> + 'static;

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self;

    fn perform_g_cmd<C>(&mut self, g_cmd: C) -> &mut Self
    where
        C: Future<Item = GMs, Error = GMs> + 'static;
}

// ------ Orders ------

pub struct Orders<Ms, GMs = ()> {
    pub(crate) should_render: ShouldRender,
    pub(crate) effects: VecDeque<Effect<Ms, GMs>>,
}

impl<Ms, GMs> Default for Orders<Ms, GMs> {
    fn default() -> Self {
        Self {
            should_render: ShouldRender::Render,
            effects: VecDeque::new(),
        }
    }
}

impl<Ms: 'static, GMs> Orders<Ms, GMs> {
    pub fn proxy<ChildMs: 'static>(
        &mut self,
        f: fn(ChildMs) -> Ms,
    ) -> OrdersProxy<ChildMs, Ms, GMs> {
        OrdersProxy::new(self, f)
    }
}

impl<Ms: 'static, OtherMs: 'static, GMs> MessageMapper<Ms, OtherMs> for Orders<Ms, GMs> {
    type SelfWithOtherMs = Orders<OtherMs, GMs>;
    fn map_message(self, f: fn(Ms) -> OtherMs) -> Orders<OtherMs, GMs> {
        Orders {
            should_render: self.should_render,
            effects: self
                .effects
                .into_iter()
                .map(|effect| effect.map_message(f))
                .collect(),
        }
    }
}

impl<Ms, GMs> OrdersTrait<Ms, GMs> for Orders<Ms, GMs> {
    /// Schedule web page rerender after model update. It's the default behaviour.
    fn render(&mut self) -> &mut Self {
        self.should_render = ShouldRender::Render;
        self
    }

    /// Force web page to rerender immediately after model update.
    fn force_render_now(&mut self) -> &mut Self {
        self.should_render = ShouldRender::ForceRenderNow;
        self
    }

    /// Don't rerender web page after model update.
    fn skip(&mut self) -> &mut Self {
        self.should_render = ShouldRender::Skip;
        self
    }

    /// Call function `update` with the given `msg` after model update.
    /// You can call this function more times - messages will be sent in the same order.
    fn send_msg(&mut self, msg: Ms) -> &mut Self {
        self.effects.push_back(msg.into());
        self
    }

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
pub struct OrdersProxy<'a, Ms, ParentMs: 'static, GMs: 'static = ()> {
    orders: &'a mut Orders<ParentMs, GMs>,
    f: fn(Ms) -> ParentMs,
}

impl<'a, Ms: 'static, ParentMs: 'static, GMs> OrdersProxy<'a, Ms, ParentMs, GMs> {
    pub fn new(orders: &'a mut Orders<ParentMs, GMs>, f: fn(Ms) -> ParentMs) -> Self {
        OrdersProxy { orders, f }
    }
}

impl<'a, Ms: 'static, ParentMs: 'static, GMs> OrdersTrait<Ms, GMs>
    for OrdersProxy<'a, Ms, ParentMs, GMs>
{
    fn render(&mut self) -> &mut Self {
        self.orders.render();
        self
    }

    /// Force web page to rerender immediately after model update.
    fn force_render_now(&mut self) -> &mut Self {
        self.orders.force_render_now();
        self
    }

    /// Don't rerender web page after model update.
    fn skip(&mut self) -> &mut Self {
        self.orders.skip();
        self
    }

    /// Call function `update` with the given `msg` after model update.
    /// You can call this function more times - messages will be sent in the same order.
    fn send_msg(&mut self, msg: Ms) -> &mut Self {
        self.orders
            .effects
            .push_back(Effect::Msg(msg).map_message(self.f));
        self
    }

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
        C: Future<Item = Ms, Error = Ms> + 'static,
    {
        let effect = Effect::Cmd(Box::new(cmd)).map_message(self.f);
        self.orders.effects.push_back(effect);
        self
    }

    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self {
        let effect = Effect::GMsg(g_msg);
        self.orders.effects.push_back(effect);
        self
    }

    fn perform_g_cmd<C>(&mut self, g_cmd: C) -> &mut Self
    where
        C: Future<Item = GMs, Error = GMs> + 'static,
    {
        let effect = Effect::GCmd(Box::new(g_cmd));
        self.orders.effects.push_back(effect);
        self
    }
}
