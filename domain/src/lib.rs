use std::collections::btree_set::Difference;
use std::collections::HashMap;
use std::os::raw;

use chrono::prelude::*;
use chrono::Days;
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct EngineerIdentifier {
    pub value: Uuid,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Engineer {
    name: String,
    identifier: EngineerIdentifier,
    last_time_served: AppDate,
    today_strategy: TodayStrategy,
}

impl Engineer {
    pub fn new(name: &str, identifier: EngineerIdentifier, last_time_served: AppDate) -> Self {
        Engineer {
            name: name.to_string(),
            identifier: identifier,
            last_time_served: last_time_served,
            today_strategy: TodayStrategy::OSDate,
        }
    }

    pub fn identifier(&self) -> EngineerIdentifier {
        self.identifier.clone()
    }

    pub fn support_days_for_month(self, month: Month) -> Vec<AppDate> {
        todo!()
    }

    pub fn serve_support(self) -> Result<Self, DomainError> {
        todo!()
    }

    pub fn last_time_served(&self) -> AppDate {
        self.last_time_served.clone()
    }
}

pub enum Month {}

#[derive(Clone, Hash, PartialEq, Eq)]
enum TodayStrategy {
    OSDate,
    Thursday,
    Friday,
    Weekend,
}

impl TodayStrategy {
    pub fn execute(&self) -> AppDate {
        match self {
            Self::OSDate => AppDate::new(Utc::now().date_naive()),
            Self::Thursday => AppDate::new(
                Utc.with_ymd_and_hms(2022, 12, 15, 10, 0, 0)
                    .unwrap()
                    .date_naive(),
            ),
            Self::Friday => AppDate::new(
                Utc.with_ymd_and_hms(2022, 12, 16, 10, 0, 0)
                    .unwrap()
                    .date_naive(),
            ),
            Self::Weekend => AppDate::new(
                Utc.with_ymd_and_hms(2022, 12, 18, 10, 0, 0)
                    .unwrap()
                    .date_naive(),
            ),
        }
    }
}

pub struct EngineeringDepartment {
    engineer_serving_support: Option<Engineer>,
    engineers_by_date: HashMap<AppDate, Engineer>,
    engineers_list: Vec<Engineer>,
}

impl EngineeringDepartment {
    pub fn new(engineers: Vec<Engineer>, engineer_serving_support: Option<Engineer>) -> Self {
        Self {
            engineers_by_date: Self::engineers_by_date(&engineers),
            engineer_serving_support: engineer_serving_support,
            engineers_list: engineers,
        }
    }

    pub fn mark_support_service_for_engineer(
        _eng: Engineer,
    ) -> Result<EngineeringDepartment, DomainError> {
        todo!()
    }

    pub fn engineer_serving_on_date(self, date: AppDate) -> Result<Engineer, DomainError> {
        let previous_date_when_engineer_served =
            date.previous_date_when_engineer_served(self.rota_length_in_days())?;
        self.engineers_by_date
            .get(&previous_date_when_engineer_served)
            .map_or_else(
                || Err(DomainError::no_engineer_found()),
                |e| Ok(e.clone()),
            )
    }

    pub fn calendar(period: Period) -> Calendar {
        todo!()
    }

    fn engineers_by_date(engineers: &Vec<Engineer>) -> HashMap<AppDate, Engineer> {
        engineers
            .to_vec()
            .into_iter()
            .map(|e| (e.clone().last_time_served(), e))
            .collect::<HashMap<AppDate, Engineer>>()
    }

    fn rota_length_in_days(&self) -> i64 {
        let number_of_engineers = self.engineers_list.len() as i64;
        number_of_engineers / 5 * 2 + number_of_engineers
    }
}

pub struct Calendar {
    dates: Vec<AppDate>,
}

impl Calendar {
    pub fn new(dates: Vec<AppDate>) -> Self {
        Self { dates: dates }
    }
}

pub struct Period {
    engineer_identifier: EngineerIdentifier,
    month: Month,
    year: Year,
}

impl Period {
    pub fn new(engineer_identifier: EngineerIdentifier, month: Month, year: Year) -> Self {
        Self {
            engineer_identifier: engineer_identifier,
            month: month,
            year: year,
        }
    }
}

pub struct Year {
    value: u16,
}

impl Year {
    pub fn new(value: u16) -> Self {
        Self { value: value }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct AppDate {
    value: NaiveDate,
}

impl AppDate {
    pub fn new(value: NaiveDate) -> Self {
        Self { value: value }
    }

    pub fn is_business_day(&self) -> bool {
        self.value.weekday() != Weekday::Sat && self.value.weekday() != Weekday::Sun
    }

    pub fn previous_date_when_engineer_served(
        self,
        rota_length_in_days: i64,
    ) -> Result<AppDate, DomainError> {
        let number_of_days_to_go_back = self.number_of_days_to_go_back(rota_length_in_days)?;
        self.go_back_to_nearest_business_day(number_of_days_to_go_back)
    }

    fn number_of_days_to_go_back(&self, rota_length_in_days: i64) -> Result<i64, DomainError> {
        let number_of_days_from_today = self.number_of_days_from_today()?;
        let number_of_days_to_complete_rota = number_of_days_from_today % rota_length_in_days;
        Ok(rota_length_in_days - number_of_days_to_complete_rota)
    }

    fn go_back_to_nearest_business_day(
        self,
        number_of_days_to_go_back: i64,
    ) -> Result<AppDate, DomainError> {
        self.go_back_to_nearest_business_day_opt(number_of_days_to_go_back)
            .map_or_else(|| Err(DomainError::date_is_out_of_range()), |date| Ok(date))
    }

    fn go_back_to_nearest_business_day_opt(
        self,
        number_of_days_to_go_back: i64,
    ) -> Option<AppDate> {
        let date = self.go_back_n_days(number_of_days_to_go_back)?;
        if date.is_business_day() {
            Some(date)
        } else {
            Some(date.go_back_n_days(2)?)
        }
    }

    fn go_back_n_days(self, number_of_days_to_go_back: i64) -> Option<AppDate> {
        let date = self
            .value
            .checked_sub_days(Days::new(number_of_days_to_go_back as u64))?;
        Some(AppDate::new(date))
    }

    fn number_of_days_from_today(&self) -> Result<i64, DomainError> {
        let days_delta = (self.value - chrono::Utc::now().date_naive()).num_days();
        if days_delta > 0 {
            Ok(days_delta)
        } else {
            Err(DomainError {
                message: String::from("Can't tell you who served in the past!"),
            })
        }
    }
}

#[derive(Debug)]
pub struct DomainError {
    pub message: String,
}

impl DomainError {
    pub fn date_is_out_of_range() -> Self {
        Self {
            message: String::from("date is out of range"),
        }
    }

    pub fn no_engineer_found() -> Self { 
        Self { 
            message: String::from("no engineer found")
        }
    }
}

#[cfg(test)]
mod tests {}
