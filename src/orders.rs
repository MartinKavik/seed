use crate::dom_types::ElContainer;
use crate::vdom::{App, Effect, ShouldRender};
use futures::Future;
use std::collections::VecDeque;
use std::rc::Rc;

// ------ Orders ------

pub trait Orders<Ms: 'static, GMs = ()> {
    type AppMs: 'static;
    type Mdl: 'static;
    type ElC: ElContainer<Self::AppMs>;

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
    ) -> OrdersProxy<ChildMs, Self::AppMs, Self::Mdl, Self::ElC, GMs>;

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
    /// Result is send to function `update`.
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

    /// Similar to `send_msg`, but calls function `g_msg_handler` with the given global message.
    fn send_g_msg(&mut self, g_msg: GMs) -> &mut Self;

    /// Similar to `perform_cmd`, but result is send to function `g_msg_handler`.
    fn perform_g_cmd<C>(&mut self, g_cmd: C) -> &mut Self
    where
        C: Future<Item = GMs, Error = GMs> + 'static;

    /// Get app instance. Cloning is cheap because `App` contains only `Rc` fields.
    /// See example `src/example/animation_frame/src/lib.rs`.
    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::ElC, GMs>;
}

// ------ OrdersContainer ------

#[allow(clippy::module_name_repetitions)]
pub struct OrdersContainer<Ms: 'static, Mdl: 'static, ElC: ElContainer<Ms>, GMs = ()> {
    pub(crate) should_render: ShouldRender,
    pub(crate) effects: VecDeque<Effect<Ms, GMs>>,
    app: App<Ms, Mdl, ElC, GMs>,
}

impl<Ms, Mdl, ElC: ElContainer<Ms>, GMs> OrdersContainer<Ms, Mdl, ElC, GMs> {
    pub fn new(app: App<Ms, Mdl, ElC, GMs>) -> Self {
        Self {
            should_render: ShouldRender::Render,
            effects: VecDeque::new(),
            app,
        }
    }
}

impl<Ms: 'static, Mdl, ElC: ElContainer<Ms>, GMs> Orders<Ms, GMs>
    for OrdersContainer<Ms, Mdl, ElC, GMs>
{
    type AppMs = Ms;
    type Mdl = Mdl;
    type ElC = ElC;

    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl Fn(ChildMs) -> Ms + 'static,
    ) -> OrdersProxy<ChildMs, Ms, Mdl, ElC, GMs> {
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

    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::ElC, GMs> {
        self.app.clone()
    }
}

// ------ OrdersProxy ------

#[allow(clippy::module_name_repetitions)]
pub struct OrdersProxy<
    'a,
    Ms,
    AppMs: 'static,
    Mdl: 'static,
    ElC: ElContainer<AppMs>,
    GMs: 'static = (),
> {
    orders_container: &'a mut OrdersContainer<AppMs, Mdl, ElC, GMs>,
    f: Rc<Fn(Ms) -> AppMs>,
}

impl<'a, Ms: 'static, AppMs: 'static, Mdl, ElC: ElContainer<AppMs>, GMs>
    OrdersProxy<'a, Ms, AppMs, Mdl, ElC, GMs>
{
    pub fn new(
        orders_container: &'a mut OrdersContainer<AppMs, Mdl, ElC, GMs>,
        f: impl Fn(Ms) -> AppMs + 'static,
    ) -> Self {
        OrdersProxy {
            orders_container,
            f: Rc::new(f),
        }
    }
}

impl<'a, Ms: 'static, AppMs: 'static, Mdl, ElC: ElContainer<AppMs>, GMs> Orders<Ms, GMs>
    for OrdersProxy<'a, Ms, AppMs, Mdl, ElC, GMs>
{
    type AppMs = AppMs;
    type Mdl = Mdl;
    type ElC = ElC;

    fn proxy<ChildMs: 'static>(
        &mut self,
        f: impl Fn(ChildMs) -> Ms + 'static,
    ) -> OrdersProxy<ChildMs, AppMs, Mdl, ElC, GMs> {
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

    fn clone_app(&self) -> App<Self::AppMs, Self::Mdl, Self::ElC, GMs> {
        self.orders_container.clone_app()
    }
}
