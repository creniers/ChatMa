use matrix_sdk::ruma::OwnedRoomId;

// GUI -> Matrix
pub enum Command {
    SendMessage { room_id: OwnedRoomId, body: String },
    Logout,
}

// Matrix -> GUI
pub enum UiEvent {
    LoggedIn,
    NewMessage {
        room: String,
        sender: String,
        body: String,
    },
    Error(String),
}
