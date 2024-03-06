mod calcul;

use std::io;

fn main() {
    loop {
        println!("Choisissez une option :");
        println!("1. Calculer un périmètre");
        println!("2. Calculer une aire");
        println!("2. Calculer la longueur manquante dans un triangle similaire avec Thales");
        println!("3. Quitter");

        let mut choix = String::new();
        io::stdin().read_line(&mut choix).expect("Échec de la lecture de l'entrée");
        let choix: i32 = choix.trim().parse().unwrap_or(0);

        match choix {
            1 => calcul::perimetre::calcul_perimetre(),
            2 => calcul::aires::calcul_aire(),
            3 => calcul::thales::thales(),
            4 => break,
            _ => println!("Choix non valide"),
        }
    }
}


