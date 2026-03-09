mod dd {
    use mkdedede::double_dash::*;

    #[test]
    fn wrong_length_is_rejected() {
        assert_eq!(decode("TOOSHORT"), Err(DecodeError::WrongLength));
    }

    #[test]
    fn bad_checksum_is_rejected() {
        assert_eq!(
            decode("GGGGGGGGGGGGGGGG"),
            Err(DecodeError::InvalidChecksum)
        );
    }

    #[test]
    fn laptime_display() {
        assert_eq!(format!("{}", LapTime(524_287)), "08 : 44 . 287");
    }

    #[test]
    fn course_display() {
        assert_eq!(format!("{}", Course::BowsersCastle), "Bowser's Castle");
    }

    #[test]
    fn character_display() {
        assert_eq!(format!("{}", Character::PeteyPiranha), "Petey Piranha");
    }
}

mod ds {
    use mkdedede::ds::*;

    #[test]
    fn wrong_length_is_rejected() {
        assert_eq!(decode("TOOSHORT"), Err(DecodeError::WrongLength));
    }

    #[test]
    fn bad_checksum_is_rejected() {
        // 16 valid DS alphabet chars that won't have a matching CRC
        assert_eq!(
            decode("SSSSSSSSSSSSSSSS"),
            Err(DecodeError::InvalidChecksum)
        );
    }

    #[test]
    fn racetime_display() {
        assert_eq!(format!("{}", RaceTime(90_500)), "1:30.500");
    }

    #[test]
    fn course_display() {
        assert_eq!(format!("{}", Course::WaluigiPinball), "Waluigi Pinball");
    }

    #[test]
    fn character_display() {
        assert_eq!(format!("{}", Character::Rob), "R.O.B.");
    }

    #[test]
    fn kart_display() {
        assert_eq!(format!("{}", Kart::Poltergust4000), "Poltergust 4000");
    }
}
