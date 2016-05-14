pub mod update;

use vcs::Repo;

trait Cmd {
    fn run(repos: Vec<Box<Repo>>) -> Box<Fn() -> ()>;
}
