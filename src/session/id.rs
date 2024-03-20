use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::{DecodeError, Engine};
use std::fmt::Display;
use std::str::FromStr;
use std::{fmt, str};

use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct Id(i128);

impl Default for Id {
    fn default() -> Self {
        Self(random())
    }
}

impl AsRef<i128> for Id {
    fn as_ref(&self) -> &i128 {
        &self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut encoded = [0; 22];
        BASE64_URL_SAFE_NO_PAD
            .encode_slice(self.0.to_le_bytes(), &mut encoded)
            .expect("Encoded ID must be exactly 22 bytes");
        let encoded = str::from_utf8(&encoded).expect("Encoded ID must be valid UTF-8");

        f.write_str(encoded)
    }
}

impl FromStr for Id {
    type Err = base64::DecodeSliceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut decoded = [0; 16];
        let bytes_decoded = URL_SAFE_NO_PAD.decode_slice(s.as_bytes(), &mut decoded)?;
        if bytes_decoded != 16 {
            let err = DecodeError::InvalidLength(bytes_decoded);
            return Err(base64::DecodeSliceError::DecodeError(err));
        }

        Ok(Self(i128::from_le_bytes(decoded)))
    }
}
