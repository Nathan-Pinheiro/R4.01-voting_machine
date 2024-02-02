use std::{self, io};
use crate::{configuration::Configuration, domain::{VotingMachine, Candidate}};

pub fn run_app(configuration: Configuration) -> anyhow::Result<()> {

    let candidates_config = configuration.candidates;
    
    let mut candidates: Vec<Candidate> = Vec::new();

    for candidate_config in &candidates_config {
        candidates.push(Candidate(String::from(candidate_config)));
    }

    let voting_machine = VotingMachine::new(candidates);

    let stdin = io::stdin();
    
    loop {
        let mut user_input = String::new();
        stdin.read_line(&mut user_input).expect("result");

        let args: Vec<String> = user_input.trim().split_whitespace().map(String::from).collect();

        if user_input.eq("") {

            println!("\n -voter <votant> <candidat> : voter pour un candidat");
            println!("\n -voter <votant> : vote nul");
            println!("\n -votants : voir les votants");
            println!("\n -scores : voir les scores");

        } else if args[0].eq("voter") {

            if args.len() == 1 {

                println!("Veuillez entrer au moins le nom du votant");

            }
            else {

                if voting_machine.get_voters().0.iter().any(|voter| voter.0 == args[1]) {

                    println!("{} à déjà voté. Il ne peut pas voter 2 fois !", args[1]);

                } else {

                    voting_machine.get_voters().0.insert(Voter(args[1].clone()));
                    println!("{} à voté", args[1]);

                    if args.len() == 3 {

                        if let Some(candidate) = voting_machine.get_scoreboard().scores.get_mut(&Candidate(args[2].clone())) {
                            candidate.0 += 1;
                            println!("Un vote a été ajouté pour {}", args[2]);
                        } else {
                            voting_machine.get_scoreboard().invalid_score.0 += 1;
                            println!("Le candidat : {} n'existe pas !", args[2]);
                        }
                    } else {
                        voting_machine.get_scoreboard().blank_score.0 = voting_machine.get_scoreboard().blank_score.0 + 1;
                        println!("un vote blanc à été ajouté");
                    }
                }
            }
        } else if args[0].eq("votants") {
            println!("Votants :");
            for votant in &voting_machine.get_voters().0 {
                println!(" - {}", votant.0);
            }
        } else if args[0].eq("scores") {
            println!("Scores :");
            for (key, value) in &voting_machine.get_scoreboard().scores {
                println!(" - {} : {}", key.0, value.0);
            }
        } else {
            println!("Commande invalide ...");
        }
    }
}