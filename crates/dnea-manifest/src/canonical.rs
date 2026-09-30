//! RFC 8785 (JCS) canonicalization.

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CanonicalError {
    #[error("number is not finite")]
    NonFinite,
    #[error("number cannot be represented in JCS: {0}")]
    UnrepresentableNumber(String),
}

pub fn canonicalize(value: &Value) -> Result<String, CanonicalError> {
    let mut out = String::with_capacity(256);
    write_value(value, &mut out)?;
    Ok(out)
}

fn write_value(v: &Value, out: &mut String) -> Result<(), CanonicalError> {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(n) => write_number(n, out)?,
        Value::String(s) => write_string(s, out),
        Value::Array(a) => {
            out.push('[');
            for (i, item) in a.iter().enumerate() {
                if i > 0 { out.push(','); }
                write_value(item, out)?;
            }
            out.push(']');
        }
        Value::Object(o) => {
            out.push('{');
            let mut keys: Vec<&String> = o.keys().collect();
            keys.sort_by(|a, b| cmp_utf16(a, b));
            for (i, k) in keys.iter().enumerate() {
                if i > 0 { out.push(','); }
                write_string(k, out);
                out.push(':');
                write_value(&o[*k], out)?;
            }
            out.push('}');
        }
    }
    Ok(())
}

fn cmp_utf16(a: &str, b: &str) -> std::cmp::Ordering {
    let mut ai = a.encode_utf16();
    let mut bi = b.encode_utf16();
    loop {
        match (ai.next(), bi.next()) {
            (None, None) => return std::cmp::Ordering::Equal,
            (None, Some(_)) => return std::cmp::Ordering::Less,
            (Some(_), None) => return std::cmp::Ordering::Greater,
            (Some(x), Some(y)) if x == y => continue,
            (Some(x), Some(y)) => return x.cmp(&y),
        }
    }
}

fn write_string(s: &str, out: &mut String) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                use std::fmt::Write;
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

fn write_number(n: &serde_json::Number, out: &mut String) -> Result<(), CanonicalError> {
    if let Some(i) = n.as_i64() { out.push_str(&i.to_string()); return Ok(()); }
    if let Some(u) = n.as_u64() { out.push_str(&u.to_string()); return Ok(()); }
    if let Some(f) = n.as_f64() { return write_f64(f, out); }
    Err(CanonicalError::UnrepresentableNumber(n.to_string()))
}

fn write_f64(f: f64, out: &mut String) -> Result<(), CanonicalError> {
    if !f.is_finite() { return Err(CanonicalError::NonFinite); }
    if f == 0.0 { out.push('0'); return Ok(()); }
    if f.fract() == 0.0 && f.abs() < 9.0e15 {
        use std::fmt::Write;
        let _ = write!(out, "{}", f as i64);
        return Ok(());
    }
    use std::fmt::Write;
    let _ = write!(out, "{}", f);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sorts_object_keys() {
        assert_eq!(canonicalize(&json!({"b": 1, "a": 2, "c": 3})).unwrap(), r#"{"a":2,"b":1,"c":3}"#);
    }

    #[test]
    fn removes_whitespace() {
        let v: Value = serde_json::from_str(r#"{ "a" : 1 , "b" : [ 1 , 2 ] }"#).unwrap();
        assert_eq!(canonicalize(&v).unwrap(), r#"{"a":1,"b":[1,2]}"#);
    }

    #[test]
    fn nested_structures_sort_recursively() {
        let v = json!({"z": {"y": 1, "x": 2}, "a": [{"b": 3, "a": 4}]});
        assert_eq!(canonicalize(&v).unwrap(), r#"{"a":[{"a":4,"b":3}],"z":{"x":2,"y":1}}"#);
    }

    #[test]
    fn determinism() {
        let v1: Value = serde_json::from_str(r#"{"a":1,"b":[2,3],"c":{"d":4}}"#).unwrap();
        let v2: Value = serde_json::from_str(r#"{"c":{"d":4},"b":[2,3],"a":1}"#).unwrap();
        assert_eq!(canonicalize(&v1).unwrap(), canonicalize(&v2).unwrap());
    }
}