use memoize::memoize;


// Fonction pour convertir la carte en chaîne
#[memoize]
pub fn map_to_string(map: Vec<char>) -> String {
    let mut map_string = String::new();
    
    map_string.push_str(&map.iter().collect::<String>());
    
    map_string
}