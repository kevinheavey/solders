use pyo3::prelude::*;
use solana_sdk::compute_budget::{ComputeBudgetInstruction, ID};
use solders_instruction::Instruction;
use solders_pubkey::Pubkey;

/// Request a specific transaction-wide program heap region size in bytes.
/// The value requested must be a multiple of 1024. This new heap region
/// size applies to each program executed in the transaction, including all
/// calls to CPIs.
#[pyfunction]
pub fn request_heap_frame(bytes_: u32) -> Instruction {
    ComputeBudgetInstruction::request_heap_frame(bytes_).into()
}

/// Set a specific compute unit limit that the transaction is allowed to consume.
#[pyfunction]
pub fn set_compute_unit_limit(units: u32) -> Instruction {
    ComputeBudgetInstruction::set_compute_unit_limit(units).into()
}

/// Set a compute unit price in "micro-lamports" to pay a higher transaction
/// fee for higher transaction prioritization.
#[pyfunction]
pub fn set_compute_unit_price(micro_lamports: u64) -> Instruction {
    ComputeBudgetInstruction::set_compute_unit_price(micro_lamports).into()
}

pub fn create_compute_budget_mod(py: Python<'_>) -> PyResult<&PyModule> {
    let m = PyModule::new(py, "compute_budget")?;
    m.add("ID", Pubkey(ID))?;
    let funcs = [
        wrap_pyfunction!(request_heap_frame, m)?,
        wrap_pyfunction!(set_compute_unit_limit, m)?,
        wrap_pyfunction!(set_compute_unit_price, m)?,
    ];
    for func in funcs {
        m.add_function(func)?;
    }
    Ok(m)
}
