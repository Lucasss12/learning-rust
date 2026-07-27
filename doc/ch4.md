# Chapitre 4 — Comprendre la possession

## Objectif du chapitre

Comprendre le système de possession (ownership), la fonctionnalité la plus distincte de Rust. Ce chapitre explique comment Rust gère la mémoire sans ramasse-miettes grâce à trois mécanismes : la possession, l'emprunt (borrowing) via les références, et le type slice. Ces règles sont vérifiées à la compilation et n'ont aucun impact sur les performances à l'exécution.

## Concepts abordés

### 4.1 La possession (ownership)

#### 1. Stack et Heap

- **Définition** : la stack (pile) stocke des données de taille connue à la compilation ; le heap (tas) stocke des données de taille inconnue ou dynamique.
- **Pourquoi** : la possession existe principalement pour gérer les données sur le heap. La stack est automatique (LIFO), le heap nécessite une allocation et une libération explicites.
- **À retenir** : les types simples comme les entiers sont sur la stack ; les `String`, les `Vec` etc. allouent sur le heap.

#### 2. Les trois règles de la possession

- **Définition** :
  1. Chaque valeur en Rust a un propriétaire (owner).
  2. Il ne peut y avoir qu'un seul propriétaire à la fois.
  3. Quand le propriétaire sort de la portée (scope), la valeur est libérée via `drop`.
- **Pourquoi** : garantir la sécurité mémoire sans ramasse-miettes ni `malloc`/`free` manuel.
- **Erreurs fréquentes** : `use of moved value` — utiliser une variable après avoir déplacé sa valeur.
- **À retenir** : ces trois règles sont vérifiées à la compilation, coût zéro à l'exécution.

#### 3. `String` vs littéral de chaîne

- **Définition** : `String::from("texte")` alloue sur le heap (taille inconnue à la compilation) ; les littéraux `"texte"` sont codés en dur dans le binaire (immuables, type `&str`).
- **Pourquoi** : les littéraux ne peuvent pas représenter du texte dynamique (saisie utilisateur, lecture fichier).
- **À retenir** : `String` est modifiable et allouée sur le heap ; les littéraux sont des slices `&str` immuables.

#### 4. `drop` — libération automatique

- **Définition** : Rust appelle automatiquement la fonction `drop` sur une valeur quand son propriétaire sort de la portée.
- **Pourquoi** : pas besoin de `free` manuel — `drop` est appelé à l'accolade fermante `}`.
- **À retenir** : similaire au RAII du C++ ; garantit l'absence de fuite mémoire.

#### 5. Move (déplacement)

- **Définition** : `let s2 = s1;` déplace la valeur de `s1` vers `s2`. `s1` n'est plus valide après l'assignation.
- **Pourquoi** : éviter la double libération (double free). Seules les données stack (pointeur, length, capacity) sont copiées, pas les données heap. Rust invalide `s1` pour qu'un seul `drop` ait lieu.
- **Erreur fréquente** : `borrow of moved value` — utiliser `s1` après `let s2 = s1;`.
- **À retenir** : Rust ne fait pas de shallow copy silencieuse mais un *move* : la source est invalidée. Pas de deep copy non plus sans `clone`.

#### 6. `clone` — copie profonde explicite

- **Définition** : `let s2 = s1.clone();` copie les données heap et stack.
- **Pourquoi** : quand on a besoin de deux instances indépendantes.
- **Quand l'utiliser** : uniquement quand une vraie copie des données est nécessaire — coûteux en mémoire.
- **À retenir** : `clone` est explicite ; une simple assignation (`=`) fait un move.

#### 7. Le trait `Copy`

- **Définition** : les types stockés entièrement sur la stack (entiers, booléens, flottants, `char`, tuples de types `Copy`) implémentent le trait `Copy`. Ils sont copiés implicitement, pas déplacés.
- **Pourquoi** : copier un entier est trivial (quelques octets) ; la source reste valide après assignation.
- **Quand l'utiliser** : automatique sur les types qui l'implémentent. Un type personnalisé (struct) peut dériver `Copy` si tous ses champs sont `Copy`.
- **Erreur fréquente** : oublier qu'un type n'implémente pas `Copy` et le traiter comme copiable — `use of moved value`.
- **À retenir** : `Copy` ≠ `clone`. `Copy` est implicite et bon marché ; `clone` est explicite et potentiellement coûteux.

#### 8. Transfert de possession dans les fonctions

- **Définition** : passer une variable à une fonction déplace (ou copie si `Copy`) la valeur. La fonction en devient le nouveau propriétaire.
- **Pourquoi** : la fonction peut libérer la mémoire quand son paramètre sort de sa portée.
- **Erreur fréquente** : ne pas pouvoir utiliser une variable après l'avoir passée à une fonction. Solution : la retourner dans un tuple ou (mieux) utiliser une référence.
- **À retenir** : les mêmes règles d'assignation s'appliquent aux appels de fonction. Retourner une valeur transfère aussi la possession.

