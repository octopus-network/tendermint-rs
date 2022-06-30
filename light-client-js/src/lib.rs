//! Tendermint Light Client JavaScript/WASM interface.
//!
//! This crate exposes some of the [`tendermint-light-client-verifier`] crate's
//! functionality to be used from the JavaScript ecosystem.
//!
//! For a detailed example, please see the [`verifier-web` example] in the
//! repository.
//!
//! [`tendermint-light-client-verifier`]: https://github.com/informalsystems/tendermint-rs/tree/master/light-client-verifier
//! [`verifier-web` example]: https://github.com/informalsystems/tendermint-rs/tree/master/light-client-js/examples/verifier-web

mod utils;

use serde::{Deserialize, Serialize};
use std::time::Duration;
use tendermint::Time;
use tendermint_light_client_verifier::host_functions::helper::TestHostFunctions;
use tendermint_light_client_verifier::host_functions::HostFunctionsProvider;
use tendermint_light_client_verifier::options::Options;
use tendermint_light_client_verifier::types::{LightBlock, TrustThreshold};
use tendermint_light_client_verifier::{ProdVerifier, Verifier};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc<'_> = wee_alloc::WeeAlloc::INIT;

/// Check whether a given untrusted block can be trusted.
#[wasm_bindgen]
pub fn verify(untrusted: &JsValue, trusted: &JsValue, options: &JsValue, now: &JsValue) -> JsValue {
    let result = deserialize_params(untrusted, trusted, options, now).map(
        |(untrusted, trusted, options, now)| {
            let verifier = ProdVerifier::<TestHostFunctions>::default();
            verifier.verify(
                untrusted.as_untrusted_state(),
                trusted.as_trusted_state(),
                &options,
                now,
            )
        },
    );
    JsValue::from_serde(&result).unwrap()
}

fn deserialize_params(
    untrusted: &JsValue,
    trusted: &JsValue,
    options: &JsValue,
    now: &JsValue,
) -> Result<(LightBlock, LightBlock, Options, Time), Error> {
    let untrusted = untrusted.into_serde().map_err(|e| Error::Serialization {
        param: "untrusted".to_string(),
        msg: e.to_string(),
    })?;

    let trusted = trusted.into_serde().map_err(|e| Error::Serialization {
        param: "trusted".to_string(),
        msg: e.to_string(),
    })?;

    let options = options
        .into_serde::<JsOptions>()
        .map(Into::into)
        .map_err(|e| Error::Serialization {
            param: "options".to_string(),
            msg: e.to_string(),
        })?;

    let now = now.into_serde().map_err(|e| Error::Serialization {
        param: "now".to_string(),
        msg: e.to_string(),
    })?;

    Ok((untrusted, trusted, options, now))
}

/// Errors produced by this crate.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Error {
    /// A serialization/deserialization error occurred.
    #[serde(rename = "serialization")]
    Serialization { param: String, msg: String },
}

// Simplified options supplied from JavaScript.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsOptions {
    pub trust_threshold: (u64, u64),
    pub trusting_period: u64,
    pub clock_drift: u64,
}

impl From<JsOptions> for Options {
    fn from(o: JsOptions) -> Self {
        let (num, den) = o.trust_threshold;
        Self {
            trust_threshold: TrustThreshold::new(num, den).unwrap(),
            trusting_period: Duration::from_secs(o.trusting_period),
            clock_drift: Duration::from_secs(o.clock_drift),
        }
    }
}
