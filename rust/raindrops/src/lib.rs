use std::collections::HashMap;

pub fn raindrops(n: u32) -> String {
    let mut tunes = HashMap::new();
    let mut melody = vec![];

    tunes.insert(3, "Pling".to_string());
    tunes.insert(5, "Plang".to_string());
    tunes.insert(7, "Plong".to_string());
    
    for (num, sound) in &tunes {
        if n % num == 0 {
            melody.push(sound.clone());
        } else { 
            continue;
        };
    };

    if melody.is_empty() {
        return n.to_string()
    } else {return melody.join("")}

    
}
