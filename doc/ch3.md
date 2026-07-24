# Chapitre 3 — Les concepts courants de programmation

## Objectif du chapitre

Faire le tour des concepts communs à tous les langages (variables, types, fonctions, commentaires, structures de contrôle) vus à travers le prisme de Rust — avec ses spécificités comme l'immuabilité par défaut, le système de types statique, la distinction instruction/expression, et les boucles.

## Concepts abordés

### 3.1 Variables et mutabilité

#### 1. Immuabilité par défaut

- **Définition** : une variable déclarée avec `let` ne peut pas être réassignée.
- **Pourquoi** : le compilateur empêche les bugs liés à des changements inattendus de valeur. `let x = 5; x = 6;` → erreur.
- **Quand l'utiliser** : toujours, sauf quand la mutabilité est explicitement nécessaire.
- **Erreur fréquente** : `cannot assign twice to immutable variable` — ajouter `mut`.
- **À retenir** : l'immuabilité est un choix de conception de Rust pour la sûreté, pas une limitation.

#### 2. `mut` (mutabilité)

- **Définition** : `let mut x = 5;` permet de modifier `x` plus tard.
- **Pourquoi** : certaines situations (grandes structures, performances) bénéficient de la mutation.
- **Quand l'utiliser** : quand la logique exige de changer la valeur.
- **À retenir** : `mut` signale l'intention au lecteur du code.

#### 3. Constantes (`const`)

- **Définition** : `const TROIS_HEURES_EN_SECONDES: u32 = 60 * 60 * 3;` — valeur immuable, type obligatoire, portée globale.
- **Différence avec `let`** : toujours immuable (pas de `mut`), type obligatoire, peut être déclarée dans n'importe quelle portée.
- **Quand l'utiliser** : pour des valeurs magiques, des limites, des constantes de configuration.
- **Convention** : nom en `SCREAMING_SNAKE_CASE`.
- **À retenir** : les constantes sont valables toute la durée d'exécution du programme. Leur valeur est calculée à la compilation.

#### 4. Shadowing (masquage)

- **Définition** : `let x = 5; let x = x + 1;` — la deuxième déclaration masque la première.
- **Différence avec `mut`** : `mut` change la valeur, le shadowing peut changer **le type** (ex: `let espaces = "   "; let espaces = espaces.len();`).
- **Quand l'utiliser** : après une transformation, pour réutiliser le même nom.
- **Portée** : un shadowing dans un bloc `{ }` ne dure qu'à l'intérieur de ce bloc.
- **À retenir** : le shadowing crée une nouvelle variable ; l'ancienne existe toujours mais n'est plus accessible.

### 3.2 Types de données

#### 5. Inférence de type et annotations

- **Définition** : Rust infère le type, mais on peut l'annoter : `let x: u32 = 5;`.
- **Pourquoi** : Rust est statiquement typé — il doit connaître le type de chaque variable à la compilation.
- **Quand annoter** : quand plusieurs types sont possibles (ex: `parse()`).
- **À retenir** : l'inférence évite la redondance ; les annotations lèvent les ambiguïtés.

#### 6. Types scalaires

##### Entiers

