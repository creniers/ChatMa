use gpui_kit::{Context, Entity, EventEmitter, Global};
use tokio::sync::mpsc::UnboundedReceiver;

use crate::channels::enums::UiEvent;

pub struct MatrixModel;
impl EventEmitter<UiEvent> for MatrixModel {}

impl MatrixModel {
    pub fn new(mut rx: UnboundedReceiver<UiEvent>, cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this, cx| {
            while let Some(ev) = rx.recv().await {
                if this.update(cx, |_, cx| cx.emit(ev)).is_err() {
                    break; // l'entité n'existe plus
                }
            }
        })
        .detach();
        Self
    }
}

pub struct MatrixReceiver(pub Entity<MatrixModel>);
impl Global for MatrixReceiver {}
