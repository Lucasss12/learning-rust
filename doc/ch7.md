# Chapitre 7 — Gérer des projets d'envergure avec les packages, les crates et les modules

## Objectif du chapitre

Comprendre les outils que Rust offre pour structurer un projet qui grandit : les *packages* (projets Cargo), les *crates* (unités de compilation) et les *modules* (organisation interne du code). Le chapitre montre comment découper du code en fichiers et envolets réutilisables, contrôler la visibilité avec `pub`, et gérer les noms avec `use`. Toutes ces notions servent à garder un **code lisible et extensible** à mesure que le programme grossit.

## Concepts abordés

### 1. Package et Crate

- **Définition** : une **crate** est une unité de compilation (binaire ou bibliothèque) ; un **package** est un paquet de crates contenant un `Cargo.toml`. Un package peut contenir un maximum d'une crate bibliothèque (lib) et autant de crates binaires que voulu, avec des règles de nommage précises.
- **Crate bin** (binaire) : a une fonction `main` et produit un exécutable (`src/main.rs`).
- **Crate lib** (bibliothèque) : expose son code via `lib.rs`, réutilisable ailleurs (`src/lib.rs`).
- **Pourquoi** : regrouper le code qui travaille ensemble et en garder la propriété dans `src/` ; séparer "programme" (bin) et "logique réutilisable" (lib).
- **À retenir** : l'organisation classique définit `src/main.rs` pour le binaire et `src/lib.rs` pour la bibliothèque. Un fichier `main.rs` dans une crate nommée `foo` crée une crate nommée `foo` pour le binaire ; un `lib.rs` nomme une crate `foo` pour la bibliothèque. Deux crates peuvent partager le même nom `foo` (bin + lib) — c'est normal.
- **Erreur fréquente** : oublier que le module de niveau racine de la crate correspond au fichier d'entrée (`main.rs` ou `lib.rs`).

### 2. Le module (`mod`) et l'arborescence de modules

- **Définition** : un **module** (`mod`) est un morceau de code nommé qui regroupe des items (fonctions, structs, enums, traits, autres modules). Les modules s'imbriquent pour former une arborescence dont la **racine** est le fichier d'entrée de la crate.
- **Pourquoi** : organiser le code, contrôler la **visibilité** et contrôler l'accès aux détails de mise en œuvre.
- **Syntaxe racine** :
  ```rust
  // lib.rs
  mod sonUtilitaire;   // déclare le module « sonUtility » : option statique
  fn afficherTuile() {}
  ```
  Le compilateur cherche d'abord `src/sonUtilitaire.rs`, puis `src/sonUtilitaire/mod.rs`.
- **Déclaration vs définition** : `mod est_test` dans le fichier racine et le fichier `src/est_test.rs` forment le même module ; la dénomination des fichiers doit correspondre au nom du module.
- **Chemin d'accès** : on référence un item par son chemin, ex. `crate::sonUtilitaire::`.
- **À retenir** : le `crate` correspond aux imports/bibliothèque racine ; les modules consistent en fichiers et « dossiers » alternatives. Tous les items sont **privés par défaut**.
- **Erreur fréquente** : croire qu'un fichier est un module. Non : c'est le mot-clé `mod` dans un fichier parent qui fait *pointer* le compilateur vers ce fichier/dossier.

### 3. La visibilité et le mot-clé `pub`

- **Définition** : tous les items (fonctions, structs, modules...) sont **privés par défaut**. Un item privé n'est visible que dans son module et ses descendants. Le mot-clé `pub` le rend visible **à l'extérieur de son module** (et publiquement via `use`).
- **Pourquoi** : masquer la mise en œuvre interne, exposer seulement l'API choisie — c'est une protection de l'encapsulation.
- **Syntaxe** : `pub fn`, `pub struct`, `pub mod`... Un item privé reste utilisable *dans* son module et dans les modules qui en descendent.
- **Nuance pour les structs** : rendre une struct `pub` ne rend pas ses champs publics. Il faut `pub` sur chaque champ voulu ; un champ privé empêche la construction à l'extérieur.
- **Enums** : à la différence d'une struct, rendre une enum `pub` rend aussi **publiques** ses variantes (sinon on ne pourrait pas les utiliser).
- **Nuance d'usage** : un item `pub` reste inaccessible tant que ses parents ne sont pas `pub` — il faut les rendre `pub` sur toute la chaîne du chemin.
- **À retenir** : « privé » signifie invisible **aux modules parents et frères** (et au monde extérieur), mais visible dans le module et ses **sous-modules**.
- **Erreur fréquente** : rendre `pub` une struct mais oublier `pub` sur ses champs ; ou rendre `pub mod` mais oublier `pub` sur les fonctions des enfants.

### 4. Le mot-clé `use` et les chemins

- **Définition** : `use` importe un chemin (module ou item) dans la portée courante, pour pouvoir l'utiliser par son nom court au lieu du chemin complet.
- **Pourquoi** : raccourcir les chemins répétés et rendre le code plus lisible.
- **Syntaxe** :
  ```rust
  use crate::sonUtilities::affiherSom;        // import d'une fonction
  use crate::sonUtility::Adresse;              // import d'un type
  ```
  On référence ensuite simplement `somflerSom();` ou `Adresse::` au lieu du chemin complet.
- **Idiomatique** : on importe généralement une **fonction par son nom**, et un **type/struct/enum** par son nom (plus rarement son module). Le but est de garder le code `self-documentant`.
- **Re-export `pub use`** : `pub use` réexporte un nom sous le chemin courant, créant une **API publique différente** de la structure interne.
- **Création de grep :** utiliser `pub` pour rendre accessible aux consommateurs externes.
- **À retenir** : `use` crée juste un *raccourci* de nom dans la portée présente.

