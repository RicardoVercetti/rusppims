pub enum KycTypes {
    NoKyc,
    MinKyc,
    FullKyc
}

impl KycTypes {
    pub fn from_code(code: &str) -> Result<Self, String> {
        match code {
            "N" => Ok(KycTypes::NoKyc),
            "P" => Ok(KycTypes::MinKyc),
            "F" => Ok(KycTypes::FullKyc),
            rest => Err(format!("Invalid KYC code: {}", rest))
        }
    }

    pub fn is_under_limit(&self, consumed: &f32, amount: &f32) -> bool {
        match self {
            KycTypes::NoKyc | KycTypes::MinKyc => {
                !(consumed + amount > 10_000.)
            },
            KycTypes::FullKyc => {
                !(consumed + amount > 10_000.)
            }
        }
    }
}