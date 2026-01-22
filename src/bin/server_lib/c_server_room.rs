use std::sync::atomic::AtomicU32;

pub static ROOMS_IDS_SOLVER: AtomicU32 = AtomicU32::new(0);

pub struct ServerRoom{
    pub id: u32,
    name: String,
    size: u8,

    users: Vec<u32>,
}

impl ServerRoom{
    pub fn new(name: String, size: u8) ->Self{
        Self{
            id: ROOMS_IDS_SOLVER.fetch_add(1, std::sync::atomic::Ordering::Release),
            name,
            size,
            users: vec![]
        }
    }
    pub fn add_user(&mut self, id: u32) {
        self.users.push(id);
    }

    pub fn remove_user(&mut self, id: u32) {
        let pos = self.users.iter().position(|&x| x == id);

        if let Some(pos) = pos {
            self.users.remove(pos);
        }
    }
    pub fn users_inside(&self) -> usize {
        self.users.len()
    }
    pub fn has_empty(&self) -> bool {
        self.users.len() < self.size as usize
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get_all_users_except_sender(&self, sender_id: u32) -> Vec<u32> {
        let filtered: Vec<u32> = self.users
            .iter()
            .copied()                 // &u32 -> u32
            .filter(|&x| x != sender_id)
            .collect();
        filtered
    }
}