use bincode::ErrorKind;
#[cfg(feature = "litesvm")]
use litesvm::error::LiteSVMError as LiteSVMErrorOriginal;
use pyo3::{create_exception, exceptions::PyException, prelude::*, pyclass::CompareOp};
#[cfg(feature = "banks-client")]
use solana_banks_client::BanksClientError as BanksClientErrorOriginal;
use solders_traits_core::richcmp_type_error;
use {
    solana_commitment_config::ParseCommitmentLevelError as ParseCommitmentLevelErrorOriginal,
    solana_hash::ParseHashError as ParseHashErrorOriginal,
    solana_pubkey::{Pubkey as PubkeyOriginal, PubkeyError as PubkeyErrorOriginal},
    solana_sanitize::SanitizeError as SanitizeErrorOriginal,
    solana_signature::Signature as SignatureOriginal,
    solana_signer::{Signer as SignerTrait, SignerError as SignerErrorOriginal},
    solana_transaction_error::TransactionError as TransactionErrorOriginal,
};
pub struct PyErrWrapper(pub PyErr);

impl From<PyErrWrapper> for PyErr {
    fn from(e: PyErrWrapper) -> Self {
        e.0
    }
}

create_exception!(
    solders,
    ParseCommitmentLevelError,
    PyException,
    "Raised when an error is encountered converting a string into a ``CommitmentConfig``."
);

create_exception!(
    solders,
    SerdeJSONError,
    PyException,
    "Raised when an error is encountered during JSON (de)serialization."
);

impl From<serde_json::Error> for PyErrWrapper {
    fn from(e: serde_json::Error) -> Self {
        Self(SerdeJSONError::new_err(e.to_string()))
    }
}

impl From<ParseCommitmentLevelErrorOriginal> for PyErrWrapper {
    fn from(e: ParseCommitmentLevelErrorOriginal) -> Self {
        Self(ParseCommitmentLevelError::new_err(e.to_string()))
    }
}

create_exception!(
    solders,
    ParseHashError,
    PyException,
    "Raised when an error is encountered converting a string into a ``Hash``."
);

impl From<ParseHashErrorOriginal> for PyErrWrapper {
    fn from(e: ParseHashErrorOriginal) -> Self {
        Self(ParseHashError::new_err(e.to_string()))
    }
}

create_exception!(
    solders,
    SignerError,
    PyException,
    "Raised when an error is encountered during transaction signing."
);

impl From<SignerErrorOriginal> for PyErrWrapper {
    fn from(e: SignerErrorOriginal) -> Self {
        Self(SignerError::new_err(e.to_string()))
    }
}

create_exception!(
    solders,
    TransactionError,
    PyException,
    "Umbrella error for the ``Transaction`` object."
);

impl From<TransactionErrorOriginal> for PyErrWrapper {
    fn from(e: TransactionErrorOriginal) -> Self {
        Self(TransactionError::new_err(e.to_string()))
    }
}

create_exception!(
    solders,
    SanitizeError,
    PyException,
    "Raised when an error is encountered during transaction sanitization."
);

impl From<SanitizeErrorOriginal> for PyErrWrapper {
    fn from(e: SanitizeErrorOriginal) -> Self {
        Self(SanitizeError::new_err(e.to_string()))
    }
}

pub fn to_py_err<T: Into<PyErrWrapper>>(e: T) -> PyErr {
    let wrapped: PyErrWrapper = e.into();
    wrapped.into()
}

pub fn handle_py_err<T: Into<P>, E: ToString + Into<PyErrWrapper>, P>(
    res: Result<T, E>,
) -> PyResult<P> {
    res.map_or_else(|e| Err(to_py_err(e)), |v| Ok(v.into()))
}

create_exception!(
    solders,
    BincodeError,
    PyException,
    "Raised when the Rust bincode library returns an error during (de)serialization."
);

create_exception!(
    solders,
    CborError,
    PyException,
    "Raised when the Rust cbor library returns an error during (de)serialization."
);

create_exception!(
    solders,
    PubkeyError,
    PyException,
    "Umbrella error for the ``Pubkey`` object."
);

impl From<PubkeyErrorOriginal> for PyErrWrapper {
    fn from(e: PubkeyErrorOriginal) -> Self {
        Self(PubkeyError::new_err(e.to_string()))
    }
}

impl From<Box<ErrorKind>> for PyErrWrapper {
    fn from(e: Box<ErrorKind>) -> Self {
        Self(BincodeError::new_err(e.to_string()))
    }
}

impl From<serde_cbor::Error> for PyErrWrapper {
    fn from(e: serde_cbor::Error) -> Self {
        Self(CborError::new_err(e.to_string()))
    }
}

#[cfg(feature = "banks-client")]
create_exception!(
    solders,
    BanksClientError,
    PyException,
    "Raised when BanksClient encounters an error."
);

#[cfg(feature = "banks-client")]
impl From<BanksClientErrorOriginal> for PyErrWrapper {
    fn from(e: BanksClientErrorOriginal) -> Self {
        Self(BanksClientError::new_err(e.to_string()))
    }
}

#[cfg(feature = "litesvm")]
create_exception!(
    solders,
    LiteSVMError,
    PyException,
    "Raised when LiteSVM encounters an error."
);

#[cfg(feature = "litesvm")]
impl From<LiteSVMErrorOriginal> for PyErrWrapper {
    fn from(e: LiteSVMErrorOriginal) -> Self {
        Self(LiteSVMError::new_err(e.to_string()))
    }
}

create_exception!(
    solders,
    AddProgramError,
    PyException,
    "Raised when adding a program from a file fails."
);

pub trait ToSignerOriginal {
    fn to_inner(&self) -> Box<dyn SignerTrait>;
}

pub trait SignerTraitWrapper: ToSignerOriginal {
    fn pubkey(&self) -> PubkeyOriginal {
        self.to_inner().pubkey()
    }
    fn try_pubkey(&self) -> Result<PubkeyOriginal, SignerErrorOriginal> {
        self.to_inner().try_pubkey()
    }
    fn sign_message(&self, message: &[u8]) -> SignatureOriginal {
        self.to_inner().sign_message(message)
    }
    fn try_sign_message(&self, message: &[u8]) -> Result<SignatureOriginal, SignerErrorOriginal> {
        self.to_inner().try_sign_message(message)
    }
    fn is_interactive(&self) -> bool {
        self.to_inner().is_interactive()
    }
}

pub trait RichcmpSigner: SignerTraitWrapper {
    fn richcmp(&self, other: impl SignerTraitWrapper, op: CompareOp) -> PyResult<bool> {
        let eq_val = self.pubkey() == other.pubkey();
        match op {
            CompareOp::Eq => Ok(eq_val),
            CompareOp::Ne => Ok(!eq_val),
            CompareOp::Lt => Err(richcmp_type_error("<")),
            CompareOp::Gt => Err(richcmp_type_error(">")),
            CompareOp::Le => Err(richcmp_type_error("<=")),
            CompareOp::Ge => Err(richcmp_type_error(">=")),
        }
    }
}

#[macro_export]
macro_rules! impl_signer_hash {
    ($ident:ident) => {
        #[allow(clippy::derived_hash_with_manual_eq)]
        impl std::hash::Hash for $ident {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                $crate::SignerTraitWrapper::pubkey(self).hash(state);
            }
        }
    };
}
