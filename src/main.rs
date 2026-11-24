#[path = "DACL.rs"]
mod dacl;
mod menu;
mod installer_music;
mod installer_video;

use std::env;
use std::io::{self, Read, Write};
use windows_sys::Win32::System::Console::{
    GetStdHandle, GetConsoleMode, SetConsoleMode, SetConsoleTitleW, ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, STD_INPUT_HANDLE
};

fn definir_titre_console() {
    let titre: Vec<u16> = "YT-install\0".encode_utf16().collect();
    unsafe {
        SetConsoleTitleW(titre.as_ptr());
    }
}

fn verifier_exes_console() {
    let dossier_application = if let Ok(exe_path) = env::current_exe() {
        exe_path.parent().unwrap().to_path_buf()
    } else {
        env::current_dir().unwrap()
    };

    let lib_dir = dossier_application.join("lib");
    let ytdlp_path = lib_dir.join("Runtime-yt-dlp.exe");
    let ixd_path = lib_dir.join("Runtime-IXD.exe");
    let ytdlp_existe = ytdlp_path.exists();
    let ixd_existe = ixd_path.exists();

    if ytdlp_existe && ixd_existe {
        return;
    }

    println!("====================================================");
    println!("          CRITICAL ERROR: INCOMPLETE INSTALL        ");
    println!("====================================================");
    println!("Your application installation is incomplete!\n");

    if !ytdlp_existe && !ixd_existe {
        println!("- Missing component: Runtime-yt-dlp.exe");
        println!("- Missing component: Runtime-IXD.exe");
    } else if !ytdlp_existe {
        println!("- Missing component: Runtime-yt-dlp.exe");
    } else {
        println!("- Missing component: Runtime-IXD.exe");
    }

    println!("\nYou must reinstall the package from the official");
    println!("Corex Institut website to restore full functionality.");

    let url = "https://corexinstitut.netlify.app";
    println!("Website -> \x1b[94m\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\x1b[0m", url, url);
    
    println!("====================================================");
    println!("\nPress Enter to exit...");

    let _ = io::stdout().flush();

    unsafe {
        let handle = GetStdHandle(STD_INPUT_HANDLE);
        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) != 0 {
            let new_mode = mode & !(ENABLE_ECHO_INPUT | ENABLE_LINE_INPUT);
            SetConsoleMode(handle, new_mode);
        }
    }

    let mut dummy = [0u8; 1];
    let _ = io::stdin().read(&mut dummy);

    std::process::exit(0);
}

fn main() {
    definir_titre_console();
    verifier_exes_console();

    if let Err(e) = dacl::securiser_processus_dacl() {
        eprintln!("Avertissement de sécurité DACL : {}", e);
    }

    if let Err(e) = menu::menu() {
        eprintln!("Erreur dans le menu : {}", e);
    }
}