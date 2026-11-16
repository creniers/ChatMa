use gpui_kit::base::Input;
use gpui_kit::base::input::InputState;
use gpui_kit::component::Theme;
use gpui_kit::*;

pub fn text_field(input_state: &Entity<InputState>, theme: &Theme) -> impl IntoElement {
    div()
        .border_b_2()
        .px_6()
        .py_3()
        .border_color(theme.border)
        .bg(theme.secondary)
        .child(Input::new(input_state))
}
