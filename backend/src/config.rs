use axum::http::{HeaderValue, Uri};
use std::{env, io};

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
