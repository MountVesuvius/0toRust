// use std::fmt;

#[derive(PartialEq, PartialOrd)]
#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

// impl fmt::Display for Clock {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:02}:{:02}", self.hours, self.minutes)
//     }
// }

impl Clock {
    fn new(hours: i32, minutes: i32) -> Self {
        let mut h: i32 = hours; 
        if hours >= 24 {
            h = hours % 24;
        }

        let mut m: i32 = minutes;
        if minutes >= 60 {
            h += minutes / 60;
            m = minutes % 60;
            h = h % 24;
        }

        if hours < 0 {
            h = 24 + (hours % 24);
        }

        if minutes < 0 {
            m = 60 + (minutes % 60);
            // Handles rolling negative time
            for _i in (0..(-1 * minutes)).step_by(60) {
                h -= 1;
                if h < 0 { h = 24 + h; }
            }

            if m == 60 {
                m = 0;
            }
        }


        Clock {
            hours: h,
            minutes: m,
        }
    }

    /*
    I would prefer to borrow self here, however the exercise requires that add_minutes() return a new Clock instance.
    This is not by choice, but by exercise demand.
     */
    fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;

        while self.minutes >= 60 {
            self.minutes -= 60;
            self.hours += 1;
        }

        while self.minutes < 0 {
            self.minutes += 60;
            self.hours -= 1;
        }

        self.hours = (self.hours + 24) % 24;

        self 
    }

    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }


}

fn main() {
    let clock = Clock::new(5, 32);
    println!("{}", clock.to_string());

    let clock = Clock::new(5, 32).add_minutes(1500);
    println!("{}", clock.to_string());

    // 20:20
    let clock: Clock = Clock::new(-25, -160);
    println!("{}", clock.to_string());

    // // 23:00
    let clock: Clock = Clock::new(1, -120);
    println!("{}", clock.to_string());

    // // 00:20
    // let clock: Clock = Clock::new(1, -40);
    // println!("{}", clock.to_string());





    
    
    // clock.add_minutes(30);
    // println!("{}", clock);

}
#[cfg(test)]
mod test;