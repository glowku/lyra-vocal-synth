use clap::{Parser, Subcommand};
use lyra::Voice;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "lyra", about = "Lyra vocal synthesizer — offline tools")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Render a sustained vowel (placeholder)
    Render {
        #[command(subcommand)]
        target: RenderTarget,
    },
    /// Show engine info
    Info,
}

#[derive(Subcommand)]
enum RenderTarget {
    Vowel {
        /// Vowel quality (a, e, i, o, u)
        quality: String,
        /// Frequency in Hz
        #[arg(default_value = "220")]
        freq: f32,
        /// Output WAV path
        #[arg(short, long, default_value = "out.wav")]
        out: PathBuf,
        /// Body type
        #[arg(long, default_value = "female")]
        body: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info => {
            println!("Lyra Vocal Synth v0.1.0");
            println!("Physical-modeling engine — every sound from a simulated airway.");
            println!("https://github.com/glowku/lyra-vocal-synth");
        }
        Commands::Render { target } => match target {
            RenderTarget::Vowel {
                quality,
                freq,
                out,
                body,
            } => {
                let mut voice = match body.as_str() {
                    "male" => Voice::new_male(),
                    "child" => {
                        let mut v = Voice::new_female();
                        v.body = lyra::Body::child();
                        v
                    }
                    _ => Voice::new_female(),
                };
                voice.f0 = freq;

                println!(
                    "Rendering vowel /{}/ at {:.1} Hz ({}) → {:?}",
                    quality, freq, body, out
                );
                println!("(Full FDTD + radiation pipeline coming in Phase 0)");

                // Placeholder: just create an empty file for now
                let _ = std::fs::write(&out, b"RIFF....WAVEfmt "); // tiny stub
                println!("Done (stub). Full audio rendering in progress.");
            }
        },
    }
}
