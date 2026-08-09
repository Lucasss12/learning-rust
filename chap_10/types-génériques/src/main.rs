// fn le_plus_grand(liste: &[i32]) -> i32 {
//     let mut le_plus_grand = liste[0];

//     for &element in liste {
//         if element > le_plus_grand {
//             le_plus_grand = element;
//         }
//     }

//     le_plus_grand
// }

// fn main() {
//     let liste_de_nombres = vec![34, 50, 25, 100, 65];

//     let resultat = le_plus_grand(&liste_de_nombres);
//     println!("Le nombre le plus grand est {}", resultat);
//     assert_eq!(resultat, 100);

//     let liste_de_nombres = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let resultat = le_plus_grand(&liste_de_nombres);
//     println!("Le nombre le plus grand est {}", resultat);
//     assert_eq!(resultat, 6000);
// }

// fn le_plus_grand<T>(liste: &[T]) -> T {
//     let mut le_plus_grand = liste[0];

//     for &element in liste {
//         if element > le_plus_grand {
//             le_plus_grand = element;
//         }
//     }

//     le_plus_grand
// }

// fn main() {
//     let liste_de_nombres = vec![34, 50, 25, 100, 65];

//     let resultat = le_plus_grand(&liste_de_nombres);
//     println!("Le nombre le plus grand est {}", resultat);

//     let liste_de_caracteres = vec!['y', 'm', 'a', 'q'];

//     let resultat = le_plus_grand(&liste_de_caracteres);
//     println!("Le plus grand caractère est {}", resultat);
// }

// pub trait Resumable {
//     fn resumer(&self) -> String;
// }

// pub struct ArticleDePresse {
//     pub titre: String,
//     pub lieu: String,
//     pub auteur: String,
//     pub contenu: String,
// }

// impl Resumable for ArticleDePresse {
//     fn resumer(&self) -> String {
//         format!("{}, par {} ({})", self.titre, self.auteur, self.lieu)
//     }
// }

// pub struct Tweet {
//     pub nom_utilisateur: String,
//     pub contenu: String,
//     pub reponse: bool,
//     pub retweet: bool,
// }

// impl Resumable for Tweet {
//     fn resumer(&self) -> String {
//         format!("{} : {}", self.nom_utilisateur, self.contenu)
//     }
// }

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let resultat = la_plus_longue(string1.as_str(), string2);
    println!("La plus grande chaîne est {}", resultat);
}

fn la_plus_longue<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
