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
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RRuleError {
    #[error("Invalid RRULE format: {0}")]
    InvalidFormat(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Unsupported frequency: {0}")]
    UnsupportedFrequency(String),
    #[error("Invalid interval: {0}")]
    InvalidInterval(String),
    #[error("Invalid day: {0}")]
    InvalidDay(String),
    #[error("Invalid count: {0}")]
    InvalidCount(String),
    #[error("Invalid until date: {0}")]
    InvalidUntilDate(String),
}
impl RRule {
    pub fn parse(rrule_str: Option<String>) -> Result<Option<Self>, RRuleError> {
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

        for part in rrule.split(';') {
            let mut split = part.split('=');
            let key = split.next().ok_or_else(|| {
                RRuleError::InvalidFormat("Missing key in RRULE part".to_string())
            })?;
            let value = split.next().ok_or_else(|| {
                RRuleError::InvalidFormat(format!("Missing value for key: {}", key))
            })?;

            match key {
                "FREQ" => {
                    freq = Some(match value {
                        "DAILY" => Frequency::Daily,
                        "WEEKLY" => Frequency::Weekly,
                        "MONTHLY" => Frequency::Monthly,
                        "YEARLY" => Frequency::Yearly,
                        _ => return Err(RRuleError::UnsupportedFrequency(value.to_string())),
                    });
                }
                "INTERVAL" => {
                    interval = value
                        .parse()
                        .map_err(|_| RRuleError::InvalidInterval(value.to_string()))?;
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
                            _ => Err(RRuleError::InvalidDay(day.to_string())),
                        })
                        .collect::<Result<Vec<_>, _>>()?;
                    by_day = Some(days);
                }
                "COUNT" => {
                    count = Some(
                        value
                            .parse()
                            .map_err(|_| RRuleError::InvalidCount(value.to_string()))?,
                    );
                }
                "UNTIL" => {
                    // Support du format iCal compact: 20250925T000000Z
                    until = Some(
                        DateTime::parse_from_str(&format!("{}+00:00", value), "%Y%m%dT%H%M%SZ%z")
                            .or_else(|_| {
                                // Fallback vers RFC3339 pour rétrocompatibilité
                                DateTime::parse_from_rfc3339(value)
                            })
                            .map_err(|_| RRuleError::InvalidUntilDate(value.to_string()))?
                            .with_timezone(&Utc),
                    );
                }
                _ => {} // Ignore unknown fields for now
            }
        }

        let freq = freq.ok_or_else(|| RRuleError::MissingField("FREQ".to_string()))?;

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
        dtstart: DateTime<Utc>,
        range_start: DateTime<Utc>,
        range_end: DateTime<Utc>,
    ) -> Vec<DateTime<Utc>> {
        let mut occurrences = Vec::new();

        match &self.freq {
            Frequency::Weekly if self.by_day.is_some() => {
                // Special handling for weekly events with BYDAY
                if let Some(ref days) = self.by_day {
                    let mut week_start =
                        dtstart - Duration::days(dtstart.weekday().num_days_from_monday() as i64);
                    let mut week_count = 0;
                    let mut occurrence_count = 0;

                    // Fast-forward to the first week that could contain occurrences in range
                    if week_start < range_start {
                        let weeks_diff = (range_start - week_start).num_weeks();
                        let intervals_to_skip = weeks_diff / self.interval as i64;
                        week_start += Duration::weeks(intervals_to_skip * self.interval as i64);
                        week_count = intervals_to_skip;
                    }

                    while week_start <= range_end {
                        // Check count limit
                        if let Some(max_count) = self.count
                            && occurrence_count >= max_count
                        {
                            break;
                        }

                        // Check until limit
                        if let Some(until) = self.until
                            && week_start > until
                        {
                            break;
                        }

                        // Generate occurrences for this week
                        for &day in days {
                            let occurrence =
                                week_start + Duration::days(day.num_days_from_monday() as i64);

                            // Check all constraints
                            if occurrence >= dtstart
                                && occurrence >= range_start
                                && occurrence <= range_end
                            {
                                if let Some(until) = self.until
                                    && occurrence > until
                                {
                                    continue;
                                }

                                if let Some(max_count) = self.count
                                    && occurrence_count >= max_count
                                {
                                    break;
                                }

                                occurrences.push(occurrence);
                                occurrence_count += 1;
                            }
                        }

                        // Move to next week
                        week_start += Duration::weeks(self.interval as i64);
                        week_count += 1;

                        // Safety break
                        if week_count > 1000 {
                            break;
                        }
                    }
                }
            }
            _ => {
                // Original logic for non-weekly-BYDAY events
                let mut current = dtstart;
                let mut count = 0;

                // Fast-forward to the first occurrence that could be in range
                if current < range_start {
                    current = self.fast_forward_to_range(dtstart, range_start);
                }

                while current <= range_end {
                    // Check count limit
                    if let Some(max_count) = self.count
                        && count >= max_count
                    {
                        break;
                    }

                    // Check until limit
                    if let Some(until) = self.until
                        && current > until
                    {
                        break;
                    }

                    // If in range, add occurrence
                    if current >= range_start {
                        occurrences.push(current);
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
                        Frequency::Yearly => current + Duration::days(365 * self.interval as i64), /* Rough */
                    };

                    count += 1;

                    // Safety break
                    if count > 1000 {
                        break;
                    }
                }
            }
        }

        occurrences
    }

    fn fast_forward_to_range(
        &self,
        dtstart: DateTime<Utc>,
        range_start: DateTime<Utc>,
    ) -> DateTime<Utc> {
        if dtstart >= range_start {
            return dtstart;
        }

        match self.freq {
            Frequency::Daily => {
                let days_diff = (range_start - dtstart).num_days();
                let intervals_to_skip = days_diff / self.interval as i64;
                dtstart + Duration::days(intervals_to_skip * self.interval as i64)
            }
            Frequency::Weekly => {
                let weeks_diff = (range_start - dtstart).num_weeks();
                let intervals_to_skip = weeks_diff / self.interval as i64;
                dtstart + Duration::weeks(intervals_to_skip * self.interval as i64)
            }
            Frequency::Monthly => {
                let current = dtstart;
                let months_diff = (range_start.year() - dtstart.year()) * 12
                    + (range_start.month() as i32 - dtstart.month() as i32);
                let intervals_to_skip = months_diff / self.interval;

                let mut year = current.year();
                let mut month = current.month() as i32 + (intervals_to_skip * self.interval);

                while month > 12 {
                    year += 1;
                    month -= 12;
                }
                while month < 1 {
                    year -= 1;
                    month += 12;
                }

                current
                    .with_year(year)
                    .unwrap()
                    .with_month(month as u32)
                    .unwrap()
            }
            Frequency::Yearly => {
                let years_diff = range_start.year() - dtstart.year();
                let intervals_to_skip = years_diff / self.interval;
                let target_year = dtstart.year() + (intervals_to_skip * self.interval);
                dtstart.with_year(target_year).unwrap()
            }
        }
    }
}
