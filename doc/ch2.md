# Chapitre 2 — Le jeu du plus ou du moins

## Dépendance `rand`

```toml
[dependencies]
rand = "0.8"
```

```rust
use rand::Rng;
let secret = rand::thread_rng().gen_range(1..=100);
```

- `gen_range(1..=100)` → entier entre 1 et 100 inclus
- `..=` opérateur de range inclusif
- `..` range exclusif (ex: `1..100` = 1 à 99)

## Variables & mutabilité

```rust
let x = 5;               // immuable (défaut)
let mut y = 10;          // mutable
let mut guess = String::new();
```

- Tout est **immuable par défaut** en Rust → sécurité
- `mut` = opt-in explicite pour la mutation

## Lire une entrée utilisateur

```rust
use std::io;

let mut input = String::new();
io::stdin().read_line(&mut input).expect("erreur");
```

- `read_line` lit jusqu'à `\n` inclus
- `&mut input` = **référence mutable** (on prête la variable)
- `.expect()` panique si erreur (usage simpliste, on verra mieux après)

## Gestion d'erreur avec match

```rust
let input: u32 = match input.trim().parse() {
    Ok(nombre) => nombre,
    Err(_) => {
        println!("Nombre invalide");
        continue;
    }
};
```

- `.trim()` enlève `\n` et espaces
- `.parse()` tente de convertir → retourne `Result<u32, Error>`
- `Ok(valeur)` → on récupère la valeur
- `Err(_)` → on ignore l'erreur avec `_` et on continue la boucle
- `continue` passe au tour de boucle suivant

## Shadowing (masquage)

```rust
let guess = String::new();          // String
let guess: u32 = guess              // on réutilise le nom
    .trim().parse().expect("...");  // mais c'est une nouvelle variable u32
```

Utile pour **transformer** une variable sans multiplier les noms.  
La première `guess` est détruite, la seconde prend sa place.

## Comparaison avec `match`

```rust
use std::cmp::Ordering;

match nombre.cmp(&secret) {
    Ordering::Less => println!("Trop petit"),
    Ordering::Greater => println!("Trop grand"),
    Ordering::Equal => {
        println!("Gagné !");
        break;
    }
}
```

- `cmp` compare deux valeurs → retourne un `Ordering`
- `match` est **exhaustif** : Rust force à gérer tous les cas

## Boucle `loop`

```rust
loop {
    // instructions
    break; // sort de la boucle
}
```

- `loop` = boucle infinie (pas de condition initiale)
- On en sort avec `break`

## Structure du programme complet

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Devine le nombre !");
    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("erreur");

        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => { println!("Invalide"); continue; }
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Trop petit"),
            Ordering::Greater => println!("Trop grand"),
            Ordering::Equal => {
                println!("Gagné !");
                break;
            }
        }
    }
}
```

## Concepts clés découverts

| Concept | Rôle |
|---|---|
| `let mut` | Variable mutable (contraire du défaut immutable) |
| `loop` | Boucle infinie |
| `match` | Pattern matching exhaustif |
| `Ordering` | Enum à 3 variants : Less, Greater, Equal |
| `Result` | Enum pour gestion d'erreur : Ok / Err |
| Shadowing | Réutiliser un nom avec `let` pour changer de type |
| `.expect()` | Panique rapide sur erreur (à améliorer plus tard) |
| `&` | Référence (emprunt) |
| `use` | Importer un module dans le scope |
| `rand` | Premier crate externe |
