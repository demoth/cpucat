
use std::{
    io::{self, stdin, BufRead, BufReader, Write},
    process::exit,
    sync::mpsc::{sync_channel, TrySendError},
    time::Duration,
};
use sysinfo::{CpuExt, System, SystemExt};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use cpucat::resample;

fn main() -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut buffer = String::new();
    let mut stdin = BufReader::new(stdin());
    let mut last_load = 0.0;

    // with sampling interval of 100 ms this amounts to 1000 seconds of data
    // if it takes so long to produce a line of data, then the cpu load does not really matter
    let (cpu_tx, cpu_rx) = sync_channel(100_000);

    // This thread will provide a stream of cpu measurement values
    std::thread::spawn(move || {
        let mut sys = System::new();
        sys.refresh_cpu();

        loop {
            if let Err(TrySendError::Disconnected(..)) =
                cpu_tx.try_send(sys.global_cpu_info().cpu_usage())
            {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
            sys.refresh_cpu();
        }
    });

    loop {
        match stdin.read_line(&mut buffer) {
            Ok(0) => exit(0),
            Ok(_) => {
                if let Err(e) = print_colored(&cpu_rx, &buffer, &mut stdout, &mut last_load) {
                    println!("Error {e}");
                    exit(1)
                }
                buffer.clear();
            }
            Err(e) => {
                println!("{e}");
                exit(1)
            }
        }
    }
}

fn print_colored(
    cpu_rx: &std::sync::mpsc::Receiver<f32>,
    buffer: &str,
    stdout: &mut StandardStream,
    last_load: &mut f32,
) -> anyhow::Result<()> {
    // So we don't lose cpu load information on empty lines
    if buffer.trim().is_empty() {
        write!(stdout, "{buffer}")?;
    }

    let cpu_load_samples = {
        let mut result = vec![*last_load];
        while let Ok(v) = cpu_rx.try_recv() {
            result.push(v)
        }
        result
    };
    if cpu_load_samples.len() > 1 {
        let chars = buffer.chars().collect::<Vec<_>>();
        *last_load = *cpu_load_samples.last().unwrap();
        let load_per_char = resample(cpu_load_samples, chars.len());

        // FIXME: spaces still lose information, is it ok?
        for (cpu_load, c) in load_per_char.into_iter().zip(chars) {
            let color = get_color(cpu_load / 100.0);
            stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
            stdout.write_all(c.to_string().as_bytes())?;
        }
    } else {
        // last color from the previous calculation still applies
        write!(stdout, "{buffer}")?;
    }

    Ok(())
}

// Interpolate between two colors using a linear gradient
fn get_color(t: f32) -> Color {
    let start_r = 0.0;
    let start_g = 1.0;
    let start_b = 0.0;
    let end_r = 1.0;
    let end_g = 0.0;
    let end_b = 0.0;
    let r = (1.0 - t) * start_r + t * end_r;
    let g = (1.0 - t) * start_g + t * end_g;
    let b = (1.0 - t) * start_b + t * end_b;
    Color::Rgb((255.0 * r) as u8, (255.0 * g) as u8, (255.0 * b) as u8)
}
