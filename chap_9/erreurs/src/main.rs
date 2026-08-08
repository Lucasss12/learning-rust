// fn main() {
//     let v = vec![1, 2, 3];
    
//     v[99];
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(fichier) => fichier,
//         Err(erreur) => match erreur.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("Erreur de création du fichier : {:?}", e),
//             },
//             autre_erreur => {
//                 panic!("Erreur d'ouverture du fichier : {:?}", autre_erreur)
//             }
//         },
//     };
// }

// use std::fs::File;

// fn main() {
//     let f = File::open("hello.txt").expect("Échec à l'ouverture de hello.txt");
// }


// #![allow(unused)]
// fn main() {
// use std::fs::File;
// use std::io::{self, Read};

// fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(fichier) => fichier,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
// }


// #![allow(unused)]
// fn main() {
// use std::fs::File;
// use std::io;
// use std::io::Read;

// fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
// }


#![allow(unused)]
fn main() {
use std::fs::File;
use std::io;
use std::io::Read;

fn lire_pseudo_depuis_fichier() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
}