### 6. Le `use` imbriqué et les aliases

- **Définition** : proposer plusieurs importations d'une même crate/module en `glob` : `use` permet aussi les *imbrications* et les *alias*.
- **Alias `as`** :
  ```rust
  use std::fmt::Result;
  use std::io::Result as IoResult;  // éviter la collision de deux `Result`
  ```
- **Imports imbriqués** :
  ```rust
  use std::io::{self, Write};   // réimporte `std::io` plus `Write`
  ```
- **Glob `*`** :
  ```rust
  use std::collections::*;  // importe tous les items publics (à utiliser
  ```
  judicieusement : rend difficile de savoir quels noms sont dans la portée).
- **Pourquoi** : réduire les lignes de code et régler les conflits de noms.
- **Erreur fréquente** : confondre l'*alias* (`as`) qui change le nom local, et l'import imbriqué qui regroupe plusieurs use ligne par ligne.

### 7. Séparer modules dans plusieurs fichiers (pousser les modules)

- **Définition** : `mod sonUtilitaire;` dans la racine peut pointer vers `src/sonUtilitaire.rs` (fichier) ou `src/sonUtilitaire/mod.rs` (dossier). Chaque module devient un fichier, et les chemins continuent de fonctionner identiquement.
- **Pourquoi** : le code reste organisé et lisible quand les fichiers deviennent trop gros.
- **À retenir** : seul le module à la Racine ne porte pas prénom `pub` ; les modules en dessous peuvent être `pub`. La *déclaration* (le `mod`) et la *définition* (le contenu du fichier) restent liées par le nom du fichier.

## Nouveaux mots-clés

| Mot-clé | Utilisation |
|---|---|
| `mod` | Déclare un module (nouveau sous-arbre du `crate`) |
| `pub` | Rend un item visible de l'extérieur (public) |
| `use` | Importe un chemin dans la portée (raccourci) |
| `as` | Alias pour renommer un import et éviter les collisions |
| `crate` | Racine du module d'une crate |
| `pub use` | Re-export : import et rend disponible le nom |

## Fonctions, méthodes et macros importantes

| Fonction / Macro | Description |
|---|---|
| `crate::<module>::<item>` | Chemin absolu depuis la racine de la crate |
| `mod <name>;` | Déclare le module `<name>` (cherche `.rs` / `/mod.rs`) |
| `pub fn / pub mod / pub struct` | Rend l'item public et donc accessible via `use` |
| `use ...::*;` | Glob : importe tous les items publics du module |
| `pub use` | Re-export d'un nom sous le chemin courant |
| `use a::{self, b}` | Import imbriqué du module et d'un sous-item |

## Schéma mental

```
        PACKAGE (Cargo.toml)
        ├── crate BIN → src/main.rs (fn main), exécutable
        └── crate LIB → src/lib.rs (racine de la « crate »)

        crate racine
        └── mod  ↓  (ce sont des fichiers)
            sonUtilitaire.rs (ou sonUtilitaire/mod.rs)
            └── items (fn, struct...)
                Visible : privé par défaut
                pub → visible par les parents / via use

   Chemins :  use crate::sonUtilitaire::...
   use = raccourci du chemin complet
   pub use = ré-export
```

Tout part de la **crate** (unité de compilation). Le **module** organise le code à l'intérieur ; la **visibilité** (`pub`) décide ce qui est exposé ; **`use`** et ses variantes (**`as`**, import imbriqué, **`pub use`**) raccourcissent et permettent de re-déclarer les noms. `main.rs`/`lib.rs` sont les racines appelées `crate`.

## Pièges classiques

1. **Croire qu'un fichier est *de soi* un module** — c'est `mod` (avec le nom de fichier) qui crée le module.
2. **Rendre `pub` un module mais pas les fonctions/structs dedans** — la visibilité doit être `pub` sur chaque niveau du chemin.
3. **Rendre `pub` une struct sans rendue `pub` les champs** — les champs restent privés (à l'inverse des variantes d'enums).
4. **Oublier que les items sont privés par défaut** — demander un item `pub` chacune des fois qu'on veut y accéder ailleurs.
5. **Collision de noms** (`Result`, etc.) — régler avec un alias `as`.
6. **Le 'glob `*` systématique** — importe tout, mais rend confus les incontournables et peut entrer en conflit.
7. **Ne pas matcher le nom de module/fichier** — `mod foo;` demande `src/foo.rs` (et non un nom différent).

## Résumé

1. Une **crate** est une unité de compilation : binaire (`main.rs`) ou bibliothèque (`lib.rs`). Un **package** regroupe des crates avec un `Cargo.toml`.
2. Les **modules** (`mod`) organisent le code en arborescence dont la racine est le `crate`.
3. Les items sont **privés par défaut** : inaccessible depuis les modules parents et frères, visibles dans le module et ses enfants.
4. **`pub`** rend un item accessible à l'extérieur. Pour structs il faut `pub` par champ ; pour enums, `pub` rend tout.
5. **`use`** amène un chemin dans la portée ; **`as`** renomme ; les importats imbriqués et `pub` permettent de ré-exporter.
6. Séparer les modules dans plusieurs fichiers (`.rs` ou dossier `/mod.rs`) au fur et à mesure qu'elles `grossiss`.
7. La combinaison package → crate → module → `pub`/`use` permet de faire croître le projet tout en restant lisible.