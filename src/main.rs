use std::io::{self, Read};
use anyhow::{Result, Context};

mod analyzer;
mod gui;

fn main() -> Result<()> {
    // Check if we're receiving piped input
    let is_piped = !is_tty();

    if is_piped {
        analyze_stdin()
    } else {
        gui::run_gui().map_err(|e| anyhow::anyhow!("GUI error: {}", e))
    }
}

fn analyze_stdin() -> Result<()> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)
        .context("Failed to read from stdin")?;

    if content.trim().is_empty() {
        return Ok(());
    }

    let analysis_result = analyzer::analyze_content(&content)?;

    if analysis_result.is_safe {
        println!("SAFE");
        std::process::exit(0);
    } else {
        for threat in &analysis_result.threats {
            eprintln!("THREAT: {}", threat);
        }
        std::process::exit(1);
    }
}


fn is_tty() -> bool {
    #[cfg(unix)]
    {
        unsafe { libc::isatty(libc::STDIN_FILENO) != 0 }
    }
    #[cfg(not(unix))]
    {
        false
    }
} 