use pyo3::prelude::*;
use solana_compute_budget::compute_budget::ComputeBudget as ComputeBudgetOriginal;
use solana_compute_budget_interface::ComputeBudgetInstruction;
use solana_sdk_ids::compute_budget::ID;
use solders_instruction::Instruction;
use solders_pubkey::Pubkey;
use solders_traits_core::RichcmpEqualityOnly;

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

#[pyclass(module = "solders.compute_budget", subclass)]
#[derive(Debug, Clone, PartialEq)]
pub struct ComputeBudget(pub ComputeBudgetOriginal);

impl RichcmpEqualityOnly for ComputeBudget {}

#[solders_macros::richcmp_eq_only]
#[pymethods]
impl ComputeBudget {
    fn __str__(&self) -> String {
        self.__repr__()
    }
    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }

    #[allow(clippy::new_without_default)]
    #[new]
    pub fn new() -> Self {
        Self(ComputeBudgetOriginal::default())
    }

    #[getter]
    pub fn compute_unit_limit(&self) -> u64 {
        self.0.compute_unit_limit
    }

    #[setter]
    pub fn set_compute_unit_limit(&mut self, limit: u64) {
        self.0.compute_unit_limit = limit;
    }

    #[setter]
    pub fn set_log_64_units(&mut self, val: u64) {
        self.0.log_64_units = val;
    }
    #[getter]
    pub fn log_64_units(&self) -> u64 {
        self.0.log_64_units
    }
    #[setter]
    pub fn set_create_program_address_units(&mut self, val: u64) {
        self.0.create_program_address_units = val;
    }
    #[getter]
    pub fn create_program_address_units(&self) -> u64 {
        self.0.create_program_address_units
    }
    #[setter]
    pub fn set_invoke_units(&mut self, val: u64) {
        self.0.invoke_units = val;
    }
    #[getter]
    pub fn invoke_units(&self) -> u64 {
        self.0.invoke_units
    }
    #[setter]
    pub fn set_max_instruction_stack_depth(&mut self, val: usize) {
        self.0.max_instruction_stack_depth = val;
    }
    #[getter]
    pub fn max_instruction_stack_depth(&self) -> usize {
        self.0.max_instruction_stack_depth
    }
    #[setter]
    pub fn set_max_instruction_trace_length(&mut self, val: usize) {
        self.0.max_instruction_trace_length = val;
    }
    #[getter]
    pub fn max_instruction_trace_length(&self) -> usize {
        self.0.max_instruction_trace_length
    }
    #[setter]
    pub fn set_sha256_base_cost(&mut self, val: u64) {
        self.0.sha256_base_cost = val;
    }
    #[getter]
    pub fn sha256_base_cost(&self) -> u64 {
        self.0.sha256_base_cost
    }
    #[setter]
    pub fn set_sha256_byte_cost(&mut self, val: u64) {
        self.0.sha256_byte_cost = val;
    }
    #[getter]
    pub fn sha256_byte_cost(&self) -> u64 {
        self.0.sha256_byte_cost
    }
    #[setter]
    pub fn set_sha256_max_slices(&mut self, val: u64) {
        self.0.sha256_max_slices = val;
    }
    #[getter]
    pub fn sha256_max_slices(&self) -> u64 {
        self.0.sha256_max_slices
    }
    #[setter]
    pub fn set_max_call_depth(&mut self, val: usize) {
        self.0.max_call_depth = val;
    }
    #[getter]
    pub fn max_call_depth(&self) -> usize {
        self.0.max_call_depth
    }
    #[setter]
    pub fn set_stack_frame_size(&mut self, val: usize) {
        self.0.stack_frame_size = val
    }
    #[getter]
    pub fn stack_frame_size(&self) -> usize {
        self.0.stack_frame_size
    }
    #[setter]
    pub fn set_log_pubkey_units(&mut self, val: u64) {
        self.0.log_pubkey_units = val;
    }
    #[getter]
    pub fn log_pubkey_units(&self) -> u64 {
        self.0.log_pubkey_units
    }
    #[setter]
    pub fn set_max_cpi_instruction_size(&mut self, val: usize) {
        self.0.max_cpi_instruction_size = val
    }
    #[getter]
    pub fn max_cpi_instruction_size(&self) -> usize {
        self.0.max_cpi_instruction_size
    }
    #[setter]
    pub fn set_cpi_bytes_per_unit(&mut self, val: u64) {
        self.0.cpi_bytes_per_unit = val;
    }
    #[getter]
    pub fn cpi_bytes_per_unit(&self) -> u64 {
        self.0.cpi_bytes_per_unit
    }
    #[setter]
    pub fn set_sysvar_base_cost(&mut self, val: u64) {
        self.0.sysvar_base_cost = val;
    }
    #[getter]
    pub fn sysvar_base_cost(&self) -> u64 {
        self.0.sysvar_base_cost
    }
    #[setter]
    pub fn set_secp256k1_recover_cost(&mut self, val: u64) {
        self.0.secp256k1_recover_cost = val;
    }
    #[getter]
    pub fn secp256k1_recover_cost(&self) -> u64 {
        self.0.secp256k1_recover_cost
    }
    #[setter]
    pub fn set_syscall_base_cost(&mut self, val: u64) {
        self.0.syscall_base_cost = val;
    }
    #[getter]
    pub fn syscall_base_cost(&self) -> u64 {
        self.0.syscall_base_cost
    }
    #[setter]
    pub fn set_curve25519_edwards_validate_point_cost(&mut self, val: u64) {
        self.0.curve25519_edwards_validate_point_cost = val;
    }
    #[getter]
    pub fn curve25519_edwards_validate_point_cost(&self) -> u64 {
        self.0.curve25519_edwards_validate_point_cost
    }
    #[setter]
    pub fn set_curve25519_edwards_add_cost(&mut self, val: u64) {
        self.0.curve25519_edwards_add_cost = val;
    }
    #[getter]
    pub fn curve25519_edwards_add_cost(&self) -> u64 {
        self.0.curve25519_edwards_add_cost
    }
    #[setter]
    pub fn set_curve25519_edwards_subtract_cost(&mut self, val: u64) {
        self.0.curve25519_edwards_subtract_cost = val;
    }
    #[getter]
    pub fn curve25519_edwards_subtract_cost(&self) -> u64 {
        self.0.curve25519_edwards_subtract_cost
    }
    #[setter]
    pub fn set_curve25519_edwards_multiply_cost(&mut self, val: u64) {
        self.0.curve25519_edwards_multiply_cost = val;
    }
    #[getter]
    pub fn curve25519_edwards_multiply_cost(&self) -> u64 {
        self.0.curve25519_edwards_multiply_cost
    }
    #[setter]
    pub fn set_curve25519_edwards_msm_base_cost(&mut self, val: u64) {
        self.0.curve25519_edwards_msm_base_cost = val;
    }
    #[getter]
    pub fn curve25519_edwards_msm_base_cost(&self) -> u64 {
        self.0.curve25519_edwards_msm_base_cost
    }
    #[setter]
    pub fn set_curve25519_edwards_msm_incremental_cost(&mut self, val: u64) {
        self.0.curve25519_edwards_msm_incremental_cost = val;
    }
    #[getter]
    pub fn curve25519_edwards_msm_incremental_cost(&self) -> u64 {
        self.0.curve25519_edwards_msm_incremental_cost
    }
    #[setter]
    pub fn set_curve25519_ristretto_validate_point_cost(&mut self, val: u64) {
        self.0.curve25519_ristretto_validate_point_cost = val;
    }
    #[getter]
    pub fn curve25519_ristretto_validate_point_cost(&self) -> u64 {
        self.0.curve25519_ristretto_validate_point_cost
    }
    #[setter]
    pub fn set_curve25519_ristretto_add_cost(&mut self, val: u64) {
        self.0.curve25519_ristretto_add_cost = val;
    }
    #[getter]
    pub fn curve25519_ristretto_add_cost(&self) -> u64 {
        self.0.curve25519_ristretto_add_cost
    }
    #[setter]
    pub fn set_curve25519_ristretto_subtract_cost(&mut self, val: u64) {
        self.0.curve25519_ristretto_subtract_cost = val;
    }
    #[getter]
    pub fn curve25519_ristretto_subtract_cost(&self) -> u64 {
        self.0.curve25519_ristretto_subtract_cost
    }
    #[setter]
    pub fn set_curve25519_ristretto_multiply_cost(&mut self, val: u64) {
        self.0.curve25519_ristretto_multiply_cost = val;
    }
    #[getter]
    pub fn curve25519_ristretto_multiply_cost(&self) -> u64 {
        self.0.curve25519_ristretto_multiply_cost
    }
    #[setter]
    pub fn set_curve25519_ristretto_msm_base_cost(&mut self, val: u64) {
        self.0.curve25519_ristretto_msm_base_cost = val;
    }
    #[getter]
    pub fn curve25519_ristretto_msm_base_cost(&self) -> u64 {
        self.0.curve25519_ristretto_msm_base_cost
    }
    #[setter]
    pub fn set_curve25519_ristretto_msm_incremental_cost(&mut self, val: u64) {
        self.0.curve25519_ristretto_msm_incremental_cost = val;
    }
    #[getter]
    pub fn curve25519_ristretto_msm_incremental_cost(&self) -> u64 {
        self.0.curve25519_ristretto_msm_incremental_cost
    }
    #[setter]
    pub fn set_heap_size(&mut self, val: u32) {
        self.0.heap_size = val;
    }
    #[getter]
    pub fn heap_size(&self) -> u32 {
        self.0.heap_size
    }
    #[setter]
    pub fn set_heap_cost(&mut self, val: u64) {
        self.0.heap_cost = val;
    }
    #[getter]
    pub fn heap_cost(&self) -> u64 {
        self.0.heap_cost
    }
    #[setter]
    pub fn set_mem_op_base_cost(&mut self, val: u64) {
        self.0.mem_op_base_cost = val;
    }
    #[getter]
    pub fn mem_op_base_cost(&self) -> u64 {
        self.0.mem_op_base_cost
    }
    #[setter]
    pub fn set_alt_bn128_addition_cost(&mut self, val: u64) {
        self.0.alt_bn128_addition_cost = val;
    }
    #[getter]
    pub fn alt_bn128_addition_cost(&self) -> u64 {
        self.0.alt_bn128_addition_cost
    }
    #[setter]
    pub fn set_alt_bn128_multiplication_cost(&mut self, val: u64) {
        self.0.alt_bn128_multiplication_cost = val;
    }
    #[getter]
    pub fn alt_bn128_multiplication_cost(&self) -> u64 {
        self.0.alt_bn128_multiplication_cost
    }
    #[setter]
    pub fn set_alt_bn128_pairing_one_pair_cost_first(&mut self, val: u64) {
        self.0.alt_bn128_pairing_one_pair_cost_first = val;
    }
    #[getter]
    pub fn alt_bn128_pairing_one_pair_cost_first(&self) -> u64 {
        self.0.alt_bn128_pairing_one_pair_cost_first
    }
    #[setter]
    pub fn set_alt_bn128_pairing_one_pair_cost_other(&mut self, val: u64) {
        self.0.alt_bn128_pairing_one_pair_cost_other = val;
    }
    #[getter]
    pub fn alt_bn128_pairing_one_pair_cost_other(&self) -> u64 {
        self.0.alt_bn128_pairing_one_pair_cost_other
    }
    #[setter]
    pub fn set_big_modular_exponentiation_base_cost(&mut self, val: u64) {
        self.0.big_modular_exponentiation_base_cost = val;
    }
    #[getter]
    pub fn big_modular_exponentiation_base_cost(&self) -> u64 {
        self.0.big_modular_exponentiation_base_cost
    }
    #[setter]
    pub fn set_big_modular_exponentiation_cost_divisor(&mut self, val: u64) {
        self.0.big_modular_exponentiation_cost_divisor = val;
    }
    #[getter]
    pub fn big_modular_exponentiation_cost_divisor(&self) -> u64 {
        self.0.big_modular_exponentiation_cost_divisor
    }
    #[setter]
    pub fn set_poseidon_cost_coefficient_a(&mut self, val: u64) {
        self.0.poseidon_cost_coefficient_a = val;
    }
    #[getter]
    pub fn poseidon_cost_coefficient_a(&self) -> u64 {
        self.0.poseidon_cost_coefficient_a
    }
    #[setter]
    pub fn set_poseidon_cost_coefficient_c(&mut self, val: u64) {
        self.0.poseidon_cost_coefficient_c = val;
    }
    #[getter]
    pub fn poseidon_cost_coefficient_c(&self) -> u64 {
        self.0.poseidon_cost_coefficient_c
    }
    #[setter]
    pub fn set_get_remaining_compute_units_cost(&mut self, val: u64) {
        self.0.get_remaining_compute_units_cost = val;
    }
    #[pyo3(name = "get_remaining_compute_units_cost")]
    #[getter]
    pub fn remaining_compute_units_cost(&self) -> u64 {
        self.0.get_remaining_compute_units_cost
    }
    #[setter]
    pub fn set_alt_bn128_g1_compress(&mut self, val: u64) {
        self.0.alt_bn128_g1_compress = val;
    }
    #[getter]
    pub fn alt_bn128_g1_compress(&self) -> u64 {
        self.0.alt_bn128_g1_compress
    }
    #[setter]
    pub fn set_alt_bn128_g1_decompress(&mut self, val: u64) {
        self.0.alt_bn128_g1_decompress = val;
    }
    #[getter]
    pub fn alt_bn128_g1_decompress(&self) -> u64 {
        self.0.alt_bn128_g1_decompress
    }
    #[setter]
    pub fn set_alt_bn128_g2_compress(&mut self, val: u64) {
        self.0.alt_bn128_g2_compress = val;
    }
    #[getter]
    pub fn alt_bn128_g2_compress(&self) -> u64 {
        self.0.alt_bn128_g2_compress
    }
    #[setter]
    pub fn set_alt_bn128_g2_decompress(&mut self, val: u64) {
        self.0.alt_bn128_g2_decompress = val;
    }
    #[getter]
    pub fn alt_bn128_g2_decompress(&self) -> u64 {
        self.0.alt_bn128_g2_decompress
    }
}

pub fn include_compute_budget(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("COMPUTE_BUDGET_ID", Pubkey(ID))?;
    let funcs = [
        wrap_pyfunction!(request_heap_frame, m)?,
        wrap_pyfunction!(set_compute_unit_limit, m)?,
        wrap_pyfunction!(set_compute_unit_price, m)?,
    ];
    for func in funcs {
        m.add_function(func)?;
    }
    m.add_class::<ComputeBudget>()?;
    Ok(())
}
