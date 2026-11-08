use gpui::{Context, SharedString, Window, div, prelude::*, px, rgb};

use crate::theme;

pub struct HelloWorld {
    text: SharedString,
}

impl HelloWorld {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(theme::BACKGROUND))
            .child(render_titlebar())
            .child(self.render_content())
    }
}

fn render_titlebar() -> impl IntoElement {
    div()
        .id("titlebar")
        .flex_none()
        .h(px(theme::TITLEBAR_HEIGHT))
        .flex()
        .items_center()
        .justify_center()
        .bg(rgb(theme::TITLEBAR_BACKGROUND))
        .border_b_1()
        .border_color(rgb(theme::TITLEBAR_BORDER_COLOR))
        .text_sm()
        .text_color(rgb(theme::TEXT))
        .child("nb")
        .on_click(|event, window, _| {
            if event.click_count() == 2 {
                window.titlebar_double_click();
            }
        })
}

impl HelloWorld {
    fn render_content(&self) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .gap_3()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(theme::TEXT))
            .child(format!("Hello, {}!", &self.text))
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(div().size_8().bg(rgb(theme::everforest::RED)))
                    .child(div().size_8().bg(rgb(theme::everforest::YELLOW)))
                    .child(div().size_8().bg(rgb(theme::everforest::GREEN)))
                    .child(div().size_8().bg(rgb(theme::everforest::BLUE)))
                    .child(div().size_8().bg(rgb(theme::everforest::PURPLE))),
            )
    }
}
