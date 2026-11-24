use std::io::{self, Write};
use std::process::Command;
use std::thread;
use std::time::Duration;

pub fn menu() -> io::Result<()> {
    let mut ixd = false;
    let mut ytdlp = true;
    let mut vlsond = true;
    let url_site = "https://corexinstitut.netlify.app";
    let _ = Command::new("cmd").args(["/c", "color 07"]).status();
    let _ = Command::new("cmd").args(["/c", "cls"]).status();

    println!(
        "YT-install\nBy Corex institut\nWebSite -> \x1b[96m\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\\x1b[0m",
        url_site, url_site
    );
    println!("----------------------------------------------------");
    thread::sleep(Duration::from_millis(1500));

    loop {
        let _ = Command::new("cmd").args(["/c", "color 07"]).status();
        let _ = Command::new("cmd").args(["/c", "cls"]).status();
        let _ = Command::new("cmd")
            .args(["/c", "title YT-Install -- Corex institut"])
            .status();

        if vlsond {
            println!("1 : For the sounds (than the sound)\x1b[0m\n");
        } else {
            println!("\x1b[90m1 not available\x1b[0m\n");
        }
        println!("\x1b[32m2 : For the videos (video and sound)\x1b[0m\n\n\x1b[35mlib : to switch libraries\x1b[0m\n\n\x1b[36m0: exit\x1b[0m\n");

        print!("\x1b[33m-> !\x1b[0m");
        io::stdout().flush()?;

        let mut choix = String::new();
        io::stdin().read_line(&mut choix)?;
        let choix = choix.trim();

        match choix {
            "1" => {
                if vlsond {
                    crate::installer_music::main(ytdlp);
                    break;
                } else {
                    println!("You cannot use the audio option with IXD, use the video option instead");
                    thread::sleep(Duration::from_millis(2500));
                }
            }
            "2" => {
                crate::installer_video::main(ytdlp, ixd);
                break;
            }
            "lib" => {
                let _ = Command::new("cmd").args(["/c", "color 07"]).status();
                let _ = Command::new("cmd").args(["/c", "cls"]).status();
                println!("\x1b[31mChoose the library you want to use\x1b[0m\n");
                println!("1 : yt-dlp Extractor\n2 : IXD Extractor\n0 : return to home\n");

                print!("-> !");
                io::stdout().flush()?;

                let mut lbchoix = String::new();
                io::stdin().read_line(&mut lbchoix)?;
                let lbchoix = lbchoix.trim();

                match lbchoix {
                    "1" => {
                        println!("\x1b[31mYou have chosen : yt-dlp Extractor\x1b[0m");
                        ytdlp = true;
                        ixd = false;
                        vlsond = true;
                        thread::sleep(Duration::from_millis(2500));
                    }
                    "2" => {
                        println!("\x1b[33mYou have chosen : IXD Extractor\x1b[0m");
                        ytdlp = false;
                        ixd = true;
                        vlsond = false;
                        thread::sleep(Duration::from_millis(2500));
                        let _ = Command::new("cmd").args(["/c", "color 07"]).status();
                        let _ = Command::new("cmd").args(["/c", "cls"]).status();
                    }
                    "0" => {
                        println!("Return to home");
                        thread::sleep(Duration::from_millis(2500));
                        let _ = Command::new("cmd").args(["/c", "color 07"]).status();
                        let _ = Command::new("cmd").args(["/c", "cls"]).status();
                    }
                    _ => {
                        println!("Choose a valid option");
                        thread::sleep(Duration::from_millis(2500));
                        let _ = Command::new("cmd").args(["/c", "color 07"]).status();
                        let _ = Command::new("cmd").args(["/c", "cls"]).status();
                    }
                }
            }
            "0" => {
                let _ = Command::new("cmd").args(["/c", "color 07"]).status();
                let _ = Command::new("cmd").args(["/c", "cls"]).status();
                println!("exit");
                thread::sleep(Duration::from_millis(1000));
                std::process::exit(0);
            }
            _ => {
                println!("Choose a valid option");
                thread::sleep(Duration::from_secs(5));
                let _ = Command::new("cmd").args(["/c", "color 07"]).status();
                let _ = Command::new("cmd").args(["/c", "cls"]).status();
            }
        }
    }

    Ok(())
}