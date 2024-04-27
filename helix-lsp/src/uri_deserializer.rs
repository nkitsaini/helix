use lsp_types::Url;
use std::fmt;

use serde::de::{self, Visitor};

pub struct LspUriVisitor;

impl<'de> Visitor<'de> for LspUriVisitor {
    type Value = lsp_types::Url;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an uri")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        return lsp_types::Url::parse(&std::borrow::Cow::into_owned(
            percent_encoding::percent_decode_str(v).decode_utf8_lossy(),
        ))
        .map_err(|e| E::custom(e));
    }

    // Similar for other methods:
    //   - visit_i16
    //   - visit_u8
    //   - visit_u16
    //   - visit_u32
    //   - visit_u64
}

// Stopgap until: https://github.com/gluon-lang/lsp-types/issues/276
pub fn to_percent_decode_url(url: &Url) -> Url {
    return Url::parse(&std::borrow::Cow::into_owned(
        percent_encoding::percent_decode_str(url.as_str()).decode_utf8_lossy(),
    ))
    .expect("[Custom code]: failed to re-encode percent_decoded uri");
}
