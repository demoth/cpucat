use std::io;
use std::io::{Read, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use nix::sys::sysinfo::{sysinfo, SysInfo};

extern crate num_cpus;

fn main() -> io::Result<()> {
    let cpu_count = num_cpus::get();

    let mut buffer = [0; 1024];
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Set the color of the output to red
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;

    // Read input into the buffer in a loop until we reach the end of the input stream
    loop {
        let bytes_read = io::stdin().read(&mut buffer)?;

        // If we didn't read any bytes, we've reached the end of the input
        if bytes_read == 0 {
            break;
        }

        // Get the current system information
        let sys_info: SysInfo = sysinfo().unwrap();
        // Calculate the current (1 minute average) CPU load as a percentage
        let cpu_load = sys_info.load_average().0 as f32 / cpu_count as f32 * 100.0;

        // Calculate the color for the current CPU load using a gradient function
        let color = gradient(Color::Red, Color::Green, cpu_load / 100.0);
        // Set the color of the output based on the CPU load
        stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
        // Write the bytes we read to standard output
        let _ = io::stdout().write(&buffer[..bytes_read])?;
    }

    Ok(())
}

// Interpolate between two colors using a linear gradient
fn gradient(start: Color, end: Color, t: f32) -> Color {
    let start_r = start.r as f32 / 255.0;
    let start_g = start.g as f32 / 255.0;
    let start_b = start.b as f32 / 255.0;
    let end_r = end.r as f32 / 255.0;
    let end_g = end.g as f32 / 255.0;
    let end_b = end.b as f32 / 255.0;
    let r = (1.0 - t) * start_r + t * end_r;
    let g = (1.0 - t) * start_g + t * end_g;
    let b = (1.0 - t) * start_b + t * end_b;
    Color::RGB(r as u8, g as u8, b as u8)
}
