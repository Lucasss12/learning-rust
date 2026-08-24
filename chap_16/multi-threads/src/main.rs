// use std::thread;
// use std::time::Duration;

// fn main() {
//     let manipulateur = thread::spawn(|| {
//         for i in 1..10 {
//             println!("Bonjour n°{} à partir de la nouvelle tâche !", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//     manipulateur.join().unwrap();

//     for i in 1..5 {
//         println!("Bonjour n°{} à partir de la tâche principale !", i);
//         thread::sleep(Duration::from_millis(1));
//     }

// }

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     let manipulateur = thread::spawn(move || {
//         println!("Voici un vecteur : {:?}", v);
//     });

//     manipulateur.join().unwrap();
// }

// use std::sync::mpsc;
// use std::thread;
// use std::time::Duration;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let tx1 = tx.clone();
//     thread::spawn(move || {
//         let valeurs = vec![
//             String::from("salutations"),
//             String::from("à partir"),
//             String::from("de la"),
//             String::from("nouvelle tâche"),
//         ];

//         for valeur in valeurs {
//             tx1.send(valeur).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     thread::spawn(move || {
//         let valeurs = vec![
//             String::from("encore plus"),
//             String::from("de messages"),
//             String::from("pour"),
//             String::from("vous"),
//         ];

//         for valeur in valeurs {
//             tx.send(valeur).unwrap();
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     for recu in rx {
//         println!("On a reçu : {}", recu);
//     }
// }

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let compteur = Arc::new(Mutex::new(0));
    let mut manipulateurs = vec![];

    for _ in 0..10 {
        let compteur = Arc::clone(&compteur);
        let manipulateur = thread::spawn(move || {
            let mut nombre = compteur.lock().unwrap();

            *nombre += 1;
        });
        manipulateurs.push(manipulateur);
    }

    for manipulateur in manipulateurs {
        manipulateur.join().unwrap();
    }

    println!("Résultat : {}", *compteur.lock().unwrap());
}
