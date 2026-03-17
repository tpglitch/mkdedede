use mkdedede::double_dash::{Character, Course, GhostData, Kart, LapTime, decode, encode};

fn main() {
    println!("--- Mario Kart: Double Dash!! Password Example ---");

    // 1. Create up some data
    let original_data = GhostData {
        course: Course::MushroomBridge,
        kart: Kart::RedFire,
        driver1: Character::Mario,
        driver2: Character::Luigi,
        total_time: LapTime(123_456), // ms
        best_lap: LapTime(40_000),    // ms
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
