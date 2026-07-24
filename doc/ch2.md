# Chapitre 2 — Programmer le jeu du plus ou du moins

## Objectif du chapitre

Construire un jeu de devinettes complet (nombre aléatoire, saisie utilisateur, comparaison, boucle de jeu) pour découvrir en pratique les concepts fondamentaux de Rust : variables, entrées/sorties, `match`, crates externes, conversion de types, et boucles.

## Concepts abordés

### 1. `use` et la bibliothèque standard

- **Définition** : `use std::io;` importe le module d'entrée/sortie dans la portée. Par défaut, seuls les types du *prelude* sont disponibles.
- **Pourquoi** : Rust ne charge que l'essentiel par défaut. Il faut importer explicitement ce dont on a besoin.
- **Quand l'utiliser** : dès qu'on utilise un type ou une fonction qui n'est pas dans le prelude (`io`, `cmp::Ordering`, etc.).
- **À retenir** : on peut aussi utiliser le chemin complet (`std::io::stdin()`) sans `use`.

### 2. Variables et mutabilité (`let`, `mut`)

- **Définition** : `let` déclare une variable. Par défaut immuable. `let mut` la rend mutable.
- **Pourquoi** : la sécurité mémoire de Rust commence par l'immuabilité par défaut.
- **Quand l'utiliser** : `let` pour toute variable ; `mut` uniquement quand la valeur doit changer.
- **Erreurs fréquentes** : `cannot assign twice to immutable variable` — oublier `mut`.
- **À retenir** : `let mut supposition = String::new();` crée une `String` vide et mutable.

### 3. `String` et `String::new()`

- **Définition** : `String` est un type de chaîne UTF-8, extensible, alloué sur le tas. `String::new()` crée une chaîne vide.
- **Pourquoi** : `String` peut grandir dynamiquement, contrairement aux `&str` (littéraux).
- **Quand l'utiliser** : pour toute chaîne manipulée ou construite à l'exécution.
- **À retenir** : `::` est la syntaxe pour appeler une fonction associée à un type.

### 4. Saisie utilisateur : `io::stdin().read_line()`

- **Définition** : `io::stdin()` retourne un handle vers l'entrée standard. `.read_line(&mut ma_string)` lit une ligne et l'ajoute à la chaîne.
- **Pourquoi** : `read_line` prend une **référence mutable** (`&mut`) pour modifier la chaîne sans la copier.
- **Quand l'utiliser** : pour toute lecture interactive.
- **Erreurs fréquentes** : oublier `&mut` (référence immuable) ou oublier `mut` sur la variable.
- **À retenir** : le `&` crée une référence ; `&mut` la rend mutable.

### 5. Le type `Result` et la méthode `expect()`

- **Définition** : `Result` est une énumération avec deux variantes : `Ok(valeur)` et `Err(erreur)`. `expect(msg)` retourne la valeur si `Ok`, ou *panique* (plante le programme) avec le message si `Err`.
- **Pourquoi** : Rust force la gestion des erreurs. `expect` est une solution rapide pour les prototypes.
- **Quand l'utiliser** : en phase d'apprentissage / prototypage. Dans du code robuste, on utilise `match` (voir plus bas).
- **Erreurs fréquentes** : ignorer le `Result` — le compilateur émet un avertissement (`unused Result`).
- **À retenir** : `read_line` et `parse` retournent un `Result`.

### 6. Espaces réservés `{}` dans `println!`

- **Définition** : `println!("x = {}", x)` affiche la valeur de `x` à la place de `{}`.
- **Pourquoi** : formatage type-safe — le compilateur vérifie que le type de `x` implémente `Display`.
- **Quand l'utiliser** : pour afficher des valeurs.
- **À retenir** : plusieurs `{}` correspondent aux arguments dans l'ordre.

### 7. Crates et dépendances (Cargo)

