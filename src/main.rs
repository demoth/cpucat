use std::io;

fn main() -> io::Result<()> {
    let mut buffer = [0; 1024];

    // Read input into the buffer in a loop until we reach the end of the input stream
    loop {
        let bytes_read = io::stdin().read(&mut buffer)?;

        // If we didn't read any bytes, we've reached the end of the input
        if bytes_read == 0 {
            break;
        }

        // Write the bytes we read to standard output
        io::stdout().write_all(&buffer[..bytes_read])?;
    }

    Ok(())
}

