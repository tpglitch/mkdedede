mod dd {
    use mkdedede::double_dash::*;

    fn sample() -> GhostData {
        GhostData {
            course: Course::MushroomBridge,
            kart: Kart::RedFire,
            driver1: Character::Mario,
            driver2: Character::Luigi,
            total_time: LapTime(123_456),
            best_lap: LapTime(40_000),
        }
    }

    #[test]
    fn encode_length_and_alphabet() {
        let pw = encode(&sample());
        assert_eq!(pw.len(), 16);
        let alphabet = b"G6EQTXYN4WRHBFKOIJAPCD5S8V7UZ3LM";
        assert!(pw.bytes().all(|b| alphabet.contains(&b)));
    }

    #[test]
    fn encode_decode_roundtrip() {
        let original = sample();
        let pw = encode(&original);
        let decoded = decode(&pw).expect("encoded password should decode successfully");
        assert_eq!(decoded, original);
    }

    #[test]
    fn roundtrip_all_courses() {
        for course in [
            Course::LuigiCircuit,
            Course::PeachBeach,
            Course::BabyPark,
            Course::DryDryDesert,
            Course::MushroomBridge,
            Course::MarioCircuit,
            Course::DaisyCruiser,
            Course::WaluigiStadium,
            Course::SherbetLand,
            Course::MushroomCity,
            Course::YoshiCircuit,
            Course::DkMountain,
            Course::WarioColosseum,
            Course::DinoDinoJungle,
            Course::BowsersCastle,
            Course::RainbowRoad,
        ] {
            let data = GhostData {
                course,
                kart: Kart::GreenFire,
                driver1: Character::Toad,
                driver2: Character::Peach,
                // 7 * 29_000 = 203_000 so total must cover Baby Park's 7 laps
                total_time: LapTime(210_000),
                best_lap: LapTime(29_000),
            };
            let pw = encode(&data);
            let decoded = decode(&pw).expect("should decode");
            assert_eq!(decoded.course, course, "course mismatch for {course}");
        }
    }

    #[test]
    fn roundtrip_all_karts() {
        for kart in [
            Kart::RedFire,
            Kart::DkJumbo,
            Kart::TurboYoshi,
            Kart::KoopaDasher,
            Kart::HeartCoach,
            Kart::GooGooBuggy,
            Kart::WarioCar,
            Kart::KoopaKing,
            Kart::GreenFire,
            Kart::BarrelTrain,
            Kart::TurboBirdo,
            Kart::ParaWing,
            Kart::BloomCoach,
            Kart::RattleBuggy,
            Kart::WaluigiRacer,
            Kart::BulletBlaster,
            Kart::ToadKart,
            Kart::ToadetteKart,
            Kart::BooPipes,
            Kart::PiranhaPipes,
            Kart::ParadeKart,
        ] {
            let data = GhostData {
                course: Course::MarioCircuit,
                kart,
                driver1: Character::Bowser,
                driver2: Character::Wario,
                total_time: LapTime(80_000),
                best_lap: LapTime(26_000),
            };
            let pw = encode(&data);
            let decoded = decode(&pw).expect("should decode");
            assert_eq!(decoded.kart, kart, "kart mismatch for {kart}");
        }
    }

    #[test]
    fn roundtrip_baby_park_laps() {
        // Baby Park has 7 laps; best_lap * 7 must not exceed total_time
        let data = GhostData {
            course: Course::BabyPark,
            kart: Kart::GooGooBuggy,
            driver1: Character::BabyMario,
            driver2: Character::BabyLuigi,
            total_time: LapTime(140_000),
            best_lap: LapTime(20_000),
        };
        let pw = encode(&data);
        let decoded = decode(&pw).expect("should decode");
        assert_eq!(decoded, data);
    }

    #[test]
    fn roundtrip_wario_colosseum_laps() {
        // Wario Colosseum has 2 laps; best_lap * 2 must not exceed total_time
        let data = GhostData {
            course: Course::WarioColosseum,
            kart: Kart::WarioCar,
            driver1: Character::Wario,
            driver2: Character::Waluigi,
            total_time: LapTime(180_000),
            best_lap: LapTime(89_000),
        };
        let pw = encode(&data);
        let decoded = decode(&pw).expect("should decode");
        assert_eq!(decoded, data);
    }

    #[test]
    fn roundtrip_time_zero() {
        let data = GhostData {
            course: Course::LuigiCircuit,
            kart: Kart::RedFire,
            driver1: Character::Mario,
            driver2: Character::Peach,
            total_time: LapTime(0),
            best_lap: LapTime(0),
        };
        let pw = encode(&data);
        let decoded = decode(&pw).expect("should decode");
        assert_eq!(decoded, data);
    }
}

mod ds {
    use mkdedede::ds::*;

    fn sample() -> GhostData {
        GhostData {
            course: Course::WaluigiPinball,
            character: Character::Waluigi,
            kart: Kart::Zipper,
            time: RaceTime(96_720),
            player: ['\0', '\0'],
        }
    }

    #[test]
    fn encode_length_and_alphabet() {
        let pw = encode(&sample());
        assert_eq!(pw.len(), 16);
        let alphabet = b"S7LCX3JZE8FG4HBKWN52YPA6RTU9VMDQ";
        assert!(pw.bytes().all(|b| alphabet.contains(&b)));
    }

