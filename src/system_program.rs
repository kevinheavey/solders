use pyo3::prelude::*;
use solana_sdk::system_instruction::transfer;

use crate::{Instruction, Pubkey};

#[pyclass]
pub struct SystemProgram;

#[pymethods]
impl SystemProgram {
    #[staticmethod]
    pub fn transfer(from_pubkey: &Pubkey, to_pubkey: &Pubkey, lamports: u64) -> Instruction {
        transfer(from_pubkey.as_ref(), to_pubkey.as_ref(), lamports).into()
    }
}
