use crate::channels::enums::{Command, UiEvent};
use matrix_sdk::{
    Client,
    ruma::{UserId, user_id},
};
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
                            self.client = Some(client.clone());
                            match client
                                .matrix_auth()
                                .login_username(username, &password)
                                .initial_device_display_name("chatma")
                                .await
                            {
                                Ok(_) => {
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
