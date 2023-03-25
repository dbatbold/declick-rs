mod wav;

fn main() {
    // Read stdin as a raw WAVE stream
    let mut file = std::io::stdin();

    // Parse WAVE stream header
    let header = match wav::parse_wave_header(&mut file) {
        Err(e) => {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
        Ok(h) => h,
    };

    // Scan audio stream for clicks

    println!("{}", header);
}
