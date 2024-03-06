use std::io;

pub fn calcul_aire() {
    println!("Calculons une aire.");
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
        _ => println!("Forme non valide"),
    }
}

pub fn carre() {
    println!("Entrer la taille du coté du carre (en cm)");
    let mut cote_str = String::new();  
    io::stdin().read_line(&mut cote_str).expect("Échec de la lecture de l'entrée");
    let cote: i32 = cote_str.trim().parse().expect("Veuillez entrer un nombre valide");
    let aire = cote * cote;
    println!("L'aire du carré est : {} cm²", aire);
}

pub fn rectangle() {
    println!("Entrer la taille du coté du carre (en cm)");
    let mut cote_str = String::new();  
    io::stdin().read_line(&mut cote_str).expect("Échec de la lecture de l'entrée");
    let cote: i32 = cote_str.trim().parse().expect("Veuillez entrer un nombre valide");
    let aire = cote * cote;
    println!("L'aire du carré est : {} cm²", aire);
}




// Notes :
// 5 × 5 = 25 cm2