use std::{self, io};
use crate::{configuration::Configuration, domain::{Voter, VotingMachine, Candidate, BallotPaper, VoteOutcome}};

pub fn run_app(configuration: Configuration) -> anyhow::Result<()> {

    let candidates_config: Vec<String> = configuration.candidates;
    
    let mut candidates: Vec<Candidate> = Vec::new();

    for candidate_config in &candidates_config {
        candidates.push(Candidate(String::from(candidate_config)));
    }

    let mut voting_machine : VotingMachine = VotingMachine::new(candidates);

    let stdin = io::stdin();
    
    loop {

        let mut user_input : String = String::new();
        stdin.read_line(&mut user_input).expect("result");
        let args: Vec<String> = user_input.trim().split_whitespace().map(String::from).collect();

        if user_input.eq("") 
        {
            println!("\n -voter <votant> <candidat> : voter pour un candidat");
            println!("\n -voter <votant> : vote nul");
            println!("\n -votants : voir les votants");
            println!("\n -scores : voir les scores");
        } 
        else if args[0].eq("voter")
        {
            if args.len() == 1 
            {
                println!("Veuillez entrer au moins le nom du votant");
            }
            else 
            {
                let ballot_paper : BallotPaper;
                if args.len() >= 3 
                { 
                    ballot_paper = BallotPaper 
                    { 
                        voter: Voter(args[1].clone()), 
                        candidate: Some(Candidate(args[2].clone())),
                    }
                } 
                else 
                {
                    ballot_paper = BallotPaper 
                    { 
                        voter: Voter(args[1].clone()), 
                        candidate: None, 
                    }; 
                }

                let outcome : VoteOutcome = voting_machine.vote(ballot_paper);

                match outcome 
                {
                    VoteOutcome::AcceptedVote(voter, candidate) => 
                    {
                        voting_machine.get_voters().0.insert(voter);
                        voting_machine.get_scoreboard().scores.entry(candidate).and_modify(|score| score.0 += 1);
                        println!("Vote accepté !");
                    }
                    VoteOutcome::BlankVote(voter) =>
                    {
                        voting_machine.get_voters().0.insert(voter);
                        voting_machine.get_scoreboard().blank_score.0 += 1;
                        println!("Vote blanc");
                    }
                    VoteOutcome::InvalidVote(voter) => 
                    {
                        voting_machine.get_voters().0.insert(voter);
                        voting_machine.get_scoreboard().invalid_score.0 += 1;
                        println!("Vote invalide");
                    }
                    VoteOutcome::HasAlreadyVoted(voter) => println!("{} à déjà voté. Il ne peut pas voter 2 fois !", voter.0),
                }
            }
        } 
        else if args[0].eq("votants") 
        {
            println!("Votants :");
            for votant in &voting_machine.get_voters().0 
            {
                println!(" - {}", votant.0);
            }
        } 
        else if args[0].eq("scores") 
        {
            println!("Scores :");
            for (key, value) in &voting_machine.get_scoreboard().scores 
            {
                println!(" - {} : {}", key.0, value.0);
            }
            println!(" - votes blancs : {}", voting_machine.get_scoreboard().blank_score.0);
            println!(" - votes invalides : {}", voting_machine.get_scoreboard().invalid_score.0);
        } 
        else 
        {
            println!("Commande invalide ...");
        }
    }
}