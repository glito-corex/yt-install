fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        // Indiquez ici le nom de votre fichier .ico placé à la racine
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}