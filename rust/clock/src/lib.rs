#[derive(PartialEq, Eq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let minutes_in_a_day = 24 * 60;
        let minutes = (minutes_in_a_day + (total_minutes % minutes_in_a_day)) % minutes_in_a_day;

        let hours_ = minutes / 60;
        let minutes_ = minutes % 60;

        return Clock {
            hours: hours_,
            minutes: minutes_,
        };
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        return Self::new(self.hours, self.minutes + minutes);
    }

    pub fn to_string(&self) -> String {
        // todo!("Return a string representation of the Clock time in HH:MM format");
        return format!("{:02}:{:02}", self.hours, self.minutes);
    }
}
