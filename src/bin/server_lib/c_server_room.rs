use std::sync::atomic::AtomicU32;

pub static ROOMS_IDS_SOLVER: AtomicU32 = AtomicU32::new(0);

pub struct ServerRoom{
    id: u32,
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
    pub fn users_inside(&self) -> usize {
        self.users.len()
    }
    pub fn has_empty(&self) -> bool {
        self.users.len() < self.size as usize
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}