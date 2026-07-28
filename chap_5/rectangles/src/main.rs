// Version de base 
// fn main() {
//     let largeur1 = 30;
//     let hauteur1 = 50;
    
//     println!("l'aire du rectangle: {} pixels", aire(largeur1, hauteur1));
// }

// fn aire(largeur: i32, hauteur: i32) -> i32 {
//     largeur * hauteur
// }

// Version avec tuples
// fn main() {
//     let rect1 = (30, 50);
    
//     println!("l'aire du rectangle: {} pixels", aire(rect1));
// }

// fn aire(dimensions: (i32, i32)) -> i32 {
//     dimensions.0 * dimensions.1
// }
// 

// Version avec struc
// struct Rectangle {
//     largeur: u32,
//     hauteur: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         largeur: 30,
//         hauteur: 50,
//     };
    
//     println!("l'aire du rectangle: {} pixels", aire(&rect1));
// }

// fn aire(rectangle: &Rectangle) -> u32 {
//     rectangle.largeur * rectangle.hauteur
// }

// #[derive(Debug)]
// struct Rectangle {
//     largeur: u32,
//     hauteur: u32,
// }

// fn main() {
//     let echelle = 2;
//     let rect1 = Rectangle {
//         largeur: dbg!(30 * echelle),
//         hauteur: 50,
//     };

//     dbg!(&rect1);
// }

#[derive(Debug)]
struct Rectangle {
    largeur: u32,
    hauteur: u32,
}

impl Rectangle {
    fn aire(&self) -> u32 {
        self.largeur * self.hauteur
    }

    fn peut_contenir(&self, autre: &Rectangle) -> bool {
        self.largeur > autre.largeur && self.hauteur > autre.hauteur
    }
}

fn main() {
    let rect1 = Rectangle {
        largeur: 30,
        hauteur: 50
    };
    let rect2 = Rectangle {
        largeur: 10,
        hauteur: 40
    };
    let rect3 = Rectangle {
        largeur: 60,
        hauteur: 45
    };

    println!("rect1 peut-il contenir rect2 ? {}", rect1.peut_contenir(&rect2));
    println!("rect1 peut-il contenir rect3 ? {}", rect1.peut_contenir(&rect3));
}
