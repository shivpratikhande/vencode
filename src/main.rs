mod cli;
mod recorder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = cli::parse_cli();

    match command {
        cli::Command::Start => {
            recorder::start_recording()?;
        }
        cli::Command::Stop => {
            recorder::stop_recording()?;
        }
    }

    Ok(())
}
