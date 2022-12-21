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

        // Set the color of the output based on the CPU load
        if cpu_load > 75.0 {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Red)))?;
        } else if cpu_load > 50.0 {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
        } else {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green)))?;
        }
        print!("cpu load: {}", cpu_load);
        // Write the bytes we read to standard output
        let _ = io::stdout().write(&buffer[..bytes_read])?;
    }

    Ok(())
}
