use pyo3::prelude::*;
use solders_pubkey::Pubkey;
use spl_associated_token_account_client::address::get_associated_token_address_with_program_id as get_ata;

/// Derives the associated token account address for the given wallet address and token mint.
///
/// Args:
///     wallet_address (Pubkey): The address of the wallet that owns the token account.
///     token_mint_address (Pubkey): The token mint.
///     token_program_id (Pubkey | None): The token program ID. Defaults to the SPL Token Program.
///
/// Returns:
///     Pubkey: The associated token address
///
#[pyfunction]
#[pyo3(signature = (wallet_address, token_mint_address, token_program_id=None))]
pub fn get_associated_token_address(
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
    token_program_id: Option<&Pubkey>,
) -> Pubkey {
    get_ata(
        wallet_address.as_ref(),
        token_mint_address.as_ref(),
        token_program_id.map_or(&spl_token::ID, |x| x.as_ref()),
    )
    .into()
}

pub fn include_associated(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_associated_token_address, m)?)?;
    Ok(())
}
