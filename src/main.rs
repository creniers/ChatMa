#![recursion_limit = "256"]
use tokio::{runtime::Runtime, sync::mpsc};

use crate::{
    channels::enums::{Command, UiEvent},
    matrix::sdk::Sdk,
};

mod channels;
mod gui;
mod matrix;

fn main() {
    let rt = Runtime::new().expect("runtime tokio");

    let (ui_tx, ui_rx) = mpsc::unbounded_channel::<UiEvent>();
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel::<Command>();

    // launch the sdk
    let sdk = Sdk::new(ui_tx, cmd_rx);
    rt.spawn(sdk.launch_sdk());

    // launch the gui
    gui::applauncher::launch(ui_rx, cmd_tx);
}
