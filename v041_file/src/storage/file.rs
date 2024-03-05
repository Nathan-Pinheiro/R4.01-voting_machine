use std::path::Path;
use std::sync::{Arc, RwLock};
use crate::domain::{VotingMachine, Scoreboard, Candidate};
use crate::storage::Storage;
use anyhow::{Result, Ok};
use serde::{Serialize, Deserialize};
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Serialize, Deserialize)]
pub struct ScoreboardDao {
    pub scores: Map<String, usize>,
    pub blank_scores: usize,
    pub invalid_score: usize,
}

#[derive(Serialize, Deserialize)]
pub struct VotingMachineDao {
    pub voters: Set<String>,
    pub scoreboard: ScoreboardDao,
}

impl From<Scoreboard> for ScoreboardDao {
    fn from(scoreboard: Scoreboard) -> Self {
        let scores: Map<String, usize> = scoreboard.scores
            .into_iter()
            .map(|(candidate, score)| (candidate.0, score.0))
            .collect();

        ScoreboardDao { 
            scores, 
            blank_scores: scoreboard.blank_scores.0, 
            invalid_score: scoreboard.invalid_scores.0,
        }
    }
}

impl From<VotingMachine> for VotingMachineDao {
    fn from(voting_machine: VotingMachine) -> Self {

        let voters: Set<String> = voting_machine.voters.0
            .iter()
            .map(|voter| (voter.0.clone()))
            .collect();

        VotingMachineDao {
            voters, 
            scoreboard: ScoreboardDao::from(voting_machine.scoreboard), 
        }
    }
}

pub struct FileStore {
    filepath: Arc<RwLock<String>>
}

#[async_trait::async_trait]
impl Storage for FileStore {
    async fn get_voting_machine(&self) -> Result<VotingMachine> {

        let filepath = self.filepath.read().unwrap().clone();
        let mut my_file = File::open(filepath).await?;
        
        let mut my_slice = vec![];
        my_file.read_to_end(&mut my_slice).await?;

        let voting_machine_dao: VotingMachineDao = serde_json::from_slice::<VotingMachineDao>(&my_slice)?;

        Ok(VotingMachine::from(voting_machine_dao))
    }

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> 
    {
        let filepath = self.filepath.read().unwrap().clone();
        let mut my_file = File::create(filepath).await?;
        let machine_dao = VotingMachineDao::from(machine);
        let serialized_machine = serde_json::to_string(&machine_dao)?;
        my_file.write_all(serialized_machine.as_bytes()).await?;
        Ok(())
    }
}

impl FileStore 
{
    pub async fn new(machine: &VotingMachine, filepath: &str) -> anyhow::Result<Self> {
        
        let file_path : &Path = Path::new(filepath);
    
        let machine : VotingMachine = if file_path.exists() 
        {
            let mut file : File = File::open(file_path).await?;
            let mut content: Vec<u8> = vec![];
            file.read_to_end(&mut content).await?;
            let content : VotingMachineDao = serde_json::from_slice::<VotingMachineDao>(&content)?;
            content.into()
        } 
        else { machine.clone() };
    
        if !file_path.exists() { let _ = File::create(file_path).await; }

        let mut file_store : FileStore = FileStore { filepath: Arc::new(RwLock::new(filepath.to_string())), };
        let _ = file_store.put_voting_machine(machine).await;
    
        Ok(file_store)
    }
}

#[cfg(test)]
mod tests 
{
    use std::fs;

    use crate::storage::Storage;
    use crate::domain::{VotingMachine, Candidate};
    use crate::storage::file::FileStore;
    use std::sync::{Arc, RwLock};

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
        let mut machine : VotingMachine = setup_voting_machine();
        let filepath : &str = "test.txt";
        let memory : Arc<RwLock<FileStore>> = Arc::new(RwLock::new(FileStore::new(&machine, filepath).await?));

        let stored_machine = {
            let memory_guard = memory.read(); // Acquire lock on RwLock
            let memory = memory_guard.as_ref().expect("Failed to get memory");
            memory.get_voting_machine().await?
        };

        fs::remove_file(filepath)?;

        assert_eq!(stored_machine, machine);
        Ok(())
    }

    #[tokio::test]
    async fn store_value_is_conserved() -> anyhow::Result<()> 
    {
        let mut machine : VotingMachine = setup_voting_machine();
        let filepath : &str = "test.txt";

        let first_memory : Arc<RwLock<FileStore>> = Arc::new(RwLock::new(FileStore::new(&machine, filepath).await?));

        let first_stored_machine = {
            let memory_guard = first_memory.read(); // Acquire lock on RwLock
            let memory = memory_guard.as_ref().expect("Failed to get memory");
            memory.get_voting_machine().await?
        };

        let second_memory : Arc<RwLock<FileStore>> = Arc::new(RwLock::new(FileStore::new(&machine, filepath).await?));

        let second_stored_machine = {
            let memory_guard = first_memory.read(); // Acquire lock on RwLock
            let memory = memory_guard.as_ref().expect("Failed to get memory");
            memory.get_voting_machine().await?
        };

        fs::remove_file(filepath)?;

        assert_eq!(first_stored_machine, second_stored_machine);
        Ok(())
    }
}

