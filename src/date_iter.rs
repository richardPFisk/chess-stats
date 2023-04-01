use chrono::{DateTime, Datelike, Duration, Months, NaiveDate, TimeZone, Utc};

pub struct MonthIter<T: TimeZone> {
    start: DateTime<T>,
    end: DateTime<T>,
}

impl<T> MonthIter<T>
where
    T: TimeZone,
{
    pub fn new(start: DateTime<T>, end: DateTime<T>) -> MonthIter<T> {
        MonthIter {
            start: start,
            end: end,
        }
    }
}

impl<T> Iterator for MonthIter<T>
where
    T: TimeZone,
{
    type Item = DateTime<T>;
    fn next(&mut self) -> Option<DateTime<T>> {
        if self.start.gt(&self.end) {
            None
        } else {
            match self.start.clone().checked_add_months(Months::new(1)) {
                Some(d) => {
                    self.start = d.clone();
                    Some(d.clone())
                }
                _ => None,
            }
        }
    }
}

pub fn get_all_month_years_from_now(
    earliest_date_time: DateTime<Utc>,
    latest_date_time_option: Option<DateTime<Utc>>,
) -> Vec<(u32, i32)> {
    let latest_date_time = latest_date_time_option.unwrap_or(Utc::now());

    let month_iter = MonthIter::new(earliest_date_time, latest_date_time);
    month_iter
        .into_iter()
        .fold(vec![], |mut month_years, date| {
            month_years.push((date.month(), date.year()));
            month_years
        })
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_month_years() {
        let now_str = "2023-04-01T23:59:60.234567+05:00";
        let now_date = DateTime::<Utc>::from_str(now_str).unwrap();
        let str = "2022-03-18T23:59:60.234567+05:00";
        let past_date = DateTime::<Utc>::from_str(str).unwrap();
        let m_y = get_all_month_years_from_now(past_date, Some(now_date));
        assert_eq!(
            m_y,
            [
                (4, 2022),
                (5, 2022),
                (6, 2022),
                (7, 2022),
                (8, 2022),
                (9, 2022),
                (10, 2022),
                (11, 2022),
                (12, 2022),
                (1, 2023),
                (2, 2023),
                (3, 2023),
                (4, 2023)
            ]
            .iter()
            .map(|&(x, y)| (x as u32, y as i32))
            .collect::<Vec<(u32, i32)>>()
        );
    }
}
