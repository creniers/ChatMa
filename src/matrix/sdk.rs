use crate::channels::enums::{Command, UiEvent};
use matrix_sdk::Client;
use tokio::sync::mpsc;

pub struct Sdk {
    client: Option<Client>,
    tx: mpsc::UnboundedSender<UiEvent>,
    rx: mpsc::UnboundedReceiver<Command>,
}

impl Sdk {
    pub fn new(tx: mpsc::UnboundedSender<UiEvent>, rx: mpsc::UnboundedReceiver<Command>) -> Self {
        Self {
            client: None,
            tx,
            rx,
        }
    }

    pub async fn launch_sdk(mut self) {
        println!("Sdk lancé");

        while let Some(cmd) = self.rx.recv().await {
            match cmd {
                Command::Connect {
                    homeserver_url,
                    username,
                    password,
                } => {
                    match Client::builder()
                        .homeserver_url(homeserver_url)
                        .build()
                        .await
                    {
                        Ok(client) => {
                            self.client = Some(client);
                            let _ = self.tx.send(UiEvent::LoggedIn);

                            // Gérer l'auth
                        }
                        Err(cbe) => {
                            eprintln!("Error while connecting to the server : {}", cbe.to_string())
                        }
                    }
                }
                Command::SendMessage { room_id, body } => {
                    // if let Some(room) = client.get_room(&room_id) {
                    //     use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
                    //     if let Err(e) = room.send(RoomMessageEventContent::text_plain(body)).await {
                    //         let _ = ui_tx.send(UiEvent::Error(e.to_string()));
                    //     }
                    // }
                }
                Command::Logout => break,
            }
        }
    }
}

// // tâche matrix dummy faite par claude pour la compil
// async fn matrix_task(
//     ui_tx: mpsc::Sender<UiEvent>,
//     mut cmd_rx: tokio_mpsc::UnboundedReceiver<Command>,
// ) {
//     let client = Client::builder()
//         .homeserver_url("https://matrix.clement-reniers.fr")
//         .build()
//         .await
//         .unwrap();

//     client
//         .matrix_auth()
//         .login_username("user", "password")
//         .send()
//         .await
//         .unwrap();
//     let _ = ui_tx.send(UiEvent::LoggedIn);

//     // Handler d'événements : pousse vers la GUI
//     let tx = ui_tx.clone();
//     client.add_event_handler(
//         move |ev: matrix_sdk::ruma::events::room::message::OriginalSyncRoomMessageEvent,
//               room: matrix_sdk::Room| {
//             let tx = tx.clone();
//             async move {
//                 let _ = tx.send(UiEvent::NewMessage {
//                     room: room.room_id().to_string(),
//                     sender: ev.sender.to_string(),
//                     body: format!("{:?}", ev.content.msgtype), // à adapter
//                 });
//             }
//         },
//     );

//     // Sync en tâche séparée, pendant qu'on écoute les commandes
//     let sync_client = client.clone(); // Client est un Arc en interne, clone peu coûteux
//     tokio::spawn(async move {
//         let _ = sync_client.sync(SyncSettings::default()).await;
//     });

//     while let Some(cmd) = cmd_rx.recv().await {
//         match cmd {
//             Command::SendMessage { room_id, body } => {
//                 if let Some(room) = client.get_room(&room_id) {
//                     use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
//                     if let Err(e) = room.send(RoomMessageEventContent::text_plain(body)).await {
//                         let _ = ui_tx.send(UiEvent::Error(e.to_string()));
//                     }
//                 }
//             }
//             Command::Logout => break,
//         }
//     }
// }
