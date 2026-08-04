# Chapitre 8 — Les collections courantes

## Objectif du chapitre

Présenter les trois collections les plus utilisées de la bibliothèque standard : les **vecteurs** (`Vec<T>`), les **chaînes de caractères** (`String`) et les **tables de hachage** (`HashMap<K, V>`). Ces structures permettent de stocker plusieurs valeurs (en séquence, en texte ou en association clé-valeur). Le chapitre insiste surtout sur deux points récurrents en Rust : **l'allocation sur le tas (heap)** et **les règles d'ownership/borrowing** qui s'appliquent à ces collections. C'est la première fois que l'on manipule des données dont la taille est inconnue au moment de la compilation.

## Concepts abordés

### 1. Le vecteur (`Vec<T>`)

- **Définition** : un **vecteur** stocke plusieurs valeurs du **même type** dans une structure contiguë en mémoire, allouée sur le **tas**. Générique : `Vec<T>`.
- **Pourquoi** : avoir accès à une liste de taille dynamique (qui peut grandir) plutôt qu'à un tableau de taille fixe.
- **Créer** :
  - vide : `Vec::new()` (type annoté si pas d'éléments) ;
  - avec valeurs : la macro `vec![1, 2, 3]` (l'annotation est inutile ici).
- **Ajouter** : `push` (nécessite un vecteur **mutable**).
- **Lire un élément** : deux façons — l'indexation `v[2]` et la méthode `get(2)`.
  - `v[i]` : renvoie une référence directe ; **danger** : panique (crash) si l'index est hors bornes.
  - `v.get(i)` : renvoie un `Option<&T>` (`Some` / `None`) ; plus sûr car géré par pattern matching.
- **Règles d'ownership/borrowing** : comme partout, on ne peut pas avoir en même temps une référence **immuable** et une référence **mutable** sur le vecteur. Le borrow checker interdit d'emprunter immuablement puis d'utiliser `push` (emprunt mutable) ensuite.
- **Itérer** : `for i in &v` (lecture), `for i in &mut v` (modification — il faut déréférencer `*i` pour toucher la valeur).
- **Plusieurs types dans un seul vecteur** : impossible directement pour un `Vec<T>` ; on utilise une **enum** dont les variantes stockent les différents types (l'enum est un type unique).
- **Libération** : le vecteur est libéré du tas quand il sort de portée (drop), et libère avec lui ses éléments.

### 2. Les chaînes de caractères (`String`)

- **Définition** : `String` est une chaîne **modifiable, extensible** allouée sur le **tas** ; `&str` (appelé souvent "string slice") est une *référence* vers une chaîne immuable (couramment une string littérale).
- **Créer** : `String::new()` (vide), `String::from("…")`, `"…".to_string()`.
  - Toutes les chaînes littérales encodées en UTF-8 se convertissent avec `to_string()`.
- **Modifier** :
  - `push_str` : ajoute une `&str` à la fin ;
  - `push` : ajoute un seul caractère (`char`).
- **Concaténation `+`** : `s1 + &s2` — fait une *deref coercion* (le `&String` est converti en `&str`). **Important** : `+` **prend possession** de `s1` (l'opérande de gauche) mais seulement **emprunte** `s2`. `format!` permet de concaténer sans prendre possession d'aucune des chaînes.
- **L'indexation est interdite** : en Rust, `s[0]` ne fonctionne **pas** pour `String`. Pourquoi ? Car `String` n'est pas stockée en caractères individuels, mais en **octets** d'encodage UTF-8 (un caractère peut occuper 1 à 4 octets). Un index de type `usize` serait le numéro d'**octet**, pas de caractère.
- **Ne pas confondre** trois vues d'une même chaîne : les **octets**, les **scalaires Unicode** (`.chars()`), et les **graphes de caractères** (lettres visibles, dont Rust ne gère pas directement). La « taille » (`len()`) renvoie le **nombre d'octets**, pas de caractères.
- **Slices** : on peut prendre un slice d'une chaîne `&s[0..4]`, mais il **panique** si l'on coupe au milieu d'un caractère multi-octets (les bornes sont des indices d'octets).
- **Itérer** : `.chars()` pour parcourir les scalaires Unicode, `.bytes()` pour les octets.
- **À retenir** : `String` est en réalité une `Vec<u8>` qui garantit un contenu UTF-8 valide.

### 3. La table de hachage (`HashMap<K, V>`)

- **Définition** : stocke des paires clé-valeur, chaque clé pointant vers une valeur. Notée `HashMap<K, V>`, c'est le type d'une association "dictionnaire".
- **Pourquoi** : chercher une valeur par une clé (type) plutôt que par un index numérique, quand on ne connaît pas l'index à l'avance.
- **Créer / ajouter** : `HashMap::new()` puis `map.insert(clé, valeur)`.
- **Ownership** : pour les types possédés (comme `String`), `insert` **déplace** la clé et la valeur dans la table ; pour les types copiables (`Copy`), elles sont **copiées**. Insérer une référence ne déplace pas (elle reste valide, tant qu'elle le reste).
- **Lire** : `get(&clé)` renvoie un `Option<&V>` (`Some`/`None`). Attention : on passe une **référence** à `get`. Le retour est une référence — emprunt immuable.
- **Itérer** : `for (clé, valeur) in &map` — l'ordre n'est pas garanti.
- **Mettre à jour** :
  - réécrire une clé existante → `insert` **écrase** l'ancienne valeur ;
  - ajouter seulement si absente → `entry(clé).or_insert(valeur)` retourne une référence à l'entrée (existante ou nouvellement insérée) ;
  - mettre à jour à partir de l'ancienne valeur → via la référence renvoyée par `or_insert`.
- **À retenir** : basée sur une fonction de hachage ; la perf dépend de la qualité de cette fonction. De base, tout ce qui implémente la comparaison peut servir de clé.

## Nouveaux mots-clés

| Mot-clé / type | Utilisation |
|---|---|
| `Vec<T>` | Collection modifiable de valeurs du même type sur le tas |
| `vec![...]` | Macro pour créer un vecteur avec des valeurs |
| `String` | Chaîne modifiable, extensible, encodée en UTF-8, sur le tas |
| `&str` | Référence vers une chaîne immuable (string slice) |
| `HashMap<K, V>` | Table associative clé → valeur |
| `entry(...)` / `or_insert(...)` | Accès à une entrée existante ou insertion si absente |

## Fonctions, méthodes et macros importantes

| Fonction / Méthode | Description |
|---|---|
| `Vec::new()` | Créer un vecteur vide |
| `vec![1,2,3]` | Macro : créer un vecteur pré-rempli |
| `v.push(x)` | Ajouter `x` à la fin du vecteur (emprunt mutable) |
| `v[i]` | Indexer (panique si hors bornes) |
| `v.get(i)` | Retourner `Option<&T>` (sûr) |
| `String::new()` | Créer une chaîne vide |
| `String::from("…")` / `"…".to_string()` | Créer une chaîne depuis un littéral |
| `push_str` / `push` | Ajouter une `&str` / un `char` |
| `s1 + &s2` | Concaténer (prend possession de `s1`) |
| `format!` | Concaténer sans prendre possession |
| `.chars()` / `.bytes()` | Itérer sur les scalaires Unicode / les octets |
| `HashMap::new()` | Créer une table vide |
| `map.insert(k, v)` | Insérer une paire clé/valeur |
| `map.get(&k)` | Lire une valeur, retourne `Option<&V>` |
| `map.entry(k).or_insert(v)` | Insérer si absente, sinon renvoyer l'existante |

## Schéma mental

```
        COLLECTIONS (allocations sur le TAS)

   Vec<T>          String (= Vec<u8> UTF-8)      HashMap<K,V>
  plusieurs          suite d'octets                paires clé→valeur
  valeurs            (donc texte)
  même type
     │                   │                            │
     ├─ index [i] / get  ├─ pas d'index (UTF-8)       ├─ get(&k)
     ├─ push             ├─ push / push_str / +       ├─ insert
     └─ for in &v        └─ chars / bytes             └─ entry.or_insert

   Toutes obéissent à l'ownership :
   - vécus en portée → libérés (drop)
   - immuable vs mutable : les deux ne coexistent pas
   - certaines opérations déplacent (String + , HashMap::insert)
```

Le point commun est l'**allocation sur le tas** : les `Vec`, `String` et `HashMap` peuvent grandir dynamiquement. Leur taille étant inconnue au compile-time, leur gestion relève d'un pointeur + longueur + capacité. C'est la première expérience de structures qui vivent sur le tas, et les **règles d'ownership/borrowing** deviennent essentielles : référence immuable vs mutable, opérations qui déplacent vs celles qui empruntent.

## Pièges classiques

1. **Indexer avec `v[i]` sans vérifier les bornes** — risque de panique (crash) ; préférer `get` quand l'index est inconnu.
2. **Mélanger emprunt immuable et mutable** — lire via `get` (immuable) puis appeler `push` (mutable) provoque une erreur du borrow checker.
3. **Croire que `s[i]` fonctionne avec les `String`** — indexer une chaîne est interdit car Rust indexe les **octets** UTF-8, pas les caractères.
4. **Couper un slice `&s[0..n]` au milieu d'un caractère multi-octets** — panique ; il faut utiliser `chars()` pour compter par caractère.
5. **Prendre `len()` pour un nombre de caractères** — c'est un nombre d'**octets**.
6. **Oublier que `+` déplace la première String** — `s1 + &s2` consomme `s1` et n'est possible que sur une `String` (pas sur deux `&str`).
7. **Ne pas passer une référence à `get`** — `map.get(k)` ne compile pas ; il faut `map.get(&k)`.

## Résumé

1. **`Vec<T>`** : liste dynamique de valeurs du **même type** sur le tas ; `push` pour ajouter, `get` (sûr) ou `[i]` (paniquant) pour lire.
2. Un vecteur ne peut contenir qu'un type ; pour mélanger, utiliser une **enum**.
3. **`String`** : chaîne modifiable UTF-8 ; **`&str`** : référence immuable. On crée avec `to_string()` / `String::from()`.
4. `push_str` / `push` modifient ; `+` concatène **en prenant possession** de la première chaîne ; `format!` n'en prend aucune.
5. **Impossible d'indexer une `String`** : c'est une suite d'octets UTF-8 ; on itère avec `chars()` / `bytes()`, et `len()` compte les octets.
6. **`HashMap<K, V>`** : paires clé→valeur, créées via `new()` + `insert`, lues via `get(&k)` qui renvoie `Option`.
7. `insert` réécrit ; `entry().or_insert()` n'insère que si la clé est **absente**.
8. Les trois collections **libèrent leur mémoire** quand elles sortent de portée, et obéissent toutes aux règles d'ownership/borrowing.