use tokio::time::{sleep, Duration};
use rand::RngExt;

#[tokio::main]
async fn main() {
    // Create a random number generator
    let mut rng = rand::rng(); 
    
    // Generate a random number of milliseconds between 1000 and 3000
    let duration = rng.random_range(1000..=3000);

    let task_a = async {
        sleep(Duration::from_millis(duration)).await;
        println!("A done");
    };

    let task_b = async {
        sleep(Duration::from_millis(duration)).await;
        println!("B done");
    };

    // This runs both futures concurrently on the current thread
    tokio::join!(task_a, task_b);
}