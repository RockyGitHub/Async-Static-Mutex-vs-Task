pub mod static_lock;
pub mod task_db;
pub mod results_displayer;

const CLIENT_THREAD_COUNT: usize = 100;
fn main() {
    static_lock::run(CLIENT_THREAD_COUNT);
    //task_db::run(CLIENT_THREAD_COUNT);
}
