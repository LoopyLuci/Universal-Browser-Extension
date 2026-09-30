//! Host bindings for the DNEA module world.

#![forbid(unsafe_code)]
#![allow(clippy::all, missing_docs)]

wasmtime::component::bindgen!({
    path: "../../wit",
    world: "module",
    async: true,
});
