# Chapitre 9 — La gestion des erreurs

## Objectif du chapitre

Présenter les deux grandes façons de gérer les erreurs en Rust : les erreurs **irrécupérables** avec la macro `panic!` (le programme s'arrête) et les erreurs **récupérables** avec le type `Result<T, E>` (on laisse l'appelant décider quoi faire). Le chapitre montre comment propager proprement les erreurs grâce à l'opérateur `?`, comment utiliser les raccourcis `unwrap` / `expect`, et propose des règles pour choisir entre `panic!` et `Result`. Il introduit enfin l'idée de **type personnalisé de validation** pour garantir des invariants.

## Concepts abordés

### 1. Les erreurs irrécupérables : `panic!`

- **Définition** : `panic!` est une macro qui affiche un message d'erreur, puis **déroule la pile** (unwind) : le programme libère les ressources de toutes les fonctions en cours d'appel avant de s'arrêter.
- **Pourquoi il existe** : certaines situations ne peuvent tout simplement pas être gérées (état invalide, bug, invariant impossible à tenir). À ce moment-là, arrêter le programme est préférable à continuer dans un état corrompu.
- **Quand il est déclenché** :
  - de façon **explicite** : `panic!("message")` ;
  - de façon **implicite** : une action qui enfreint une règle de Rust, comme l'indexation hors bornes `v[i]` avec un index trop grand.
- **Erreurs fréquentes** : ne pas chercher la cause réelle d'un panique. Le message ne suffit pas toujours ; il faut regarder le **backtrace** (historique des appels) en réglant la variable d'environnement `RUST_BACKTRACE=1`.
- **À retenir** : on peut aussi choisir d'**abandonner** la pile (abort) au lieu de la dérouler, en réglant le profil `release` dans `Cargo.toml` (`panic = "abort"`) — le binaire est alors plus petit mais la mémoire n'est pas libérée.

### 2. Les erreurs récupérables : `Result<T, E>`

- **Définition** : `Result<T, E>` est une **enum** avec deux variantes : `Ok(T)` (opération réussie, contient la valeur) et `Err(E)` (opération échouée, contient l'erreur). C'est le retour standard des opérations susceptibles d'échouer (ouverture de fichier, accès réseau...).
- **Pourquoi il existe** : contrairement à `panic!`, on veut parfois **survivre** à l'échec et laisser l'appelant décider. `Result` rend l'échec un **type de retour** visible, donc impossible à ignorer par erreur (il faut le traiter, ne serait-ce qu'avec `match`).
- **Quand l'utiliser** : dès que l'échec est un scénario **prévisible** (fichier absent, permission refusée...) et que l'on veut pouvoir continuer ou réagir.
- **Erreurs fréquentes** : écrire un `match` qui gère les deux cas est verbeux ; utiliser `unwrap` partout pour raccourcir fait revenir le programme à un crash sans explication.
- **À retenir** : `Result` et `Option` ne doivent pas être confondus — `Option` signifie « une valeur ou rien », `Result` signifie « une valeur ou une **erreur** (avec sa cause) ».

### 3. Les raccourcis : `unwrap` et `expect`

- **Définition** : des méthodes de `Result` qui évitent d'écrire le `match` à la main :
  - `unwrap()` : renvoie la valeur de `Ok`, sinon **panique** ;
  - `expect("message")` : pareil, mais on peut fournir un message d'erreur personnalisé, ce qui aide au débogage.
- **Pourquoi ils existent** : quand on est certain que l'opération ne peut pas échouer, ou en phase de prototypage/écriture de tests, on évite le code verbeux.
- **Quand les utiliser** : code d'exemple, prototype, tests — ou quand on a plus d'informations que le compilateur (par exemple une logique qui garantit la réussite).
- **Erreurs fréquentes** : utiliser `unwrap` en production sur une opération qui peut légitimement échouer (lire un fichier utilisateur...) ; le programme crash alors sans message clair.
- **À retenir** : `expect` est presque toujours à préférer à `unwrap` pour obtenir un message explicite.

### 4. La propagation des erreurs et l'opérateur `?`

