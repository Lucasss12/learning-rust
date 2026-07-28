# Chapitre 5 — Utiliser les structures pour structurer des données apparentées

## Objectif du chapitre

Apprendre à créer des types personnalisés avec les structures (structs) pour regrouper des données cohérentes et leur associer un comportement via des méthodes. Le chapitre montre pourquoi les structures sont plus expressives que les tuples et introduit le fonctionnement des méthodes en Rust.

## Concepts abordés

### 1. Définition et instanciation d'une structure

- **Définition** : une structure se définit avec le mot-clé `struct` suivi d'un nom (PascalCase) et de champs nommés entre accolades.
- **Pourquoi** : regrouper des valeurs associées sous un même nom, chaque champ étant identifié par son nom (contrairement aux tuples).
- **Syntaxe** :
  ```rust
  struct Utilisateur {
      actif: bool,
      pseudo: String,
  }
  ```
  Créer une instance : `let u = Utilisateur { actif: true, pseudo: String::from("alice") };`.
- **Accès** : `u.pseudo`. L'instance entière doit être `mut` pour modifier un champ.
- **À retenir** : l'ordre des champs à l'instanciation n'a pas d'importance ; on ne peut pas rendre certains champs mutables individuellement.

### 2. Raccourci d'initialisation des champs (field init shorthand)

- **Définition** : quand un paramètre de fonction a le même nom qu'un champ, on peut écrire `champ` au lieu de `champ: valeur`.
- **Pourquoi** : éviter la répétition.
- **Exemple** :
  ```rust
  fn creer_utilisateur(email: String, pseudo: String) -> Utilisateur {
      Utilisateur { email, pseudo, actif: true, nombre_de_connexions: 1 }
  }
  ```

### 3. Syntaxe de mise à jour de structure (struct update syntax)

- **Définition** : `..instance` copie les champs restants depuis une autre instance.
- **Pourquoi** : créer une nouvelle instance en ne modifiant que quelques champs.
- **Exemple** : `let u2 = Utilisateur { email: String::from("autre@mail.com"), ..u1 };`
- **Attention** : `..u1` *déplace* les champs. Si `u1` a des `String` non remplacées, `u1` n'est plus valide après. Les champs de type `Copy` (entiers, booléens) restent valides.

### 4. Structures tuples (tuple structs)

- **Définition** : `struct Couleur(i32, i32, i32);` — les champs n'ont pas de nom, seulement un type.
- **Pourquoi** : quand nommer chaque champ est trop verbeux mais qu'on veut un type distinct.
- **À retenir** : `Couleur(0, 0, 0)` et `Point(0, 0, 0)` sont de **types différents** même si leurs champs ont les mêmes types.

### 5. Structures unité (unit-like structs)

- **Définition** : `struct ToujoursEgal;` — structure sans aucun champ.
- **Pourquoi** : utile pour implémenter un trait sur un type qui n'a pas besoin de données (vu au chapitre 10).
- **À retenir** : s'instancie sans accolades ni parenthèses : `let sujet = ToujoursEgal;`.

### 6. Possession et références dans les structs

- **Définition** : une structure doit posséder ses données (utiliser `String` plutôt que `&str`) ou utiliser des lifetimes (chapitre 10).
- **Pourquoi** : garantir que les données pointées restent valides tant que la structure existe.
- **Erreur fréquente** : essayer de stocker `&str` dans une structure sans lifetime → `error[E0106]: missing lifetime specifier`.
- **À retenir** : tant qu'on n'a pas vu les lifetimes, toujours utiliser des types possédés (`String`, `Vec`, etc.) dans les champs.

### 7. Le trait `Debug` et `#[derive(Debug)]`

- **Définition** : `#[derive(Debug)]` ajoute automatiquement le trait `Debug` à une structure, permettant de l'afficher avec `{:?}` ou `{:#?}`.
- **Pourquoi** : `println!` ne peut pas afficher une structure par défaut (trait `Display` non implémenté). `Debug` est le formatage de débogage.
- **Syntaxe** :
  ```rust
  #[derive(Debug)]
  struct Rectangle { largeur: u32, hauteur: u32 }
  ```
- **À retenir** : `{:#?}` donne un affichage plus lisible (pretty-print) ; `{:?}` affiche sur une ligne.

### 8. La macro `dbg!`

- **Définition** : `dbg!(expression)` affiche le fichier, la ligne, l'expression et sa valeur, puis retourne la possession de la valeur.
- **Pourquoi** : outil de débogage plus riche que `println!` (inclut le fichier et la ligne, écrit sur `stderr`).
- **Exemple** : `dbg!(&rect1);` n'emprunte pas (prend une référence) pour ne pas perdre la possession.
- **À retenir** : `dbg!` prend possession de l'expression, donc on passe souvent une référence : `dbg!(&rect1)`.

### 9. Méthodes (`impl`)

- **Définition** : une fonction définie dans un bloc `impl` avec `self` (ou `&self`, `&mut self`) comme premier paramètre.
- **Pourquoi** : associer un comportement à un type de façon organisée (regroupé dans le `impl`).
- **Syntaxe** :
  ```rust
  impl Rectangle {
      fn aire(&self) -> u32 {
          self.largeur * self.hauteur
      }
  }
  ```
- **Appel** : `rect1.aire()` — syntaxe par méthode (point).
- **Paramètres additionnels** : `fn peut_contenir(&self, autre: &Rectangle) -> bool`.
- **À retenir** : `&self` est un raccourci pour `self: &Self`. `Self` est un alias du type du `impl`.

### 10. Fonctions associées

- **Définition** : fonctions définies dans `impl` sans paramètre `self`.
- **Pourquoi** : servir de constructeurs, comme `String::from`.
- **Exemple** : `Rectangle::carre(3)` retourne un `Rectangle` avec `largeur` et `hauteur` égales.
- **Appel** : `NomDeLaStructure::fonction(...)` — syntaxe `::`.

