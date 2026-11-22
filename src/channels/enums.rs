use matrix_sdk::ruma::OwnedRoomId;

use crate::channels::enums_gui::RoomGui;

// GUI -> Matrix
pub enum Command {
    Connect {
        homeserver_url: String,
        username: String,
        password: String,
    },
    GetRooms,
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
    Rooms(Vec<RoomGui>),
    NewMessage {
        room: String,
        sender: String,
        body: String,
    },
    Error(String),
}