### 4.2 Les références et l'emprunt (borrowing)

#### 9. Références (`&`)

- **Définition** : `&s` crée une référence vers `s` sans en prendre possession. Le type `&String` est une référence à une `String`.
- **Pourquoi** : éviter de déplacer la valeur dans une fonction et devoir la retourner.
- **Quand l'utiliser** : quand on a besoin de lire (ou modifier via `&mut`) une valeur sans en devenir propriétaire.
- **Erreur fréquente** : confondre `&String` et `String`. Une fonction qui prend `&String` n'a pas besoin de retourner la valeur.
- **À retenir** : créer une référence s'appelle *emprunter*. La référence doit toujours pointer vers une valeur valide.

#### 10. Références mutables (`&mut`)

- **Définition** : `&mut s` permet de modifier la valeur empruntée. Le type est `&mut String`.
- **Pourquoi** : permettre à une fonction de modifier une valeur sans en prendre possession.
- **Règle** : une seule référence mutable à la fois pour une même donnée.
- **Erreur fréquente** : `cannot borrow as mutable` — créer deux `&mut` vers la même donnée dans la même portée.
- **À retenir** : `&mut` empêche les accès concurrents (data races) à la compilation.

#### 11. Règles combinées des références

- **Définition** : à un instant donné, pour une donnée : soit une référence mutable, soit un nombre quelconque de références immuables.
- **Pourquoi** : les lectures simultanées sont sûres ; une écriture simultanée à une lecture ne l'est pas.
- **Erreur fréquente** : `cannot borrow as mutable because it is also borrowed as immutable` — utiliser une référence mutable alors qu'une référence immuable est encore en vigueur.
- **À retenir** : la portée d'une référence s'arrête à sa dernière utilisation (NLL — Non-Lexical Lifetimes). On peut avoir `&` puis `&mut` si le `&` n'est plus utilisé.

#### 12. Références pendouillantes (dangling references)

- **Définition** : une référence qui pointe vers une mémoire déjà libérée.
- **Pourquoi Rust les interdit** : le compilateur garantit qu'une référence ne survit jamais à la donnée qu'elle référence — grâce aux lifetimes (abordées au chapitre 10).
- **Erreur fréquente** : retourner `&s` où `s` est créée dans la fonction. Solution : retourner `s` directement (transfert de possession).
- **À retenir** : Rust refuse de compiler du code avec des dangling references. Pas de `&` dans le type de retour si la donnée est créée dans la fonction.

### 4.3 Le type slice

#### 13. Slice de chaîne de caractères (`&str`)

- **Définition** : `&s[0..5]` est une référence vers une partie de la `String`. Le type s'écrit `&str`.
- **Pourquoi** : un indice `usize` seul n'est pas lié à la `String` d'origine — problème de sync. Une slice est liée à la donnée d'origine par le système d'emprunt.
- **Syntaxe** : `&s[début..fin]`, `&s[..fin]`, `&s[début..]`, `&s[..]` (slice totale).
- **Erreur fréquente** : créer une slice qui coupe au milieu d'un caractère UTF-8 → `panic!`.
- **À retenir** : `&str` est immuable et lié à la donnée source. Le compilateur empêche de modifier la `String` tant que la slice existe.

#### 14. Littéraux de chaîne sont des `&str`

- **Définition** : `let s = "hello";` — le type de `s` est `&str`, une slice pointant vers une zone du binaire.
- **Pourquoi** : explique pourquoi les littéraux de chaîne sont immuables.
- **À retenir** : `&str` est le type par défaut pour une chaîne qu'on ne fait que lire.

#### 15. `&str` en paramètre (préféré à `&String`)

- **Définition** : `fn premier_mot(s: &str) -> &str` accepte à la fois `&String`, `&s[..]`, et `&str`.
- **Pourquoi** : plus flexible — la fonction fonctionne avec tous les types de chaînes sans perte de fonctionnalité.
- **À retenir** : toujours préférer `&str` à `&String` dans les paramètres de fonction.

#### 16. Slice de tableau (`&[i32]`)

- **Définition** : `&a[1..3]` crée une slice sur un tableau. Type : `&[i32]`.
- **Pourquoi** : même principe que `&str` mais pour les tableaux et autres collections.
- **À retenir** : une slice stocke un pointeur vers le premier élément et une longueur.

## Nouveaux mots-clés

Aucun nouveau mot-clé n'est introduit au chapitre 4. Les concepts de possession, emprunt et slice utilisent des mécanismes décrits par le compilateur (règles) et la syntaxe existante (`&`, `&mut`, `*`).

