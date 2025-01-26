use crate::models::user::User;
use std::sync::Mutex;

pub struct FakeDb {
    pub users: Mutex<Vec<User>>,
}

impl FakeDb {
    pub fn new() -> Self {
        FakeDb {
            users: Mutex::new(Vec::new()),
        }
    }

    // Metodo per aggiungere un utente
    pub fn add_user(&self, user: User) {
        let mut users = self.users.lock().unwrap();
        users.push(user);
    }

    // Metodo per recuperare tutti gli utenti
    pub fn get_users(&self) -> Vec<User> {
        let users = self.users.lock().unwrap();
        users.clone()
    }

    pub fn get_user_by_id(&self, id: i32) -> Option<User> {
        let users = self.users.lock().unwrap();
        users.iter().find(|user| user.id == id).cloned()
    }

    pub fn delete_user_by_id(&self, id: i32) -> Option<User> {
        let mut users = self.users.lock().unwrap();
        let position = users.iter().position(|user| user.id == id);
        match position {
            Some(index) => Some(users.remove(index)),
            None => None,
        }
    }
}
