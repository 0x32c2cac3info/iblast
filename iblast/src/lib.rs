pub mod common {
    use anyhow::{Result, Error};
    use chronoutil::RelativeDuration;
    use chrono::NaiveDate;
    pub(crate) struct License;

    impl License {
        fn new() -> Self { Self {} }
        fn renew_for<Time: Into<RelativeDuration> + Sized>(&mut self, time: Time) -> Result<(), Error> {
            Ok(())
        }
        fn renew_until<Time: Into<NaiveDate> + Sized>(&mut self, time: Time) -> Result<(), Error> {
            Ok(()) 
        }
        fn check_expired<Time: Into<NaiveDate> + Sized>(&self, time: Time) -> bool {
            false
        }
        fn cancel(mut self) -> Result<Self, Error> { Ok(self) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
