use derive_more::{From, Into};
use pyo3::{prelude::*, types::PyBytes};
use solana_program::{program_option::COption, program_pack::Pack};
use solders_macros::{common_methods_core, enum_original_mapping, richcmp_eq_only};
use solders_pubkey::Pubkey;
use solders_traits_core::{
    impl_display, to_py_value_err, CommonMethodsCore, PyBytesGeneral, PyFromBytesGeneral,
    RichcmpEqualityOnly,
};
use spl_token::state::{
    Account as TokenAccountOriginal, AccountState, Mint as MintOriginal,
    Multisig as MultisigOriginal,
};

macro_rules! token_boilerplate {
    ($typ:ident, $inner:ident) => {
        impl_display!($typ);
        impl PyBytesGeneral for $typ {
            fn pybytes_general<'a>(&self, py: Python<'a>) -> &'a PyBytes {
                let mut inner = [0u8; $inner::LEN];
                self.0.pack_into_slice(&mut inner);
                PyBytes::new(py, &inner)
            }
        }

        impl PyFromBytesGeneral for $typ {
            fn py_from_bytes_general(raw: &[u8]) -> PyResult<Self> {
                let inner = $inner::unpack(raw).map_err(|e| to_py_value_err(&e))?;
                Ok(inner.into())
            }
        }

        impl CommonMethodsCore for $typ {}
        impl RichcmpEqualityOnly for $typ {}
    };
}

/// A token mint.
///
/// Args:
///     mint_authority (Optional[Pubkey]): Optional authority used to mint new tokens.
///         The mint authority may only be provided during mint creation.
///         If no mint authority is present then the mint has a fixed supply and no
///         further tokens may be minted.
///     supply (int): Total supply of tokens.
///     decimals (int): Number of base 10 digits to the right of the decimal place.
///     is_initialized (bool): Is ``True`` if this structure has been initialized.
///     freeze_authority (Optional[Pubkey]): Optional authority to freeze token accounts.
///
#[pyclass(module = "solders.token.state", subclass)]
#[derive(Clone, Copy, Debug, PartialEq, Default, From, Into)]
pub struct Mint(pub MintOriginal);

#[richcmp_eq_only]
#[common_methods_core]
#[pymethods]
impl Mint {
    #[new]
    #[pyo3(signature = (mint_authority, supply, decimals, is_initialized = false, freeze_authority = None))]
    pub fn new(
        mint_authority: Option<Pubkey>,
        supply: u64,
        decimals: u8,
        is_initialized: bool,
        freeze_authority: Option<Pubkey>,
    ) -> Self {
        MintOriginal {
            mint_authority: COption::from(mint_authority.map(|p| p.0)),
            supply,
            decimals,
            is_initialized,
            freeze_authority: COption::from(freeze_authority.map(|p| p.0)),
        }
        .into()
    }

    /// Optional[Pubkey]: Optional authority used to mint new tokens.
    #[getter]
    pub fn mint_authority(&self) -> Option<Pubkey> {
        Option::from(self.0.mint_authority).map(Pubkey)
    }

    /// Optional[Pubkey]: Optional authority to freeze token accounts.
    #[getter]
    pub fn freeze_authority(&self) -> Option<Pubkey> {
        Option::from(self.0.freeze_authority).map(Pubkey)
    }

    /// int: Total supply of tokens.
    #[getter]
    pub fn supply(&self) -> u64 {
        self.0.supply
    }

    /// bool: Is ``True`` if this structure has been initialized.
    #[getter]
    pub fn is_initialized(&self) -> bool {
        self.0.is_initialized
    }

