use gpui_kit::{
    base::{ResizableState, h_resizable, resizable_panel},
    component::scroll::ScrollableElement,
    prelude::FluentBuilder,
    *,
};

use crate::{
    channels::{
        enums::{Command, UiEvent},
        enums_gui::RoomGui,
    },
    matrix::{
        receiver::{MatrixModel, MatrixReceiver},
        sender::MatrixSender,
    },
};

pub struct ChatMa {
    resizable_state: Entity<ResizableState>,
    rooms: Option<Vec<RoomGui>>,
}

impl ChatMa {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        cx.global::<MatrixSender>().send(Command::GetRooms);

        let matrix = cx.global::<MatrixReceiver>().0.clone();
        cx.subscribe_in(&matrix, window, Self::on_event).detach();

        Self {
            resizable_state: cx.new(|_| ResizableState::default()),
            rooms: None,
        }
    }

    fn main_layout(&self) -> AnyElement {
        h_resizable("main-layout")
            .with_state(&self.resizable_state)
            .child(resizable_panel().size(px(300.)).child(self.side_panel()))
            .child(resizable_panel().child(self.main_page()))
            .into_any_element()
    }

    fn on_event(
        &mut self,
        _m: &Entity<MatrixModel>,
        ev: &UiEvent,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        match ev {
            UiEvent::LoggedIn | UiEvent::ErrorLogin(_) => {}
            UiEvent::Rooms(room_guis) => self.rooms = Some(room_guis.clone()),
            UiEvent::NewMessage {
                room: _,
                sender: _,
                body: _,
            } => todo!(),
            UiEvent::Error(_) => todo!(),
        }
    }

    pub fn main_page(&self) -> Div {
        div().size_full().child("Hello from ChatMa !")
    }

    pub fn side_panel(&self) -> impl IntoElement {
        div()
            .size_full()
            .overflow_y_scrollbar()
            .when_none(&self.rooms, |this| {
                this.flex()
                    .items_center()
                    .justify_center()
                    .child("You have no rooms for now")
            })
            .when_some(self.rooms.clone(), |this, rooms| {
                this.children(rooms.iter().map(|r| r.name.to_string()))
            })
    }
}

impl Render for ChatMa {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().flex_1().min_h_0().w_full().child(self.main_layout())
    }
}