- **Définition** : une *crate* est un paquet de code Rust. On ajoute des dépendances dans `[dependencies]` du `Cargo.toml`.
- **Pourquoi** : Cargo télécharge, compile et gère les versions des crates automatiquement.
- **Quand l'utiliser** : pour toute fonctionnalité qui n'est pas dans la bibliothèque standard (ex: `rand` pour l'aléatoire).
- **Erreurs fréquentes** : oublier d'ajouter la dépendance dans `Cargo.toml` avant d'utiliser la crate.
- **À retenir** : `Cargo.lock` verrouille les versions ; `cargo update` les met à jour dans les limites de `Cargo.toml`.

### 8. `rand::thread_rng().gen_range()`

- **Définition** : `rand::thread_rng()` donne un générateur d'aléatoire local au thread. `.gen_range(début..fin)` génère un nombre entre `début` (inclus) et `fin` (exclu).
- **Pourquoi** : la bibliothèque standard ne contient pas de générateur aléatoire.
- **Quand l'utiliser** : dès qu'on a besoin de hasard.
- **À retenir** : `1..101` génère un nombre entre 1 et 100 ; `1..=100` est équivalent (intervalle fermé). Le trait `Rng` doit être importé avec `use rand::Rng`.

### 9. `match` et `Ordering`

- **Définition** : `match` est une structure de contrôle qui compare une valeur à des motifs (*patterns*). `Ordering` est une énumération (`Less`, `Greater`, `Equal`) retournée par `cmp()`.
- **Pourquoi** : `match` est exhaustif — le compilateur vérifie que tous les cas sont couverts.
- **Quand l'utiliser** : pour remplacer des `if/else` complexes ou quand on travaille avec des énumérations.
- **Erreurs fréquentes** : oublier une branche (le compilateur refuse de compiler). Ne pas utiliser `=>` correctement.
- **À retenir** : chaque branche = `motif => code`, séparée par des virgules.

### 10. Conversion de type : `trim().parse()`

- **Définition** : `trim()` enlève les whitespaces (dont `\n`). `parse()` convertit une `&str` en un type numérique.
- **Pourquoi** : `read_line` inclut le `\n` de la touche Entrée.
- **Quand l'utiliser** : pour convertir une saisie utilisateur en nombre.
- **Erreurs fréquentes** : oublier `trim()` — `parse` échoue à cause du `\n`. Ne pas annoter le type — `parse` ne peut pas inférer le type cible.
- **À retenir** : syntaxe : `let supposition: u32 = supposition.trim().parse().expect("msg");`

### 11. Shadowing (masquage)

- **Définition** : déclarer une nouvelle variable avec `let` et le même nom qu'une précédente — la nouvelle masque l'ancienne.
- **Pourquoi** : permet de réutiliser un nom après conversion de type, sans créer de nouvelle variable.
- **Quand l'utiliser** : quand on transforme une valeur d'un type à un autre (ex: `String` → `u32`).
- **Différence avec `mut`** : `mut` change la valeur mais pas le type ; le shadowing peut changer le type.
- **À retenir** : `let supposition = supposition.trim().parse()...` — la deuxième `supposition` masque la première.

### 12. Boucles : `loop`, `break`, `continue`

- **Définition** : `loop` crée une boucle infinie. `break` en sort. `continue` passe à l'itération suivante.
- **Pourquoi** : `loop` est la forme la plus simple et flexible de boucle.
- **Quand l'utiliser** : quand on ne connaît pas à l'avance le nombre d'itérations.
- **Erreurs fréquentes** : oublier `break` → boucle infinie.
- **À retenir** : `break` peut retourner une valeur : `break valeur;`

### 13. Gestion d'erreur avec `match` sur `Result`

- **Définition** : au lieu de `expect`, on utilise `match` pour gérer proprement `Ok` et `Err`.
  ```rust
  let supposition: u32 = match supposition.trim().parse() {
      Ok(nombre) => nombre,
      Err(_) => continue,
  };
  ```
- **Pourquoi** : permet de continuer le jeu au lieu de planter sur une saisie invalide.
- **À retenir** : `Err(_)` avec `_` ignore le contenu de l'erreur.

## Nouveaux mots-clés

- `use` — importe un module ou un type dans la portée
- `let` — déclare une variable (immuable par défaut)
- `mut` — rend une variable mutable
- `match` — structure de contrôle par motifs
- `loop` — boucle infinie
- `break` — sort d'une boucle
- `continue` — passe à l'itération suivante
- `fn` — déclaration de fonction
- `Ok` / `Err` — variantes de l'énumération `Result`

## Fonctions, méthodes et macros importantes

| Commande / Méthode | Description |
|---|---|
| `println!("...", val)` | Macro : affiche du texte formaté |
| `String::new()` | Crée une `String` vide |
| `io::stdin().read_line(&mut s)` | Lit une ligne depuis l'entrée standard |
| `s.trim()` | Supprime les whitespaces au début/fin |
| `s.parse::<T>()` | Convertit une `&str` en type `T` |
| `s.cmp(&autre)` | Compare deux valeurs, retourne `Ordering` |
| `expect(msg)` | Sur `Result` : retourne la valeur ou panique |
| `rand::thread_rng()` | Crée un générateur aléatoire |
| `r.gen_range(début..fin)` | Génère un nombre aléatoire dans l'intervalle |
| `cargo doc --open` | Ouvre la documentation des dépendances |

## Schéma mental

```
Programme :
  1. use std::io / use rand::Rng
  2. Générer nombre_secret avec rand
  3. loop :
       a. Demander un nombre
       b. Lire stdin → String
       c. trim().parse() → u32 (ou continue si échec)
       d. Comparer avec match + cmp
       e. break si égal
```

## Pièges classiques

1. **Oublier `mut`** sur une variable qu'on modifie (ex: `supposition` passée à `read_line`).
2. **Oublier `&mut`** dans `read_line(&mut supposition)` au lieu de `&supposition`.
3. **Oublier `trim()`** avant `parse()` — le `\n` fait échouer la conversion.
4. **Oublier l'annotation de type** dans `let supposition: u32 = ...parse()` — le compilateur ne peut pas inférer.
5. **Oublier `use rand::Rng`** — `gen_range` n'est pas accessible.
6. **Ignorer le `Result`** de `read_line` — le compilateur avertit.
7. **Ne pas gérer `Err` de `parse`** — le jeu plante si l'utilisateur entre du texte.
8. **Oublier `break`** dans la branche `Equal` — boucle infinie même après avoir gagné.
9. **Mettre un point-virgule après `Err(_) => continue`** — `continue` est une expression, pas besoin de `;` dans ce contexte (mais Rust l'accepte).
10. **Confondre `=` et `==`** dans les conditions.

## Résumé

1. `use std::io` importe les entrées/sorties ; `use rand::Rng` importe le trait pour l'aléatoire.
2. Les variables sont immuables par défaut ; `mut` les rend modifiables.
3. `io::stdin().read_line(&mut s)` lit une ligne dans une `String`.
4. `read_line` et `parse` retournent un `Result` — `expect` panique, `match` gère proprement.
5. `trim()` enlève `\n` ; `parse()` convertit en nombre.
6. Le shadowing (`let x = x...`) permet de réutiliser un nom après changement de type.
7. `match` compare une valeur à des motifs de façon exhaustive.
8. `rand::thread_rng().gen_range(1..101)` génère un nombre aléatoire entre 1 et 100.
9. `loop` / `break` / `continue` sont les outils de boucle de base.
10. `Cargo.toml` déclare les dépendances ; `Cargo.lock` fige les versions ; `cargo update` les met à jour.
