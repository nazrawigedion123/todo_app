// todo_cli/src/cmd/main.rs
use todo_app::cli::run;
fn main() {
    if let Err(e) = run() {
        eprint!("Error {e}");
        std::process::exit(1);
    }

}
