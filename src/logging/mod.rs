pub mod layer;

use tracing::{error, info, instrument, span, Instrument, Level};

/// https://youtu.be/JjItsfqFIdo
pub async fn logging_example() -> Result<(), &'static str> {
    let yaks: Vec<i32> = vec![1, 2, 3, 4, 8, 9, 10, 14, 22, 31];

    let span = span!(Level::INFO, "shaving_yaks", yak_count = yaks.len());
    let _enter = span.enter();

    for yak in yaks {
        // attaches span context to a future (includes argument values, etc..)
        tokio::spawn(shave_yak(yak).instrument(tracing::info_span!("shave_yak")));
    }
    Ok(())
}
// should error when arg is multiples of 7
#[instrument]
async fn shave_yak(yak_number: i32) -> Result<(), String> {
    info!("start shaving {}", yak_number);
    let good_yak = yak_number % 7;
    match good_yak != 0 {
        true => {
            info!("shaved {}", yak_number);
            Ok(())
        }
        false => {
            error!("shaving failed @number: {}, rest: {}", yak_number, good_yak);
            Err(format!("I don't like this number {}", yak_number))
        }
    }
}
