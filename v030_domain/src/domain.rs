use std::collections::BTreeMap as Map;
use std::collections::BTreeSet as Set;

#[derive(Ord, PartialOrd, PartialEq, Eq)]
pub struct Voter(pub String);

#[derive(Ord, PartialOrd, PartialEq, Eq)]
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

    pub fn get_scoreboard(&self) -> &Scoreboard {
        return &self.scoreboard;
    }

    pub fn get_voters(&mut self) -> &mut AttendanceSheet {
        return &mut self.voters;
    }

}