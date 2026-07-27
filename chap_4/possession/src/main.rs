fn main() {
    // === COPY ===
    // Types 100% stack : Rust les duplique sans réfléchir
    let x = 42;
    let y = x;          // x est COPIÉ (4 octets sur la stack)
    println!("{x} {y}"); // les deux vivent — normal, deux blocs distincts

    let a = true;
    let b = a;          // copié (1 octet)
    println!("{a} {b}");

    // === MOVE ===
    // Types avec heap : Rust TRANSFÈRE la propriété
    let s1 = String::from("hello");
    let s2 = s1;        // s1 est DÉPLACÉ dans s2
    // println!("{s1}"); // REFUSÉ : s1 n'est plus propriétaire
    println!("{s2}");

    // === Pourquoi cette différence ? ===
    // Copier un i32 = 4 octets sur la stack (presque gratuit)
    // Copier une String = copier toute la heap (malloc + memcpy)
    // Rust refuse les copies cachées coûteuses

    // === Mais on PEUT copier la heap si on paie ===
    let s3 = String::from("hello");
    let s4 = s3.clone(); // copie explicite toute la heap
    println!("{s3} {s4}"); // les deux vivent
}
