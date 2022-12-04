use pyo3::prelude::*;

use solana_sdk::{
    pubkey::Pubkey as PubkeyOriginal,
    signature::Signature as SignatureOriginal,
    signer::{signers::Signers, Signer as SignerTrait, SignerError as SignerErrorOriginal},
};
use solders_traits::{SignerTraitWrapper, ToSignerOriginal};

use crate::{keypair::Keypair, null_signer::NullSigner, presigner::Presigner};

#[derive(FromPyObject, Debug)]
pub enum Signer {
    KeypairWrapper(Keypair),
    PresignerWrapper(Presigner),
    NullSignerWrapper(NullSigner),
}

impl ToSignerOriginal for Signer {
    fn to_inner(&self) -> Box<dyn SignerTrait> {
        match self {
            Signer::KeypairWrapper(x) => x.to_inner(),
            Signer::PresignerWrapper(x) => x.to_inner(),
            Signer::NullSignerWrapper(x) => x.to_inner(),
        }
    }
}

impl SignerTraitWrapper for Signer {}

pub struct SignerVec(pub Vec<Signer>);

impl Signers for SignerVec {
    fn pubkeys(&self) -> Vec<PubkeyOriginal> {
        self.0.iter().map(|keypair| keypair.pubkey()).collect()
    }

    fn try_pubkeys(&self) -> Result<Vec<PubkeyOriginal>, SignerErrorOriginal> {
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

    fn try_sign_message(
        &self,
        message: &[u8],
    ) -> Result<Vec<SignatureOriginal>, SignerErrorOriginal> {
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
