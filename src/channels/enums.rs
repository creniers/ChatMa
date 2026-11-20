use matrix_sdk::ruma::OwnedRoomId;

// GUI -> Matrix
pub enum Command {
    Connect {
        homeserver_url: String,
        username: String,
        password: String,
    },
    SendMessage {
        room_id: OwnedRoomId,
        body: String,
    },
    Logout,
}

// Matrix -> GUI
pub enum UiEvent {
    LoggedIn,
    ErrorLogin(String),
    NewMessage {
        room: String,
        sender: String,
        body: String,
    },
    Error(String),
}
