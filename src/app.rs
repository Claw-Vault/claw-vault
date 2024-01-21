use crate::core::cipher;
use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct App {
    cipher: cipher::Cipher,
    db: DatabaseConnection,
}

impl App {
    pub fn new(db_conn: DatabaseConnection) -> Self {
        return App {
            cipher: cipher::Cipher::new(),
            db: db_conn,
        };
    }

    pub fn expand(self) -> (cipher::Cipher, DatabaseConnection) {
        return (self.cipher, self.db);
    }
}
