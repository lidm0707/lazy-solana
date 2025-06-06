use dioxus::prelude::*;

use crate::components::canvas_component::canvas_hook::PropDragNode;

use super::node_hook::PropNode;

#[component]
pub fn Node(prop: PropNode, id: usize) -> Element {
    let mut select = use_signal(|| false);
    let pubkey = prop.data.get("pubkey").cloned().unwrap_or_default();

    if id == 0 { // Program ID Node
        let display_pubkey = if pubkey.len() > 20 {
            format!("{}...{}", &pubkey[..8], &pubkey[pubkey.len()-8..])
        } else {
            pubkey.clone()
        };
        let rect_width = 220.0;
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
                "Program ID: {display_pubkey}"
            }
        }
    } else { // Account Nodes
        let display_pubkey = if pubkey.len() > 8 {
            format!("{}...", &pubkey[..8])
        } else {
            pubkey.clone()
        };
        let sol = prop.lamports as f64 / 1_000_000_000.0;
        let lamports_text = format!("{:.4} SOL", sol);
        let executable_text = if prop.executable { "Exec: Yes" } else { "Exec: No" };
        
        let account_data_str = &prop.account_data_display;
        let max_data_len = 20; // Max length for data string preview
        let data_preview = if account_data_str.len() > max_data_len {
            // Ensure we don't panic on short strings if max_data_len is small
            let end_index = max_data_len.saturating_sub(3);
            if end_index > 0 && end_index <= account_data_str.len() {
                 format!("Data: {}...", &account_data_str[..end_index])
            } else if !account_data_str.is_empty() {
                format!("Data: {}", account_data_str) // Show full if too short to truncate with "..."
            } else {
                String::from("Data: (empty)")
            }
        } else {
            format!("Data: {}", account_data_str)
        };

        let rect_width = 160.0; // Slightly wider for more data
        let rect_height = 90.0; // Increased height for four lines
        let line_spacing = 16.0; // Space between lines

        // Calculate y positions for four lines, centered around prop.y
        // Total height of text block = 3 * line_spacing
        // Top of text block = prop.y - (1.5 * line_spacing)
        let y_line1 = prop.y - 1.5 * line_spacing;
        let y_line2 = prop.y - 0.5 * line_spacing;
        let y_line3 = prop.y + 0.5 * line_spacing;
        let y_line4 = prop.y + 1.5 * line_spacing;

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
                "{display_pubkey}"
            }
            text { // Lamports
                x: "{prop.x}", y: "{y_line2}",
                text_anchor: "middle", dominant_baseline: "middle", font_size: "12", fill: "{main_text_color}",
                style: "user-select: none; pointer-events: none;",
                "{lamports_text}"
            }
            text { // Executable
                x: "{prop.x}", y: "{y_line3}",
                text_anchor: "middle", dominant_baseline: "middle", font_size: "12", fill: "{main_text_color}",
                style: "user-select: none; pointer-events: none;",
                "{executable_text}"
            }
            text { // Account Data Preview
                x: "{prop.x}", y: "{y_line4}",
                text_anchor: "middle", dominant_baseline: "middle", font_size: "10", fill: "{data_text_color}",
                style: "user-select: none; pointer-events: none;",
                "{data_preview}"
            }
        }
    }
}