pub fn get_interest_emoji(interest: &str) -> String {
    let emoji = match interest {
        "France" => Some("🇫🇷"),
        "Old Meme" => Some("💾"),
        "Anime & Manga" => Some("🍥"),
        "Latest News" => Some("📰"),
        "Cosplay" => Some("🎭"),
        "Politics" => Some("🏛️"),
        "Humor" => Some("😂"),
        "Memes" => Some("💎"),
        "Gaming" => Some("🎮"),
        "WTF" => Some("🤯"),
        "Relationship & Dating" => Some("💗"),
        "Music" => Some("🎵"),
        "Motor Vehicles" => Some("🏍️"),
        "Animals & Pets" => Some("🐾"),
        "Science & Tech" => Some("🚀"),
        "Comic" => Some("🗯️"),
        "Wholesome" => Some("😍"),
        "Sports" => Some("⚽"),
        "Movies & TV" => Some("🍿"),
        "Cat" => Some("🐱"),
        "Food & Drinks" => Some("🍔"),
        "Lifestyle" => Some("☕"),
        "Superhero" => Some("🦸"),
        "Crypto" => Some("🪙"),
        "Random" => Some("🎲"),
        "Waoh" => Some("✨"),
        _ => None,
    };
    emoji
        .map(|emoji| format!("{emoji} "))
        .unwrap_or_else(|| "".into())
}
