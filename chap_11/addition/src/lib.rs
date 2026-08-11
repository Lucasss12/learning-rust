// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn exploration() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
    
//     #[test]
//     fn un_autre() {
//         panic!("Fait échouer le test");
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     largeur: u32,
//     hauteur: u32,
// }

// impl Rectangle {
//     fn peut_contenir(&self, other: &Rectangle) -> bool {
//         self.largeur < other.largeur && self.hauteur > other.hauteur
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn un_grand_peut_contenir_un_petit() {
//         let le_grand = Rectangle { largeur: 8, hauteur: 7 };
//         let le_petit = Rectangle { largeur: 5, hauteur: 1 };

//         assert!(le_grand.peut_contenir(&le_petit));
//     }
    
//     #[test]
//         fn un_petit_ne_peut_pas_contenir_un_plus_grand() {
//             let le_grand = Rectangle {
//                 largeur: 8,
//                 hauteur: 7,
//             };
//             let le_petit = Rectangle {
//                 largeur: 5,
//                 hauteur: 1,
//             };
    
//             assert!(!le_petit.peut_contenir(&le_grand));
//         }
// }

pub fn ajouter_deux(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ajouter_deux_a_deux() {
        assert_eq!(4, ajouter_deux(2));
    }

    #[test]
    fn ajouter_deux_a_trois() {
        assert_eq!(5, ajouter_deux(3));
    }

    #[test]
    fn cent() {
        assert_eq!(102, ajouter_deux(100));
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn cela_ajoute_deux() {
//         assert_eq!(4, ajouter_deux(2));
//     }
// }

// pub fn accueil(nom: &str) -> String {
//     format!("Salut !")
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn accueil_contient_le_nom() {
//         let resultat = accueil("Carole");
//         assert!(
//             resultat.contains("Carole"),
//             "Le message d'accueil ne contient pas le nom, il vaut `{}`",
//             resultat
//         );
//     }
// }

// pub struct Supposition {
//     valeur: i32,
// }

// impl Supposition {
//     pub fn new(valeur: i32) -> Supposition {
//         if valeur < 1 {
//             panic!(
//                 "La supposition doit être plus petite ou égale à 100, et nous avons {}.",
//                 valeur
//             );
//         } else if valeur > 100 {
//             panic!(
//                 "La supposition doit être plus grande ou égale à 1, et nous avons {}.",
//                 valeur
//             );
//         }

//         Supposition { valeur }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     #[should_panic(expected = "La supposition doit être plus petite ou égale à 100")]
//     fn plus_grand_que_100() {
//         Supposition::new(200);
//     }
// }

// fn affiche_et_retourne_10(a: i32) -> i32 {
//     println!("J'ai obtenu la valeur {}", a);
//     10
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn ce_test_reussit() {
//         let valeur = affiche_et_retourne_10(4);
//         assert_eq!(10, valeur);
//     }

//     #[test]
//     fn ce_test_echoue() {
//         let valeur = affiche_et_retourne_10(8);
//         assert_eq!(5, valeur);
//     }
// }
