# Chapitre 6 — Les enums et la correspondance de motifs (pattern matching)

## Objectif du chapitre

Apprendre à représenter un ensemble limité de variantes possibles avec les enums, puis à traiter ces variantes de façon sûre grâce à `match` et `if let`. Le chapitre introduit aussi `Option<T>`, la façon dont Rust gère l'absence de valeur sans utiliser `null`.

## Concepts abordés

### 1. Définition d'une enum

- **Définition** : une enum (`enum`) est un type qui peut prendre l'une de plusieurs variantes possibles.
- **Pourquoi** : modéliser des données dont la forme varie, tout en regroupant toutes les possibilités sous un seul type.
- **Syntaxe** :
  ```rust
  enum Direction {
      Haut,
      Bas,
  }
  ```
  On crée une valeur avec `Direction::Haut`.
- **À retenir** : chaque variante est préfixée par le nom de l'enum et `::`. La syntaxe avec point `.` du chapitre 5 ne s'applique pas ici.

### 2. Enums avec données intégrées

- **Définition** : chaque variante peut embarquer des données directement dans sa définition.
- **Pourquoi** : plus besoin d'une structure séparée pour chaque variante — l'information est stockée *dans* la variante.
- **Exemple** :
  ```rust
  enum AdresseIp {
      V4(String),
      V6(String),
  }
  let local = AdresseIp::V4(String::from("127.0.0.1"));
  ```
- **À retenir** : chaque variante peut avoir un type différent :
  ```rust
  enum Message {
      Quitter,                       // pas de données
      Deplacer { x: i32, y: i32 },   // structure anonyme
      Ecrire(String),                // un String
      ChangerCouleur(u8, u8, u8),    // trois entiers
  }
  ```

### 3. Méthodes sur les enums

- **Définition** : comme les structures, les enums peuvent avoir des méthodes et fonctions associées dans un bloc `impl`.
- **Exemple** : `impl Message { fn appeler(&self) {} }` puis `let m = Message::Ecrire(...); m.appeler();`
- **À retenir** : une enum est un type comme un autre : on peut lui attacher un comportement.

### 4. L'enum `Option<T>` et l'absence de `null`

- **Définition** : Rust n'a pas de valeur `null`. L'enum standard `Option<T>` a deux variantes :
  ```rust
  enum Option<T> {
      None,
      Some(T),
  }
  ```
- **Pourquoi** : éviter les erreurs dues aux valeurs nulles. Le compilateur *oblige* à gérer le cas `None` quand une valeur peut être absente.
- **Syntaxe** : `Some` et `None` peuvent être utilisés sans préfixe `Option::` (ils sont dans le prélude).
- **À retenir** : `Option<T>` et `T` sont des **types différents**. On ne peut pas les additionner : il faut d'abord extraire la valeur du `Some` (via `match`, `if let`, `unwrap`... vu plus loin).
- **Erreur fréquente** : essayer d'utiliser une `Option<i32>` comme un `i32` → erreur de type à la compilation.

### 5. La structure de contrôle `match`

- **Définition** : `match` compare une valeur à une série de motifs (arms) et exécute le code de la première arm qui correspond.
- **Syntaxe** :
  ```rust
  match piece {
      Piece::Penny => 1,
      Piece::Quarter => 25,
      _ => 0,
  }
  ```
- **Pourquoi** : traitement exhaustif et sûr des variantes — le compilateur vérifie que tous les cas sont couverts.
- **À retenir** : chaque arm a la forme `motif => code`. Les arms sont évaluées dans l'ordre. `match` est une **expression** : sa valeur est celle de l'arm exécutée.

### 6. L'exhaustivité et le motif `_`

- **Définition** : `match` exige que toutes les variantes soient couvertes, sinon le code ne compile pas. Le motif `_` (« n'importe quoi ») couvre tous les cas restants.
- **Pourquoi** : le compilateur protège contre l'oubli d'un cas — contrairement aux `switch` de nombreux langages.
- **À retenir** : on place souvent `_ => ...` en dernier arm pour les cas « tous les autres ». Le bloc catch-all doit être le dernier arm.

### 7. `match` avec `Option<T>`

- **Définition** : le pattern matching est la façon idiomatique de récupérer la valeur contenue dans un `Option`.
- **Exemple** :
  ```rust
  fn plus_un(x: Option<i32>) -> Option<i32> {
      match x {
          None => None,
          Some(i) => Some(i + 1),
      }
  }
  ```
- **Pourquoi** : chaque arm lie la valeur contenue dans `Some` à une variable (`Some(i) => ...`), ce qui permet de l'utiliser.
- **À retenir** : quand on gère `Some`, on accède à la valeur en la liant dans le motif. La liaison est le seul moyen d'utiliser la donnée embarquée.

### 8. `if let` — une syntaxe raccourcie

- **Définition** : `if let` combine un `match` à une seule arm avec un `if`.
- **Pourquoi** : quand on ne veut traiter qu'un seul cas précis et ignorer tous les autres — plus concis qu'un `match` avec `_ => ()`.
- **Syntaxe** :
  ```rust
  if let Some(max) = maximum {
      println!("Le maximum est {}", max);
  }
  ```
- **À retenir** : `if let` ne vérifie pas l'exhaustivité (c'est son avantage et sa limite). On peut y ajouter `else`, équivalent à l'arm `_`.

