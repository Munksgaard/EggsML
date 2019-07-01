use rand::seq::SliceRandom;

static PREFIXES: [&str; 9] = [
    "Visual ",
    "Microsoft ",
    "Cloud ",
    "Objective ",
    "Turbo ",
    "Object ",
    "Standard ",
    "Common ",
    "",
];

static ROOTS: [&str; 22] = [
    "Pascal", "Haskell", "Blaise", "Curry", "Go", "Rust", "A", "B", "C", "D", "E", "F", "Erlang",
    "Prolog", "Neumann", "Euclid", "Fermat", "ML", "Scheme", "Lisp", "Church", "Alonzo",
];

static SUFFIXES: [&str; 15] = [
    "#", "++", ".NET", "*", "--", "script", " 2", " 3", " 4", " 5", " 6", " 7", " 8", " 9", "",
];

fn main() {
    // prepare a non-deterministic random number generator:
    let mut rng = rand::thread_rng();

    let mut result = String::new();

    result.push_str(PREFIXES.choose(&mut rng).unwrap_or(&""));

    result.push_str(ROOTS.choose(&mut rng).unwrap_or(&""));

    result.push_str(SUFFIXES.choose(&mut rng).unwrap_or(&""));

    println!("{}", result);
}
