use super::types::{SinkFn, UpdateFn, ViewFn, WindowEventsFn};
use super::{builder::IntoAfterMount, MountType};
use crate::virtual_dom::IntoNodes;
use std::marker::PhantomData;
use std::rc::Rc;

// @TODO_B: Remove.
#[allow(clippy::module_name_repetitions)]
pub struct AppInitCfg<Ms, Mdl, INodes, GMs, IAM: ?Sized>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
    IAM: IntoAfterMount<Ms, Mdl, INodes, GMs>,
{
    pub mount_type: MountType,
    pub into_after_mount: Box<IAM>,
    pub phantom: PhantomData<(Ms, Mdl, INodes, GMs)>,
}

#[allow(clippy::module_name_repetitions)]
pub struct AppCfg<Ms, Mdl, INodes, GMs>
where
    Ms: 'static,
    Mdl: 'static,
    INodes: IntoNodes<Ms>,
{
    pub document: web_sys::Document,
    // @TODO_B: Remove?
    pub mount_point: web_sys::Element,
    pub update: UpdateFn<Ms, Mdl, INodes, GMs>,
    // @TODO_B: Remove.
    pub sink: Option<SinkFn<Ms, Mdl, INodes, GMs>>,
    pub view: ViewFn<Mdl, INodes>,
    // @TODO_B: Remove.
    pub window_events: Option<WindowEventsFn<Ms, Mdl>>,
    pub base_path: Rc<Vec<String>>,
}