## Nouveaux mots-clés

| Mot-clé | Utilisation |
|---|---|
| `enum` | Définit un type énuméré avec plusieurs variantes |
| `match` | Structure de contrôle qui compare une valeur à des motifs (exhaustive) |
| `if let` | Raccourci de `match` pour un seul cas |
| `Some` | Variante de `Option<T>` qui contient une valeur |
| `None` | Variante de `Option<T>` qui représente l'absence de valeur |
| `_` | Motif passe-partout qui correspond à n'importe quelle valeur |

## Fonctions, méthodes et macros importantes

| Fonction / Macro | Description |
|---|---|
| `Option::Some(valeur)` | Contient une valeur (souvent écrit simplement `Some(v)`) |
| `Option::None` | Représente l'absence de valeur |
| `enum::Variante` | Syntaxe pour construire une variante d'enum (ex. `Message::Ecrire`) |
| `match` | Expression qui branche sur le motif correspondant, retourne une valeur |
| `if let` | Traite un seul motif, sinon exécute `else` optionnel |

## Schéma mental

```
        ┌──────────────────────────────────────────────┐
        │                   ENUM                       │
        │   Un type, plusieurs variantes possibles     │
        │   Variantes avec ou sans données             │
        └──────────────┬───────────────────────────────┘
                       │
        ┌──────────────▼────────────────┐
        │      Option<T> (std)          │
        │   Some(valeur)  |  None       │
        │   Remplace l'absence de null  │
        └──────────────┬────────────────┘
                       │
        ┌──────────────▼──────────────────────────────┐
        │              match                           │
        │  Compare valeur → motifs (arms)             │
        │  Doit couvrir TOUTES les variantes          │
        │  _ couvre les cas restants                  │
        └──────────────┬──────────────────────────────┘
                       │
        ┌──────────────▼──────────────┐
        │   if let (un seul cas)      │
        │   Concis, non exhaustif     │
        │   optionnellement + else    │
        └─────────────────────────────┘
```

L'enum définit les possibilités, `Option` représente la présence/absence d'une valeur, et `match`/`if let` sont les seuls outils propres pour consommer ces variantes de façon sûre et exhaustive.

## Pièges classiques

1. **Utiliser la syntaxe `.` pour les variantes** — `Direction.Haut` est faux, il faut `Direction::Haut` (le `::` du chapitre 3/5).
2. **Traiter une `Option<T>` comme un `T`** — `Some(5) + 3` ne compile pas : il faut extraire la valeur avec `match` ou `if let`.
3. **Oublier un arm dans `match`** — le compilateur refuse (`non-exhaustive patterns`) : c'est la protection du compilateur, pas une punition.
4. **Mettre le motif `_` au début** — il capturerait toutes les valeurs et rendrait les autres arms inaccessibles.
5. **Croire que `if let` est exhaustif** — il ignore silencieusement les autres cas ; l'utiliser quand on veut gérer *tous* les cas est une source de bugs.
6. **Confondre `Some` (variante) et l'opération `match`** — `Some` ne sert qu'à *construire* ou *lier* une valeur, pas à la transformer directement.

## Résumé

1. `enum` regroupe plusieurs variantes possibles sous un seul type, chaque variante pouvant embarquer des données.
2. Les enums acceptent des méthodes via `impl`, comme les structures.
3. Rust n'a pas de `null` : l'absence de valeur est représentée par `Option<T>` (`Some` / `None`).
4. `Option<T>` et `T` sont des types différents — le compilateur empêche d'utiliser une valeur potentiellement absente comme si elle était sûrement là.
5. `match` compare une valeur à des motifs et doit être exhaustif pour compiler.
6. Le motif `_` couvre tous les cas non listés et doit être le dernier arm.
7. Le `match` lie les données embarquées (ex. `Some(i) => ...`) et permet de les utiliser.
8. `if let` est un raccourci pour ne traiter qu'un seul motif, avec `else` optionnel équivalent à `_`.
9. La combinaison enum + `match` est le cœur du style Rust : types expressifs + contrôle exhaustif à la compilation.
