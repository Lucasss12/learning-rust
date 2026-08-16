// fn main() {
//     let b = Box::new(5);
//     println!("b = {}", b);
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// fn main() {
//     let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
// }

// use std::ops::Deref;

// impl<T> Deref for MaBoite<T> {
//     type Target = T;

//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

// struct MaBoite<T>(T);

// impl<T> MaBoite<T> {
//     fn new(x: T) -> MaBoite<T> {
//         MaBoite(x)
//     }
// }

// fn saluer(nom: &str) {
//     println!("Salutations, {} !", nom);
// }

// fn main() {
//     let m = MaBoite::new(String::from("Rust"));
//     saluer(&m);
// }

// struct PointeurPerso {
//     donnee: String,
// }

// impl Drop for PointeurPerso {
//     fn drop(&mut self) {
//         println!("Nettoyage d'un PointeurPerso avec la donnée `{}` !", self.donnee);
//     }
// }

// fn main() {
//     let c = PointeurPerso {
//         donnee: String::from("des trucs"),
//     };
//     println!("PointeurPerso créé.");
//     drop(c);
//     println!("PointeurPerso libéré avant la fin du main.");
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("compteur après la création de a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("compteur après la création de b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("compteur après la création de c = {}", Rc::strong_count(&a));
    }
    println!("compteur après que c est sorti de la portée = {}", Rc::strong_count(&a));
}
