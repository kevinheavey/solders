use pyo3::prelude::*;

use solana_sdk::{
    pubkey::Pubkey as PubkeyOriginal,
    signature::Signature as SignatureOriginal,
    signer::{signers::Signers, Signer as SignerTrait, SignerError},
};

use crate::{Keypair, Presigner};

#[derive(FromPyObject, Debug)]
pub enum Signer {
    KeypairWrapper(Keypair),
    PresignerWrapper(Presigner),
}

impl SignerTrait for Signer {
    fn pubkey(&self) -> PubkeyOriginal {
        match self {
            Signer::KeypairWrapper(x) => x.0.pubkey(),
            Signer::PresignerWrapper(x) => x.0.pubkey(),
        }
    }
    fn try_pubkey(&self) -> Result<PubkeyOriginal, SignerError> {
        match self {
            Signer::KeypairWrapper(x) => x.0.try_pubkey(),
            Signer::PresignerWrapper(x) => x.0.try_pubkey(),
        }
    }
    fn sign_message(&self, message: &[u8]) -> SignatureOriginal {
        match self {
            Signer::KeypairWrapper(x) => x.0.sign_message(message),
            Signer::PresignerWrapper(x) => x.0.sign_message(message),
        }
    }
    fn try_sign_message(&self, message: &[u8]) -> Result<SignatureOriginal, SignerError> {
        match self {
            Signer::KeypairWrapper(x) => x.0.try_sign_message(message),
            Signer::PresignerWrapper(x) => x.0.try_sign_message(message),
        }
    }
    fn is_interactive(&self) -> bool {
        match self {
            Signer::KeypairWrapper(x) => x.0.is_interactive(),
            Signer::PresignerWrapper(x) => x.0.is_interactive(),
        }
    }
}

pub struct SignerVec(pub Vec<Signer>);

impl Signers for SignerVec {
    fn pubkeys(&self) -> Vec<PubkeyOriginal> {
        self.0.iter().map(|keypair| keypair.pubkey()).collect()
    }

    fn try_pubkeys(&self) -> Result<Vec<PubkeyOriginal>, SignerError> {
        let mut pubkeys = Vec::new();
        for keypair in self.0.iter() {
            pubkeys.push(keypair.try_pubkey()?);
        }
        Ok(pubkeys)
    }

    fn sign_message(&self, message: &[u8]) -> Vec<SignatureOriginal> {
        self.0
            .iter()
            .map(|keypair| keypair.sign_message(message))
            .collect()
    }

    fn try_sign_message(&self, message: &[u8]) -> Result<Vec<SignatureOriginal>, SignerError> {
        let mut signatures = Vec::new();
        for keypair in self.0.iter() {
            signatures.push(keypair.try_sign_message(message)?);
        }
        Ok(signatures)
    }

    fn is_interactive(&self) -> bool {
        self.0.iter().any(|s| s.is_interactive())
    }
}
