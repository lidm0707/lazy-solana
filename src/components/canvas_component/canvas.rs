use dioxus::prelude::*;

use crate::{
    components::{
        canvas_component::canvas_hook::PropDragNode, node_component::node_hook::PropNodes,
    },
    theme::Theme,
};

#[component]
pub fn Canvas(nodes: PropNodes, child: Element) -> Element {
    let drag_node = PropDragNode {
        drag_node_id: use_signal(|| None::<usize>),
    };
    provide_context(drag_node.to_owned());
    rsx! {
        {
            let drag_onmouse = drag_node.to_owned();
            let drag_upmouse = drag_node.to_owned();
            let is_dark_mode = use_context::<Signal<Theme>>()() == Theme::Dark;
            rsx! {
                div { class: "relative",
                    svg {
                        width: "800",
                        height: "600",
                        class: if is_dark_mode{
                            "border border-slate-600 bg-slate-800"
                        } else {
                            "border border-gray-300 bg-white"
                        },
                        style: "cursor: grab;",
                        // Handle dragging
                        onmousemove: move |e| {
                            let nodes = nodes.to_owned();
                            if let Some(dragged) = *drag_onmouse.to_owned().drag_node_id.read() {
                                if let Some(node) = nodes
                                    .to_owned()
                                    .list_nodes
                                    .write()
                                    .iter_mut()
                                    .find(|n| n.id == dragged)
                                {
                                    let rect = e.data().element_coordinates();
                                    node.x = rect.x as f32;
                                    node.y = rect.y as f32;
                                }
                            }
                        },
                        onmouseup: move |_| drag_upmouse.to_owned().drag_node_id.set(None),
                        {child}
                    }
                }
            }
        }
    }
}
