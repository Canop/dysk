#[allow(clippy::match_like_matches_macro)]
fn main() {
    if dysk_cli::run().is_err() {
        std::process::exit(141);
    }
}
