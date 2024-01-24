

// Fonction pour convertir la carte en chaîne
pub fn map_to_string(map: &mut Vec<char>) -> String {
    let mut map_string = String::new();
    
    map_string.push_str(&map.iter().collect::<String>());
    
    map_string
}