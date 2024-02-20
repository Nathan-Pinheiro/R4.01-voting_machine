use std::sync::{Arc, RwLock};
use crate::domain::{VotingMachine, Scoreboard};
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

    async fn put_voting_machine(&mut self, machine: VotingMachine) -> Result<()> {
        
        let filepath = self.filepath.read().unwrap().clone();
        let mut my_file = File::open(filepath).await?;

        let _ = my_file.write_all(&serde_json::to_vec(&VotingMachineDao::from(machine)).expect(""));
        Ok(())
    }   
}
impl FileStore {
    pub async fn new(machine: &VotingMachine, filepath: &str) -> anyhow::Result<Self> {

        let _ = File::create(filepath);
        let mut file_store: FileStore = FileStore { filepath : Arc::new(RwLock::new(String::from(filepath))) };
        let _ = file_store.put_voting_machine(machine.clone());
        Ok(file_store)
    }
}


