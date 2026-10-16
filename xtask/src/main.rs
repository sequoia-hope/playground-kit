//! `cargo xtask <command>`: the kit's own tasks, from `pk_xtask`.
//! A game's xtask is the same forty lines plus its own commands
//! (`template/xtask`).

fn main() {
    if let Err(e) = pk_xtask::main_with(std::env::args().skip(1), &[]) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
