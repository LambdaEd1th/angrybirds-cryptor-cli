use crate::cli::{FileType, GameName};

/// The default Initialization Vector (IV) used by legacy Angry Birds games.
/// It is a block of 16 zero bytes.
pub const DEFAULT_IV: [u8; 16] = [0u8; 16];

/// Suffix appended to encrypted files when no output path is specified.
pub const SUFFIX_ENCRYPTED: &str = "_encrypted";

/// Suffix appended to decrypted files when no output path is specified.
pub const SUFFIX_DECRYPTED: &str = "_decrypted";

/// Retrieves the static AES-256 key for a specific game and file type combination.
/// Returns None if the combination is not supported.
pub const fn get_key(file_type: FileType, game_name: GameName) -> Option<&'static [u8; 32]> {
    match (file_type, game_name) {
        // --- Native Files ---
        (FileType::Native, GameName::Classic) => Some(b"USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2"),
        (FileType::Native, GameName::Rio) => Some(b"USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2"),
        (FileType::Native, GameName::Seasons) => Some(b"zePhest5faQuX2S2Apre@4reChAtEvUt"),
        (FileType::Native, GameName::Space) => Some(b"RmgdZ0JenLFgWwkYvCL2lSahFbEhFec4"),
        (FileType::Native, GameName::Friends) => Some(b"EJRbcWh81YG4YzjfLAPMssAnnzxQaDn1"),
        (FileType::Native, GameName::Starwars) => Some(b"An8t3mn8U6spiQ0zHHr3a1loDrRa3mtE"),
        (FileType::Native, GameName::Starwarsii) => Some(b"B0pm3TAlzkN9ghzoe2NizEllPdN0hQni"),
        (FileType::Native, GameName::Stella) => Some(b"4FzZOae60yAmxTClzdgfcr4BAbPIgj7X"),

        // --- Save Files ---
        (FileType::Save, GameName::Classic) => Some(b"44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"),
        (FileType::Save, GameName::Rio) => Some(b"44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"),
        (FileType::Save, GameName::Seasons) => Some(b"brU4u=EbR4s_A3APu6U#7B!axAm*We#5"),
        (FileType::Save, GameName::Space) => Some(b"TpeczKQL07HVdPbVUhAr6FjUsmRctyc5"),
        (FileType::Save, GameName::Friends) => Some(b"XN3OCmUFL6kINHuca2ZQL4gqJg0r18ol"),
        (FileType::Save, GameName::Starwars) => Some(b"e83Tph0R3aZ2jGK6eS91uLvQpL33vzNi"),
        (FileType::Save, GameName::Starwarsii) => Some(b"taT3vigDoNlqd44yiPbt21biCpVma6nb"),
        (FileType::Save, GameName::Stella) => Some(b"Bll3qkcy5fKrNVxZqtkFH19Ojn2sdJFu"),

        // --- Downloaded Files ---
        (FileType::Downloaded, GameName::Friends) => Some(b"rF1pFq2wDzgR7PQ94dTFuXww0YvY7nfK"),

        // Unsupported combinations
        _ => None,
    }
}
