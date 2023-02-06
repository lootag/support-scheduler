use std::collections::HashMap;

use chrono::format::format;
use chrono::Utc;
use chrono::{prelude::*, Days};
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
    pub fn serve_support(self) -> Result<Self, DomainError> {
        todo!()
    }

    fn is_business_day(&self, today: NaiveDate) -> bool {
        today.weekday().num_days_from_monday() <= 4
    }
}

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
    engineers_by_date: HashMap<AppDate, Engineer>,
    dates_by_engineer: HashMap<Engineer, AppDate>,
}

impl EngineeringDepartment {
    pub fn new(engineers: Vec<Engineer>) -> Self {
        todo!()
    }

    pub fn engineers_by_date(engineers: &Vec<Engineer>) -> HashMap<AppDate, Engineer> {
        engineers
            .to_vec()
            .into_iter()
            .map(|e| (e.clone().last_time_served, e))
            .collect::<HashMap<AppDate, Engineer>>()
    }

    pub fn mark_support_service_for_engineer(
        eng: Engineer,
    ) -> Result<EngineeringDepartment, DomainError> {
        todo!()
    }

    pub fn engineer_serving_on_date(self, date: AppDate) -> Engineer {
        todo!()
    }

    fn rota_length_in_days() -> i16 {
        todo!()
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
}

#[derive(Debug)]
pub struct DomainError {
    pub message: String,
}

#[cfg(test)]
mod tests {}
