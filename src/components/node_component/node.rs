use dioxus::prelude::*;

use crate::components::canvas_component::canvas_hook::PropDragNode;

use super::node_hook::PropNode;

#[component]
pub fn Node(prop: PropNode, id: usize) -> Element {
    let mut select = use_signal(|| false);

    if id == 0 {
        let programe_id = &prop.name_account;
        let base_width = 100.0;
        let char_width = 10.0;
        let rect_width = base_width + programe_id.len() as f32 * char_width;
        let rect_height = 40.0;
        let fill_color = "lightcoral"; // Always light mode
        let stroke_color = if *select.read() { "black" } else { "darkred" }; // Always light mode
        let text_color = "black"; // Always light mode

        rsx! {
            rect {
                x: "{prop.x - rect_width / 2.0}",
                y: "{prop.y - rect_height / 2.0}",
                width: "{rect_width}",
                height: "{rect_height}",
                fill: "{fill_color}",
                stroke: "{stroke_color}",
                stroke_width: "2",
                rx: "5",
                onmousedown: move |_| {
                    select.set(true);
                    use_context::<PropDragNode>().drag_node_id.set(Some(id));
                },
                onmouseup: move |_| {
                    select.set(false);
                },
                style: "cursor: pointer;",
            }
            text {
                x: "{prop.x}",
                y: "{prop.y}",
                text_anchor: "middle",
                dominant_baseline: "middle",
                font_size: "12",
                fill: "{text_color}",
                style: "user-select: none; pointer-events: none;",
                "Program ID: {programe_id}"
            }
        }
    } else {
        let account_name = prop.name_account;

        let rect_width = 160.0; // Slightly wider for more data
        let rect_height = 90.0; // Increased height for four lines
        let line_spacing = 16.0; // Space between lines

        // Calculate y positions for four lines, centered around prop.y
        // Total height of text block = 3 * line_spacing
        // Top of text block = prop.y - (1.5 * line_spacing)
        let y_line1 = prop.y - 1.5 * line_spacing;

        let fill_color = "lightblue"; // Always light mode
        let stroke_color = if *select.read() { "red" } else { "navy" }; // Always light mode
        let main_text_color = "black"; // Always light mode
        let data_text_color = "dimgray"; // Always light mode

        rsx! {
            rect {
                x: "{prop.x - rect_width / 2.0}",
                y: "{prop.y - rect_height / 2.0}",
                width: "{rect_width}",
                height: "{rect_height}",
                fill: "{fill_color}",
                stroke: "{stroke_color}",
                stroke_width: "2",
                rx: "5",
                onmousedown: move |_| {
                    select.set(true);
                    use_context::<PropDragNode>().drag_node_id.set(Some(id));
                },
                onmouseup: move |_| {
                    select.set(false);
                },
                style: "cursor: pointer;",
            }

            text { // Pubkey
                x: "{prop.x}", y: "{y_line1}",
                text_anchor: "middle", dominant_baseline: "middle", font_size: "12", fill: "{main_text_color}",
                style: "user-select: none; pointer-events: none;",
                "{account_name}"
            }
            for (i,data) in prop.fields.iter().enumerate() {
                text {
                    x: "{prop.x}", y: "{prop.y + i as f32 * line_spacing}",
                    text_anchor: "middle", dominant_baseline: "middle", font_size: "12", fill: "{data_text_color}",
                    style: "user-select: none; pointer-events: none;",
                    "{data.name}: {data.ty}"
                }
            }

        }
    }
}