    /// int: Number of base 10 digits to the right of the decimal place.
    #[getter]
    pub fn decimals(&self) -> u8 {
        self.0.decimals
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default mint.
    ///
    /// Returns:
    ///     Mint: The default mint.
    ///
    pub fn new_default() -> Self {
        Self::default()
    }
}

token_boilerplate!(Mint, MintOriginal);

/// Token account state.
#[pyclass(module = "solders.token.state")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[enum_original_mapping(AccountState)]
pub enum TokenAccountState {
    /// Account is not yet initialized
    Uninitialized,
    /// Account is initialized; the account owner and/or delegate may perform permitted operations
    /// on this account
    Initialized,
    /// Account has been frozen by the mint freeze authority. Neither the account owner nor
    /// the delegate are able to perform operations on this account.
    Frozen,
}

/// A user token account.
///
/// Args:
///     mint (Pubkey): The mint associated with this account
///     owner (Pubkey): The owner of this account.
///     amount (int): The amount of tokens this account holds.
///     delegate (Optional[Pubkey]): If ``delegate`` is not ``None`` then
///         ``delegated_amount`` represents the amount authorized by the delegate.
///     state (TokenAccountState): The account's state.
///     is_native (Optional[int]): If is_native is not ``None``,
///         this is a native token, and the value logs the rent-exempt reserve.
///         An Account is required to be rent-exempt, so the value is used by
///         the Processor to ensure that wrapped SOL accounts do not
///         drop below this threshold.
///     delegated_amount (int): The amount delegated.
///     close_authority (Optional[Pubkey]): Optional authority to close the account.
///
#[pyclass(module = "solders.token.state", subclass)]
#[derive(Clone, Copy, Debug, PartialEq, Default, From, Into)]
pub struct TokenAccount(pub TokenAccountOriginal);

#[richcmp_eq_only]
#[common_methods_core]
#[pymethods]
impl TokenAccount {
    #[allow(clippy::too_many_arguments)]
    #[new]
    #[pyo3(signature = (mint, owner, amount, delegate, state, is_native, delegated_amount, close_authority = None))]
    pub fn new(
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        state: TokenAccountState,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Self {
        TokenAccountOriginal {
            mint: mint.into(),
            owner: owner.into(),
            amount,
            delegate: COption::from(delegate.map(|x| x.0)),
            state: state.into(),
            is_native: COption::from(is_native),
            delegated_amount,
            close_authority: COption::from(close_authority.map(|x| x.0)),
        }
        .into()
    }

    /// Pubkey: The mint associated with this account
    #[getter]
    pub fn mint(&self) -> Pubkey {
        self.0.mint.into()
    }

    /// Pubkey: The owner of this account.
    #[getter]
    pub fn owner(&self) -> Pubkey {
        self.0.owner.into()
    }

    /// int: The amount of tokens this account holds.
    #[getter]
    pub fn amount(&self) -> u64 {
        self.0.amount
    }

    /// Optional[Pubkey]: If not ``None`` then ``delegated_amount`` represents the amount authorized by the delegate.
    #[getter]
    pub fn delegate(&self) -> Option<Pubkey> {
        Option::from(self.0.delegate.map(Pubkey))
    }

    /// TokenAccountState: The account's state
    #[getter]
    pub fn state(&self) -> TokenAccountState {
        self.0.state.into()
    }

    /// Optional[int]: If not ``None``, this is a native token.
    #[getter]
    pub fn is_native(&self) -> Option<u64> {
        self.0.is_native.into()
    }

    /// int: The amount delegated.
    #[getter]
    pub fn delegated_amount(&self) -> u64 {
        self.0.delegated_amount
    }

    /// Optional[Pubkey]: Optional authority to close the account.
    #[getter]
    pub fn close_authority(&self) -> Option<Pubkey> {
        Option::from(self.0.close_authority.map(Pubkey))
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default token account.
    ///
    /// Returns:
    ///     TokenAccount: The default token account.
    ///
    pub fn new_default() -> Self {
        Self::default()
    }
}

token_boilerplate!(TokenAccount, TokenAccountOriginal);

/// A user token account.
///
/// Args:
///     m (int): The number of signers required.
///     n (int): The number of valid signers.
///     is_initialized (bool): Is ``True`` if this structure has been initialized.
///     signers (Sequence[Pubkey]): Signer public keys.
///
#[pyclass(module = "solders.token.state", subclass)]
#[derive(Clone, Copy, Debug, PartialEq, Default, From, Into)]
pub struct Multisig(pub MultisigOriginal);

#[richcmp_eq_only]
#[common_methods_core]
#[pymethods]
impl Multisig {
    #[new]
    pub fn new(m: u8, n: u8, is_initialized: bool, signers: [Pubkey; 11]) -> Self {
        MultisigOriginal {
            m,
            n,
            is_initialized,
            signers: signers.map(Into::into),
        }
        .into()
    }

    /// int: The number of signers required.
    #[getter]
    pub fn m(&self) -> u8 {
        self.0.m
    }

    /// int: The number of valid signers.
    #[getter]
    pub fn n(&self) -> u8 {
        self.0.n
    }

    /// bool: Is ``True`` if this structure has been initialized.
    #[getter]
    pub fn is_initialized(&self) -> bool {
        self.0.is_initialized
    }

    /// List[Pubkey]: Signer public keys.
    #[getter]
    pub fn signers(&self) -> [Pubkey; 11] {
        self.0.signers.map(Pubkey)
    }

    #[staticmethod]
    #[pyo3(name = "default")]
    /// Create a new default multisig.
    ///
    /// Returns:
    ///     Multisig: The default multisig.
    ///
    pub fn new_default() -> Self {
        Self::default()
    }
}

token_boilerplate!(Multisig, MultisigOriginal);

pub fn create_state_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "state")?;
    m.add_class::<Mint>()?;
    m.add_class::<TokenAccountState>()?;
    m.add_class::<TokenAccount>()?;
    m.add_class::<Multisig>()?;
    Ok(m)
}
