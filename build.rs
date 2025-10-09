fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/img/icon.ico");
        res.compile().expect("Failed to compile resource");
    }
}
