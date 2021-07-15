pub fn raindrops(n: u32) -> String {
    let factors = [3, 5, 7];
    let tunes = ["Pling", "Plang", "Plong"];
    let mut melody = vec![];

    for (i, num) in factors.iter().enumerate() {
        if n % num == 0 {
            melody.push(tunes[i]);
        } else {
            continue;
        };
    }

    if melody.is_empty() {
        n.to_string()
    } else {
        melody.join("")
    }
}
