use std::{self, collections::HashMap, io};


fn main() -> anyhow::Result<()> {
   
    let stdin = io::stdin();

    let mut votants: Vec<String> = vec![];
    let mut candidatesVotes: HashMap<String, i32> = HashMap::new();
   
    candidatesVotes.insert(String::from("Nul"), 0);
    candidatesVotes.insert(String::from("Blanc"), 0);
    candidatesVotes.insert(String::from("M. Lepen"), 0);
    candidatesVotes.insert(String::from("E. Macron"), 0);
    candidatesVotes.insert(String::from("J. Chirac"), 0);

    while true {
        let mut user_input = String::new();
        stdin.read_line(&mut user_input);

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
                        match candidatesVotes.get(&args[2]) {

                            Some(candidateScore) => {
                                candidatesVotes.insert(args[2].clone(), candidateScore + 1);
                                println!("un vote à été ajouté pour {}", args[2]);
                            }
                            None => {
                                let candidateScore = candidatesVotes.get(&String::from("Nul")).expect("candidat nul n'existe pas");
                                candidatesVotes.insert(String::from("Nul"), candidateScore + 1);
                                println!("Le candidat : {} n'existe pas !", args[2]);
                            }
                        }
                    } else {
                        let candidateScore = candidatesVotes.get(&String::from("Blanc")).expect("candidat blanc n'existe pas");
                        candidatesVotes.insert(String::from("Blanc"), candidateScore + 1);
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
            for (key, value) in &candidatesVotes {
                println!(" - {} : {}", key, value);
            }
        } else {
            println!("Commande invalide ...");
        }
    }
   
    Ok(())
}