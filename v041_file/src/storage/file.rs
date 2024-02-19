use std::sync::{Arc, RwLock};

pub struct FileStore {
    filepath: Arc<RwLock<String>>
}

