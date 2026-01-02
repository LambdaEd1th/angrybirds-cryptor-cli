use crate::constants::DEFAULT_IV;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    /// Structure: games -> game_name -> category -> CryptoEntry
    pub games: HashMap<String, GameConfig>,
}

pub type GameConfig = HashMap<String, CryptoEntry>;

/// Represents a configuration entry for a specific file category.
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum CryptoEntry {
    /// Case 1: Simple string, e.g., native = "HEX_STRING"
    KeyOnly(String),

    /// Case 2: Detailed object, e.g., native = { key = "HEX_STRING", iv = "HEX_STRING" }
    Detailed { key: String, iv: Option<String> },
}

impl Config {
    pub fn get_params(&self, game_name: &str, category: &str) -> Option<(Vec<u8>, [u8; 16])> {
        let game_key = game_name.to_lowercase();
        let category_key = category.to_lowercase();

        let game_config = self.games.get(&game_key)?;
        let entry = game_config.get(&category_key)?;

        let (key_str, iv_str_opt) = match entry {
            CryptoEntry::KeyOnly(k) => (k, None),
            CryptoEntry::Detailed { key, iv } => (key, iv.as_ref()),
        };

        // Use strict hex decoding.
        // If the key string is not valid hex, this returns an empty vector.
        // The empty vector will trigger a length validation error in Cryptor::new later.
        let key_bytes = decode_hex_strict(key_str);

        let iv_bytes = if let Some(iv_s) = iv_str_opt {
            let decoded = decode_hex_strict(iv_s);
            // Validate IV length immediately (must be 16 bytes for AES-256-CBC)
            if decoded.len() == 16 {
                let mut arr = [0u8; 16];
                arr.copy_from_slice(&decoded);
                arr
            } else {
                return None;
            }
        } else {
            DEFAULT_IV
        };

        Some((key_bytes, iv_bytes))
    }

    pub fn load_or_default(path: Option<&Path>) -> Result<Self> {
        let mut config = Self::default();

        if let Some(p) = path {
            if p.exists() {
                let content = fs::read_to_string(p)
                    .with_context(|| format!("Failed to read config file at {:?}", p))?;
                let user_config: Config =
                    toml::from_str(&content).context("Failed to parse TOML config file")?;

                // Merge user config into default config
                for (game, categories) in user_config.games {
                    let game_lower = game.to_lowercase();
                    let entry = config.games.entry(game_lower).or_insert_with(HashMap::new);
                    for (category, crypto_entry) in categories {
                        entry.insert(category.to_lowercase(), crypto_entry);
                    }
                }
            }
        }

        Ok(config)
    }
}

/// Helper function to decode hex strings strictly.
/// Returns an empty Vec if decoding fails, ensuring we don't accidentally interpret
/// malformed hex as raw bytes.
fn decode_hex_strict(s: &str) -> Vec<u8> {
    hex::decode(s).unwrap_or_default()
}

impl Default for Config {
    fn default() -> Self {
        let mut games = HashMap::new();

        macro_rules! add_game {
            ($name:expr, $( $category:expr => $key:expr ),* ) => {
                let mut categories = HashMap::new();
                $(
                    categories.insert(
                        $category.to_string(),
                        // Automatically encode the hardcoded ASCII keys into Hex strings.
                        // This ensures that the Config struct always holds Hex strings,
                        // consistent with the new strict decoding logic.
                        CryptoEntry::KeyOnly(hex::encode($key))
                    );
                )*
                games.insert($name.to_string(), categories);
            };
        }

        // Define default keys for known games.
        // These ASCII keys will be converted to Hex by the macro above.
        add_game!("classic",
            "native" => "USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2",
            "save"   => "44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"
        );
        add_game!("rio",
            "native" => "USCaPQpA4TSNVxMI1v9SK9UC0yZuAnb2",
            "save"   => "44iUY5aTrlaYoet9lapRlaK1Ehlec5i0"
        );
        add_game!("seasons",
            "native" => "zePhest5faQuX2S2Apre@4reChAtEvUt",
            "save"   => "brU4u=EbR4s_A3APu6U#7B!axAm*We#5"
        );
        add_game!("space",
            "native" => "RmgdZ0JenLFgWwkYvCL2lSahFbEhFec4",
            "save"   => "TpeczKQL07HVdPbVUhAr6FjUsmRctyc5"
        );
        add_game!("friends",
            "native"     => "EJRbcWh81YG4YzjfLAPMssAnnzxQaDn1",
            "save"       => "XN3OCmUFL6kINHuca2ZQL4gqJg0r18ol",
            "downloaded" => "rF1pFq2wDzgR7PQ94dTFuXww0YvY7nfK"
        );
        add_game!("starwars",
            "native" => "An8t3mn8U6spiQ0zHHr3a1loDrRa3mtE",
            "save"   => "e83Tph0R3aZ2jGK6eS91uLvQpL33vzNi"
        );
        add_game!("starwarsii",
            "native" => "B0pm3TAlzkN9ghzoe2NizEllPdN0hQni",
            "save"   => "taT3vigDoNlqd44yiPbt21biCpVma6nb"
        );
        add_game!("stella",
            "native" => "4FzZOae60yAmxTClzdgfcr4BAbPIgj7X",
            "save"   => "Bll3qkcy5fKrNVxZqtkFH19Ojn2sdJFu"
        );

        Self { games }
    }
}
