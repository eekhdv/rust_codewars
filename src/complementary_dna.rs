pub fn run(dna: &str) -> String {
    dna.to_string()
        .replace('T', "$")
        .replace('A', "T")
        .replace('$', "A")
        .replace('C', "$")
        .replace('G', "C")
        .replace('$', "G")

}
