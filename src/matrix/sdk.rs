use crate::channels::{
    enums::{Command, UiEvent},
    enums_gui::RoomGui,
};
use matrix_sdk::{Client, Room, config::SyncSettings, sliding_sync::VersionBuilder};
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
                        .sliding_sync_version_builder(VersionBuilder::DiscoverNative)
                        .build()
                        .await
                    {
                        Ok(client) => {
                            self.client = Some(client.clone());
                            match client
                                .matrix_auth()
                                .login_username(username, &password)
                                .initial_device_display_name("chatma")
                                .await
                            {
                                Ok(_) => {
                                    // first sync
                                    client
                                        .sync_once(SyncSettings::default())
                                        .await
                                        .expect("Sync failed !");

                                    let _ = self.tx.send(UiEvent::LoggedIn);
                                }
                                Err(e) => {
                                    let _ = self
                                        .tx
                                        .send(UiEvent::Error(format!("Connexion error : {}", e)));
                                }
                            };

                            // Gérer l'auth
                        }
                        Err(cbe) => {
                            let _ = self.tx.send(UiEvent::Error(format!(
                                "Connexion error : {}",
                                cbe.to_string()
                            )));
                        }
                    }
                }
                Command::GetRooms => {
                    if let Some(client) = self.client.clone() {
                        let mut rooms_info = Vec::new();
                        for room in client.joined_rooms() {
                            rooms_info.push(to_room_info(&room).await);
                        }
                        let _ = self.tx.send(UiEvent::Rooms(rooms_info));
                    }
                }
                Command::SendMessage {
                    room_id: _,
                    body: _,
                } => {
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

async fn to_room_info(room: &Room) -> RoomGui {
    let name = room
        .display_name()
        .await
        .map(|name| name.to_string())
        .unwrap_or(room.room_id().to_string());

    // no avatar for now

    RoomGui {
        id: room.room_id().to_string(),
        name,
        avatar: None,
    }
}
