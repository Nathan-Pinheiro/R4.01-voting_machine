use std::sync::{Arc, RwLock};
use crate::storage::Storage;

pub struct FileStore {
    filepath: Arc<RwLock<String>>
}

#[async_trait::async_trait]
impl Storage for FileStore {
    async fn get_voting_machine(&self) -> Result<VotingMachine> {
        let machine = self.machine.read().map_err(|_| anyhow!("Failed to acquire read lock"))?;
        Ok(machine.clone())
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        let mut write_guard = self.machine.write().map_err(|_| anyhow!("Failed to acquire write lock"))?;
        *write_guard = machine;
        Ok(())
    }
}