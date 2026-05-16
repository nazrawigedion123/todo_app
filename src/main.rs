// todo_cli/src/cmd/main.rs
use todo_app::cli::App;
fn main() ->Result<(), Box<dyn std::error::Error>>{
    let mut app= App::new();
    // if let Err(e) = {
    //     eprint!("Error {e}");
    //     std::process::exit(1);
    // }
    app.run()

}