## Opérateurs et syntaxe importants

| Syntaxe | Description |
|---|---|
| `&variable` | Crée une référence (immuable) vers `variable` |
| `&mut variable` | Crée une référence mutable |
| `*reference` | Déréférencement (accéder à la valeur pointée) |
| `variable.clone()` | Copie profonde explicite des données heap |
| `&s[debut..fin]` | Slice d'une `String` ou d'un tableau |

## Fonctions, méthodes et macros importantes

| Méthode | Description |
|---|---|
| `String::from("texte")` | Crée une `String` allouée sur le heap à partir d'un littéral |
| `s.push_str("...")` | Ajoute une chaîne à la fin d'une `String` (nécessite `&mut`) |
| `s.len()` | Retourne la longueur d'une `String` en octets |
| `s.as_bytes()` | Convertit une `String` en slice d'octets (`&[u8]`) |
| `iterateur.enumerate()` | Transforme un itérateur en tuples `(index, élément)` |
| `assert_eq!(a, b)` | Vérifie que `a == b` (panique si faux) |

## Schéma mental

```
            ┌──────────────────────────────────────┐
            │           POSSESSION (ownership)      │
            │  1 valeur = 1 propriétaire            │
            │  drop() quand le owner sort du scope  │
            └──────┬─────────────────────────┬──────┘
                   │                         │
        ┌──────────▼──────────┐    ┌─────────▼──────────┐
        │        MOVE         │    │      BORROWING      │
        │  let s2 = s1;       │    │  let r = &s;        │
        │  → s1 invalidé      │    │  → prêt, pas move   │
        │  → 1 propriétaire   │    │  → r ne possède pas │
        └─────────────────────┘    └─────────┬───────────┘
                                     ┌───────┴────────┐
                                     │                │
                              ┌──────▼──────┐  ┌─────▼──────┐
                              │  & (immuable)│  │ &mut (mut.)│
                              │  N références│  │ 1 seule    │
                              │  simultanées │  │ à la fois  │
                              └──────────────┘  └────────────┘
                        ┌──────────────────────────────────────┐
                        │              SLICE (&str)            │
                        │  référence vers une partie des data  │
                        │  liée à la source par le borrowing    │
                        │  empêche la mutation tant qu'elle vit │
                        └──────────────────────────────────────┘
```

En résumé : la possession est le propriétaire unique. L'emprunt permet de lire (ou modifier) sans devenir propriétaire. Les slices sont des emprunts sur une partie des données.

## Pièges classiques

1. **Utiliser une variable après un move** — `let s2 = s1; println!("{}", s1);` → `use of moved value`.
2. **Oublier que les types `Copy` (entiers, booléens) ne bougent pas** — ils sont copiés implicitement, donc toujours valides après assignation.
3. **Ne pas retourner la valeur d'une fonction** qui prend possession — impossible de l'utiliser après l'appel.
4. **Utiliser `&` alors qu'on a besoin de `&mut`** — `cannot borrow as mutable`.
5. **Créer deux `&mut` vers la même donnée** — `cannot borrow as mutable more than once at a time`.
6. **Avoir une référence immuable en vigueur pendant qu'on tente un `&mut`** — `cannot borrow as mutable because it is also borrowed as immutable`.
7. **Retourner une référence vers une variable locale** — dangling reference, le compilateur la refuse.
8. **Créer une slice au milieu d'un caractère UTF-8** — `panic!` à l'exécution.
9. **Utiliser `&String` en paramètre au lieu de `&str`** — `&str` est plus flexible et accepte aussi les `&String`.
10. **Confondre `&s[..]` (slice de toute la chaîne) et `clone()`** — la slice emprunte, `clone` copie.

## Résumé

1. La possession suit trois règles : chaque valeur a un propriétaire, un seul propriétaire à la fois, la valeur est libérée quand le propriétaire sort de la portée.
2. L'assignation (`let s2 = s1`) *déplace* la valeur — `s1` n'est plus valide. Pas de shallow copy silencieuse.
3. `clone()` effectue une copie profonde explicite.
4. Les types `Copy` (entiers, flottants, `bool`, `char`) sont copiés implicitement, jamais déplacés.
5. Les références (`&`) permettent d'emprunter sans prendre possession.
6. Les références mutables (`&mut`) sont limitées à une seule à la fois — prévient les data races.
7. Règle d'or : soit N références immuables, soit 1 référence mutable.
8. Rust interdit les dangling references à la compilation.
9. Une slice (`&str`, `&[i32]`) est une référence vers une partie contiguë d'une collection — liée à la source par les règles d'emprunt.
10. Toujours préférer `&str` à `&String` dans les paramètres de fonction pour plus de flexibilité.
