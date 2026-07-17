use crate::error::Error;
use crate::result::Result;
use sydar_consensus_core::constants::KANA_PER_sydar;
use std::fmt::Display;

pub fn try_parse_required_nonzero_sydar_as_kana_u64<S: ToString + Display>(sydar_amount: Option<S>) -> Result<u64> {
    if let Some(sydar_amount) = sydar_amount {
        let kana_amount = sydar_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied sydar amount is not valid: '{sydar_amount}'")))?
            * KANA_PER_sydar as f64;
        if kana_amount < 0.0 {
            Err(Error::custom("Supplied sydar amount is not valid: '{sydar_amount}'"))
        } else {
            let kana_amount = kana_amount as u64;
            if kana_amount == 0 {
                Err(Error::custom("Supplied required sydar amount must not be a zero: '{sydar_amount}'"))
            } else {
                Ok(kana_amount)
            }
        }
    } else {
        Err(Error::custom("Missing sydar amount"))
    }
}

pub fn try_parse_required_sydar_as_kana_u64<S: ToString + Display>(sydar_amount: Option<S>) -> Result<u64> {
    if let Some(sydar_amount) = sydar_amount {
        let kana_amount = sydar_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_| Error::custom(format!("Supplied Kasapa amount is not valid: '{sydar_amount}'")))?
            * KANA_PER_sydar as f64;
        if kana_amount < 0.0 {
            Err(Error::custom("Supplied sydar amount is not valid: '{sydar_amount}'"))
        } else {
            Ok(kana_amount as u64)
        }
    } else {
        Err(Error::custom("Missing sydar amount"))
    }
}

pub fn try_parse_optional_sydar_as_kana_i64<S: ToString + Display>(sydar_amount: Option<S>) -> Result<Option<i64>> {
    if let Some(sydar_amount) = sydar_amount {
        let kana_amount = sydar_amount
            .to_string()
            .parse::<f64>()
            .map_err(|_e| Error::custom(format!("Supplied Kasapa amount is not valid: '{sydar_amount}'")))?
            * KANA_PER_sydar as f64;
        if kana_amount < 0.0 {
            Err(Error::custom("Supplied sydar amount is not valid: '{sydar_amount}'"))
        } else {
            Ok(Some(kana_amount as i64))
        }
    } else {
        Ok(None)
    }
}
