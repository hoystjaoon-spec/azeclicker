fn main() {
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=assets/branding/aze.ico");
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/branding/aze.ico");
        if let Err(e) = res.compile() {
            println!("cargo:warning=icon embed failed: {e}");
        }
    }
}
