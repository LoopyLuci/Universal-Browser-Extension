//! DNEA Native Messaging bridge.

use std::{
    env,
    io::{self, Read, Write},
    process::ExitCode,
};

use anyhow::{bail, Context, Result};

const MAX_FRAME: usize = 64 * 1024 * 1024;

#[cfg(unix)]
mod transport {
    use super::*;
    use std::os::unix::net::UnixStream;
    use std::path::PathBuf;

    pub fn connect() -> Result<UnixStream> {
        let path = socket_path()?;
        UnixStream::connect(&path)
            .with_context(|| format!("connect to runtime at {}", path.display()))
    }

    fn socket_path() -> Result<PathBuf> {
        if let Ok(p) = env::var("DNEA_SOCKET") {
            return Ok(PathBuf::from(p));
        }
        let proj = directories::ProjectDirs::from("org", "dnea", "runtime")
            .context("no home directory for runtime dir")?;
        let dir = proj.runtime_dir().context("no runtime dir")?;
        Ok(dir.join("nm.sock"))
    }
}

#[cfg(windows)]
mod transport {
    use super::*;
    use std::fs::OpenOptions;
    use std::path::PathBuf;

    pub fn connect() -> Result<std::fs::File> {
        let path = pipe_path()?;
        OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .with_context(|| format!("open named pipe {}", path.display()))
    }

    fn pipe_path() -> Result<PathBuf> {
        if let Ok(p) = env::var("DNEA_PIPE") {
            return Ok(PathBuf::from(p));
        }
        let user = env::var("USERNAME").unwrap_or_else(|_| "default".into());
        Ok(PathBuf::from(format!(r"\\.\pipe\dnea-nm-{user}")))
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("dnea-nm-bridge: {e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let mut upstream = transport::connect()?;
    let preamble = serde_json::json!({
        "browser_family": detect_browser_family(),
        "extension_id":   env::var("DNEA_EXTENSION_ID").ok(),
        "pid":            std::process::id(),
        "protocol":       "dnea-nm-v1",
    });
    let preamble_bytes = serde_json::to_vec(&preamble)?;
    write_frame(&mut upstream, &preamble_bytes)?;

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut upstream_r = upstream.try_clone()?;
    let mut upstream_w = upstream;

    let in_thread = std::thread::Builder::new()
        .name("browser->runtime".into())
        .spawn(move || pump_framed(stdin.lock(), upstream_w))?;
    let out_result = pump_framed(upstream_r, stdout.lock());
    let _ = in_thread.join();
    out_result
}

fn pump_framed<R, W>(mut r: R, mut w: W) -> Result<()>
where R: Read, W: Write {
    loop {
        match read_frame(&mut r)? {
            None => return Ok(()),
            Some(bytes) => write_frame(&mut w, &bytes)?,
        }
    }
}

fn read_frame<R: Read>(r: &mut R) -> Result<Option<Vec<u8>>> {
    let mut len_buf = [0u8; 4];
    match r.read_exact(&mut len_buf) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e).context("read length prefix"),
    }
    let len = u32::from_ne_bytes(len_buf) as usize;
    if len == 0 { bail!("zero-length frame"); }
    if len > MAX_FRAME { bail!("frame size {len} exceeds maximum {MAX_FRAME}"); }
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf).context("read frame body")?;
    Ok(Some(buf))
}

fn write_frame<W: Write>(w: &mut W, bytes: &[u8]) -> Result<()> {
    if bytes.is_empty() { bail!("refusing to write empty frame"); }
    let len = u32::try_from(bytes.len()).context("frame too large for u32 prefix")?;
    w.write_all(&len.to_ne_bytes())?;
    w.write_all(bytes)?;
    w.flush()?;
    Ok(())
}

fn detect_browser_family() -> &'static str {
    if env::var_os("MOZ_CRASHREPORTER").is_some() { return "firefox"; }
    if let Ok(v) = env::var("DNEA_BROWSER_FAMILY") {
        return match v.as_str() {
            "chrome" => "chromium", "edge" => "chromium",
            "brave" => "chromium", "vivaldi" => "chromium",
            "opera" => "chromium", "safari" => "safari",
            "firefox" => "firefox", _ => "unknown",
        };
    }
    "unknown"
}