### 11. Référencement et déréférencement automatiques

- **Définition** : quand on appelle `objet.methode()`, Rust ajoute automatiquement `&`, `&mut` ou `*` pour que le type de `self` corresponde.
- **Pourquoi** : pas besoin d'opérateur `->` comme en C — Rust déduit automatiquement comment emprunter l'instance.
- **Exemple** : `p1.distance(&p2)` est équivalent à `(&p1).distance(&p2)`.

### 12. Plusieurs blocs `impl`

- **Définition** : une structure peut avoir plusieurs blocs `impl`, chacun pouvant contenir des méthodes et fonctions associées.
- **Pourquoi** : utile avec les types génériques (chapitre 10) ; pas d'obligation de tout mettre dans un seul bloc.

## Nouveaux mots-clés

| Mot-clé | Utilisation |
|---|---|
| `struct` | Définit une structure, une structure tuple ou une structure unité |
| `impl` | Bloc d'implémentation pour ajouter des méthodes/fonctions associées à un type |
| `Self` | Alias du type dans un bloc `impl` (ex. `self: &Self`) |
| `self` | Premier paramètre d'une méthode — représente l'instance sur laquelle la méthode est appelée |

## Fonctions, méthodes et macros importantes

| Fonction / Macro | Description |
|---|---|
| `println!("{:?}", instance)` | Affiche une valeur avec le formatage `Debug` (nécessite `#[derive(Debug)]`) |
| `println!("{:#?}", instance)` | Affiche avec `Debug` en mode pretty-print (plus lisible) |
| `dbg!(expression)` | Affiche fichier, ligne et valeur sur `stderr` ; retourne la possession de l'expression |
| `#[derive(Debug)]` | Attribut qui dérive automatiquement le trait `Debug` sur une structure |
| `NomStruct::fonction()` | Appel d'une fonction associée (constructeur) via la syntaxe `::` |

## Schéma mental

```
                    ┌──────────────────────────────────┐
                    │          STRUCT (struct)          │
                    │  Type personnalisé nommé          │
                    │  avec des champs typés            │
                    └──────┬───────────────┬────────────┘
                           │               │
              ┌────────────▼────┐    ┌─────▼───────────┐
              │   CLASSIQUE     │    │ STRUCTURE TUPLE │
              │  champs nommés  │    │  champs sans nom│
              │  { nom: Type }  │    │  (Type, Type)   │
              │  accès par .nom │    │  accès par .0   │
              └─────────────────┘    └─────────────────┘

        ┌──────────────────────────────────────────────────┐
        │           BLOC impl (implementation)             │
        │  ┌─────────────────────┐  ┌──────────────────┐   │
        │  │   MÉTHODES          │  │ FONCTIONS ASSOC. │   │
        │  │  fn aire(&self)     │  │ fn carre(cote)   │   │
        │  │  appel: rect.aire() │  │ appel: Rect::carre│  │
        │  └─────────────────────┘  └──────────────────┘   │
        └──────────────────────────────────────────────────┘

        ┌──────────────────────────────────────────────────┐
        │           DEBUG / AFFICHAGE                      │
        │  #[derive(Debug)] → permet {:?} et dbg!()        │
        │  dbg!() écrit sur stderr, utile pour débugger    │
        └──────────────────────────────────────────────────┘
```

Les structures sont aux données ce que les `impl` sont au comportement : on définit la forme des données avec `struct`, puis on attache les opérations possibles avec `impl`.

## Pièges classiques

1. **Oublier `mut` sur l'instance** — impossible de modifier un champ si l'instance n'est pas déclarée `let mut`.
2. **Utiliser `&str` dans une structure** — le compilateur réclame des lifetimes (`error[E0106]`). Utiliser `String` jusqu'au chapitre 10.
3. **Oublier `#[derive(Debug)]`** — `println!("{:?}", rect1)` ne compile pas : `Debug` n'est pas implémenté.
4. **Confondre méthode et fonction associée** — une méthode a `&self` et s'appelle avec `.` ; une fonction associée s'appelle avec `::`.
5. **Oublier `&` dans `&self`** — si on n'a que besoin de lire, `self` (sans `&`) prendrait possession de l'instance (rarement souhaitable).
6. **Ne pas réaliser que `..u1` déplace les champs** — après `let u2 = Utilisateur { email: "...", ..u1 };`, `u1` peut être invalide si des champs non-Copy (comme `String`) ont été déplacés.
7. **Croire que deux structures tuples avec les mêmes types sont interchangeables** — `Couleur(0,0,0)` et `Point(0,0,0)` sont de types distincts.

## Résumé

1. `struct` permet de créer un type personnalisé avec des champs nommés — plus expressif qu'un tuple.
2. Le field init shorthand évite la répétition quand paramètre et champ ont le même nom.
3. La syntaxe `..instance` copie les champs restants depuis une autre instance (move pour les types non-Copy).
4. Les structures tuples (`struct Nom(Type, Type)`) créent des types distincts sans nommer les champs.
5. Les structures unité (`struct Nom;`) servent pour implémenter des traits sans données.
6. `#[derive(Debug)]` permet d'afficher une structure avec `{:?}` ou `{:#?}`.
7. `dbg!()` affiche fichier, ligne et valeur sur `stderr` — outil de débogage pratique.
8. Les méthodes sont définies dans un bloc `impl`, avec `&self` (ou `&mut self`, `self`) en premier paramètre.
9. Les fonctions associées (sans `self`) s'appellent avec `::` et servent souvent de constructeurs.
10. Rust applique le référencement/déréférencement automatique lors des appels de méthode — pas besoin d'opérateur `->`.
