// enum SorteAdrresseIp {
//     V4,
//     V6
// }

// struct AdrresseIp {
//     sorte: SorteAdrresseIp,
//     adresse: String,
// }

// fn main() {    
//     let local_ip = AdrresseIp {
//         sorte: SorteAdrresseIp::V4,
//         adresse: String::from("127.0.0.1"),
//     };
    
//     let rebouclage = AdrresseIp {
//         sorte: SorteAdrresseIp::V6,
//         adresse: String::from("::1"),
//     };
// }

// fn router( sorte_ip: SorteAdrresseIp) {
    
// }


// fn main() {
//     enum AdresseIp {
//         V4(u8, u8, u8, u8),
//         V6(String),
//     }
    
//     let local = AdresseIp::V4(127, 0, 0, 1);
    
//     let rebouclage = AdresseIp::V6(String::from("::1"));
// }


// #![allow(unused)]
// fn main() {
// struct Ipv4Addr {
//     // -- code masqué ici --
// }

// struct Ipv6Addr {
//     // -- code masqué ici --
// }

// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
// }

// fn main() {
//     enum Message {
//         Quitter,
//         Deplacer { x: i32, y: i32 },
//         Ecrire(String),
//         ChangerCouleur(i32, i32, i32),
//     }

//     impl Message {
//         fn appeler(&self) {
//             // le corps de la méthode sera défini ici
//         }
//     }
    
//     let m = Message::Ecrire(String::from("hello"));
//     m.appeler();
// }

#[derive(Debug)]
enum EtatUs {
    Alabama,
    Alaska,
}

enum PieceUs {
    Penny,
    Nickel,
    Dime,
    Quarter(EtatUs),
}

fn valeur_en_centimes(piece: PieceUs) -> u8 {
    match piece {
        PieceUs::Penny => 1,
        PieceUs::Nickel => 5,
        PieceUs::Dime => 10,
        PieceUs::Quarter(etat) => {
            println!("Il s'agit d'un quarter de l'État de {:?} !", etat);
            25
        },
    }
}

fn main() {
    valeur_en_centimes(PieceUs::Quarter(EtatUs::Alaska));
}
