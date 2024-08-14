use std::{env, fs, io, path};

fn main() -> io::Result<()> {
    // https://stackoverflow.com/a/65393488
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        winresource::WindowsResource::new()
            .set_icon("icon.ico")
            .compile()?;
        let out = get_output_path();
        fs::copy("../climatemap.png", out.join("climatemap.png"))?;
    }
    Ok(())
}

fn get_output_path() -> path::PathBuf {
    // https://stackoverflow.com/a/67516503
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = path::Path::new(&manifest_dir_string)
        .parent()
        .unwrap()
        .join("target")
        .join(build_type);
    return path::PathBuf::from(path);
}
