use gpui_kit::{
    base::{ResizableState, h_resizable, resizable_panel},
    *,
};

pub struct ChatMa {
    resizable_state: Entity<ResizableState>,
}

impl ChatMa {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            resizable_state: cx.new(|_| ResizableState::default()),
        }
    }

    fn main_layout(&self) -> AnyElement {
        h_resizable("main-layout")
            .with_state(&self.resizable_state)
            .child(resizable_panel().size(px(300.)).child(side_panel()))
            .child(resizable_panel().child(main_page()))
            .into_any_element()
    }
}

impl Render for ChatMa {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().flex_1().min_h_0().w_full().child(self.main_layout())
    }
}

pub fn main_page() -> Div {
    div().size_full().child("Hello from ChatMa !")
}

pub fn side_panel() -> Div {
    div().size_full().child("Side Panel")
}
