use std::io;

fn calculer_thales(a: f64, a_prime: f64, b: f64) -> f64 {
    let b_prime = (a_prime / a) * b;
    b_prime
}

pub fn thales() {
    println!("Entrer la longueur du premier segment dans le premier triangle. (en cm):");
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("Échec de la lecture de l'entrée");
    let a: f64 = a_str.trim().parse().expect("Veuillez entrer une longueur valide");

    println!("Entrer la longueur du segment correspondant dans le deuxième triangle. (en cm):");
    let mut a_prime_str = String::new();
    io::stdin().read_line(&mut a_prime_str).expect("Échec de la lecture de l'entrée");
    let a_prime: f64 = a_prime_str.trim().parse().expect("Veuillez entrer une longueur valide");

    println!("Entrer la longueur d'un autre segment dans le premier triangle dont la longueur correspondante `b_prime` est recherchée (en cm):");
    let mut b_str = String::new();
    io::stdin().read_line(&mut b_str).expect("Échec de la lecture de l'entrée");
    let b: f64 = b_str.trim().parse().expect("Veuillez entrer une longueur valide");

    let b_prime = calculer_thales(a, a_prime, b);
    println!("La longueur correspondante dans le deuxième triangle est : {:.2} cm", b_prime);
}

