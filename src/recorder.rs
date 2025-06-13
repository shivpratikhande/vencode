use std::fs;
use std::process::{Command, Stdio};

const PID_FILE: &str = "/tmp/vencode_ffmpeg.pid";

pub fn start_recording() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let output = format!("{}/Videos/recording.mp4", home);
    let framerate = 30;

    println!("Starting screen recording to '{}' at {} FPS...", output, framerate);

    let child = Command::new("ffmpeg")
        .args([
            "-f", "x11grab",
            "-r", &framerate.to_string(),
            "-i", ":0.0",
            "-vcodec", "libx264",
            "-preset", "ultrafast",
            &output,
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start ffmpeg");

    fs::write(PID_FILE, child.id().to_string())?;
    println!("Recording started with PID {}.", child.id());

    Ok(())
}

pub fn stop_recording() -> Result<(), Box<dyn std::error::Error>> {
    println!("Stopping screen recording...");
    if let Ok(pid_str) = fs::read_to_string(PID_FILE) {
        let pid = pid_str.trim().parse::<u32>()?;
        let _ = Command::new("kill")
            .arg("-2") // SIGINT
            .arg(pid.to_string())
            .status();
        fs::remove_file(PID_FILE).ok();
        println!("Recording stopped.");
    } else {
        println!("No active recording found.");
    }
    Ok(())
}
