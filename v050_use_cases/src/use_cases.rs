use std::sync::Arc;

use serde::Deserialize;

use crate::{domain::{BallotPaper, Candidate, Voter, VotingMachine, VoteOutcome}, storage::Storage};

#[derive(Deserialize)]
pub struct VoteForm 
{
    pub voter: String,
    pub candidate: String,
}

impl From<VoteForm> for BallotPaper 
{
    fn from(form: VoteForm) -> Self 
    {
        let voter: Voter = Voter(form.voter);
        let candidate : Option<Candidate>;
        if form.candidate.is_empty() {
            candidate = None;
        } else {
            candidate = Some(Candidate(form.candidate));
        }

        BallotPaper
        {
            voter,
            candidate,
        }
    }
}

pub async fn vote(store: Arc<dyn Storage>, vote_form: VoteForm) -> anyhow::Result<VoteOutcome> {
    Ok(store.get_voting_machine().await?.vote(BallotPaper::from(vote_form)))
}

pub async fn get_voting_machine(store: Arc<dyn Storage>) -> anyhow::Result<VotingMachine> {
    Ok(store.get_voting_machine().await?)
}