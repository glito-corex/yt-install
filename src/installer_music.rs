use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn main(ytdlp: bool) {
    let _ = Command::new("cmd").args(["/c", "cls"]).status();

    let dossier_application = if let Ok(exe_path) = env::current_exe() {
        exe_path.parent().unwrap().to_path_buf()
    } else {
        env::current_dir().unwrap()
    };

    let ytdlp_path = dossier_application.join("lib").join("Runtime-yt-dlp.exe");

    let home_dir = dirs::home_dir().expect("Impossible de trouver le dossier utilisateur");
    let chm = home_dir.join("Downloads").join("video");
    let _ = fs::create_dir_all(&chm);
    
    let output_template = chm.join("%(title)s.%(ext)s");
    let output_template_str = output_template.to_str().unwrap();

    loop {
        let _ = Command::new("cmd").args(["/c", "cls"]).status();
        println!("Return to home = 0\n   - Download sounds -\n");
        
        print!("URL : ");
        io::stdout().flush().unwrap();

        let mut url = String::new();
        io::stdin().read_line(&mut url).unwrap();
        let url = url.trim();

        if url == "0" {
            let _ = crate::menu::menu();
            return;
        }

        if !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("www.") {
            println!("Invalid URL");
            thread::sleep(Duration::from_secs(2));
            let _ = Command::new("cmd").args(["/c", "cls"]).status();
            continue;
        }

        if ytdlp {
            let status = Command::new(&ytdlp_path)
                .args([
                    "--no-playlist",
                    "-f",
                    "bestaudio",
                    "-o",
                    output_template_str,
                    "--force-overwrites", // Force l'écrasement si le fichier existe déjà
                    url,
                ])
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .status();

            println!("Downloading...");

            match status {
                Ok(s) if s.success() => {}
                _ => {
                    let _ = Command::new("cmd").args(["/c", "cls"]).status();
                    println!("Error: Unable to install with yt-dlp");
                    thread::sleep(Duration::from_secs(1));
                    
                    print!("Press Enter...");
                    let _ = io::stdout().flush();
                    let mut dummy = String::new();
                    let _ = io::stdin().read_line(&mut dummy);

                    let _ = crate::menu::menu();
                    return;
                }
            }
        }
        println!("\nDone. Press Enter to continue...");
        let _ = io::stdout().flush();
        let mut dummy = String::new();
        let _ = io::stdin().read_line(&mut dummy);
        let _ = Command::new("cmd").args(["/c", "cls"]).status();
    }
}