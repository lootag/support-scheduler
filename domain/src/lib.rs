use chrono::{prelude::*, Days};
use chrono::{Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct EngineerIdentifier {
    pub value: Uuid,
}

#[derive(Clone)]
pub struct Engineer {
    name: String,
    identifier: EngineerIdentifier,
    day_on_support: Day,
    last_time_served: NaiveDate,
    today_strategy: TodayStrategy,
}

impl Engineer {
    pub fn new(
        name: &str,
        identifier: EngineerIdentifier,
        day_on_support: Day,
        last_time_served: NaiveDate,
    ) -> Self {
        Engineer {
            name: name.to_string(),
            identifier: identifier,
            day_on_support: day_on_support,
            last_time_served: last_time_served,
            today_strategy: TodayStrategy::OSDate,
        }
    }
    pub fn serve_support(self) -> Result<Self, DomainError> {
        let today = self.today_strategy.execute();
        if !self.is_business_day(today) { 
            Err(DomainError {
                message: String::from("You should not be serving support on weekends. Go out, enjoy, get a life :-)")
            })
        } else { 
            Ok(Engineer {
                name: self.name,
                identifier: self.identifier,
                day_on_support: self.day_on_support,
                last_time_served: self.today_strategy.execute(),
                today_strategy: self.today_strategy,
            })
        }
    }

    fn is_business_day(&self, today: NaiveDate) -> bool { 
        today.weekday().num_days_from_monday() <= 4
    }

    pub fn is_support_candidate_for_today(&self) -> bool {
        let today = self.today_strategy.execute().weekday().num_days_from_monday();
        let day_on_support_as_u32: u32 = self.day_on_support.into();
        today == day_on_support_as_u32
    }

    pub fn is_support_candidate_for_tomorrow(&self) -> bool {
        let tomorrow = self.today_strategy.execute()
            .checked_add_days(Days::new(1))
            .unwrap()
            .weekday()
            .num_days_from_monday();
        let day_on_support_as_u32: u32 = self.day_on_support.into();
        tomorrow == day_on_support_as_u32
    }

    pub fn is_serving_support_today(&self) -> bool {
        let today = Utc::now().date_naive();
        today.eq(&self.last_time_served)
    }
}

#[derive(Clone)]
enum TodayStrategy {
    OSDate,
    Weekday,
    Weekend,
}

impl TodayStrategy {
    pub fn execute(&self) -> NaiveDate {
        match self {
            Self::OSDate => Utc::now().date_naive(),
            Self::Weekday => Utc
                .with_ymd_and_hms(2022, 12, 16, 10, 0, 0)
                .unwrap()
                .date_naive(),
            Self::Weekend => Utc
                .with_ymd_and_hms(2022, 12, 18, 10, 0, 0)
                .unwrap()
                .date_naive(),
        }
    }
}

pub struct EngineeringDepartment {
    engineers: Vec<Engineer>,
}

impl EngineeringDepartment {
    pub fn new(engineers: Vec<Engineer>) -> Self {
        EngineeringDepartment { engineers: engineers }
    }

    pub fn engineer_on_support_today(self) -> Option<Engineer> {
        match self.engineer_who_has_marked_support_service_today() { 
            Some(eng) => Some(eng),
            None => self.support_candidate_with_least_recent_support_service()
        }
    }

    fn engineer_who_has_marked_support_service_today(&self) -> Option<Engineer> { 
        self.engineers
            .to_vec()
            .into_iter()
            .filter(|eng| eng.is_serving_support_today())
            .next()
    }

    fn support_candidate_with_least_recent_support_service(&self) -> Option<Engineer> { 
        self.engineers
            .to_vec()
            .into_iter()
            .filter(|eng| eng.is_support_candidate_for_today())
            .min_by(|eng1, eng2| eng1.last_time_served.cmp(&eng2.last_time_served))
    }

    fn mark_support_service_for_engineer(eng: Engineer) -> EngineeringDepartment {
        todo!()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl Into<u32> for Day {
    fn into(self) -> u32 {
        match self {
            Self::Monday => 0,
            Self::Tuesday => 1,
            Self::Wednesday => 2,
            Self::Thursday => 3,
            Self::Friday => 4,
        }
    }
}

#[derive(Debug)]
pub struct DomainError {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use chrono::{Days};

    use super::*;
    #[test]
    fn should_serve_support_throw_an_error_on_weekends() {
        let engineer = mock_engineer("bobby", Day::Monday, 3, TodayStrategy::Weekend);

        let result = engineer.serve_support();

        assert!(result.is_err())
    }

    #[test]
    fn should_last_time_served_be_today_when_engineer_serves_support() {
        let engineer = mock_engineer("bobby", Day::Monday, 3, TodayStrategy::Weekday);

        let engineer_that_has_served_support = engineer.serve_support().unwrap();

        assert!(engineer_that_has_served_support
            .last_time_served
            .eq(&Utc
                .with_ymd_and_hms(2022, 12, 16, 10, 0, 0).unwrap().date_naive()));
    }

    #[test]
    fn should_engineer_on_support_have_today_as_support_day() {
        let eng1 = mock_engineer("eng1", Day::Monday, 3, TodayStrategy::Weekday);
        let eng2 = mock_engineer("eng2", Day::Tuesday, 3, TodayStrategy::Weekday);
        let eng3 = mock_engineer("eng3", Day::Wednesday, 3, TodayStrategy::Weekday);
        let eng4 = mock_engineer("eng4", Day::Thursday, 3, TodayStrategy::Weekday);
        let eng5 = mock_engineer("eng5", Day::Friday, 3, TodayStrategy::Weekday);
        let engineers = vec![eng1, eng2, eng3, eng4, eng5];

        let dpt = EngineeringDepartment::new(engineers);
        let engineer_on_support = dpt.engineer_on_support_today();

        let day_as_u32: u32 = engineer_on_support.unwrap().day_on_support.into();
        assert_eq!(day_as_u32, 4);
    }

    fn mock_engineer(
        name: &str,
        day_on_support: Day,
        days_since_last_service: u64,
        today_strategy: TodayStrategy,
    ) -> Engineer {
        Engineer {
            name: name.to_string(),
            identifier: EngineerIdentifier {
                value: uuid::Uuid::new_v4(),
            },
            day_on_support: day_on_support,
            last_time_served: Utc::now()
                .checked_sub_days(Days::new(days_since_last_service))
                .unwrap()
                .date_naive(),
            today_strategy: today_strategy,
        }
    }
}
