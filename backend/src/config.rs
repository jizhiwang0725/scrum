use axum::http::{HeaderValue, Uri, uri::Authority};
use std::{
    env, io,
    net::{Ipv4Addr, Ipv6Addr},
};

pub fn frontend_origins() -> io::Result<Vec<HeaderValue>> {
    // Use our local frontend addresses unless a custom list is provided.
    let value = match env::var("FRONTEND_ORIGINS") {
        Ok(value) => value,
        Err(env::VarError::NotPresent) => "http://localhost:5173,http://127.0.0.1:5173".to_owned(),
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "FRONTEND_ORIGINS must contain valid Unicode",
            ));
        }
    };

    let invalid = || {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Use frontend origins like http://localhost:5173, without paths or trailing slashes",
        )
    };

    let mut origins = Vec::new();

    for origin in value.split(',').map(str::trim) {
        // Check the address before adding it to the allowed list.
        let uri: Uri = origin.parse().map_err(|_| invalid())?;
        let scheme = uri.scheme_str().ok_or_else(invalid)?;
        let authority = uri.authority().ok_or_else(invalid)?;

        if !matches!(scheme, "http" | "https")
            || authority.host().is_empty()
            || authority.as_str().contains('@')
            || authority.as_str().contains('*')
            || origin != format!("{scheme}://{authority}")
            || !valid_host(authority.host())
            || !valid_port(authority)
        {
            return Err(invalid());
        }

        let header = HeaderValue::from_str(origin).map_err(|_| invalid())?;

        if !origins.contains(&header) {
            origins.push(header);
        }
    }

    Ok(origins)
}

fn valid_host(host: &str) -> bool {
    // IPv6 addresses must be inside square brackets.
    if let Some(address) = host
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return address.parse::<Ipv6Addr>().is_ok();
    }

    // Check numeric addresses as IPv4, so invalid IPs cannot pass as domain names.
    if host
        .bytes()
        .all(|byte| byte.is_ascii_digit() || byte == b'.')
    {
        return host.parse::<Ipv4Addr>().is_ok();
    }

    // Allow localhost and domain names. A final dot is allowed for domain names.
    let name = host.strip_suffix('.').unwrap_or(host);

    if name.is_empty() || name.len() > 253 {
        return false;
    }

    name.split('.').all(|label| {
        let bytes = label.as_bytes();

        // Each part must be 1–63 characters and cannot start or end with a hyphen.
        !bytes.is_empty()
            && bytes.len() <= 63
            && bytes[0].is_ascii_alphanumeric()
            && bytes[bytes.len() - 1].is_ascii_alphanumeric()
            && bytes
                .iter()
                .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
    })
}

fn valid_port(authority: &Authority) -> bool {
    // Read the original text so an invalid port is not mistaken for a missing port.
    let Some(suffix) = authority.as_str().strip_prefix(authority.host()) else {
        return false;
    };

    // Without an explicit port, HTTP or HTTPS uses its default port.
    if suffix.is_empty() {
        return true;
    }

    let Some(port) = suffix.strip_prefix(':') else {
        return false;
    };

    // An explicit frontend port must contain digits and be between 1 and 65535.
    !port.is_empty()
        && port.bytes().all(|byte| byte.is_ascii_digit())
        && port.parse::<u16>().is_ok_and(|number| number > 0)
}