- **Définition** : **propager** une erreur, c'est la faire **remonter** au code appelant plutôt que la gérer soi-même. On peut le faire à la main avec un `match`, ou avec l'opérateur `?`.
- **Pourquoi il existe** : une fonction qui ne sait pas comment réagir à une erreur doit la transmettre à son appelant, qui a plus de contexte pour décider. Sans ça, chaque couche devrait tout gérer.
- **Fonctionnement de `?`** : placé après une valeur de type `Result`, il fait :
  - si c'est `Ok(v)` → la fonction continue avec `v` ;
  - si c'est `Err(e)` → la fonction **retourne immédiatement** `Err(e)` (en convertissant `e` grâce au trait `From` si le type d'erreur diffère).
- **Contrainte** : `?` ne peut être utilisé que dans une fonction qui retourne un `Result` (ou une `Option`). Même `main` peut retourner `Result<(), Box<dyn Error>>` — `Box<dyn Error>` désigne « n'importe quel type d'erreur » et évite d'écrire le type précis.
- **Erreurs fréquentes** : utiliser `?` dans une fonction qui ne renvoie pas de `Result` (le compilateur refuse) ; ou retourner des types d'erreur différents dans plusieurs `?` sans conversion compatible.
- **À retenir** : `?` remplace le motif classique « `match` + return précoce » et rend le code beaucoup plus lisible.

### 5. Choisir entre `panic!` et `Result`

- **Définition** : le chapitre propose des **règles de décision** : quand l'échec doit-il être récupérable ?
- **Pourquoi** : `panic!` convient quand l'échec trahit un bug ou un état impossible ; `Result` quand l'échec est un cas d'usage normal. Confondre les deux produit un code fragile ou verbeux.
- **Quand `panic!` est acceptable** :
  - dans les **exemples**, les **prototypes** et les **tests** (`unwrap`/`expect`) ;
  - quand un **invariant** est garanti par la logique, et que le compilateur ne peut pas le prouver (exemple : indexer un vecteur non vide après vérification) ;
  - quand le programme se trouve dans un **mauvais état** pire que l'erreur elle-même (données incohérentes, invariants violés, comportement non fiable).
- **Quand utiliser `Result`** : quand l'échec est un résultat **prévisible** que l'appelant doit pouvoir gérer.
- **Erreurs fréquentes** : paniquer sur une erreur utilisateur banale (fichier inexistant) ou, à l'inverse, renvoyer `Result` pour un bug interne que personne ne peut réparer.
- **À retenir** : garder le code sûr de bout en bout — si un appelant a vérifié un invariant, il peut utiliser `unwrap` avec raison, mais le code « public » (API, entrées) doit valider proprement.

### 6. Créer des types personnalisés de validation

- **Définition** : pour garantir des invariants, on encapsule une donnée dans son **propre type** dont le **constructeur** fait la validation. L'exemple du chapitre est un type `Guess` qui garantit que sa valeur est comprise entre 1 et 100.
- **Pourquoi il existe** : si une valeur doit respecter une contrainte partout où elle est utilisée, il vaut mieux la valider **une seule fois à la création** plutôt que de répéter les vérifications. Le type rend l'invariant visible et vérifiable par le compilateur.
- **Quand l'utiliser** : dès qu'une contrainte métier doit être respectée en permanence (bornes, format, non-vide...).
- **Fonctionnement** : le champ est **privé** (`value: i32`) pour empêcher toute construction directe ; une fonction associée `new` valide l'entrée (avec `panic!` ou `Result` selon le cas) ; une méthode publique `value()` expose la valeur en lecture seule.
- **Erreurs fréquentes** : laisser le champ public, ce qui permet de créer une valeur invalide en contournant la validation.
- **À retenir** : rendre les invariants **exprimables dans le type** (type-safe) est un grand bénéfice de Rust — c'est le compilateur qui fait respecter les règles.

## Nouveaux mots-clés

| Mot-clé / type | Utilisation |
|---|---|
| `panic!` | Macro : arrêt immédiat du programme avec message |
| `Result<T, E>` | Enum de retour d'une opération qui peut échouer |
| `Ok(T)` / `Err(E)` | Variantes de `Result` |
| `unwrap()` | Renvoyer la valeur ou paniquer (message par défaut) |
| `expect("msg")` | Comme `unwrap`, avec message personnalisé |
| `?` | Opérateur : propage l'erreur ou extrait la valeur |
| `Box<dyn Error>` | « N'importe quelle erreur » (type d'erreur abstrait) |
| `From` | Trait utilisé par `?` pour convertir les types d'erreur |
| `RUST_BACKTRACE` | Variable d'environnement pour afficher le backtrace |
| `panic = "abort"` | Réglage `Cargo.toml` : abandonner au lieu de dérouler la pile |

