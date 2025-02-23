pub struct SM2Calculator {
    min_ease_factor: f64,
    max_ease_factor: f64,
}

impl Default for SM2Calculator {
    fn default() -> Self {
        Self {
            min_ease_factor: 1.3,
            max_ease_factor: 5.0,
        }
    }
}

impl SM2Calculator {
    pub fn calculate_next_review(
        &self,
        quality: i32,
        current_ease: f64,
        current_interval: i32,
        review_count: i32,
    ) -> (f64, i32, i32) {
        let quality = quality as f64;
        
        // Calculate new ease factor
        let new_ease = (current_ease + (0.1 - (5.0 - quality) * (0.08 + (5.0 - quality) * 0.02)))
            .clamp(self.min_ease_factor, self.max_ease_factor);

        // Determine next interval and review count
        let (new_review_count, new_interval) = if quality < 3.0 {
            (0, 1) // Reset progress for poor recall
        } else {
            match review_count {
                0 => (1, 1),      // First successful review
                1 => (2, 6),      // Second successful review
                _ => {            // Subsequent reviews
                    let interval = (current_interval as f64 * new_ease).round() as i32;
                    (review_count + 1, interval)
                }
            }
        };

        (new_ease, new_interval, new_review_count)
    }
}
