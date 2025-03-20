use clap::App;

// The program version
const VERSION: &str = env!("CARGO_PKG_VERSION");

// Let the fun begin!
fn main() {
    // Parse the command-line arguments.
    App::new("Gitstorian")
        .version(VERSION)
        .version_short("v")
        .author("Stephan Boyer <stephan@stephanboyer.com>")
        .about("Talk to your Git history.")
        .get_matches();

    // Greet the user.
    println!("Hello, World!");
}
