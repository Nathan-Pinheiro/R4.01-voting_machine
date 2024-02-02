use std::{self, collections::HashMap, io};
use crate::configuration::Configuration;

pub fn run_app(configuration: Configuration) -> anyhow::Result<()> {

    let candidates = configuration.candidates;
    
    let stdin = io::stdin();
    let mut votants: Vec<String> = vec![];
    
    let mut candidates_votes: HashMap<String, i32> = HashMap::new();
    candidates_votes.insert(String::from("Blanc"), 0);
    candidates_votes.insert(String::from("Nul"), 0);

    for candidate in &candidates {
        candidates_votes.insert(candidate.clone(), 0);
    }

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

                if votants.contains(&args[1]) {

                    println!("{} à déjà voté. Il ne peut pas voter 2 fois !", args[1]);

                } else {

                    votants.push(args[1].clone());
                    println!("{} à voté", args[1]);

                    if args.len() == 3 {
                        match candidates_votes.get(&args[2]) {

                            Some(candidate_score) => {
                                candidates_votes.insert(args[2].clone(), candidate_score + 1);
                                println!("un vote à été ajouté pour {}", args[2]);
                            }
                            None => {
                                let candidate_score = candidates_votes.get(&String::from("Nul")).expect("candidat nul n'existe pas");
                                candidates_votes.insert(String::from("Nul"), candidate_score + 1);
                                println!("Le candidat : {} n'existe pas !", args[2]);
                            }
                        }
                    } else {
                        let candidate_score = candidates_votes.get(&String::from("Blanc")).expect("candidat blanc n'existe pas");
                        candidates_votes.insert(String::from("Blanc"), candidate_score + 1);
                        println!("un vote blanc à été ajouté");
                    }
                }
            }
        } else if args[0].eq("votants") {
            println!("Votants :");
            for votant in &votants {
                println!(" - {}", votant);
            }
        } else if args[0].eq("scores") {
            println!("Scores :");
            for (key, value) in &candidates_votes {
                println!(" - {} : {}", key, value);
            }
        } else {
            println!("Commande invalide ...");
        }
    }
}