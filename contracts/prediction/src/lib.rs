pub mod contract;
mod handler;
mod manage;
mod msg;
mod query;
mod state;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
