use chrono::{DateTime, Datelike, Duration, Utc, Weekday};

#[derive(Debug, Clone)]
pub struct RRule {
    pub freq: Frequency,
    pub interval: i32,
    pub by_day: Option<Vec<Weekday>>,
    pub count: Option<i32>,
    pub until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl RRule {
    pub fn parse(rrule_str: Option<String>) -> Result<Option<Self>, String> {
        let Some(rrule) = rrule_str else {
            return Ok(None);
        };

        if rrule.is_empty() {
            return Ok(None);
        }

        let mut freq = None;
        let mut interval = 1;
        let mut by_day = None;
        let mut count = None;
        let mut until = None;

        // Parse "FREQ=WEEKLY;INTERVAL=2;BYDAY=MO,WE,FR;COUNT=10"
        for part in rrule.split(';') {
            let mut split = part.split('=');
            let key = split.next().ok_or("Invalid RRULE format")?;
            let value = split.next().ok_or("Invalid RRULE format")?;

            match key {
                "FREQ" => {
                    freq = Some(match value {
                        "DAILY" => Frequency::Daily,
                        "WEEKLY" => Frequency::Weekly,
                        "MONTHLY" => Frequency::Monthly,
                        "YEARLY" => Frequency::Yearly,
                        _ => return Err(format!("Unsupported frequency: {}", value)),
                    });
                }
                "INTERVAL" => {
                    interval = value.parse().map_err(|_| "Invalid interval")?;
                }
                "BYDAY" => {
                    let days = value
                        .split(',')
                        .map(|day| match day {
                            "MO" => Ok(Weekday::Mon),
                            "TU" => Ok(Weekday::Tue),
                            "WE" => Ok(Weekday::Wed),
                            "TH" => Ok(Weekday::Thu),
                            "FR" => Ok(Weekday::Fri),
                            "SA" => Ok(Weekday::Sat),
                            "SU" => Ok(Weekday::Sun),
                            _ => Err(format!("Invalid day: {}", day)),
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    by_day = Some(days);
                }
                "COUNT" => {
                    count = Some(value.parse().map_err(|_| "Invalid count")?);
                }
                "UNTIL" => {
                    until = Some(
                        DateTime::parse_from_rfc3339(value)
                            .map_err(|_| "Invalid until date")?
                            .with_timezone(&Utc),
                    );
                }
                _ => {} // Ignore unknown fields for now
            }
        }

        let freq = freq.ok_or("FREQ is required")?;

        Ok(Some(RRule {
            freq,
            interval,
            by_day,
            count,
            until,
        }))
    }

    pub fn generate_occurrences(
        &self,
        start: DateTime<Utc>,
        range_start: DateTime<Utc>,
        range_end: DateTime<Utc>,
    ) -> Vec<DateTime<Utc>> {
        let mut occurrences = Vec::new();
        let mut current = start;
        let mut count = 0;

        while current <= range_end {
            // Check count limit
            if let Some(max_count) = self.count {
                if count >= max_count {
                    break;
                }
            }

            // Check until limit
            if let Some(until) = self.until {
                if current > until {
                    break;
                }
            }

            // If in range, add occurrence
            if current >= range_start {
                match &self.freq {
                    Frequency::Weekly if self.by_day.is_some() => {
                        // Handle BYDAY for weekly events
                        if let Some(ref days) = self.by_day {
                            let week_start = current
                                - Duration::days(current.weekday().num_days_from_monday() as i64);

                            for &day in days {
                                let occurrence =
                                    week_start + Duration::days(day.num_days_from_monday() as i64);
                                if occurrence >= range_start && occurrence <= range_end {
                                    occurrences.push(occurrence);
                                }
                            }
                        }
                    }
                    _ => {
                        occurrences.push(current);
                    }
                }
            }

            // Move to next occurrence
            current = match self.freq {
                Frequency::Daily => current + Duration::days(self.interval as i64),
                Frequency::Weekly => current + Duration::weeks(self.interval as i64),
                Frequency::Monthly => {
                    // This is tricky - month arithmetic
                    let mut year = current.year();
                    let mut month = current.month() as i32 + self.interval;

                    while month > 12 {
                        year += 1;
                        month -= 12;
                    }

                    current
                        .with_year(year)
                        .unwrap()
                        .with_month(month as u32)
                        .unwrap()
                }
                Frequency::Yearly => current + Duration::days(365 * self.interval as i64), // Rough
            };

            count += 1;

            // Safety break
            if count > 1000 {
                break;
            }
        }

        occurrences
    }
}
