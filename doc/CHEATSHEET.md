# Aide-mémoire Cargo & Rust

## Commandes Cargo

| Commande | Description |
|---|---|
| `cargo new <nom>` | Nouveau projet binaire |
| `cargo new <nom> --lib` | Nouveau projet bibliothèque |
| `cargo build` | Compiler (debug) |
| `cargo build --release` | Compiler (release optimisé) |
| `cargo run` | Compiler + exécuter |
| `cargo check` | Vérifier la compilation (rapide) |
| `cargo test` | Lancer les tests |
| `cargo clippy` | Lint / conseils |
| `cargo fmt` | Formater le code |
| `cargo doc --open` | Générer la documentation locale |
| `cargo add <crate>` | Ajouter une dépendance |

## Structure Cargo

```
mon_projet/
├── Cargo.toml        # Manifeste (nom, version, dépendances)
├── src/
│   └── main.rs       # Point d'entrée
```

## Cargo.toml

```toml
[package]
name = "mon_projet"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

## Commandes rustc

| Commande | Description |
|---|---|
| `rustc fichier.rs` | Compiler un fichier unique |
| `rustc fichier.rs -o nom` | Compiler avec nom de sortie |
| `rustc --version` | Version du compilateur |

## Types scalaires

| Type | Description |
|---|---|
| `i8-i128, u8-u128` | Entiers signés / non-signés |
| `i32` | Entier par défaut |
| `f32, f64` | Flottants |
| `bool` | `true` / `false` |
| `char` | Unicode 4 octets (guillemets simples) |

## Chaînes

| Type | Description |
|---|---|
| `&str` | Littéral de chaîne (immutable, stack) |
| `String` | Chaîne dynamique (heap, mutable) |
| `String::new()` | Nouvelle String vide |
| `.to_string()` | Convertir &str en String |
| `s.push_str("txt")` | Concaténer une &str dans une String |

## Notions clés

- `;` termine une expression / instruction
- Pas de `;` = retour implicite (expression)
- `fn main()` — point d'entrée
- `match` est **exhaustif** (tous les cas doivent être couverts)
- Variables **immuables** par défaut → `mut` pour les rendre mutables
- `let` crée une variable, `let mut` une variable mutable
- `const` pour les constantes (type toujours explicite)
- `//` commentaire, `/* */` commentaire multi-lignes

## Ressources

- [The Rust Book FR](https://jimskapt.github.io/rust-book-fr/)
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/)
