use dioxus::prelude::*;
#[derive(Props,PartialEq,Clone)]
pub struct PropDragNode {
    pub drag_node_id:Signal<Option<usize>>
}