use clap::{Args, Parser, Subcommand};
use pass::TotpGenerator;

const QR_CODE_NAME: &str = "qr.png";

const ISSUER: &str = "cphovo@gmail.com";

#[derive(Parser)]
#[command(author="cphovo", version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generating Keys
    Generate(GenerateArgs),
    /// Verify identity
    Verify(VerifyArgs),
}

#[derive(Args, Debug)]
struct GenerateArgs {
    /// The signature for identification
    #[arg(short, long)]
    label: Option<String>,

    /// Whether to generate base64 format image, default is PNG
    #[arg(short, long)]
    base64: bool,
}

#[derive(Args, Debug)]
struct VerifyArgs {
    /// The last secret you generated
    secret: String,

    /// The verification code from Google Authenticator
    #[arg(short, long)]
    token: String,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Generate(generate) => {
            let label = generate.label.as_deref().unwrap_or("default");
            let generator = TotpGenerator::new();
            match generator.generate_qr_code(label, ISSUER) {
                Ok(t) => {
                    println!("YOUR SECRET: {}", t.secret);
                    if generate.base64 {
                        println!("data:image/png;base64,{}", t.base64);
                        return;
                    }
                    match pass::save_png(t.png, QR_CODE_NAME) {
                        Ok(_) => {}
                        Err(e) => eprintln!("Error while saving image: {}", e),
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Verify(verify) => {
            let generator = TotpGenerator::from(&verify.secret);
            match generator.verify_token(&verify.token) {
                Ok(_) => println!("SUCCESS"),
                Err(e) => eprintln!("{}", e),
            };
        }
    }
}
