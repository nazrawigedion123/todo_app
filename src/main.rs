// todo_cli/src/cmd/main.rs
use todo_app::{cli::App, repository::RepositoryConfig};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = RepositoryConfig::Sqllite {
        path: "todo.db".to_string(),
    };
    let mut app = App::new(config);
    // if let Err(e) = {
    //     eprint!("Error {e}");
    //     std::process::exit(1);
    // }
    app.run()
}
