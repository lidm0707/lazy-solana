use dioxus::prelude::*;

use crate::components::canvas_component::canvas_hook::PropDragNode;

use super::node_hook::PropNode;

#[component]
pub fn Node(prop: PropNode, id: usize) -> Element {
    let addr = prop.data.get("pubkey").cloned().unwrap_or_default();
    let addr = &addr[..8]; // ตัดข้อความให้แสดงเฉพาะ 8 ตัวแรก
    let mut select = use_signal(|| false);

    rsx! {
        rect {
            x: "{prop.x - 50.0}",
            y: "{prop.y - 25.0}",
            width: "100",
            height: "50",
            fill: "lightblue",
            stroke: if *select.read() {"red"} else {"navy"},
            stroke_width: "2",
            rx: "5", // Optional: For rounded corners
            onmousedown: move |_| {
                select.set(true);
                use_context::<PropDragNode>().drag_node_id.set(Some(id));
            },
            onmouseup: move |_| {
                select.set(false);
            },
            style: "cursor: pointer;",
        }

        // Node label
        text {
            x: "{prop.x}",
            y: "{prop.y}",
            text_anchor: "middle",
            dominant_baseline: "middle",
            font_size: "12",
            fill: "black",
            style: "user-select: none; pointer-events: none;", // ป้องกันการเลือกข้อความและการขัดจังหวะ event
            "{addr:?}..."
        }
    }
}
