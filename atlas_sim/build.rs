use std::{env, io};

fn main() -> io::Result<()> {
    // https://stackoverflow.com/a/65393488
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        winresource::WindowsResource::new()
            .set_icon("icon.ico")
            .compile()?;
    }
    Ok(())
}
