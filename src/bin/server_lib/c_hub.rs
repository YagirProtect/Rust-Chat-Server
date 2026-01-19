use crate::server_lib::c_server_room::ServerRoom;
use crate::server_lib::c_server_client::ServerClient;

#[derive(Default)]
pub struct ServerClientsHub{
    users: Vec<ServerClient>,
    rooms: Vec<ServerRoom>,
}

impl ServerClientsHub {
    pub fn create_user(&mut self, name: String) -> &ServerClient {
        let user = ServerClient::new(name);

        self.users.push(user);

        let user = &self.users[self.users.len() - 1];

        user
    }

    pub fn create_room(&mut self, name: String, size: u8){
        let room = ServerRoom::new(name, size);

        self.rooms.push(room);
    }

    pub fn join_room(&mut self, name: String, user_id: u32) -> bool{
        for room in self.rooms.iter_mut() {
            if (room.name() == name){
                if (room.has_empty()) {
                    room.add_user(user_id);
                    return true;
                }
            }
        }

        return false;
    }

    pub fn get_rooms_table(&self) -> String {
        let w_idx = 3usize;

        let w_room = self
            .rooms
            .iter()
            .map(|r| r.name().len())
            .max()
            .unwrap_or(4)
            .max("Room".len())
            .max("empty".len());

        let w_users = self
            .rooms
            .iter()
            .map(|r| r.users_inside().to_string().len())
            .max()
            .unwrap_or("Users".len())
            .max("Users".len())
            .max("0".len());

        let sep = format!(
            "+-{:-<w_idx$}-+-{:-<w_room$}-+-{:-<w_users$}-+\n",
            "", "", "",
            w_idx = w_idx,
            w_room = w_room,
            w_users = w_users
        );

        let mut out = String::new();
        out.push_str(&sep);

        out.push_str(&format!(
            "| {:<w_idx$} | {:<w_room$} | {:<w_users$} |\n",
            "#", "Room", "Users",
            w_idx = w_idx,
            w_room = w_room,
            w_users = w_users
        ));

        out.push_str(&sep);

        if self.rooms.is_empty() {
            out.push_str(&format!(
                "| {:<w_idx$} | {:<w_room$} | {:<w_users$} |\n",
                " ", "empty", " ",
                w_idx = w_idx,
                w_room = w_room,
                w_users = w_users
            ));
        } else {
            for (i, room) in self.rooms.iter().enumerate() {
                out.push_str(&format!(
                    "| {:<w_idx$} | {:<w_room$} | {:<w_users$} |\n",
                    i + 1,
                    room.name(),
                    room.users_inside(),
                    w_idx = w_idx,
                    w_room = w_room,
                    w_users = w_users
                ));
            }
        }

        out.push_str(&sep);


        out.push_str("/join [name] - to join room\n");
        out.push_str("/create_room [name] [size] - to create own room\n");
        out
    }
}

