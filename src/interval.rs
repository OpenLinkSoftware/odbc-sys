#[repr(C)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Interval {
    /// SQL_IS_YEAR
    Year = 1,
    /// SQL_IS_MONTH,
    Month = 2,
    /// SQL_IS_DAY,
    Day = 3,
    /// SQL_IS_HOUR,
    Hour = 4,
    /// SQL_IS_MINUTE,
    Minute = 5,
    /// SQL_IS_SECOND,
    Second = 6,
    /// SQL_IS_YEAR_TO_MONTH,
    YearToMonth = 7,
    /// SQL_IS_DAY_TO_HOUR,
    DayToHour = 8,
    /// SQL_IS_DAY_TO_MINUTE,
    DayToMinute = 9,
    /// SQL_IS_DAY_TO_SECOND,
    DayToSecond = 10,
    /// SQL_IS_HOUR_TO_MINUTE,
    HourToMinute = 11,
    /// SQL_IS_HOUR_TO_SECOND,
    HourToSecond = 12,
    /// SQL_IS_MINUTE_TO_SECOND,
    MinuteToSecond = 13,
}
