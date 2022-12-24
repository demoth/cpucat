use std::time::{Duration, Instant};

/// Generates linearly increasing and then decreasing load
/// Cpu load graph is supposed to look like a step-wise approximation of a triangle wave
fn main() {
    let step_width = Duration::from_millis(200);
    let cpu_num = 24;
    println!(
        "Gradually increasing and then decreasing CPU load, period is {:?}",
        step_width * cpu_num * 2
    );
    loop {
        // Ramp up
        for i in 0..cpu_num {
            let load_duration = step_width * (cpu_num - i) * 2;
            generate_load_in_bg_for(load_duration);
            std::thread::sleep(step_width);
        }

        // Ramp down (threads running to conclusion)
        std::thread::sleep(step_width * cpu_num);

        println!("This was a single load cycle to test how well cpucat can color this line")
    }
}

fn generate_load_in_bg_for(duration: Duration) {
    let start = Instant::now();
    std::thread::spawn(move || {
        while start.elapsed().as_secs_f64().sqrt() < duration.as_secs_f64().sqrt() {}
    });
}