- **Définition** : `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (signés) ; `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (non signés).
- **Par défaut** : `i32`.
- **`isize` / `usize`** : taille = architecture de la machine (utilisé pour l'indexation).
- **Littéraux** : décimal (`98_222`), hexadécimal (`0xff`), octal (`0o77`), binaire (`0b1111_0000`), octet (`b'A'`).
- **Dépassement d'entier** : en debug → panique ; en release → rebouclage (complément à deux). Méthodes `wrapping_*`, `checked_*`, `overflowing_*`, `saturating_*` pour contrôle explicite.

##### Flottants

- **Définition** : `f32` (32 bits), `f64` (64 bits). Par défaut : `f64`.
- **À retenir** : tous les flottants sont signés. IEEE-754.

##### Opérations numériques

- `+`, `-`, `*`, `/`, `%` — la division entière tronque vers zéro (ex: `2 / 3 = 0`).

##### Booléen (`bool`)

- **Définition** : `true` ou `false`, 1 octet.
- **À retenir** : Rust ne convertit pas implicitement les entiers en booléens. `if nombre` ne compile pas ; il faut `if nombre != 0`.

##### Caractère (`char`)

- **Définition** : `'a'`, `'😻'` — guillemets simples, 4 octets, représente un scalaire Unicode.
- **Différence avec `String`** : `char` est un seul caractère ; `String` est une chaîne UTF-8.
- **À retenir** : un `char` peut être un emoji, un accent, etc.

#### 7. Types composés

##### Tuple

- **Définition** : `let tup: (i32, f64, u8) = (500, 6.4, 1);` — taille fixe, types hétérogènes.
- **Accès** : déstructuration `let (x, y, z) = tup;` ou indexation `tup.0`, `tup.1`.
- **Type unité** : `()` — tuple vide. Les fonctions sans retour explicite retournent `()`.
- **À retenir** : taille fixe, pas de modification après déclaration.

##### Tableau (array)

- **Définition** : `let a = [1, 2, 3, 4, 5];` — taille fixe, tous les éléments du même type.
- **Annotation** : `let a: [i32; 5] = [1, 2, 3, 4, 5];` — `[type; taille]`.
- **Raccourci** : `let a = [3; 5];` → `[3, 3, 3, 3, 3]`.
- **Accès** : `a[0]`, `a[1]`. Rust vérifie les bornes à l'exécution → panique si indice invalide.
- **Différence avec vecteur** : l'array a une taille fixe (stack) ; le vecteur (`Vec`) peut grandir (heap).
- **Sécurité mémoire** : Rust panique plutôt que de laisser un accès mémoire invalide.
- **À retenir** : préférer `Vec` si la taille n'est pas connue à l'avance. L'index invalide → `panic!`.

### 3.3 Fonctions

#### 8. Déclaration et appel

- **Définition** : `fn ma_fonction(param: i32) -> i32 { ... }`.
- **Convention** : *snake_case* pour les noms.
- **Ordre** : les fonctions peuvent être définies avant ou après `main` — Rust s'en moque.
- **Paramètres** : type obligatoire pour chaque paramètre.
- **À retenir** : `fn` déclare une fonction ; `()` pour les paramètres ; `{}` pour le corps.

#### 9. Instructions vs expressions

- **Instruction** : effectue une action, ne retourne pas de valeur. Ex: `let y = 6;`.
- **Expression** : s'évalue en une valeur. Ex: `5 + 6`, `x + 1`, un bloc `{ }`.
- **Règle clé** : une expression sans point-virgule `;` retourne sa valeur ; un point-virgule la transforme en instruction.
- **Erreur fréquente** : mettre `;` après la dernière expression d'une fonction censée retourner une valeur.
- **À retenir** : `{}` est une expression. `let y = { let x = 3; x + 1 };` → `y = 4`.

#### 10. Valeurs de retour

- **Définition** : la dernière expression du corps est la valeur de retour (implicite). On peut aussi utiliser `return valeur`.
- **Syntaxe** : `fn cinq() -> i32 { 5 }` — pas de `;` après `5`.
- **À retenir** : toujours déclarer le type de retour avec `->`. Si pas de retour, `-> ()` implicite.

### 3.4 Commentaires

- **Syntaxe** : `// commentaire sur une ligne` ou `// commentaire\n// sur plusieurs lignes`.
- **Doc comments** : `///` (abordés au chapitre 14).
- **À retenir** : les commentaires sont ignorés par le compilateur.

### 3.5 Structures de contrôle

#### 11. `if`

- **Définition** : `if condition { ... } else { ... }`.
- **Condition** : doit être un `bool` — pas de conversion implicite (contrairement à JS/TS).
- **`else if`** : enchaînement de conditions. Exécute le premier bloc dont la condition est vraie.
- **`if` dans `let`** : `let x = if condition { 5 } else { 6 };` — les branches doivent avoir le même type.
- **Erreur fréquente** : utiliser un entier directement comme condition (`if nombre`).

#### 12. Boucles

##### `loop`

- **Définition** : boucle infinie. `break` pour sortir.
- **Retourner une valeur** : `let resultat = loop { break valeur; };`.
- **Étiquettes** : `'ma_boucle: loop { break 'ma_boucle; }` — pour `break`/`continue` sur une boucle externe.
- **À retenir** : `loop` est la boucle la plus flexible.

##### `while`

- **Définition** : `while condition { ... }` — répète tant que la condition est vraie.
- **Équivalent** : `loop { if !condition { break; } ... }`.
- **À retenir** : plus lisible que `loop + if + break` pour une condition simple.

##### `for`

- **Définition** : `for element in collection { ... }` — itère sur chaque élément.
- **Sécurité** : pas de risque d'indice invalide (contrairement à `while` + index).
- **Range** : `for nombre in (1..4).rev() { ... }` — itère de 1 à 3 en ordre inverse.
- **À retenir** : `for` est la boucle la plus utilisée en Rust — sûre, concise, idiomatique.

## Nouveaux mots-clés

- `const` — déclare une constante (toujours immuable, type obligatoire)
- `fn` — déclare une fonction
- `let` — déclare une variable
- `mut` — rend une variable mutable
- `if` / `else` — condition
- `loop` — boucle infinie
- `while` — boucle conditionnelle
- `for` — boucle d'itération
- `break` — sort d'une boucle
- `continue` — passe à l'itération suivante
- `return` — retourne prématurément une valeur
- `true` / `false` — littéraux booléens
- `bool`, `char`, `i32`, `u32`, `f64`, etc. — types primitifs

## Fonctions, méthodes et macros importantes

| Fonction / Syntaxe | Description |
|---|---|
| `s.cmp(&autre)` | Compare deux valeurs, retourne `Ordering` |
| `s.trim()` | Supprime les whitespaces |
| `s.parse::<T>()` | Convertit une `&str` en type `T` |
| `(1..4).rev()` | Crée un intervalle inversé |
| `if condition { } else { }` | Condition (doit être un `bool`) |
| `loop { break val; }` | Boucle infinie avec retour de valeur |
| `while cond { }` | Boucle conditionnelle |
| `for x in iter { }` | Itération sur une collection |
| `'label: loop { break 'label; }` | Étiquette de boucle |

## Schéma mental

```
Variables et mutabilité :
  let x = 5;       → immuable
  let mut x = 5;   → mutable
  const X: i32 = 5; → constante globale
  let x = x + 1;   → shadowing (nouvelle variable)

Types de données :
  Scalaire : i32, u32, f64, bool, char
  Composé : (i32, f64)   → tuple
            [i32; 5]     → array (taille fixe)

Fonctions :
  fn nom(params) -> Type { dernière_expression }
  Instruction ≠ Expression (pas de ;)

Contrôle :
  if bool { } else { }
  loop { break }
  while bool { }
  for x in iter { }
```

## Pièges classiques

1. **Oublier `mut`** — `let x = 5; x = 6;` ne compile pas.
2. **Mettre `;` après la dernière expression** d'une fonction qui doit retourner une valeur — ça devient `()`.
3. **Utiliser un entier dans `if`** — `if nombre` ne compile pas ; écrire `if nombre != 0`.
4. **Croire que les types sont convertis implicitement** — Rust ne fait jamais de conversion implicite.
5. **Oublier les bornes d'un tableau** — `a[indice]` avec indice invalide → `panic!`.
6. **Confondre `while` et `for`** — `for` est plus sûr pour itérer sur des collections.
7. **Oublier que `let` dans un `if` (`let x = if...`) exige des branches du même type** — erreur de compilation.
8. **Ne pas connaître les types d'entiers** — utiliser `i32` par défaut, `usize` pour l'indexation.
9. **Croire que `for (i=0; i<n; i++)` existe** — Rust n'a pas ce type de boucle C. On utilise `for i in 0..n`.
10. **Oublier `//` pour les commentaires** — pas de `/* */` multi-lignes (sauf `/* */` qui existe en Rust aussi, mais `//` est la convention).

## Résumé

1. Les variables sont immuables par défaut ; `mut` les rend modifiables ; `const` pour les constantes globales.
2. Le shadowing (`let x = ...`) permet de réutiliser un nom et de changer de type.
3. Les types scalaires : entiers (`i32`, `u32`...), flottants (`f64`, `f32`), `bool`, `char`.
4. Les types composés : tuples (taille fixe, types hétérogènes) et arrays (taille fixe, même type).
5. Les fonctions : `fn nom(params: Type) -> Type`. Type obligatoire pour les paramètres.
6. **Instruction** ≠ **Expression** : une expression sans `;` retourne une valeur.
7. `if` exige une condition de type `bool` — pas de conversion implicite.
8. Boucles : `loop` (infini), `while` (conditionnel), `for` (itération — la plus sûre).
9. `for` sur un intervalle : `for i in 0..10` ou `(1..4).rev()`.
10. Rust est un langage à typage statique fort avec inférence de type.
