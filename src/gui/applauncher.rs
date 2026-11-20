use std::path::PathBuf;

use gpui_kit::{
    component::{Theme, ThemeRegistry, TitleBar},
    *,
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    channels::enums::{Command, UiEvent},
    gui::root::RootView,
    matrix::{
        receiver::{MatrixModel, MatrixReceiver},
        sender::MatrixSender,
    },
};

pub fn launch(rx: UnboundedReceiver<UiEvent>, tx: UnboundedSender<Command>) {
    application().with_assets(assets::Assets).run(|cx| {
        init(cx);

        // set the tx as global so that every screen can use it
        cx.set_global(MatrixSender { tx });

        // set the rx as global s.t. every screen can subscribe to it
        let matrix = cx.new(|cx| MatrixModel::new(rx, cx));
        cx.set_global(MatrixReceiver(matrix));

        open_window(TitleBar::window_options(), cx, |window, cx| {
            window
                .observe_window_appearance(|window, cx| {
                    Theme::sync_system_appearance(Some(window), cx);
                })
                .detach();

            load_solarized_theme(cx);

            cx.new(|cx| RootView::new(window, cx))
        })
        .expect("Failed to open window");
    });
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
