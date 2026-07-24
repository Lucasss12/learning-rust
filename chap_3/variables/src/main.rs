// fn main() {
//     let mut x = 5;
//     println!("La valeur de x est : {}", x);
//     x = 6;
//     println!("La valeur de x est : {}", x);
// }

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("La valeur de x dans la portée interne est : {}", x);
//     }

//     println!("La valeur de x est : {}", x);
// }

// fn main() {
//     // addition
//     let somme = 5 + 10;

//     // soustraction
//     let difference = 95.5 - 4.3;

//     // multiplication
//     let produit = 4 * 30;

//     // division
//     let quotient = 56.7 / 32.2;
//     let arrondi = 2 / 3; // retournera 0

//     // modulo
//     let reste = 43 % 5;
// }

// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Veuillez entrer un indice de tableau.");

//     let mut indice = String::new();

//     io::stdin()
//         .read_line(&mut indice)
//         .expect("Échec de la lecture de l'entrée utilisateur");

//     let indice: usize = indice
//         .trim()
//         .parse()
//         .expect("L'indice entré n'est pas un nombre");

//     let element = a[indice];

//     println!(
//         "La valeur de l'élément d'indice {} est : {}",
//         indice, element
//     );
// }
