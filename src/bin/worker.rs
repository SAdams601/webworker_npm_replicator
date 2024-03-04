use yew_agent::Registrable;
use webworker_npm_replicator::agent::{FibonacciTask, Postcard};

fn main() {
    FibonacciTask::registrar().encoding::<Postcard>().register();
}