use std::time::Duration;

pub fn seconds(seconds : f32) -> Duration {
    Duration::from_secs_f32(seconds)
}

pub fn minutes(minutes : f32) -> Duration {
    seconds(60f32 * minutes)
}

pub fn hours(hours : f32) -> Duration {
    minutes(60f32 * hours)
}

pub fn days(days : f32) -> Duration {
    hours(24f32 * days)
}

pub fn weeks(weeks : f32) -> Duration {
    days(7f32 * weeks)
}
