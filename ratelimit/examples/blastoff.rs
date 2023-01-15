// Copyright 2019 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use ratelimit::Ratelimiter;
use clocksource::{DateTime, SecondsFormat};

fn main() {
    let limiter = Ratelimiter::new(1, 1, 1);
    for i in 0..10 {
        limiter.wait();
        println!(
            "{}: T -{}",
            DateTime::now().to_rfc3339_opts(SecondsFormat::Millis, false),
            10 - i
        );
    }
    limiter.wait();
    println!("Ignition");
}
