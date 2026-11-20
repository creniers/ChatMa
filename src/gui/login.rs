use gpui_kit::{
    base::{StyledExt, input::InputState},
    component::{ActiveTheme, button::Button},
    prelude::FluentBuilder,
    *,
};

use crate::{channels::enums::Command, matrix::sender::MatrixSender};
use crate::{
    channels::enums::UiEvent, gui::components::text_field, matrix::receiver::MatrixReceiver,
};

pub struct LoginView {
    homeserver_url: Entity<InputState>,
    username: Entity<InputState>,
    password: Entity<InputState>,
    error_message: Option<String>,
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

        let matrix = cx.global::<MatrixReceiver>().0.clone();
        cx.subscribe_in(&matrix, window, |this, _m, ev: &UiEvent, _window, _cx| {
            if let UiEvent::Error(e) = ev {
                this.error_message = Some(e.to_string());
            }
        })
        .detach();

        Self {
            homeserver_url,
            username,
            password,
            error_message: None,
        }
    }

    fn submit(&mut self, cx: &mut Context<Self>) {
        let homeserver_content = self.homeserver_url.read(cx).content();
        let mut homeserver = homeserver_content.text().to_string();
        // if doesn't inclure https://, we add it
        if !homeserver.starts_with("https://") {
            homeserver = format!("https://{}", homeserver)
        }
        let username_content = self.username.read(cx).content();
        let user = username_content.text().to_string();
        let pass_content = self.password.read(cx).content();
        let pass = pass_content.text().to_string();

        cx.global::<MatrixSender>().send(Command::Connect {
            homeserver_url: homeserver,
            username: user,
            password: pass,
        })
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
            .child(text_field(&self.homeserver_url, theme))
            .child(text_field(&self.username, theme))
            .child(text_field(&self.password, theme))
            .child(
                Button::new("Validate")
                    .label("Connect")
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.submit(cx);
                    })),
            )
            .when(self.error_message.is_some(), |this| {
                this.child(
                    div()
                        .border_1()
                        .border_color(rgb(0xFF0000))
                        .text_color(rgb(0xFF0000))
                        .text_center()
                        .corner_radii(Corners::all(px(10.)))
                        .child(self.error_message.clone().unwrap_or("".to_string())),
                )
            })
    }
}
