use gpui_kit::Global;
use tokio::sync::mpsc::UnboundedSender;

use crate::channels::enums::Command;

pub struct MatrixSender {
    pub tx: UnboundedSender<Command>,
}
impl Global for MatrixSender {}

impl MatrixSender {
    pub fn send(&self, cmd: Command) {
        let _ = self.tx.send(cmd);
    }
}
