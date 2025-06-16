mod cli;
mod recorder;
mod gui;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = cli::parse_cli();

    match command {
        cli::Command::Start => {
            recorder::start_recording(30)?;
        }
        cli::Command::Stop => {
            recorder::stop_recording()?;
        }
        cli::Command::Init=>{
            gui::launch_gui()?;  
        }
    }

    Ok(())
}
