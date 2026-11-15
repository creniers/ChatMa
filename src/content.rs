use gpui_kit::*;

pub fn main_page() -> Div {
    div()
        .size_full()
        .bg(rgb(0xFF0000))
        .child("Hello from ChatMa !")
}

pub fn side_panel() -> Div {
    div().size_full().bg(rgb(0x00FF00)).child("Side Panel")
}
