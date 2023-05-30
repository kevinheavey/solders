use pyo3::prelude::*;
use solders_pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address as get_ata;

/// Derives the associated token account address for the given wallet address and token mint.
#[pyfunction]
pub fn get_associated_token_address(
    wallet_address: &Pubkey,
    token_mint_address: &Pubkey,
) -> Pubkey {
    get_ata(wallet_address.as_ref(), token_mint_address.as_ref()).into()
}

pub fn create_associated_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "associated")?;
    m.add_function(wrap_pyfunction!(get_associated_token_address, m)?)?;
    Ok(m)
}
