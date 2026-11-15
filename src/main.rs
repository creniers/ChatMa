mod content;

use std::path::PathBuf;

use gpui_kit::base::{h_flex, v_flex};
use gpui_kit::component::resizable::{ResizableState, h_resizable, resizable_panel};
use gpui_kit::component::{ActiveTheme, Theme, ThemeRegistry, TitleBar};
use gpui_kit::*;

use crate::content::{main_page, side_panel};

struct ChatMa {
    resizable_state: Entity<ResizableState>,
}

impl ChatMa {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // créé une seule fois, jamais dans render()
            resizable_state: cx.new(|_| ResizableState::default()),
        }
    }

    fn main_layout(&self) -> impl IntoElement {
        h_resizable("main-layout")
            .with_state(&self.resizable_state)
            .child(resizable_panel().size(px(300.)).child(side_panel()))
            .child(resizable_panel().child(main_page()))
    }
}

impl Render for ChatMa {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .child(TitleBar::new().child("ChatMa").bg(cx.theme().background))
            .child(div().flex_1().min_h_0().w_full().child(self.main_layout()))
    }
}

fn load_solarized_theme(cx: &mut App) {
    let dark_theme_name = SharedString::from("Solarized Light");
    let light_theme_name = SharedString::from("Solarized Dark");

    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        let themes = ThemeRegistry::global(cx).themes().clone();

        if let Some(dark_theme) = themes.get(&dark_theme_name).cloned() {
            Theme::update(cx, |cur| cur.apply_config(&dark_theme))
        };

        if let Some(light_theme) = themes.get(&light_theme_name).cloned() {
            Theme::update(cx, |cur| cur.apply_config(&light_theme))
        }
    }) {
        panic!("No theme dir found {}", err);
    }
}

fn main() {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        open_window(TitleBar::window_options(), cx, |window, cx| {
            window
                .observe_window_appearance(|window, cx| {
                    Theme::sync_system_appearance(Some(window), cx);
                })
                .detach();

            load_solarized_theme(cx);

            cx.new(|cx| ChatMa::new(cx))
        })
        .expect("Failed to open window");
    });
}
