pub mod contract;
mod manage;
mod query;
mod state;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
