use pyo3::prelude::*;
pub mod associated;
pub mod state;

pub fn include_token(m: &Bound<'_, PyModule>) -> PyResult<()> {
    state::include_state(m)?;
    associated::include_associated(m)?;
    m.add("TOKEN_PROGRAM_ID", solders_pubkey::Pubkey(spl_token::ID))
        .unwrap();
    Ok(())
}
