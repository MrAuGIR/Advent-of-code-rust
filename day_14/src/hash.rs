use memoize::memoize;


// Fonction pour convertir la carte en chaîne
#[memoize]
pub fn map_to_string(map: Vec<Vec<char>>) -> String {
    let mut map_string = String::new();
    for line in map {
        map_string.push_str(&line.iter().collect::<String>());
    }
    map_string
}