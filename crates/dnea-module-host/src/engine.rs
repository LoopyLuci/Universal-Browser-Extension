//! Wasmtime engine construction.

use anyhow::{Context, Result};
use wasmtime::{Config, Engine, InstanceAllocationStrategy, PoolingAllocationConfig};

#[derive(Clone, Debug)]
pub struct EngineConfig {
    pub total_memory_slots: u32,
    pub total_stack_slots: u32,
    pub max_memory_bytes: u64,
    pub max_memory_pages: u32,
    pub max_tables_per_component: u32,
    pub table_elements: u32,
    pub total_tables: u32,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            total_memory_slots: 16,
            total_stack_slots: 16,
            max_memory_bytes: 64 * 1024 * 1024,
            max_memory_pages: 1024,
            max_tables_per_component: 4,
            table_elements: 10_000,
            total_tables: 64,
        }
    }
}

pub fn build_engine(cfg: &EngineConfig) -> Result<Engine> {
    let mut c = Config::new();
    c.wasm_component_model(true);
    c.async_support(true);
    c.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    c.wasm_multi_memory(true);
    c.wasm_bulk_memory(true);
    c.wasm_simd(true);
    c.wasm_relaxed_simd(false);
    c.consume_fuel(true);
    c.epoch_interruption(true);
    c.cranelift_opt_level(wasmtime::OptLevel::Speed);

    let mut pool = PoolingAllocationConfig::default();
    pool.total_memories(cfg.total_memory_slots);
    pool.total_stacks(cfg.total_stack_slots);
    pool.max_memory_size(cfg.max_memory_bytes as usize);
    pool.memory_pages(cfg.max_memory_pages);
    pool.max_tables_per_component(cfg.max_tables_per_component);
    pool.table_elements(cfg.table_elements);
    pool.total_tables(cfg.total_tables);
    c.allocation_strategy(InstanceAllocationStrategy::Pooling(pool));

    Engine::new(&c).context("build wasmtime engine")
}