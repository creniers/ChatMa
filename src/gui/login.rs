use gpui_kit::{
    base::input::InputState,
    component::{ActiveTheme, button::Button},
    *,
};

use crate::gui::components::text_field;

pub enum LoginEvent {
    LoggedIn,
}

impl EventEmitter<LoginEvent> for LoginView {}

pub struct LoginView {
    homeserver_url: Entity<InputState>,
    username: Entity<InputState>,
    password: Entity<InputState>,
}

impl LoginView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let homeserver_url = cx.new(|cx| InputState::new(window, cx).placeholder("Homeserver URL"));
        let username = cx.new(|cx| InputState::new(window, cx).placeholder("Username"));
        let password = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Password")
                .masked(true)
        });
        Self {
            homeserver_url,
            username,
            password,
        }
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        let homeserver_content = self.homeserver_url.read(cx).content();
        let homeserver = homeserver_content.text();
        let username_content = self.username.read(cx).content();
        let user = username_content.text();
        let pass_content = self.password.read(cx).content();
        let pass = pass_content.text();
        let _ = (user, pass, homeserver);

        cx.emit(LoginEvent::LoggedIn);
    }
}

impl Render for LoginView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        div()
            .h_full()
            .flex()
            .flex_col()
            .justify_center()
            .gap_3()
            .p_4()
            .child(text_field(&self.username, theme))
            .child(text_field(&self.password, theme))
            .child(
                Button::new("Validate")
                    .label("Connect")
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.submit(cx);
                    })),
            )
    }
}
