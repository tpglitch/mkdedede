use mkdedede::ds::{Character, Course, GhostData, Kart, RaceTime, decode, encode};

fn main() {
    println!("--- Mario Kart DS Password Example ---");

    // 1. Create up some data
    let original_data = GhostData {
        course: Course::WaluigiPinball,
        character: Character::Waluigi,
        kart: Kart::Zipper,
        time: RaceTime(96_720), // ms
        player: ['A', 'B'],     // 2-character UTF-16 limit name
    };

    println!("Original Data:\n{}\n", original_data);

    // 2. Encode to a 16-character password
    let password = encode(&original_data);
    println!("Encoded Password: {}\n", password);

    // 3. Decode it back
    match decode(&password) {
        Ok(ghost) => {
            println!("Successfully decoded back! Data matches:");
            println!("{}", ghost);
        }
        Err(e) => eprintln!("Failed to decode: {}", e),
    }
}
