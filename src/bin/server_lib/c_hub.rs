use tokio::sync::mpsc::Sender;
use crate::server_lib::c_server_room::ServerRoom;
use crate::server_lib::c_server_client::{EClientState, ServerClient};

#[derive(Default)]
pub struct ServerClientsHub{
    users: Vec<ServerClient>,
    rooms: Vec<ServerRoom>,
}

impl ServerClientsHub {


    pub fn create_user(&mut self, name: String, user_id: u32, sender: Sender<String>) -> Option<&mut ServerClient> {
        let user = ServerClient::new(name, user_id, sender);

        self.users.push(user);

        let user = &self.users[self.users.len() - 1];

        self.find_user_mut(user.get_id())
    }

    pub fn find_user_mut(&mut self, id: u32) -> Option<&mut ServerClient> {
        self.users.iter_mut().find(|u| u.get_id() == id)
    }

    pub fn create_room(&mut self, name: String, size: u8) -> i32{
        let room = ServerRoom::new(name, size);
        let id = room.id;
        self.rooms.push(room);

        id as i32
    }

    pub fn has_room(&mut self, name: String) -> bool {
        self.rooms.iter().any(|u| u.name() == name)
    }

    pub fn join_room(&mut self, name: String, user_id: u32) -> bool{

        let mut room_id = -1;

        for room in self.rooms.iter_mut() {
            if (room.name() == name){
                if (room.has_empty()) {
                    room.add_user(user_id);
                    room_id = room.id as i32;
                    break;
                }
            }
        }

        if (room_id == -1){
            return false;
        }else {
            self.set_room_for_user(user_id, room_id as i32);
            return true;
        }
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
        out
    }


    pub fn set_room_for_user(&mut self, user_id: u32, room_id: i32) {
        for u in self.users.iter_mut() {
            if (user_id == u.get_id()) {
                u.set_room_id(room_id);
                break;
            }
        }
    }

    pub fn disconnect_user(&mut self, user_id: u32) {
        for room in self.rooms.iter_mut() {
            room.remove_user(user_id);
        }

        self.rooms.retain(|room| room.users_inside() > 0);
    }
    pub fn remove_user(&mut self, user_id: u32) {
       let pos = self.users.iter().position(|u| u.get_id() == user_id);

        if let Some(pos) = pos {
            self.users.remove(pos);
        }
    }

    pub fn get_user_room(&self, user_id: u32) -> Option<&ServerRoom> {
        let user = self.users.iter().find(|u| u.get_id() == user_id);

        let Some(user) = user else { return None };


        let room_id = user.get_room_id();

        let room = self.rooms.iter().find(|x| x.id == room_id as u32);

        let Some(room) = room else { return None };

        Some(room)
    }
}

