/// List of known Dex mapping for convenience
pub enum Dex {
    Hyperliquid,
    Hyena,
    Xyz,
}

impl Dex {
    pub fn dex_name(&self) -> &str {
        match self {
            Dex::Hyperliquid => "",
            Dex::Hyena => "hyna",
            Dex::Xyz => "xyz",
        }
    }
}
