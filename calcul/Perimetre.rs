use std::io;

pub fn calcul_perimetre() {
    println!("Calculons un périmètre.");
    println!("Choisissez une forme :");
    println!("1. Carré");
    println!("2. Rectangle");
    println!("3. Cercle");
    println!("4. Triangle");

    let mut forme = String::new();
    io::stdin().read_line(&mut forme).expect("Échec de la lecture de l'entrée");
    let forme: i32 = forme.trim().parse().unwrap_or(0);

    match forme {
        1 => carre(),
        2 => rectangle(),
        3 => cercle(),
        4 => triangle(),
        _ => println!("Forme non valide"),
    }
}

pub fn carre() {
    println!("Entrez la taille du côté du carré (en cm):");
    let mut cote_str = String::new();
    
    io::stdin().read_line(&mut cote_str).expect("Échec de la lecture de l'entrée");

    let cote: i32 = cote_str.trim().parse().expect("Veuillez entrer un nombre valide");

    let perimetre = cote * 4;
    println!("Le périmètre du carré est : {} cm", perimetre);
}

pub fn rectangle() {
    println!("Entrez la taille de la longeur du rectangle (en cm):");
    let mut longeur_str = String::new();
    io::stdin().read_line(&mut longeur_str).expect("Échec de la lecture de l'entrée");
    let longeur: i32 = longeur_str.trim().parse().expect("Veuillez entrer une longeur valide");


    println!("Entrez la taille de la largeur du rectangle (en cm):");
    let mut largeur_str = String::new();
    io::stdin().read_line(&mut largeur_str).expect("Échec de la lecture de l'entrée");
    let largeur: i32 = largeur_str.trim().parse().expect("Veuillez entrer une largeur valide");

    let perimetre = longeur * 2 + largeur * 2;
    println!("Le périmètre du carré est : {} cm", perimetre);
}

pub fn cercle() {
    println!("Entrez la taille du rayon (en cm):");
    let mut rayon_str = String::new();
    io::stdin().read_line(&mut rayon_str).expect("Échec de la lecture de l'entrée");
    let rayon: f64 = rayon_str.trim().parse().expect("Veuillez entrer une longeur valide");
    let pi = std::f64::consts::PI;

    let circonference = 2.0 * pi * rayon; 
    println!("La circonférence du cercle est : {:.2} cm", circonference);
}

pub fn triangle() {
    println!("Entrez la taille de AB du triangle (en cm):");
    let mut ab_str = String::new();
    io::stdin().read_line(&mut ab_str).expect("Échec de la lecture de l'entrée");
    let ab: f64 = ab_str.trim().parse().expect("Veuillez entrer une longueur valide");

    println!("Entrez la taille de BC du triangle (en cm):");
    let mut bc_str = String::new();
    io::stdin().read_line(&mut bc_str).expect("Échec de la lecture de l'entrée");
    let bc: f64 = bc_str.trim().parse().expect("Veuillez entrer une largeur valide");

    // Calcul de la longueur de l'hypoténuse AC dans un triangle rectangle
    let ac = (ab.powi(2) + bc.powi(2)).sqrt();

    let perimetre = ab + bc + ac;
    println!("Le périmètre du triangle est : {:.2} cm", perimetre);
}

// Notes

// trim() permet de retirer les blancs ou caracteres de nouvelle ligne


