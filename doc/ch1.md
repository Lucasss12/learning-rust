# Chapitre 1 — Premiers pas

## Hello World (rustc)

```rust
fn main() {
    println!("Hello, world!");
}
```

Compiler & exécuter :
```bash
rustc main.rs
./main
```

## Hello Cargo

```bash
cargo new hello_cargo
cargo run
```

`cargo run` compile + exécute en une commande.

## Cargo.toml

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"
```

- `edition` = la version du langage Rust (2015, 2018, 2021)
- Les dépendances vont sous `[dependencies]`

## Cycle de développement

```bash
cargo check   # vite : vérifie que ça compile (sans binaire)
cargo build   # compile le binaire (debug)
cargo run     # compile + exécute
```

**Règle :** `cargo check` pendant le dev, `cargo build` / `cargo run` pour tester.

## Binaires debug vs release

```bash
cargo build             # debug : rapide, lent à l'exécution
cargo build --release   # release : lent à compiler, optimisé
```

Les binaires sont dans `target/debug/` ou `target/release/`.

## Macro println!

- `println!("{var}")` — formatage direct (Rust 2021+)
- `println!("{}", var)` — formatage classique
- `println!("{:?}", expr)` — format Debug (nécessite `#[derive(Debug)]`)
- `print!("...")` — sans saut de ligne final

## Résumé

| Concept | À retenir |
|---|---|
| `fn main()` | Point d'entrée de tout programme Rust |
| `println!` | Macro d'affichage avec saut de ligne |
| `cargo new` | Crée un projet structuré avec .git |
| `cargo check` | Vérification rapide (pas de binaire) |
| `cargo run` | Compile + exécute |
| `--release` | Mode optimisé pour la production |
| `Cargo.toml` | Manifeste du projet |
