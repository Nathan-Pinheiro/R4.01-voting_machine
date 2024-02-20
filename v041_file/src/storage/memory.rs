use crate::domain::{VotingMachine, Candidate};
use crate::storage::Storage;
use anyhow::{Result, anyhow};
use std::sync::{RwLock, Arc};
pub struct MemoryStore {
    pub machine: Arc<RwLock<VotingMachine>>,
}

impl MemoryStore {
    pub fn new(new_machine: VotingMachine) -> Self {
        MemoryStore { machine: Arc::new(RwLock::new(new_machine)) }
    }
}

#[async_trait::async_trait]
impl Storage for MemoryStore {
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

#[cfg(test)]
mod tests 
{
    use crate::storage::Storage;

    use super::{VotingMachine, Candidate, MemoryStore};

    fn setup_voting_machine() -> VotingMachine
    {
        let mut candidates : Vec<Candidate> = Vec::new();
        candidates.push(Candidate("E.Macron".to_string()));
        candidates.push(Candidate("M.Lepen".to_string()));
        candidates.push(Candidate("JL.Mélanchon".to_string()));
        return VotingMachine::new(candidates);
    }

    #[tokio::test]
    async fn test_get_and_put_voting_machine() -> anyhow::Result<()> 
    {
        let mut memory = MemoryStore::new(setup_voting_machine());

        let mut new_voting_machine = memory.get_voting_machine().await?;
        new_voting_machine.get_scoreboard().invalid_scores.0 += 1;
        memory.put_voting_machine(new_voting_machine.clone()).await?;


        let mut stored_machine = memory.get_voting_machine().await?;
        assert_eq!(stored_machine.get_scoreboard().invalid_scores.0, new_voting_machine.get_scoreboard().invalid_scores.0);

        Ok(())
    }
}