## Fonctions, méthodes et macros importantes

| Fonction / Méthode | Description |
|---|---|
| `panic!("...")` | Déclencher une erreur irrécupérable avec un message |
| `File::open(chemin)` | Ouvrir un fichier ; renvoie `Result<File, io::Error>` |
| `result.unwrap()` | Extraire la valeur `Ok` ou paniquer |
| `result.expect("msg")` | Extraire la valeur `Ok` ou paniquer avec message |
| `result.unwrap_or_else(fn)` | Renvoyer la valeur `Ok`, sinon exécuter une fonction de repli sur l'erreur |
| `result?` | Propager `Err` en remontant, sinon poursuivre avec `Ok` |
| `.read_to_string(...)` | Lire le contenu d'un fichier dans une chaîne (peut échouer) |

## Schéma mental

```
            UNE ERREUR SURVIENT
                     │
        ┌────────────┴────────────┐
        │                         │
   IRRECUPÉRABLE             RÉCUPÉRABLE
   (arrêter)                (continuer / décider)
        │                         │
   panic!("...")            Result<T, E>
   (implicite ou                │
    explicite)         ┌────────┴────────┐
   RUST_BACKTRACE=1    Ok(T)          Err(E)
        │                 │              │
        │            je continue   je propage ?
        │                 │              │
        │                 │       ┌──────┴──────┐
        │                 │       │             │
        │                 │    match (retour   │
        │                 │     précoce)       │
        │                 │       │             │
        │                 │    résumé par  ?   │
        │                 │  (convertit via From)
        │                 │
        │          unwrap / expect (si sûr)
        │          type personnalisé (invariants)
```

Le fil conducteur : **le type de retour encode l'échec**. Une fonction qui peut échouer renvoie `Result<T, E>` ; celui qui l'appelle peut soit gérer le cas `Err` (via `match`), soit le **propager** (`?`), soit décider que l'échec est impossible et utiliser `unwrap`/`expect`. `panic!` reste réservé aux cas où l'on a décidé que l'échec signale un bug ou un état impossible. Enfin, les **types personnalisés** transforment les contraintes métier en invariants que le compilateur fait respecter.

## Pièges classiques

1. **`unwrap` en production** — sur une entrée utilisateur ou un fichier, l'échec est prévisible ; `unwrap` fait crasher sans message utile.
2. **Ignorer le retour d'un `Result`** — ne pas traiter un `Result` produit un avertissement ; le compilateur rappelle qu'un échec possible doit être traité.
3. **Utiliser `?` dans une fonction qui ne renvoie pas `Result`** — le compilateur refuse ; `main` doit donc aussi retourner `Result` pour utiliser `?`.
4. **Confondre `Option` et `Result`** — l'un dit « rien », l'autre dit « une erreur » ; les deux ont leur utilité et leurs méthodes propres.
5. **Ne pas consulter le backtrace** — quand un `panic!` survient, le message seul est souvent insuffisant ; lancer avec `RUST_BACKTRACE=1` révèle l'origine.
6. **Rendre public un champ validé** — un champ privé + constructeur garantit l'invariant ; un champ public le contourne.
7. **Paniquer sur des erreurs banales** — un fichier absent ou une permission refusée sont des cas normaux ; ils méritent un `Result`, pas un `panic!`.

## Résumé

1. **Deux familles d'erreurs** : irrécupérables (`panic!`) et récupérables (`Result<T, E>`).
2. **`panic!`** arrête le programme et déroule la pile ; `RUST_BACKTRACE=1` aide à en trouver la cause.
3. **`Result<T, E>`** est une enum `Ok(T)` / `Err(E)` : l'échec devient un type de retour impossible à ignorer.
4. **`unwrap`** extrait la valeur ou panique ; **`expect`** fait pareil avec un message explicite.
5. **`?`** propage l'erreur vers l'appelant : `Err` → retour anticipé, `Ok` → la valeur continue le code. Il convertit le type d'erreur via le trait `From`.
6. `main` peut lui-même retourner `Result<(), Box<dyn Error>>`.
7. **Quand paniquer** : exemples, prototypes, tests, invariants garantis, mauvais état. **Quand `Result`** : échec prévisible que l'appelant doit gérer.
8. **Types personnalisés de validation** (ex. `Guess`) : champ privé + constructeur = invariants respectés par le compilateur.
