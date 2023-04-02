use chrono::{DateTime, Months, TimeZone, Utc};

pub struct MonthIter<T: TimeZone> {
    start: DateTime<T>,
    end: DateTime<T>,
}

impl<T> MonthIter<T>
where
    T: TimeZone,
{
    pub fn new(start: DateTime<T>, end: DateTime<T>) -> MonthIter<T> {
        MonthIter { start, end }
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
                    Some(d)
                }
                _ => None,
            }
        }
    }
}

pub fn get_all_month_years_from_now(
    earliest_date_time: DateTime<Utc>,
    latest_date_time_option: Option<DateTime<Utc>>,
) -> Vec<(String, String)> {
    let latest_date_time =
        latest_date_time_option.unwrap_or(Utc::now().checked_sub_months(Months::new(1)).unwrap());

    let month_iter = MonthIter::new(earliest_date_time, latest_date_time);
    month_iter
        .into_iter()
        .fold(vec![], |mut month_years, date| {
            let month_str = date.format("%m");
            let year_str = date.format("%Y");
            month_years.push((month_str.to_string(), year_str.to_string()));
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
                ("04", "2022"),
                ("05", "2022"),
                ("06", "2022"),
                ("07", "2022"),
                ("08", "2022"),
                ("09", "2022"),
                ("10", "2022"),
                ("11", "2022"),
                ("12", "2022"),
                ("01", "2023"),
                ("02", "2023"),
                ("03", "2023"),
                ("04", "2023")
            ]
            .iter()
            .map(|&(x, y)| (x.into(), y.into()))
            .collect::<Vec<(String, String)>>()
        );
    }
}
