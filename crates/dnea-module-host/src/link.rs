//! The conditional linker.

use anyhow::Result;
use wasmtime::component::Linker;

use crate::{grant::GrantSet, host::HostState};

pub fn build_linker(
    engine: &wasmtime::Engine,
    grants: &GrantSet,
) -> Result<Linker<HostState>> {
    let mut linker: Linker<HostState> = Linker::new(engine);

    dnea_abi::dnea::host::types::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
        &mut linker, |s| s,
    )?;
    dnea_abi::dnea::host::log::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
        &mut linker, |s| s,
    )?;
    dnea_abi::dnea::host::storage::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
        &mut linker, |s| s,
    )?;
    dnea_abi::dnea::host::schedule::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
        &mut linker, |s| s,
    )?;

    if grants.any_of(&[
        "browser:tabs:query", "browser:tabs:update",
        "browser:tabs:create", "browser:tabs:close",
    ]) {
        dnea_abi::dnea::host::tabs::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
            &mut linker, |s| s,
        )?;
    }

    if grants.any_of(&["net:connect", "net:listen", "net:websocket"]) {
        dnea_abi::dnea::host::net::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
            &mut linker, |s| s,
        )?;
    }

    if grants.any_of(&[
        "ui:companion:panel", "ui:companion:notify",
        "ui:webext:popup", "ui:webext:content",
    ]) {
        dnea_abi::dnea::host::ui::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
            &mut linker, |s| s,
        )?;
    }

    if grants.any_of(&["identity:crypto:sign", "identity:crypto:verify"]) {
        dnea_abi::dnea::host::crypto::add_to_linker::<HostState, wasmtime::component::HasSelf<HostState>>(
            &mut linker, |s| s,
        )?;
    }

    Ok(linker)
}