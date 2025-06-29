use crate::components::{
    canvas_component::canvas_hook::PropDragNode, table_component::table_data::AccountDetail,
};
use dioxus::prelude::*;
use dioxus_logger::tracing;
use gloo_timers::callback::Timeout;

use super::node_hook::PropNode;

#[component]
pub fn Node(prop: PropNode, id: usize) -> Element {
    let mut select = use_signal(|| false);
    let mut long_press_timeout_handle = use_signal(|| None::<Timeout>);
    // This signal will be set to true ONLY if the 500ms timeout successfully fires.
    let mut long_press_fired = use_signal(|| false);

    if id == 0 {
        let programe_id = &prop.name_account;

        use_context::<Signal<AccountDetail>>().set(AccountDetail {
            programe_addr: programe_id.clone(),
            discriminator: vec![],
            name_account: "".to_string(),
            fields: vec![],
        });

        tracing::debug!("Program ID: {}", programe_id);
        let base_width = 100.0;
        let char_width = 10.0;
        let rect_width = base_width + prop.name_account.clone().len() as f32 * char_width;
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
                                  // Provisionally set select and drag_node_id.
                                  // These will be reverted if it's a short press.
                                  select.set(true);
                                  use_context::<PropDragNode>().drag_node_id.set(Some(id));


                                  // Ensure previous timeout is cleared if for some reason it wasn't
                                  if let Some( handle) = long_press_timeout_handle.take() {
                                      handle.cancel();
                                  }
                                  long_press_fired.set(false); // Reset for new press

                                  // Clone signals to move into the timeout closure
                                  let long_press_fired_clone = long_press_fired.to_owned();
                                  let handle = Timeout::new(500, move || {
                                      // This code runs ONLY if 500ms pass without cancellation
                                      long_press_fired_clone.clone().set(true);
                                      println!("Long press timeout successfully fired!");
                                  });
                                  long_press_timeout_handle.set(Some(handle));
                              },
                              onmouseup: move |_| {
                                  // Always clear drag_node_id on mouseup, as the drag operation naturally ends here.
                                  use_context::<PropDragNode>().drag_node_id.set(None);
                                  println!("mouseup: drag_node_id set to None");

                                  // Attempt to cancel the timeout. If it hasn't fired yet, it will be cancelled.
                                  // .take() moves the Option content out, leaving None.
                                  if let Some( handle) = long_press_timeout_handle.take() {
                                      handle.cancel();
                                      println!("mouseup: Cancelled pending timeout.");
                                  }

                                  // Check if the long press timeout actually fired
                                  if *long_press_fired.read() {
                                      // This is a long press: confirm selection and trigger Discriminator.
                                      // `select` remains true as set in `onmousedown`.
                                      //
                                      let programe_id = use_context::<Signal<AccountDetail>>()().programe_addr;
                                      use_context::<Signal<AccountDetail>>().set(AccountDetail {
                                          programe_addr: programe_id,
                                           discriminator: prop.discriminator.clone(),
                                           name_account: account_name.clone(),
                                           fields: prop.fields.clone(),
                                      });
                                      println!("Long press (timeout fired): Discriminator set. select remains true.");
                                  } else {
                                      // This is a short press: revert provisional select and
                                      // do NOT trigger Discriminator.
                                      select.set(false); // Revert selection state
                                      println!("Short press (timeout not fired): select set to false. No Discriminator action.");
                                  }
                                  // Reset long_press_fired for the next interaction
                                  long_press_fired.set(false);
                              },
                              onmouseleave: move |_| {
                                  // If mouse leaves while pressed, clear any pending timeout
                                  if let Some( handle) = long_press_timeout_handle.take() {
                                      handle.cancel();
                                      println!("mouseleave: Cancelled pending timeout.");
                                  }

                                  // Revert provisional states
                                  select.set(false);
                                  use_context::<PropDragNode>().drag_node_id.set(None);
                                  long_press_fired.set(false); // Ensure it's reset
                                  println!("mouseleave: Cleared select, drag_node_id, long_press_fired.");
                              },
                style: "cursor: pointer;",

                // ondoubleclick: move |_| {
                //     use_context::<Signal<Discriminator>>().set(Discriminator {
                //         discriminator: prop.discriminator.clone(),
                //     });
                // }
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