    #[test]
    fn encode_decode_roundtrip() {
        let original = sample();
        let pw = encode(&original);
        let decoded = decode(&pw).expect("encoded password should decode successfully");
        assert_eq!(decoded, original);
    }

    #[test]
    fn roundtrip_all_courses() {
        for course in [
            Course::Figure8Circuit,
            Course::YoshiFalls,
            Course::CheepCheepBeach,
            Course::LuigisMansion,
            Course::DesertHills,
            Course::DelfinoSquare,
            Course::WaluigiPinball,
            Course::ShroomRidge,
            Course::DkPass,
            Course::TickTockClock,
            Course::MarioCircuit,
            Course::AirshipFortress,
            Course::WarioStadium,
            Course::PeachGardens,
            Course::BowserCastle,
            Course::RainbowRoad,
            Course::SnesMarioCircuit1,
            Course::N64MooMooFarm,
            Course::GbaPeachCircuit,
            Course::GcnLuigiCircuit,
            Course::SnesDonutPlains1,
            Course::N64FrappeSnowland,
            Course::GbaBowserCastle2,
            Course::GcnBabyPark,
            Course::SnesKoopaBeach2,
            Course::N64ChocoMountain,
            Course::GbaLuigiCircuit,
            Course::GcnMushroomBridge,
            Course::SnesChocoIsland2,
            Course::N64BansheeBoardwalk,
            Course::GbaSkyGarden,
            Course::GcnYoshiCircuit,
        ] {
            let data = GhostData {
                course,
                character: Character::Mario,
                kart: Kart::StandardMr,
                time: RaceTime(90_000),
                player: ['\0', '\0'],
            };
            let pw = encode(&data);
            let decoded = decode(&pw).expect("should decode");
            assert_eq!(decoded.course, course, "course mismatch for {course}");
        }
    }

    #[test]
    fn roundtrip_all_characters() {
        for character in [
            Character::Mario,
            Character::DonkeyKong,
            Character::Toad,
            Character::Bowser,
            Character::Peach,
            Character::Wario,
            Character::Yoshi,
            Character::Luigi,
            Character::DryBones,
            Character::Daisy,
            Character::Waluigi,
            Character::Rob,
            Character::ShyGuy,
        ] {
            let data = GhostData {
                course: Course::MarioCircuit,
                character,
                kart: Kart::BDasher,
                time: RaceTime(75_000),
                player: ['\0', '\0'],
            };
            let pw = encode(&data);
            let decoded = decode(&pw).expect("should decode");
            assert_eq!(
                decoded.character, character,
                "character mismatch for {character}"
            );
        }
    }

    #[test]
    fn roundtrip_all_karts() {
        for kart in [
            Kart::StandardMr,
            Kart::ShootingStar,
            Kart::BDasher,
            Kart::StandardDk,
            Kart::Wildlife,
            Kart::RambiRider,
            Kart::StandardTd,
            Kart::Mushmellow,
            Kart::FourWheelCradle,
            Kart::StandardBw,
            Kart::Hurricane,
            Kart::Tyrant,
            Kart::StandardPc,
            Kart::LightTripper,
            Kart::Royale,
            Kart::StandardWr,
            Kart::Brute,
            Kart::Dragonfly,
            Kart::StandardYs,
            Kart::Egg1,
            Kart::Cucumber,
            Kart::StandardLg,
            Kart::Poltergust4000,
            Kart::Streamliner,
            Kart::StandardDb,
            Kart::DryBomber,
            Kart::Banisher,
            Kart::StandardDs,
            Kart::LightDancer,
            Kart::PowerFlower,
            Kart::StandardWl,
            Kart::GoldMantis,
            Kart::Zipper,
            Kart::StandardRb,
            Kart::RobBls,
            Kart::RobLgs,
            Kart::StandardSg,
        ] {
            let data = GhostData {
                course: Course::RainbowRoad,
                character: Character::Rob,
                kart,
                time: RaceTime(120_000),
                player: ['\0', '\0'],
            };
            let pw = encode(&data);
            let decoded = decode(&pw).expect("should decode");
            assert_eq!(decoded.kart, kart, "kart mismatch for {kart}");
        }
    }

    #[test]
    fn roundtrip_with_player_name() {
        let data = GhostData {
            course: Course::Figure8Circuit,
            character: Character::Mario,
            kart: Kart::StandardMr,
            time: RaceTime(60_000),
            player: ['A', 'B'],
        };
        let pw = encode(&data);
        let decoded = decode(&pw).expect("should decode");
        assert_eq!(decoded, data);
    }

    #[test]
    fn roundtrip_time_zero() {
        let data = GhostData {
            course: Course::Figure8Circuit,
            character: Character::Toad,
            kart: Kart::Mushmellow,
            time: RaceTime(0),
            player: ['\0', '\0'],
        };
        let pw = encode(&data);
        let decoded = decode(&pw).expect("should decode");
        assert_eq!(decoded, data);
    }

    #[test]
    fn encode_is_uppercase_ascii() {
        let pw = encode(&sample());
        assert!(pw.is_ascii());
        assert_eq!(pw, pw.to_ascii_uppercase());
    }
}
