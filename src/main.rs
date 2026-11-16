#![recursion_limit = "256"]
use matrix_sdk::{Client, config::SyncSettings};
use std::sync::mpsc;
use tokio::{runtime::Runtime, sync::mpsc as tokio_mpsc};

use crate::channels::enums::{Command, UiEvent};

mod channels;
mod gui;

fn main() {
    let rt = Runtime::new().expect("runtime tokio");

    let (ui_tx, ui_rx) = mpsc::channel::<UiEvent>();
    let (cmd_tx, cmd_rx) = tokio_mpsc::unbounded_channel::<Command>();

    rt.spawn(matrix_task(ui_tx, cmd_rx));

    gui::applauncher::launch(ui_rx, cmd_tx);
}

// tâche matrix dummy faite par claude pour la compil
async fn matrix_task(
    ui_tx: mpsc::Sender<UiEvent>,
    mut cmd_rx: tokio_mpsc::UnboundedReceiver<Command>,
) {
    let client = Client::builder()
        .homeserver_url("https://matrix.clement-reniers.fr")
        .build()
        .await
        .unwrap();

    client
        .matrix_auth()
        .login_username("user", "password")
        .send()
        .await
        .unwrap();
    let _ = ui_tx.send(UiEvent::LoggedIn);

    // Handler d'événements : pousse vers la GUI
    let tx = ui_tx.clone();
    client.add_event_handler(
        move |ev: matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent,
              room: matrix_sdk::Room| {
            let tx = tx.clone();
            async move {
                let _ = tx.send(UiEvent::NewMessage {
                    room: room.room_id().to_string(),
                    sender: ev.sender.to_string(),
                    body: format!("{:?}", ev.content.msgtype), // à adapter
                });
            }
        },
    );

    // Sync en tâche séparée, pendant qu'on écoute les commandes
    let sync_client = client.clone(); // Client est un Arc en interne, clone peu coûteux
    tokio::spawn(async move {
        let _ = sync_client.sync(SyncSettings::default()).await;
    });

    while let Some(cmd) = cmd_rx.recv().await {
        match cmd {
            Command::SendMessage { room_id, body } => {
                if let Some(room) = client.get_room(&room_id) {
                    use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
                    if let Err(e) = room.send(RoomMessageEventContent::text_plain(body)).await {
                        let _ = ui_tx.send(UiEvent::Error(e.to_string()));
                    }
                }
            }
            Command::Logout => break,
        }
    }
}
