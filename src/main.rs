use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use axum::extract::State;
use rand::Rng;

#[derive(Clone)]
struct AppState {
    quote_timestamps: Arc<Mutex<HashMap<String, SystemTime>>>,
}

async fn get_quote_handler(State(state): State<AppState>) -> String {
    // next we'll turn this into a quote source so maybe we can build many sources of quotes
    let quote_list = [
        "Thick Goals will change you by achieving them",
        "You're doing this for you in 2 weeks",
        "You can get anything done by locking in for just a couple of weeks",
        "Your future is shaped by the habits you repeat, not the goals you set",
        "A path is formed by walking",
        "Laziness is focussing on the labor and not the reward",
        "Everything seems impossible until it's done",
        "Success is built by what you do daily",
        "Focus on progress, not perfection",
        "The cost of procrastination is the life you could have lived",
        "Focus on the goal, not the struggle",
        "Discipline means showing up long after motivation has left",
        "Follow your plan, not your mood",
        "What you do in private shows in public",
        "What you do today will improve all of your tomorrows",
        "If you can dream, you can do it",
        "If you're going through hell, keep going",
        "It's on you to get to where you want to be",
        "Doubting yourself is normal; letting it stop you is a choice",
        "The mindset you carry shapes the life you create",
        "Everything is hard before it is easy",
        "Knowing is not enough; we must apply. willing is not enough; we must do",
        "Don't rush something you want to last forever",
        "Ready is not a feeling, ready is a decision",
        "Quality is not an act, it's a habit",
        "Change is scary, but so is staying the same",
        "Good things take time",
        "Change your life today, don't gamble on the future; act now, without delay",
        "You can't cross the sea merely by standing and staring at the water",
        "The most effective way to do it is to do it",
        "Allow yourself to be proud of yourself and all the progress you've made",
        "Trust the seeds you are planting",
        "Do not wait to strike until the iron is hot. Make it hot by striking",
        "Small deeds done are better than great deeds planned"
    ];
    
    let list_len = quote_list.len();
    let mut rng = rand::rng();
    let now = SystemTime::now();
    
    for _ in 0..10 {
        let random_index = rng.random_range(0..list_len);
        let random_quote = quote_list[random_index];
        
        // Lock the mutex to check/update state
        let mut timestamps = state.quote_timestamps.lock().unwrap();
        
        if let Some(last_used) = timestamps.get(random_quote) {
            // Check if enough time has passed (5 minutes = 300 seconds)
            if let Ok(elapsed) = now.duration_since(*last_used)
                && elapsed.as_secs() >= 300 {
                    timestamps.insert(random_quote.to_string(), now);
                    return random_quote.to_string();
                }
        } else {
            // Quote has never been used
            timestamps.insert(random_quote.to_string(), now);
            return random_quote.to_string();
        }
    }
    
    // If no quote qualifies, return a random one anyway as fallback
    let random_index = rng.random_range(0..list_len);
    let random_quote = quote_list[random_index];
    
    let mut timestamps = state.quote_timestamps.lock().unwrap();
    timestamps.insert(random_quote.to_string(), now);
    
    random_quote.to_string()
}

#[tokio::main]
async fn main() {
    
    let app_state = AppState {
        quote_timestamps: Arc::new(Mutex::new(HashMap::new())),
    };
    
    let app = axum::Router::new()
        .route("/", axum::routing::get(get_quote_handler))
        .with_state(app_state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
