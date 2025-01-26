use pyo3::prelude::*;
use solana_sdk::address_lookup_table::{
    program::ID,
    state::{LOOKUP_TABLE_MAX_ADDRESSES, LOOKUP_TABLE_META_SIZE},
};
use solders_address_lookup_table_account::{
    derive_lookup_table_address, AddressLookupTable, AddressLookupTableAccount, LookupTableMeta,
    LookupTableStatusDeactivating, LookupTableStatusFieldless, SlotHashes,
};

pub(crate) fn include_address_lookup_table_account(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<AddressLookupTableAccount>()?;
    m.add_class::<AddressLookupTable>()?;
    m.add_class::<LookupTableMeta>()?;
    m.add_class::<LookupTableStatusFieldless>()?;
    m.add_class::<LookupTableStatusDeactivating>()?;
    m.add_class::<SlotHashes>()?;
    m.add("ADDRESS_LOOKUP_TABLE_ID", solders_pubkey::Pubkey(ID))?;
    m.add("LOOKUP_TABLE_MAX_ADDRESSES", LOOKUP_TABLE_MAX_ADDRESSES)?;
    m.add("LOOKUP_TABLE_META_SIZE", LOOKUP_TABLE_META_SIZE)?;
    m.add_function(wrap_pyfunction!(derive_lookup_table_address, m)?)?;
    Ok(())
}
