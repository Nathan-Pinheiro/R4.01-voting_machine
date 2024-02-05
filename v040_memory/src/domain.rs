use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Voter(pub String);

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone)]
pub struct Candidate(pub String);

pub struct Score(pub usize);

pub struct AttendanceSheet(pub Set<Voter>);

pub struct Scoreboard {
    pub scores: Map<Candidate, Score>,
    pub blank_score: Score,
    pub invalid_score: Score,
}

impl Scoreboard {

    pub fn new(candidates: Vec<Candidate>) -> Self {
        
        let mut scores: Map<Candidate, Score> = Map::new();

        for candidate in candidates {
            scores.insert(candidate, Score(0));
        }

        let blank_score: Score  = Score(0);
        let invalid_score: Score = Score(0);

        Self {
            scores,
            blank_score,
            invalid_score,
        }
    }
}

pub struct BallotPaper {
    pub voter: Voter,
    pub candidate: Option<Candidate>
}

#[derive(Debug, PartialEq, Eq)]
pub enum VoteOutcome {
    AcceptedVote(Voter, Candidate),
    BlankVote(Voter),
    InvalidVote(Voter),
    HasAlreadyVoted(Voter),
}

pub struct VotingMachine {
    voters: AttendanceSheet,
    scoreboard: Scoreboard,
}

impl VotingMachine {
    pub fn new(candidates: Vec<Candidate>) -> Self {
        
        let scoreboard: Scoreboard = Scoreboard::new(candidates);
        let voters: AttendanceSheet = AttendanceSheet(Set::new());

        Self {
            scoreboard,
            voters
        }
    }

    pub fn vote(&mut self, ballot_paper: BallotPaper) -> VoteOutcome {
        if self.voters.0.contains(&ballot_paper.voter) {
            return VoteOutcome::HasAlreadyVoted(ballot_paper.voter);
        } 

        match ballot_paper.candidate {
            Some(candidate) => {
                if self.scoreboard.scores.contains_key(&candidate) {
                    return VoteOutcome::AcceptedVote(ballot_paper.voter, candidate);
                } else {
                    return VoteOutcome::InvalidVote(ballot_paper.voter);
                }
            }
            None => {
                return VoteOutcome::BlankVote(ballot_paper.voter);
            }
        }
    }

    pub fn get_scoreboard(&mut self) -> &mut Scoreboard {
        return &mut self.scoreboard;
    }

    pub fn get_voters(&mut self) -> &mut AttendanceSheet {
        return &mut self.voters;
    }

}

#[cfg(test)]
mod tests 
{
    use super::{VotingMachine, Candidate, BallotPaper, Voter, VoteOutcome};

    fn setup_voting_machine() -> VotingMachine
    {
        let mut candidates : Vec<Candidate> = Vec::new();
        candidates.push(Candidate("E.Macron".to_string()));
        candidates.push(Candidate("M.Lepen".to_string()));
        candidates.push(Candidate("JL.Mélanchon".to_string()));
        return VotingMachine::new(candidates);
    }

    #[test]
    fn vote_accepted()
    {
        let current_voter : Voter = Voter("Jean".to_string());
        let current_candidate : Candidate = Candidate("E.Macron".to_string());

        let ballot_paper : BallotPaper = BallotPaper { voter: current_voter.clone(), candidate: Some(current_candidate.clone()) };
        let mut voting_machine : VotingMachine = setup_voting_machine();

        let vote_outcome : VoteOutcome = voting_machine.vote(ballot_paper);

        assert_eq!(vote_outcome, VoteOutcome::AcceptedVote(current_voter, current_candidate));
    }

    #[test]
    fn vote_blank()
    {
        let current_voter : Voter = Voter("Jean".to_string());

        let ballot_paper : BallotPaper = BallotPaper { voter: current_voter.clone(), candidate: None };
        let mut voting_machine : VotingMachine = setup_voting_machine();

        let vote_outcome : VoteOutcome = voting_machine.vote(ballot_paper);

        assert_eq!(vote_outcome, VoteOutcome::BlankVote(current_voter));
    }

    #[test]
    fn vote_invalid()
    {
        let current_voter : Voter  = Voter("Jean".to_string());
        let current_candidate : Candidate = Candidate("J.Chirac".to_string());

        let ballot_paper : BallotPaper = BallotPaper { voter: current_voter.clone(), candidate: Some(current_candidate.clone()) };
        let mut voting_machine : VotingMachine = setup_voting_machine();

        let vote_outcome : VoteOutcome = voting_machine.vote(ballot_paper);

        assert_eq!(vote_outcome, VoteOutcome::InvalidVote(current_voter));
    }

    #[test]
    fn has_already_voted()
    {
        let current_voter : Voter = Voter("Jean".to_string());
        let current_candidate : Candidate = Candidate("E.Macron".to_string());

        let mut voting_machine : VotingMachine = setup_voting_machine();

        voting_machine.get_voters().0.insert(current_voter.clone());

        let ballot_paper : BallotPaper = BallotPaper { voter: current_voter.clone(), candidate: Some(current_candidate.clone()) };
        let vote_outcome : VoteOutcome = voting_machine.vote(ballot_paper);

        assert_eq!(vote_outcome, VoteOutcome::HasAlreadyVoted(current_voter));
    }
}