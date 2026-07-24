// fn main() {
//     let mut compteur = 0;
//     'increment: loop {
//         println!("compteur = {}", compteur);
//         let mut restant = 10;

//         loop {
//             println!("restant = {}", restant);
//             if restant == 9 {
//                 break;
//             }
//             if compteur == 2 {
//                 break 'increment;
//             }
//             restant -= 1;
//         }

//         compteur += 1;
//     }
//     println!("Fin du compteur = {}", compteur);
// }

// fn main() {
//     let mut compteur = 0;

//     let resultat = loop {
//         compteur += 1;

//         if compteur == 10 {
//             break compteur * 2;
//         }
//     };

//     println!("Le résultat est {}", resultat);
// }


// fn main() {
//     let mut nombre = 3;

//     while nombre != 0 {
//         println!("{} !", nombre);

//         nombre -= 1;
//     }

//     println!("DÉCOLLAGE !!!");
// }

fn main() {
    for nombre in (1..4).rev() {
        println!("{} !", nombre);
    }
    println!("DÉCOLLAGE !!!");
}
