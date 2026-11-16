use gpui_kit::{
    base::v_flex,
    component::{ActiveTheme, TitleBar},
    *,
};

use crate::gui::{
    chat::ChatMa,
    login::{LoginEvent, LoginView},
};

enum Screen {
    Login(Entity<LoginView>),
    Chat(Entity<ChatMa>),
}

pub struct RootView {
    screen: Screen,
}

impl Render for RootView {
    fn render(&mut self, _w: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let screen = match &self.screen {
            Screen::Login(v) => v.clone().into_any_element(),
            Screen::Chat(v) => v.clone().into_any_element(),
        };

        v_flex()
            .size_full()
            .child(TitleBar::new().child("ChatMa").bg(cx.theme().background))
            .child(screen)
    }
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let login = cx.new(|cx| LoginView::new(window, cx));

        cx.subscribe_in(
            &login,
            window,
            |this, login, event, window, cx| match event {
                LoginEvent::LoggedIn => {
                    let chat = cx.new(|cx| ChatMa::new(window, cx));
                    this.screen = Screen::Chat(chat);
                }
            },
        )
        .detach();

        Self {
            screen: Screen::Login(login),
        }
    }
}
