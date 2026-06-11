#![cfg_attr(target_family = "wasm", no_main)]

use gpui::{
    App, BlurEffect, Bounds, Context, Window, WindowBounds, WindowOptions, div, hsla,
    linear_color_stop, linear_gradient, prelude::*, px, rgb, rgba, size,
};
use gpui_platform::application;

/// Demonstrates [`Styled::backdrop_blur`]: an in-app modal whose background
/// frosts the content behind it, like CSS `backdrop-filter: blur(..)`.
///
/// The window paints busy, colorful content. On top of it sits a full-window
/// scrim with a backdrop blur and a dark translucent tint, and a centered
/// dialog panel with a stronger blur of its own. Click anywhere to toggle the
/// modal and compare against the unblurred content.
struct ModalDemo {
    modal_open: bool,
}

impl Render for ModalDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let swatch = |hue: f32, ix: usize| {
            div()
                .id(("swatch", ix))
                .w(px(120.))
                .h(px(80.))
                .rounded(px(12.))
                .bg(linear_gradient(
                    135.,
                    linear_color_stop(hsla(hue, 0.9, 0.6, 1.0), 0.),
                    linear_color_stop(hsla((hue + 0.12) % 1.0, 0.9, 0.45, 1.0), 1.),
                ))
        };

        let content = div()
            .flex()
            .flex_col()
            .gap_4()
            .p_8()
            .child(
                div()
                    .text_2xl()
                    .text_color(rgb(0x111111))
                    .child("Busy background content"),
            )
            .children((0..6).map(|row| {
                div().flex().flex_row().gap_4().children(
                    (0..5).map(move |col| swatch((row * 5 + col) as f32 * 0.033, row * 5 + col)),
                )
            }));

        let modal = div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            // Scrim: light blur + dark tint over the whole window.
            .backdrop_blur(px(6.))
            .bg(rgba(0x00000055))
            .child(
                div()
                    .w(px(420.))
                    .rounded(px(16.))
                    .border_1()
                    .border_color(rgba(0xffffff22))
                    // Dialog: heavier frost under a translucent dark fill, for a
                    // dark frosted-glass look that reads over bright content.
                    .backdrop_blur_effect(BlurEffect {
                        radius: px(28.),
                        ..Default::default()
                    })
                    .bg(rgba(0x1e1e28cc))
                    .p_6()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xl()
                            .text_color(rgb(0xf5f5f5))
                            .child("Frosted modal"),
                    )
                    .child(div().text_sm().text_color(rgb(0xc0c0c8)).child(
                        "This panel's backdrop_blur samples and frosts the \
                         swatches behind it. Click anywhere to dismiss.",
                    )),
            );

        div()
            .id("root")
            .size_full()
            .bg(rgb(0xf6f6f6))
            .on_click(cx.listener(|this, _, _, cx| {
                this.modal_open = !this.modal_open;
                cx.notify();
            }))
            .child(content)
            .when(self.modal_open, |root| root.child(modal))
    }
}

fn run_example() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(900.), px(640.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| ModalDemo { modal_open: true }),
        )
        .unwrap();
        cx.activate(true);
    });
}

#[cfg(not(target_family = "wasm"))]
fn main() {
    run_example();
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    gpui_platform::web_init();
    run_example();
}
