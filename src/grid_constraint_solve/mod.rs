pub mod builder;
pub mod grid;
pub mod solver;

use std::time::{Duration, Instant};

use crate::*;

pub const RUN_INVARIANT: bool = false;

#[derive(Derivative)]
#[derivative(Debug)]
#[derive(Clone)]
pub struct Runner {
    pub time_limit: Option<Duration>,
    pub success: Option<bool>,
    pub failure_message: Option<String>,
    #[derivative(Debug = "ignore")]
    pub start_time: Instant,
    #[derivative(Debug = "ignore")]
    pub end_time: Option<Instant>,
    pub time: Option<Duration>,
    pub setup_time: Option<Duration>,
    pub loop_time: Option<Duration>,
    pub return_object_time: Option<Duration>,
    pub remaining_time: Option<Duration>,
}

impl Runner {

    pub fn new(time_limit: Option<Duration>) -> Self {
        Self {
            time_limit,
            success: None,
            failure_message: None,
            start_time: Instant::now(),
            end_time: None,
            time: None,
            setup_time: None,
            loop_time: None,
            return_object_time: None,
            remaining_time: None,
        }
    }

    pub fn check_continue(&mut self) -> bool {
        if self.success.is_some() {
            // We already have a result, either success or failure.
            false
        } else {
            match self.time_limit {
                Some(time_limit) => {
                    if Instant::now() - self.start_time >= time_limit {
                        // We've gone over the time limit so cancel the build.
                        self.success = Some(false);
                        self.failure_message = Some("Exceeded time limit.".to_string());
                        false
                    } else {
                        // We haven't reached the time limit so continue the build.
                        true
                    }
                }
                _ => {
                    // No time limit was specified.
                    true
                }
            }
        }
    }

    pub fn mark_end(&mut self) {
        let now = Instant::now();
        self.end_time = Some(now);
        let duration = now - self.start_time;
        if SHOW_ELAPSED_TIME { dbg!(duration); }
        self.time = Some(duration);

        let mut remaining_time = duration;
        if let Some(t) = self.setup_time {
            remaining_time -= t;
        }
        if let Some(t) = self.loop_time {
            remaining_time -= t;
        }
        if let Some(t) = self.return_object_time {
            remaining_time -= t;
        }
        self.remaining_time = Some(remaining_time);
    }

    pub fn failure_message_clone(&self) -> String {
        return self.failure_message.as_ref().unwrap().clone();
    }

    pub fn times_as_string(&self) -> String {
        let mut s = String::new();
        if let Some(t) = self.time {
            s.push_str(&format!("time = {:?}", t));
        }
        if let Some(t) = self.setup_time {
            s.push_str(&format!(", setup_time = {:?}", t));
        }
        if let Some(t) = self.loop_time {
            s.push_str(&format!(", loop_time = {:?}", t));
        }
        if let Some(t) = self.return_object_time {
            s.push_str(&format!(", return_object_time = {:?}", t));
        }
        if let Some(t) = self.remaining_time {
            s.push_str(&format!(", remaining_time = {:?}", t));
        }
        s
    }
}





