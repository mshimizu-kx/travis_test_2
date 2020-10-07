// qtype.rs

//! This module provides a list of conversion between Rust type and q type
//! and conversion functions between IPC message and Rust Q object

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Load Library                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use super::error::*;
use std::io;
use std::fmt;
use chrono::prelude::*;
use chrono::Duration;

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                        Macros                         //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

// Macro to write simple list with whitespace as a delimiter
macro_rules! write_simple_qlist {
  ($globalfomatter: expr, $qlist: expr, $formatter: expr, $typeindicator: expr) => {
    write!($globalfomatter, "{}{}", display_attribute($qlist.get_attribute()), $qlist.get_vec().iter().map($formatter).collect::<Vec<_>>().join(" ")+$typeindicator)
  };
}

// Macro to write simple list without delimiting white space
macro_rules! write_simple_qlist_nospace {
  ($globalfomatter: expr, $qlist: expr, $formatter: expr, $typeindicator: expr) => {
    write!($globalfomatter, "{}{}", display_attribute($qlist.get_attribute()), $qlist.get_vec().iter().map($formatter).collect::<String>()+$typeindicator)
  };
}

// Macro to place "," for enlist
macro_rules! write_enlist {
  ($globalfomatter: expr, $qlist: expr) => {
    if $qlist.get_vec().len()==1{write!($globalfomatter, ",")?}
  };
}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Define Global                     //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% q Type %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Type indicator of q mixed list
pub const Q_MIXED: i8=0;

/// Type indicator of q bool
pub const Q_BOOL: i8=1;

/// Type indicator of q GUID
pub const Q_GUID: i8=2;

/// Type indicator of q byte
pub const Q_BYTE: i8=4;

/// Type indicator of q short
pub const Q_SHORT: i8=5;

/// Type indicator of q int
pub const Q_INT: i8=6;

/// Type indicator of q long
pub const Q_LONG: i8=7;

/// Type indicator of q real
pub const Q_REAL: i8=8;

/// Type indicator of q float
pub const Q_FLOAT: i8=9;

/// Type indicator of q char
pub const Q_CHAR: i8=10;

/// Type indicator of q symbol
pub const Q_SYMBOL: i8=11;

/// Type indicator of q timestamp
pub const Q_TIMESTAMP: i8=12;

/// Type indicator of q month
pub const Q_MONTH: i8=13;

/// Type indicator of q date
pub const Q_DATE: i8=14;

/// Type indicator of q datetime
pub const Q_DATETIME: i8=15;

/// Type indicator of q timespan
pub const Q_TIMESPAN: i8=16;

/// Type indicator of q minute
pub const Q_MINUTE: i8=17;

/// Type indicator of q second
pub const Q_SECOND: i8=18;

/// Type indicator of q time
pub const Q_TIME: i8=19;

/// Type indicator of q table
pub const Q_TABLE: i8=98;

/// Type indicator of q dictionary
pub const Q_DICTIONARY: i8=99;

/// Type indicator of q sorted dictionary
pub const Q_SORTED_DICTIONARY: i8=127;

/// Type indicator of q error
pub const Q_ERROR: i8=-128;

/// Type indicator of q general null
pub const Q_GENERAL_NULL: i8=101;

//%% kdb+ Offset %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// 1 day in nano second
pub const ONE_DAY_NANOS: i64=86400000000000;

/// 1 day in milli second
pub const ONE_DAY_MILLIS: i64=86400000;

/// 2000.01.01 (kdb+ epoch) - 1970.01.01 in month
pub const KDB_MONTH_OFFSET: i32 = 360;

/// 2000.01.01 (kdb+ epoch) - 1970.01.01 in day
pub const KDB_DAY_OFFSET: i64 = 10957;

/// 2000.01.01 (kdb+ epoch) - 1970.01.01 in nanosecond
pub const KDB_TIMESTAMP_OFFSET: i64=946684800000000000;

//%% kdb+ Null and Infinity %%//vvvvvvvvvvvvvvvvvvvvvvvvvv/

/// GUID null
pub const Q_0Ng: [u8; 16]=[0u8; 16];

/// Short null
pub const Q_0Nh: i16=i16::MIN;

/// Short infinity
pub const Q_0Wh: i16=i16::MAX;

/// Short negative infinity
pub const Q_NEG_0Wh: i16=0_i16 - i16::MAX;

/// Int null
pub const Q_0Ni: i32=i32::MIN;

/// Int infinity
pub const Q_0Wi: i32=i32::MAX;

/// Int negative infinity
pub const Q_NEG_0Wi: i32=0_i32 - i32::MAX;

/// Long null
pub const Q_0Nj: i64=i64::MIN;

/// Long infinity
pub const Q_0Wj: i64=i64::MAX;

/// Long negative infinity
pub const Q_NEG_0Wj: i64=0_i64 - i64::MAX;

/// Real null
pub const Q_0Ne: f32=f32::NAN;

/// Real infinity
pub const Q_0We: f32=f32::INFINITY;

/// Real negative infinity
pub const Q_NEG_0We: f32=f32::NEG_INFINITY;

/// Float null
pub const Q_0n: f64=f64::NAN;

/// Float infinity
pub const Q_0w: f64=f64::INFINITY;

/// Float negative infinity
pub const Q_NEG_0w: f64=f64::NEG_INFINITY;

/// Timestamp null
pub const Q_0Np: DateTime<Utc>=chrono::MIN_DATETIME;

/// Timestamp infinity
pub const Q_0Wp: DateTime<Utc>=chrono::MAX_DATETIME;

/// Month null
pub const Q_0Nm: Date<Utc>=chrono::MIN_DATE;

/// Month infinity
pub const Q_0Wm: Date<Utc>=chrono::MAX_DATE;

/// Date null
pub const Q_0Nd: Date<Utc>=chrono::MIN_DATE;

/// Date infinity
pub const Q_0Wd: Date<Utc>=chrono::MAX_DATE;

/// Datetime null
pub const Q_0Nz: DateTime<Utc>=chrono::MIN_DATETIME;

/// Datetime infinity
pub const Q_0Wz: DateTime<Utc>=chrono::MAX_DATETIME;

lazy_static!{
  /// Timespan null
  pub static ref Q_0Nn: Duration=Duration::nanoseconds(i64::MIN);
  /// Timespan infinity
  pub static ref Q_0Wn: Duration=Duration::nanoseconds(i64::MAX);
  /// Timespan negative infinity
  pub static ref Q_NEG_0Wn: Duration=Duration::nanoseconds(-i64::MAX);
}

/// Minute null
pub const Q_0Nu: QTime=QTime::Null(i32::MIN);

/// Minute infinity
pub const Q_0Wu: QTime=QTime::Inf(i32::MAX);

/// Second null
pub const Q_0Nv: QTime=QTime::Null(i32::MIN);

/// Second infinity
pub const Q_0Wv: QTime=QTime::Inf(i32::MAX);

/// Time null
pub const Q_0Nt: QTime=QTime::Null(i32::MIN);

/// Time infinity
pub const Q_0Wt: QTime=QTime::Inf(i32::MAX);

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Define Struct                     //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Corresponding Rust Type %%//vvvvvvvvvvvvvvvvvvvvvvvv/

/// Q Object. Each q type is represented as enum value.
#[derive(Clone, Debug, PartialEq)]
pub enum Q{
  /// bool (type ID: -1h)
  Bool(bool),
  /// GUID (type ID: -2h)
  GUID([u8; 16]),
  /// byte (type ID: -4h)
  Byte(u8),
  /// short (type ID: -5h)
  Short(i16),
  /// int (type ID: -6h)
  Int(i32),
  /// long (type ID: -7h)
  Long(i64),
  /// real (type ID: -8h)
  Real(f32),
  /// float (type ID: -9h)
  Float(f64),
  /// char (type ID: -10h)
  Char(char),
  /// symbol (type ID: -11h)
  Symbol(String),
  /// timestamp (type ID: -12h)
  Timestamp(DateTime<Utc>),
  /// month (type ID: -13h)
  Month(Date<Utc>),
  /// date (type ID: -14h)
  Date(Date<Utc>),
  /// datetime (type ID: -15h)
  Datetime(DateTime<Utc>),
  /// timespan (type ID: -16h)
  Timespan(chrono::Duration),
  /// minute (type ID: -17h)
  Minute(QTime),
  /// second (type ID: -18h)
  Second(QTime),
  /// time (type ID:-19h)
  Time(QTime),
  /// table (type ID: 98h)
  Table(QTable),
  /// dictionary (type ID: 99h)
  Dictionary(QDictionary),
  /// keyed table (type ID: 99h)
  KeyedTable(QKeyedTable),
  /// bool list (type ID: 1h)
  BoolL(QList<Vec<bool>>),
  /// GUID list (type ID: 2h)
  GUIDL(QList<Vec<[u8; 16]>>),
  /// byte list (type ID: 4h)
  ByteL(QList<Vec<u8>>),
  /// short list (type ID: 5h)
  ShortL(QList<Vec<i16>>),
  /// int list (type ID: 6h)
  IntL(QList<Vec<i32>>),
  /// long list (type ID: 7h)
  LongL(QList<Vec<i64>>),
  /// real list (type ID: 8h)
  RealL(QList<Vec<f32>>),
  /// float list (type ID: 9h)
  FloatL(QList<Vec<f64>>),
  /// string/char list (type ID: 10h)
  CharL(QList<String>),
  /// symbol list (type ID: 11h)
  SymbolL(QList<Vec<String>>),
  /// timestamp list (type ID: 12h)
  TimestampL(QList<Vec<DateTime<Utc>>>),
  /// month list (type ID: 13h)
  MonthL(QList<Vec<Date<Utc>>>),
  /// date list (type ID: 14h)
  DateL(QList<Vec<Date<Utc>>>),
  /// datetime list (type ID: 15h)
  DatetimeL(QList<Vec<DateTime<Utc>>>),
  /// timespan list (type ID: 16h)
  TimespanL(QList<Vec<chrono::Duration>>),
  /// minute list (type ID: 17h)
  MinuteL(QList<Vec<QTime>>),
  /// second list (type ID: 18h)
  SecondL(QList<Vec<QTime>>),
  /// time list (type ID: 19h)
  TimeL(QList<Vec<QTime>>),
  /// compound list (type ID: 0h)
  MixedL(QList<Vec<Q>>),
  /// general null (type ID: 101h)
  GeneralNull(QGeneralNull)
}

//%% QGEN0 %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/


/// Struct providing constructors of `Q` objects.
///  Instance is not built.
pub struct QGEN{}

impl QGEN{

  // Atom Constructor //-------------------------/

  /// Create q bool object from `bool`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 1b
  /// let qbool=QGEN::new_bool(true);
  /// ```
  pub fn new_bool(boolean: bool) -> Q{
    Q::Bool(boolean)
  }

  /// Create q GUID object from `[u8; 16]`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 5ae7962d-49f2-404d-5aec-f7c8abbae288
  /// let qGUID=QGEN::new_GUID([0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]);
  /// ```
  pub fn new_GUID(guid: [u8; 16]) -> Q{
    Q::GUID(guid)
  }

  /// Create q byte object from `u8`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 0x3c
  /// let qbyte=QGEN::new_byte(0x3c);
  /// ```
  pub fn new_byte(byte: u8) -> Q{
    Q::Byte(byte)
  }

  /// Create q short object from `i16`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // -128h
  /// let qshort=QGEN::new_short(-128_i16);
  /// ```
  pub fn new_short(h: i16) -> Q{
    Q::Short(h)
  }

  /// Create q int object from `i32`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 10392i
  /// let qint=QGEN::new_int(10392);
  /// ```
  pub fn new_int(i: i32) -> Q{
    Q::Int(i)
  }

  /// Create q long object from `i64`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 86400000000000
  /// let qlong=QGEN::new_long(86400000000000_i64);
  /// ```
  pub fn new_long(j: i64) -> Q{
    Q::Long(j)
  }

  /// Create q real object from `f32`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 12.34e
  /// let qreal=QGEN::new_real(12.34_f32);
  /// ```
  pub fn new_real(r: f32) -> Q{
    Q::Real(r)
  }

  /// Create q float object from `f64`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // -10957.5
  /// let qfloat=QGEN::new_float(-10957.5);
  /// ```
  pub fn new_float(f: f64) -> Q{
    Q::Float(f)
  }

  /// Create q char object from `char`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // "Q"
  /// let qchar=QGEN::new_char('Q');
  /// ```
  pub fn new_char(c: char) -> Q{
    Q::Char(c)
  }

  /// Create q symbol object
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // `KxSystems
  /// let qsymbol=QGEN::new_symbol("KxSystems");
  /// let qsymbol2=QGEN::new_symbol2(String::from("KxSystems"));
  /// assert_eq!(qsymbol, qsymbol2);
  /// ```
  pub fn new_symbol<T: ToString>(symbol: T) -> Q{
    Q::Symbol(symbol.to_string())
  }

  /// Create q timestamp object from chrono::DateTime<Utc>.
  ///  The precision is nanoseconds.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2015.01.18D09:40:00.000000001
  /// let qtimestamp=QGEN::new_timestamp(Utc.ymd(2015, 1, 18).and_hms_nano(9, 40, 0, 1));
  /// ```
  pub fn new_timestamp(timestamp: DateTime<Utc>) -> Q{
    Q::Timestamp(timestamp)
  }

  /// Create q timestamp object from nanoseconds from `1970-01-01`
  /// # Example
  /// ```
  /// use chrono::prelude::*;
  /// use rustkdb::qtype::*;
  /// 
  /// // 1970.01.01D00:00:00.123456789
  /// let qtimestamp=QGEN::new_timestamp(Utc.ymd(1970, 1, 1).and_hms_nano(0, 0, 0, 123456789));
  /// let qtimestamp2=QGEN::new_timestamp_nanos(123456789);
  /// assert_eq!(qtimestamp, qtimestamp2);
  /// ```
  pub fn new_timestamp_nanos(nanosecond: i64) -> Q{
    Q::Timestamp(match nanosecond{
      Q_0Nj => Q_0Np,
      Q_0Wj => Q_0Wp,
      _ => Utc.timestamp_nanos(nanosecond)
    })
  }

  /// Create q timestamp object from year, month, day, hour, minute,
  ///  second and nanosecond.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 2000.01.01D12:30:45.000000001
  /// let qtimestamp=QGEN::new_timestamp_ymd_hms_nanos(2000, 1, 1, 12, 30, 45, 1);
  /// ```
  pub fn new_timestamp_ymd_hms_nanos(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, nanosecond: u32) -> Q{
    Q::Timestamp(Utc.ymd(year, month, day).and_hms_nano(hour, minute, second, nanosecond))
  }

  /// Create q month object from `chrono::Date<Utc>` object. If the day of `Date` object is not 1,
  ///  it will be set 1 inside the constructor.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2000.01m
  /// let qmonth=QGEN::new_month(Utc.ymd(2000, 1, 3));
  /// let qmonth2=QGEN::new_month(Utc.ymd(2000, 1, 1));
  /// assert_eq!(qmonth, qmonth2);
  /// ```
  pub fn new_month(month: Date<Utc>) -> Q{
    if month.ne(&Q_0Wm) && month.ne(&Q_0Nm){
      let month=Utc.ymd(month.year(), month.month(), 1);
      return Q::Month(month);
    }
    else{
      return Q::Month(month);
    }
  }

  /// Create q month object from year and month
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2001.12m
  /// let qmonth=QGEN::new_month_ym(2001, 12));
  /// ```
  pub fn new_month_ym(year: i32, month: u32) -> Q{
    Q::Month(Utc.ymd(year, month, 1))
  }

  /// Create q date object from `chrono::Date<Utc>`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2012.03.16
  /// let qdate=QGEN::new_date(Utc.ymd(2012, 3, 16));
  /// ```
  pub fn new_date(date: Date<Utc>) -> Q{
    Q::Date(date)
  }

  /// Create q date object from year, month and date
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2008.08.12
  /// let qdate=QGEN::new_date_ymd(2008, 8, 12);
  /// ```
  pub fn new_date_ymd(year: i32, month: u32, day: u32) -> Q{
    Q::Date(Utc.ymd(year, month, day))
  }

  /// Create q datetime object from chrono::DateTime<Utc>.
  ///  The precision is milliseconds.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2015.01.18T09:40:00.123z
  /// let qdatetime=QGEN::new_datetime(Utc.ymd(2015, 1, 18).and_hms_millis(9, 40, 0, 123));
  /// ```
  pub fn new_datetime(datetime: DateTime<Utc>) -> Q{
    Q::Datetime(datetime)
  }

  /// Create q datetime object from milliseconds from `1970-01-01`
  /// # Example
  /// ```
  /// use chrono::prelude::*;
  /// use rustkdb::qtype::*;
  /// 
  /// // 1970.01.01T00:00:00.123z
  /// let qdatetime=QGEN::new_timestamp(Utc.ymd(1970, 1, 1).and_hms_milli(0, 0, 0, 123));
  /// let qdatetime2=QGEN::new_timestamp_millis(123);
  /// assert_eq!(qdatetime, qdatetime2);
  /// ```
  pub fn new_datetime_millis(millisecond: i64) -> Q{
    Q::Datetime(Utc.timestamp_millis(millisecond))
  }

  /// Create q datetime object from year, month, day, hour, minute,
  ///  second and millisecond.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 2000.01.01T12:30:45.111
  /// let qdatetime=QGEN::new_datetime_ymd_hms_millis(2000, 1, 1, 12, 30, 45, 111);
  /// ```
  pub fn new_datetime_ymd_hms_millis(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32, millisecond: u32) -> Q{
    Q::Datetime(Utc.ymd(year, month, day).and_hms_milli(hour, minute, second, millisecond))
  }

  /// Create q timespan object from `chrono::Duration`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::Duration;
  /// 
  /// // -2D
  /// let qtimespan=QGEN::new_timespan(Duration::nanoseconds(-16800000000000_i64));
  /// ```
  pub fn new_timespan(timespan: Duration) -> Q{
    Q::Timespan(timespan)
  }

  /// Create q timespan object from milliseconds.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::Duration;
  /// 
  /// // 1D
  /// let qtimespan=QGEN::new_timespan(Duration::days(1_i64));
  /// let qtimespan2=QGEN::new_timespan_millis(86400000_i64);
  /// assert_eq!(qtimespan, qtimespan2);
  /// ```
  pub fn new_timespan_millis(millisecond: i64) -> Q{
    Q::Timespan(Duration::milliseconds(millisecond))
  }

  /// Create q timespan object from nanoseconds.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::Duration;
  /// 
  /// // 1D
  /// let qtimespan=QGEN::new_timespan(Duration::days(1_i64));
  /// let qtimespan2=QGEN::new_timespan_nanos(86400000000000_i64);
  /// assert_eq!(qtimespan, qtimespan2);
  /// ```
  pub fn new_timespan_nanos(nanosecond: i64) -> Q{
    Q::Timespan(Duration::nanoseconds(nanosecond))
  }

  /// Create q minute object from `QTime`.
  ///  The only expected usage of this constructor is to create inifnity
  ///  or null object.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // Infinite minute 0Wu
  /// let qminute=QGEN::new_minute(Q_0Wu);
  /// ```
  pub fn new_minute(minute: QTime) -> Q{
    Q::Minute(minute)
  }

  /// Create q minute object from `NaiveTime`.
  ///  If second of the given `NaiveTime` is not 0, it is
  ///  set 0 inside constructor.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 10:03
  /// let qminute=QGEN::new_minute_naive(NaiveTime::from_hms(10, 3, 30));
  /// ```
  pub fn new_minute_naive(minute: NaiveTime) -> Q{
    Q::Minute(QTimeGEN::new_minute(minute))
  }

  /// Create q minute object from hour and minute.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 17:00
  /// let qminute=QGEN::new_minute_hm(17, 0));
  /// ```
  pub fn new_minute_hm(hour: u32, minute: u32) -> Q{
    // Call QTime::Time since we know the value is valid
    Q::Minute(QTime::Time(NaiveTime::from_hms(hour, minute, 0)))
  }

  /// Create q minute object from minute.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 18:23
  /// let qminute=QGEN::new_minute_min(1103));
  /// ```
  pub fn new_minute_min(minute: i32) -> Q{
    if minute == Q_0Ni{
      Q::Minute(Q_0Nu)
    }
    else if minute == Q_0Wi{
      Q::Minute(Q_0Wu)
    }
    else{
      let minute=minute as u32 % 1440;
      // Call QTime::Time since we know the value is valid
      Q::Minute(QTime::Time(NaiveTime::from_hms(minute / 60, minute % 60, 0)))
    }   
  }
  
  /// Create q second object from `QTime`.
  ///  The only expected usage of this constructor is to create inifnity
  ///  or null object.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // Null second 0Nv
  /// let qsecond=QGEN::new_second(Q_0Nv);
  /// ```
  pub fn new_second(second: QTime) -> Q{
    Q::Second(second)
  }

  /// Create q second object from `NaiveTime`.
  ///  If nanosecond of the given `NaiveTime` is not 0, it is
  ///  set 0 inside constructor.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 13:40:59
  /// let qsecond=QGEN::new_second_naive(NaiveTime::from_hms(13, 40, 59));
  /// ```
  pub fn new_second_naive(second: NaiveTime) -> Q{
    Q::Second(QTimeGEN::new_second(second))
  }

  /// Create q second object from hour, minute and second.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 04:30:00
  /// let qsecond=QGEN::new_second_hms(4, 30, 0));
  /// ```
  pub fn new_second_hms(hour: u32, minute: u32, second: u32) -> Q{
    // Call QTime::Time since we know the value is valid
    Q::Second(QTime::Time(NaiveTime::from_hms(hour, minute, second)))
  }

  /// Create q second object from second.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 02:24:30
  /// let qsecond=QGEN::new_second_sec(8660));
  /// ```
  pub fn new_second_sec(second: i32) -> Q{
    if second == Q_0Ni{
      Q::Second(Q_0Nv)
    }
    else if second == Q_0Wi{
      Q::Second(Q_0Wv)
    }
    else{
      let second = second as u32 % 86400;
      // Call QTime::Time since we know the value is valid
      Q::Second(QTime::Time(NaiveTime::from_hms(second / 3600, (second % 3600) / 60, second % 60)))
    } 
  }

  /// Create q time object from `QTime`.
  ///  The only expected usage of this constructor is to create inifnity
  ///  or null object.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // Null time 0Nt
  /// let qtime=QGEN::new_time(Q_0Nt);
  /// ```
  pub fn new_time(time: QTime) -> Q{
    Q::Time(time)
  }

  /// Create q time object from `NaiveTime`.
  ///  If precision under millisecond of the given `NaiveTime` is not 0, it is
  ///  set 0 inside constructor.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 08:15:22.905
  /// let qtime=QGEN::new_time_naive(NaiveTime::from_hms_milli(8, 15, 22, 905));
  /// ```
  pub fn new_time_naive(time: NaiveTime) -> Q{
    Q::Time(QTimeGEN::new_time(time))
  }

  /// Create q time object from hour, minute, second and millisecond.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 11:02:37.030
  /// let qtime=QGEN::new_time_hms_millis(11, 2, 37, 30);
  /// ```
  pub fn new_time_hms_millis(hour: u32, minute: u32, second: u32, millisecond: u32) -> Q{
    // Call QTime::Time since we know the value is valid
    Q::Time(QTime::Time(NaiveTime::from_hms_milli(hour, minute, second, millisecond)))
  }

  /// Create q second list from millisecond.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 14:11:00.647
  /// let qtime=QGEN::new_time_list_millis(51060647);
  /// ```
  pub fn new_time_millis(time: i32) -> Q{
    if time == Q_0Ni{
      Q::Time(Q_0Nt)
    }
    else if time == Q_0Wi{
      Q::Time(Q_0Wt)
    }
    else{
      let time = time as u32 % 86400000;
      // Call QTime::Time since we know the value is valid
      Q::Time(QTime::Time(NaiveTime::from_hms_milli(time / 3600000, (time % 3600000) / 60000, (time % 60000)/ 1000, time % 1000)))
    }
  }

  // List Constructor //-------------------------/

  /// Create q bool list from an `Attribute` and a vector of `bool`.
  /// #Exaple
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // `p#11000b
  /// let qbool_list=QGEN::new_bool_list(Attribute::Parted, vec![true, true, false, false, false]);
  /// ```
  pub fn new_bool_list(attr: Attribute, value: Vec<bool>) -> Q{
    Q::BoolL(QList::new(attr, value))
  }

  /// Create q GUID list from an `Attribute` and a vector of `[u8; 16]`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 8c6b8b64-6815-6084-0a3e-178401251b68 5ae7962d-49f2-404d-5aec-f7c8abbae288
  /// let qGUID_list=QGEN::new_GUID_list(Attribute::None, vec![[0x8c, 0x6b, 0x8b, 0x64, 0x68, 0x15, 0x60, 0x84, 0x0a, 0x3e, 0x17, 0x84, 0x01, 0x25, 0x1b, 0x68], [0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]]);
  /// ```
  pub fn new_GUID_list(attr: Attribute, value: Vec<[u8; 16]>) -> Q{
    Q::GUIDL(QList::new(attr, value))
  }

  /// Create q byte list from an `Attribute` and a vector of `u8`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 0x4b7853797374656d73
  /// let qbyte_list=QGEN::new_byte_list(Attribute::None, vec![0x4b, 0x78, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x73])
  /// ```
  pub fn new_byte_list(attr: Attribute, value: Vec<u8>) -> Q{
    Q::ByteL(QList::new(attr, value))
  }

  /// Create q short list from an `Attribute` and a vector of `i16`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 10 -30 20h
  /// let qshort_list=QGEN::new_short_list(Attribute::None, vec![10_i16, -30, 20])
  /// ```
  pub fn new_short_list(attr: Attribute, value: Vec<i16>) -> Q{
    Q::ShortL(QList::new(attr, value))
  }

  /// Create q int list from an `Attribute` and a vector of `i32`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // -3429000 120000
  /// let qint_list=QGEN::new_int_list(Attribute::None, vec![-3429000, 120000]);
  /// ```
  pub fn new_int_list(attr: Attribute, value: Vec<i32>) -> Q{
    Q::IntL(QList::new(attr, value))
  }

  /// Create q long list from an `Attribute` and a vector of `i64`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // enlist 42
  /// let qlong_list=QGEN::new_int_list(Attribute::None, vec![42_i64]);
  /// ```
  pub fn new_long_list(attr: Attribute, value: Vec<i64>) -> Q{
    Q::LongL(QList::new(attr, value))
  }

  /// Create q real list from an `Attribute` and a vector of `f32`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 0.940909 0W 2039.30499e
  /// let qreal_list=QGEN::new_real_list(Attribute::None, vec![0.940909_f32, Q_0We, 2039.30499]);
  /// ```
  pub fn new_real_list(attr: Attribute, value: Vec<f32>) -> Q{
    Q::RealL(QList::new(attr, value))
  }

  /// Create q float list from an `Attribute` and a vector of `f64`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // -0.9 -0w 1.0 -1.1 1.2 0n
  /// 
  /// let qfloat_list=QGEN::new_float_list(Attribute::None, vec![-0.9 Q_NEG_0w, 1.0 -1.1 1.2, Q_0n]);
  /// ```
  pub fn new_float_list(attr: Attribute, value: Vec<f64>) -> Q{
    Q::FloatL(QList::new(attr, value))
  }

  /// Create q string from an `Attribute` and `str` or `String`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // "Tokyo"
  /// 
  /// let qchar_list=QGEN::new_char_list(Attribute::None, "Tokyo");
  /// ```
  pub fn new_char_list<T: ToString>(attr: Attribute, value: T) -> Q{
    Q::CharL(QList::new(attr, value.to_string()))
  }
  
  /// Create q symbol list from an `Attribute` and a vector of `str` or `String`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // `u#`Last`Derivatives
  /// 
  /// let qsymbol_list=QGEN::new_symbol_list(Attribute::Unique, vec!["Last" "Derivatives"]);
  /// ```
  pub fn new_symbol_list<T: ToString>(attr: Attribute, value: Vec<T>) -> Q{
    let value=value.iter().map(|string| string.to_string()).collect();
    Q::SymbolL(QList::new(attr, value))
  }

  /// Create q timestamp list from an `Attribute` and a vector of `DateTime<Utc>`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2009.02.18D00:00:03.000000115 2000.02.19D02:14:00.000009023
  /// let qtimestamp_list=QGEN::new_timestamp_list(Attribute::None, vec![Utc.ymd(2009, 2, 18).and_hms_nano(0, 0, 3, 115), Utc.ymd(2009, 2, 19).and_hms_nano(2, 14, 0, 9023)]);
  /// ```
  pub fn new_timestamp_list(attr: Attribute, value: Vec<DateTime<Utc>>) -> Q{
    Q::TimestampL(QList::new(attr, value))
  }

  /// Create q timestamp list from an `Attribute` and a vector of nanoseconds from `1970-01-01`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // enlist 2000.01.01D00:00:00.000000000
  /// let qtimestamp_list=QGEN::new_timestamp_list_nanos(Attribute::None, vec![KDB_TIMESTAMP_OFFSET]);
  /// ```
  pub fn new_timestamp_list_nanos(attr: Attribute, value: Vec<i64>) -> Q{
    let value=value.iter().map(|&nanos| {
      match nanos{
        Q_0Nj => Q_0Np,
        Q_0Wj => Q_0Wp,
        _ => Utc.timestamp_nanos(nanos)
      }
    }).collect();
    Q::TimestampL(QList::new(attr, value))
  }

  /// Create q timestamp list from an `Attribute` and a vector of `(year, month, day, hour, minute, second, nanosecond)`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 2001.03.16D00:00:00.000001111 2002.03.16D00:00:00.000002222
  /// let qtimestamp_list=QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2001, 3, 16, 0, 0, 0, 1111), (2002, 3, 16, 0, 0, 0, 2222)]);
  /// ```
  pub fn new_timestamp_list_ymd_hms_nanos(attr: Attribute, value: Vec<(i32, u32, u32, u32, u32, u32, u32)>) -> Q{
    let value=value.iter().map(|&(y, m, d, H, M, S, nanos)| Utc.ymd(y, m, d).and_hms_nano(H, M, S, nanos)).collect();
    Q::TimestampL(QList::new(attr, value))
  }

  /// Create q month list from an `Attribute` and a vector of `Date<Utc>`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2012.07 2015.10 0N 2018.04m
  /// let qmonth_list=QGEN::new_month_list(Attribute::None, vec![Utc.ymd(2012, 7, 1), Utc.ymd(2015, 10, 1), Q_0Nm, Utc.ymd(2018, 4, 1)]);
  /// ```
  pub fn new_month_list(attr: Attribute, value: Vec<Date<Utc>>) -> Q{
    let value=value.iter().map(|&date| 
      if date.ne(&Q_0Wm) && date.ne(&Q_0Nm){
        Utc.ymd(date.year(), date.month(), 1)
      }
      else{
        date
      }
    ).collect();
    Q::MonthL(QList::new(attr, value))
  }

  /// Create q month list from an `Attribute` and a vector of `(year, month))`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 2004.12 2009.07 2000.3m
  /// let qmonth_list=QGEN::new_month_list_ym(Attribute::None, vec![(2004, 12), (2009, 7), (2000, 3)]);
  /// ```
  pub fn new_month_list_ym(attr: Attribute, value: Vec<(i32, u32)>) -> Q{
    let value=value.iter().map(|&(y, m)| Utc.ymd(y, m, 1)).collect();
    Q::MonthL(QList::new(attr, value))
  }

  /// Create q date list from an `Attribute` and a vector of `Date<Utc>`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2005.01.05 2008.03.31
  /// let qdate_list=QGEN::new_date_list(Attribute::None, vec![Utc.ymd(2005, 1, 5), Utc.ymd(2008, 3, 31)]);
  /// ```
  pub fn new_date_list(attr: Attribute, value: Vec<Date<Utc>>) -> Q{
    Q::DateL(QList::new(attr, value))
  }

  /// Create q date list from an `Attribute` and a vector of `(year, month, day)`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // enlist 2013.10.19
  /// let qdate_list=QGEN::new_date_list_ymd(Attribute::None, vec![(2013, 10, 19)]);
  /// ```
  pub fn new_date_list_ymd(attr: Attribute, value: Vec<(i32, u32, u32)>) -> Q{
    let value=value.iter().map(|&(y, m, d)| Utc.ymd(y, m, d)).collect();
    Q::DateL(QList::new(attr, value))
  }

  /// Create q datetime list from an `Attribute` and a vector of `DateTime<Utc>`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// // 2018.04.18T02:20:23.290 2009.02.03T23:34:34.878z
  /// let qdatetime_list=QGEN::new_datetime_list(Attribute::None, vec![Utc.ymd(2018, 4, 18).and_hms_milli(2, 20, 23, 290), Utc.ymd(2009, 2, 13).and_hms_milli(23, 34, 34, 878)]);
  /// ```
  pub fn new_datetime_list(attr: Attribute, value: Vec<DateTime<Utc>>) -> Q{
    Q::DatetimeL(QList::new(attr, value))
  }

  /// Create q datetime list from an `Attribute` and a vector of `(year, month, date, hour, minute, second, millisecond)`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 2020.10.09T07:18:20.388 2002.03.16T04:24:37.003 2009.03.08T17:27:07.260z
  /// let qdatetime_list=QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2020, 10, 09, 7, 18, 20, 388), (2002, 3, 16, 4, 24, 37, 3), (2009, 3, 8, 17, 27, 7, 260)]);
  /// ```
  pub fn new_datetime_list_ymd_hms_millis(attr: Attribute, value: Vec<(i32, u32, u32, u32, u32, u32, u32)>) -> Q{
    let value=value.iter().map(|&(y, m, d, H, M, S, millis)| Utc.ymd(y, m, d).and_hms_milli(H, M, S, millis)).collect();
    Q::DatetimeL(QList::new(attr, value))
  }

  /// Create q datetime list from an `Attribute` and a vector of  milliseconds from `1970-01-01`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // enlist 2003.05.09T10:51:30.373z
  /// let qdatetime_list=QGEN::new_datetime_list_millis(Attribute::None, vec![105792690373_i64]);
  /// ```
  pub fn new_datetime_list_millis(attr: Attribute, value: Vec<i64>) -> Q{
    let value=value.iter().map(|&millis| Utc.timestamp_millis(millis)).collect();
    Q::DatetimeL(QList::new(attr, value))
  }

  /// Create q timespan list from an `Attribute` and a vector of `chrono::Duration`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::Duration;
  /// 
  /// // 1D 2D00:00:00.000000001 -0Wn
  /// let qtimespan_list=QGEN::new_timespan_list(Attribute::None, vec![Duration::days(1), Duration::nanoseconds(ONE_DAY_NANOS, 1 + 2 * ONE_DAY_NANOS), Q_NEG_0Wn]);
  /// ```
  pub fn new_timespan_list(attr: Attribute, value: Vec<Duration>) -> Q{
    Q::TimespanL(QList::new(attr, value))
  }

  /// Create q timespan list from an `Attribute` and a vector of nanoseconds.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // enlist -0D00:00:01.000789238
  /// let qtimespan_list=QGEN::new_timespan_list_nanos(Attribute::None, vec![-1000789238_i64]);
  /// ```
  pub fn new_timespan_list_nanos(attr: Attribute, value: Vec<i64>) -> Q{
    let value=value.iter().map(|&nanos| Duration::nanoseconds(nanos)).collect();
    Q::TimespanL(QList::new(attr, value))
  }

  /// Create q timespan list from an `Attribute` and a vector of milliseconds.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 2D03:00:01.365 3D03:00:04.837
  /// let qtimespan_list=QGEN::new_timespan_list_millis(Attribute::None, vec![18360136_i64, 270004837]);
  /// ```
  pub fn new_timespan_list_millis(attr: Attribute, value: Vec<i64>) -> Q{
    let value=value.iter().map(|&millis| Duration::milliseconds(millis)).collect();
    Q::TimespanL(QList::new(attr, value))
  }

  /// Create q minute list from `Attribute` and a vector of `QTime`.
  ///  The only expected usage of this constructor is to include null or infinity minute
  ///  in the list.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 13:59 0Wu 20:08
  /// let qminute_list=QGEN::new_minute_list(Attribute::None, vec![QTimeGEN::new_minute(NaiveTime::from_hms(13, 59, 0)), Q_0Wu, QTimeGEN::new_minute(NaiveTime::from_hms(20, 08, 0))]);
  /// ```
  pub fn new_minute_list(attr: Attribute, value: Vec<QTime>) -> Q{
    Q::MinuteL(QList::new(attr, value))
  }

  /// Create q minute list from `Attribute` and a vector of `(hour, minute)`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // `s#11:23 14:19
  /// let qminute_list=QGEN::new_minute_list_hm(Attribute::Sorted, vec![(11, 23), (14, 19)]);
  /// ```
  pub fn new_minute_list_hm(attr: Attribute, value: Vec<(u32, u32)>) -> Q{
    // Call QTime::Time since we know the value is valid
    let value=value.iter().map(|&(h, m)| QTime::Time(NaiveTime::from_hms(h, m, 0))).collect();
    Q::MinuteL(QList::new(attr, value))
  }

  /// Create q minute list from `Attribute` and a vector of `chrono::NaiveTime`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 20:01 21:02
  /// let qminute_list=QGEN::new_minute_list_naive(Attribute::None, vec![NaiveTime::from_hms(20, 01, 0), NaiveTime::from_hms(21, 2, 0)]);
  /// ```
  pub fn new_minute_list_naive(attr: Attribute, value: Vec<NaiveTime>) -> Q{
    let value=value.iter().map(|&minute| QTimeGEN::new_minute(minute)).collect();
    Q::MinuteL(QList::new(attr, value))
  }

  /// Create q minute list from `Attribute` and a vector of minute.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 15:08 23:04 21:21
  /// let qminute_list_min=QGEN::new_minute_list_min(Attribute::Sorted, vec![908, 1384, 1281]);
  /// ```
  pub fn new_minute_list_min(attr: Attribute, value: Vec<i32>) -> Q{
    let value=value.iter().map(|&minute| {
      if minute == Q_0Wi{
        Q_0Wu
      }
      else if minute == Q_0Ni{
        Q_0Nu
      }
      else{
        let minute = minute as u32 % 1440;
        // Call QTime::Time since we know the value is valid
        QTime::Time(NaiveTime::from_hms(minute / 60, minute % 60, 0))
      }
    }).collect();
    Q::MinuteL(QList::new(attr, value))
  }

  /// Create q second list from `Attribute` and a vector of `QTime`.
  ///  The only expected usage of this constructor is to include null or infinity second
  ///  in the list.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 0Wv 0Nv 16:09:10
  /// let qsecond_list=QGEN::new_second_list(Attribute::None, vec![Q_0Wv, Q_0Nv, QTimeGEN::new_second(NaiveTime::from_hms(16, 09, 10))]);
  /// ```
  pub fn new_second_list(attr: Attribute, value: Vec<QTime>) -> Q{
    Q::SecondL(QList::new(attr, value))
  }

  /// Create q second list from `Attribute` and a vector of `chrono::NaiveTime`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 19:59:54 18:44:18
  /// let qsecond_list=QGEN::new_second_list_naive(Attribute::None, vec![NaiveTime::from_hms(19, 59, 54), NaiveTime::from_hms(18, 44, 18)]);
  /// ```
  pub fn new_second_list_naive(attr: Attribute, value: Vec<NaiveTime>) -> Q{
    let value=value.iter().map(|&second| QTimeGEN::new_second(second)).collect();
    Q::SecondL(QList::new(attr, value))
  }

  /// Create q minute list from `Attribute` and a vector of `(hour, minute, second)`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 08:00:03 06:13:29
  /// let qsecond_list=QGEN::new_second_list_hms(Attribute::None, vec![(8, 0, 3), (6, 13, 29)]);
  /// ```
  pub fn new_second_list_hms(attr: Attribute, value: Vec<(u32, u32, u32)>) -> Q{
    // Call QTime::Time since we know the value is valid
    let value=value.iter().map(|&(h, m, s)| QTime::Time(NaiveTime::from_hms(h, m, s))).collect();
    Q::SecondL(QList::new(attr, value))
  }

  /// Create q second list from `Attribute` and a vector of second.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 16:27:06 17:13:45
  /// let qsecond_list=QGEN::new_second_list_sec(Attribute::None, vec![59226, 62025]);
  /// ```
  pub fn new_second_list_sec(attr: Attribute, value: Vec<i32>) -> Q{
    let value=value.iter().map(|&second| {
      if second == Q_0Ni{
        Q_0Nv
      }
      else if second == Q_0Wi{
        Q_0Wv
      }
      else{
        let second = second as u32 % 86400;
        // Call QTime::Time since we know the value is valid
        QTime::Time(NaiveTime::from_hms(second / 3600, (second % 3600) / 60, second % 60))
      }
    }).collect();
    Q::SecondL(QList::new(attr, value))
  }

  /// Create q time list from `Attribute` and a vector of `QTime`.
  ///  The only expected usage of this constructor is to include null or infinity time
  ///  in the list. This constructor does not check validity of underlying `QTime` object.
  ///  The values of `QTime` must be created with associated `QTime` constructors.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 10:13:46.289 0Nt
  /// let qtime_list=QGEN::new_time_list(Attribute::None, vec![QTimeGEN::new_time(NaiveTime::from_hms_milli(10, 13, 46, 289)), Q_0Nt]);
  /// ```
  pub fn new_time_list(attr: Attribute, value: Vec<QTime>) -> Q{
    Q::TimeL(QList::new(attr, value))
  }

  /// Create q time list from `Attribute` and a vector of `chrono::NaiveTime`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // 21:39:48.730 00:45:40.134 23:51:18.625
  /// let qtime_list=QGEN::new_time_list_naive(Attribute::None, vec![NaiveTime::from_hms_milli(21, 39, 48, 730), NaiveTime::from_hms_milli(0, 45, 40, 134), NaiveTime::from_hms_milli(23, 51, 18, 625)]);
  /// ```
  pub fn new_time_list_naive(attr: Attribute, value: Vec<NaiveTime>) -> Q{
    let value=value.iter().map(|&time| QTimeGEN::new_time(time)).collect();
    Q::TimeL(QList::new(attr, value))
  }

  /// Create q time list from `Attribute` and a vector of `(hour, minute, second, millisecond)`.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// // enlist 04:54:11.685
  /// let qtime_list=QGEN::new_time_list_hms_millis(Attribute::None, vec![(4, 54, 11, 685)]);
  /// ```
  pub fn new_time_list_hms_millis(attr: Attribute, value: Vec<(u32, u32, u32, u32)>) -> Q{
    // Call QTime::Time since we know the value is valid
    let value=value.iter().map(|&(h, m, s, millis)| QTime::Time(NaiveTime::from_hms_milli(h, m, s, millis))).collect();
    Q::TimeL(QList::new(attr, value))
  }

  /// Create q second list from `Attribute` and a vector of millisecond.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // 05:18:45.828 02:25:54.221 11:32:19.305
  /// let qtime_list=QGEN::new_time_list_millis(Attribute::None, vec![19125828, 8754221, 41539305]);
  /// ```
  pub fn new_time_list_millis(attr: Attribute, value: Vec<i32>) -> Q{
    let value=value.iter().map(|&time| {
      if time == Q_0Ni{
        Q_0Nt
      }
      else if time == Q_0Wi{
        Q_0Wt
      }
      else{
        let time = time as u32 % 86400000;
        // Call QTime::Time since we know the value is valid
        QTime::Time(NaiveTime::from_hms_milli(time / 3600000, (time % 3600000) / 60000, (time % 60000)/ 1000, time % 1000))
      }
    }).collect();
    Q::TimeL(QList::new(attr, value))
  }

  /// Create compound list from an `Attribute` and a vector of `Q` object.
  ///  As `Attribute` is always none, only underlying vector needs to be
  ///  provided.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // (2000.01.01D00:00:00 2000.01.02D00:00:00; 1b; `Pork`Chicken`Beef; "Love and Peace")
  /// let qmixed=QGEN::new_mixed_list(vec![QGEN::new_timestamp_list_nanos(Attribute::None, vec![KDB_TIMESTAMP_OFFSET, KDB_TIMESTAMP_NANOS + ONE_DAY_NANOS]), QGEN::new_bool(true), QGEN::new_symbol_list("Pork", "Chicken", "Beef"), QGEN::new_char_list("Love and Peace")]);
  /// ```
  pub fn new_mixed_list(list: Vec<Q>) -> Q{
    Q::MixedL(QList::new(Attribute::None, list))
  }

  /// Create dictionary from key `Q` object and value `Q` object.
  ///  Conctructor does not check length of key and value.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // `s#100 200 300i!("super.firstderivatives.com"; 4f; 2010.03 2011.03m)
  /// let qdict=QGEN::new_dictionary(QGEN::new_int_list(Attribute::Sorted, vec![100, 200, 300]), QGEN::new_mixed_list(vec![QGEN::new_char_list("super.firstderivatives.com"), QGEN::new_float("4.0"), QGEN::new_month_list_ym(Attribute::None, vec![(2010, 3), (2011, 3)])]))
  /// ```
  pub fn new_dictionary(key: Q, value: Q) -> Q{
    Q::Dictionary(QDictionary{
      key: Box::new(key),
      value: Box::new(value)
    })
  }

  /// Create a table from
  /// - headers (vector of `str` or `String`)
  /// - and column values (vector of `Q` object).
  /// # Example
  /// ```
  /// use rustkdb::*;
  /// 
  /// // time                          sym     price  size   
  /// // ----------------------------------------------------
  /// // 2020.04.01D10:00:01.000000001 USD/JPY 105.64 1000000
  /// // 2020.04.01D10:00:02.000000002 GBP/JPY 135.82 2000000
  /// // 2020.04.01D10:00:03.000000003 USD/JPY 105.63 2000000
  /// let qtable=QGEN::new_table(
  ///   vec!["time", "sym", "price", "size"],
  ///   vec![
  ///     QGEN::new_timestamp_list_nanos(Attribute::None, vec![1585735201000000001_i64, 1585735202000000002, 1585735203000000003]),
  ///     QGEN::new_symbol_list(Attribute::Grouped, vec!["USD/JPY", "GBP/JPY", "USD/JPY"]),
  ///     QGEN::new_float_list(Attribute::None, vec![105.64_f64, 135.82, 105.63]),
  ///     QGEN::new_long_list(Attribute::None, vec![1000000_i64, 2000000, 2000000])
  ///   ]
  /// ).unwrap();
  /// ```
  pub fn new_table<T: ToString>(col: Vec<T>, value: Vec<Q>) -> io::Result<Q>{
    if col.len()!=value.len(){
      return Err(io::Error::from(QError::OtherError(Box::leak(format!("Length of header doesn't match the length of columns: {} and {}", col.len(), value.len()).into_boxed_str()))));
    }
    let col=col.iter().map(|c| c.to_string()).collect::<Vec<_>>();
    Ok(Q::Table(QTable{
      col: Box::new(QGEN::new_symbol_list(Attribute::None, col)),
      value: Box::new(Q::MixedL(QList::new(Attribute::None, value)))
    }))
  }

  /// Create q keyed table from
  /// - headers of key table (vector of `str` ot `String`),
  /// - column values of key table (vector of `Q` object),
  /// - headers of vaue table (vector of `str` ot `String`)
  /// - and column values of value table (vector of `Q` object).
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// // city   | area_skm population
  /// // -------| -------------------
  /// // Tokyo  | 13500    37400000  
  /// // London | 17300    9046000   
  /// // NewYork| 1740     18819000  
  /// let qkeyed_table=QGEN::new_keyed_table(
  ///   vec!["city"],
  ///   vec![
  ///     QGEN::new_symbol_list(Attribute::None, vec!["Tokyo", "London", "NewYork"])
  ///   ],
  ///   vec!["area_skm", "population"],
  ///   vec![
  ///     QGEN::new_int_list(Attribute::None, vec![13500, 17300, 1740]),
  ///     QGEN::new_long_list(Attribute::None, vec![37400000_i64, 9046000, 18819000]),
  ///   ]
  /// ).unwrap();
  pub fn new_keyed_table<T: ToString>(keyheader: Vec<T>, keydata: Vec<Q>, valueheader: Vec<T>, valuedata: Vec<Q>) -> io::Result<Q>{
    Ok(Q::KeyedTable(QKeyedTable{
      keytab: Box::new(QGEN::new_table(keyheader, keydata)?),
      valuetab: Box::new(QGEN::new_table(valueheader, valuedata)?)
    }))
  }

  /// Create q general null `(::)`.
  /// The `(::)` is expected to use when executing a remote functon which does not have any parameter.
  /// # Example
  /// First run kdb+ process on `localhost:5000` and define a initializing function `init_greeting`.
  /// ```
  /// // q -p 5000
  /// q)init_greeting:{[] -1 "Successfully initialized"; "안녕."}
  /// ```
  /// Next connect to this process and call `init_greeting`.
  /// ```
  /// use rustkdb::qtype::*;
  /// use rustkdb::connection::*;
  /// use tokio::net::TcpStream;
  /// 
  /// // Connect to kdb+ process running on localhost:5000
  /// let mut handle=connect("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");
  /// // Call `init_greeting` with no argument.
  /// // (`init_greeting; ::)
  /// // "Successfully initialized" appears on the standard out of the kdb+ process.
  /// let response = send_query(&mut handle, QGEN::mixed_list(vec![QGEN::new_symbol("init_greeting"), QGEN::new_general_null()]), Encode::LittleEndian).await?;
  /// // "안녕."
  /// println!("{}", response);
  ///```
  pub fn new_general_null() -> Q{
    Q::GeneralNull(QGeneralNull{})
  }
}

//%% List Type %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Q list object. It is not expected to directly access this object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QList<T>{
  attribute: Attribute,
  // underlying vector of Rust type or `Q` object, or String
  value: T
}

impl<T> QList<T>{

  // Constructor
  fn new(attr: Attribute, val: T) -> Self{
    QList{
      attribute: attr,
      value: val
    }
  }

  // Get a mutable reference to the underlying vector
  fn get_vec_mut(&mut self) -> &mut T{
    &mut self.value
  }

  // Get a reference to the underlying vector
  fn get_vec(&self) -> &T{
    &self.value
  }

  // Consumes struct and get the underlying vector
  fn into_vec(self) -> T{
    self.value
  }

  // Get an attribute of the underlying vector
  fn get_attribute(&self) -> Attribute{
    self.attribute
  }
}


//%% Table Type %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Table object. It is not expected to directly access this object.
#[derive(Clone, Debug, PartialEq)]
pub struct QTable{
  // header of the table. This will be a symbol list `Q::SymbolL`
  // ex.) `id`date`name...
  pub(crate) col: Box<Q>,
  // column data of the table. This will be a compound list `Q::MixedL`
  // ex.) (1 2 3; 2000.01.01 2000.01.02 2000.01.03; `Samuel`David`Luke; ...)
  pub(crate) value: Box<Q>
}

//%% Dictionary Type %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Dictionary object. It is not expected to directly access this object.
#[derive(Clone, Debug, PartialEq)]
pub struct QDictionary{
  // key of dictionary. This can be a simple list
  // ex.) `a`b`c
  pub(crate) key: Box<Q>,
  // value of dictionary. This can be a simple list or compound list
  // ex.) 2003.05 2010.09 2018.02m or (0 1 2 3; `KxSystems; 2012.01.15D00:02:00.000123456)
  pub(crate) value: Box<Q>
}

//%% Keyed Table Type %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Keyed table object. It is not expected to directly access this object.
#[derive(Clone, Debug, PartialEq)]
pub struct QKeyedTable{
  // key table `Q::Table`
  pub(crate) keytab: Box<Q>,
  // Vvalue table `Q::Table`
  pub(crate) valuetab: Box<Q>
}

//%% General Null Type %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// General null object `(::)`. It is not expected to directly build this object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct QGeneralNull{}

//%% Intermediary type for Minute, Second and Time %%//vvvvvvvvvv/

/// Intermediate object for q minute, q second and q time to hold either of time value
///  or null or inifinity value. This struct is expected to be built only
///  when q list object is built from a vector of `QTime` including null or infinity values.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum QTime{
  /// Normal value of q minute, second and time.
  ///  Use constructors to build from `NaiveTime`:
  /// - `QTimeGEN::new_minute` for q minute
  /// - `QTimeGEN::new_second` for q second
  /// - `QTimeGEN::new_time` for q time
  Time(NaiveTime),
  /// Infinity value of q minute, second and time.
  ///  This inifinity value does not need to be built. Use const:
  /// - `Q_0Wu` for q minute
  /// - `Q_0Wv` for q second
  /// - `Q_0Wt` for q time
  Inf(i32),
  /// Null value of q minute, second and time.
  ///  This null value does not need to be built. Use const:
  /// - `Q_0Nu` for q minute
  /// - `Q_0Nv` for q second
  /// - `Q_0Nt` for q time
  Null(i32)
}

impl QTime{

  fn into_time(self) -> io::Result<NaiveTime>{
    match self{
      QTime::Time(time) => Ok(time),
      _ => Err(io::Error::from(QError::OtherError("Attemted to refer Null or Inf as NaiveTime")))
    }
  }

  #[allow(dead_code)]
  fn into_i32(self) -> io::Result<i32>{
    match self{
      QTime::Inf(i) | QTime::Null(i) => Ok(i),
      _ => Err(io::Error::from(QError::OtherError("Attemted to refere NaiveTime as i32")))
    }
  }
}

//%% QTimeGEN %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Struct providing constructors of `QTime` object.
///  Instance is not built.
pub struct QTimeGEN{}

impl QTimeGEN{

    /// Create `QTime` for q minute object. Precision under minute is ignored.
    /// # Example
    /// ```
    /// use rustkdb::qtype::*;
    /// use chrono::NaiveTime;
    /// 
    /// let qtime_minute=QTimeGEN::new_minute(NaiveTime::from_hms(10, 4, 15));
    /// let qminute=QGEN::new_minute(qtime_minute);
    /// let qminute2=QGEN::new_minute_hm(10, 4);
    /// assert_eq!(qminute, qminute2);
    /// ```
    pub fn new_minute(minute: NaiveTime) -> QTime{
      if minute.nanosecond() != 0 || minute.second() != 0{
        QTime::Time(NaiveTime::from_hms(minute.hour(), minute.minute(), 0))
      }
      else{
        QTime::Time(minute)
      }
    }
  
    /// Create `QTime` for q second object. Precision under second is ignored.
    /// # Example
    /// ```
    /// use rustkdb::qtype::*;
    /// use chrono::NaiveTime;
    /// 
    /// let qtime_second=QTimeGEN::new_second(NaiveTime::from_hms_milli(10, 4, 15, 321));
    /// let qsecond=QGEN::new_second(qtime_second);
    /// let qsecond2=QGEN::new_second_hms(10, 4, 15);
    /// assert_eq!(qsecond, qsecond2);
    /// ```
    pub fn new_second(second: NaiveTime) -> QTime{
      if second.nanosecond() != 0 {
        QTime::Time(NaiveTime::from_hms(second.hour(), second.minute(), second.second()))
      }
      else{
        QTime::Time(second)
      }
    }
    
    /// Create `QTime` for q time object. Precision under millisecond is ignored.
    /// # Example
    /// ```
    /// use rustkdb::qtype::*;
    /// use chrono::NaiveTime;
    /// 
    /// let qtime_time=QTimeGEN::new_time(NaiveTime::from_hms_nano(10, 4, 15, 123456789));
    /// let qtime=QGEN::new_time(qtime_time);
    /// let qtime2=QGEN::new_time_hms_millis(10, 4, 15, 123);
    /// assert_eq!(qtime, qtime2);
    /// ```
    pub fn new_time(time: NaiveTime) -> QTime{
      if time.nanosecond() != 0 {
        QTime::Time(NaiveTime::from_hms_milli(time.hour(), time.minute(), time.second(), time.nanosecond() / 1000000))
      }
      else{
        QTime::Time(time)
      }
    }
}

//%% Attribute %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Attribute of q list object.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Attribute{
  /// `None`: No attribute
  None=0,
  /// Sorted attribute ``` `s# ```
  Sorted=1,
  /// Unique attribute ``` `u# ```
  Unique=2,
  /// Parted attribute ``` `p# ```
  Parted=3,
  /// Grouped attribute ``` `g# ```
  Grouped=4
}

// Convert u8 into Attribute
impl From<u8> for Attribute{
  fn from(attr: u8) -> Self{
    match attr{
      0 => Attribute::None,
      1 => Attribute::Sorted,
      2 => Attribute::Unique,
      3 => Attribute::Parted,
      4 => Attribute::Grouped,
      _ => unreachable!()
    }
  }
}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                 Trait Implementation                  //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Display Format %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

fn display_attribute<'a>(attr: Attribute) -> &'a str{
  match attr{
    Attribute::None => "",
    Attribute::Sorted => "`s#",
    Attribute::Unique => "`u#",
    Attribute::Parted => "`p#",
    Attribute::Grouped => "`g#",
  }
}

// Format integral type atom (short, int and long)
// Supress type indicator if 'in_list' is true
fn format_qatom<T: fmt::Display + PartialEq>(qobj: &T, null: T, inf: T, neginf: T, typeindicator: &str, in_list: bool) -> String{
  let base=if qobj.eq(&null){
    format!("0N")
  }
  else if qobj.eq(&inf){
    format!("0W")
  }
  else if qobj.eq(&neginf){
    format!("-0W")
  }
  else{
    format!("{}", qobj)
  };

  if in_list{
    format!("{}", base)
  }
  else{
    format!("{}{}", base, typeindicator)
  }
}

// Format GUID
fn format_guid(guid: &[u8]) -> String{
  let strguid=guid.iter().map(|b| format!("{:02x}", b)).collect::<String>();
  match &strguid[0..8]{
    "00000000" => String::from("0Ng"),
    _ => format!("{}-{}-{}-{}-{}", &strguid[0..8], &strguid[8..12], &strguid[12..16], &strguid[16..20], &strguid[20..32])
  }
}

// Format Real
// Supress type indicator if 'in_list' is true
fn format_real(real: &f32, in_list: bool) -> String{
  let base=if real.is_nan(){
    String::from("0N")
  }
  else if real.is_infinite(){
    if real.is_sign_positive(){
      String::from("0W")
    }
    else{
      String::from("-0W")
    }
    
  }
  else{
    format!("{}", real)
  };

  if in_list{
    format!("{}", base)
  }
  else{
    format!("{}e", base)
  }
  
}

// Format Float
// Supress type indicator if 'in_list' is true
fn format_float(float: &f64, in_list: bool) -> String{
  if float.is_nan(){
    String::from("0n")
  }
  else if float.is_infinite(){
    if float.is_sign_positive(){
      String::from("0w")
    }
    else{
      String::from("-0w")
    }
  }
  else{
    if in_list{
      format!("{}", float)
    }
    else{
      format!("{}f", float)
    }
  }
}

// Format Timestamp and Datettime
fn format_timestamp(timestamp: &DateTime<Utc>, null: DateTime<Utc>, inf: DateTime<Utc>, formatter: &str) -> String{
  if timestamp.eq(&null){
    String::from("0N") + match formatter{
      "%Y.%m.%dD%H:%M:%S%.9f" => "p",
      "%Y.%m.%dT%H:%M:%S%.3f" => "z",
      _ => unreachable!()
    }
  }
  else if timestamp.eq(&inf){
    String::from("0W") + match formatter{
      "%Y.%m.%dD%H:%M:%S%.9f" => "p",
      "%Y.%m.%dT%H:%M:%S%.3f" => "z",
      _ => unreachable!()
    }
  }
  else{
    timestamp.format(formatter).to_string()
  }
}

// Format Month
// Supress type indicator if 'in_list' is true
fn format_month(month: &Date<Utc>, in_list: bool) -> String{
  let base=if month.eq(&Q_0Nm){
    String::from("0N")
  }
  else if month.eq(&Q_0Wm){
    String::from("0W")
  }
  else{
    month.format("%Y.%m").to_string()
  };

  if in_list{
    format!("{}", base)
  }
  else{
    format!("{}m", base)
  }
}

// Format Date
fn format_date(date: &Date<Utc>) -> String{
  if date.eq(&Q_0Nd){
    String::from("0Nd")
  }
  else if date.eq(&Q_0Wd){
    String::from("0Wd")
  }
  else{
    date.format("%Y.%m.%d").to_string()
  }
}

// Format Timespan
fn format_timespan(timespan: &Duration) -> String{
  if timespan.eq(&Q_0Nn){
    String::from("0Nn")
  }
  else if timespan.eq(&Q_0Wn){
    String::from("0Wn")
  }
  else if timespan.eq(&Q_NEG_0Wn){
    String::from("-0Wn")
  }
  else{
    format!("{}D{:02}:{:02}:{:02}.{:09}", timespan.num_days(), timespan.num_hours() % 24, timespan.num_minutes() % 60, timespan.num_seconds() % 60, timespan.num_nanoseconds().unwrap_or(0) % 1000000000_i64)
  }
}

// Format Minute, Second and Time
fn format_time(time: &QTime, formatter: &str) -> String{
  match time{
    QTime::Inf(_) => String::from("0W")+match formatter{
      "%H:%M" => "u",
      "%H:%M:%S" => "v",
      "%H:%M:%S%.3f" => "t",
      _ => "Not a time"
    },
    QTime::Null(_) => String::from("0N")+match formatter{
      "%H:%M" => "u",
      "%H:%M:%S" => "v",
      "%H:%M:%S%.3f" => "t",
      _ => "Not a time"
    },
    QTime::Time(t) => t.format(formatter).to_string()
  }
}

impl fmt::Display for Q{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    match self{
      Q::Bool(b) => write!(f, "{}b", *b as u8),
      Q::GUID(g) => write!(f, "{}", format_guid(g)),
      Q::Byte(b) => write!(f, "{:#04x}", b),
      Q::Short(s) => write!(f, "{}", format_qatom(s, Q_0Nh, Q_0Wh, Q_NEG_0Wh, "h", false)),
      Q::Int(i) => write!(f, "{}", format_qatom(i, Q_0Ni, Q_0Wi, Q_NEG_0Wi, "i", false)),
      Q::Long(l) => write!(f, "{}", format_qatom(l, Q_0Nj, Q_0Wj, Q_NEG_0Wj, "j", false)),
      Q::Real(r) => write!(f, "{}", format_real(r, false)),
      Q::Float(fl) => write!(f, "{}", format_float(fl, false)), 
      Q::Char(c) => write!(f, "\"{}\"", c),
      Q::Symbol(s) => write!(f, "`{}", s),
      Q::Timestamp(t) => write!(f, "{}", format_timestamp(t, Q_0Np, Q_0Wp, "%Y.%m.%dD%H:%M:%S%.9f")),
      Q::Month(m) => write!(f, "{}", format_month(m, false)),
      Q::Date(d) => write!(f, "{}", format_date(d)),
      Q::Datetime(d) => write!(f, "{}", format_timestamp(d, Q_0Nz, Q_0Wz, "%Y.%m.%dT%H:%M:%S%.3f")), 
      Q::Timespan(t) => write!(f, "{}", format_timespan(t)),
      Q::Minute(m) => write!(f, "{}", format_time(m, "%H:%M")),
      Q::Second(s) => write!(f, "{}", format_time(s, "%H:%M:%S")),
      Q::Time(t) => write!(f, "{}", format_time(t, "%H:%M:%S%.3f")),
      Q::BoolL(ql) => {write_enlist!(f, ql); write_simple_qlist_nospace!(f, ql, |item|{format!("{}", *item as u8)}, "b")},
      Q::GUIDL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_guid(item)}, "")},
      Q::ByteL(ql) => {write_enlist!(f, ql); write!(f, "{}", "0x")?; write_simple_qlist_nospace!(f, ql, |item|{format!("{:02x}", item)}, "")},
      Q::ShortL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_qatom(item, Q_0Nh, Q_0Wh, Q_NEG_0Wh, "h", true)}, "h")},
      Q::IntL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_qatom(item, Q_0Ni, Q_0Wi, Q_NEG_0Wi, "i", true)}, "i")},
      Q::LongL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_qatom(item, Q_0Nj, Q_0Wj, Q_NEG_0Wj, "j", true)}, "j")},
      Q::RealL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_real(item, true)}, "e")},
      Q::FloatL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_float(item, true)}, "f")},
      Q::CharL(ql) => {write_enlist!(f, ql); write!(f, "\"{}\"", ql.get_vec())},
      Q::SymbolL(ql) => {write_enlist!(f, ql); write_simple_qlist_nospace!(f, ql, |item|{format!("`{}", item)}, "")},
      Q::TimestampL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_timestamp(item, Q_0Np, Q_0Wp, "%Y.%m.%dD%H:%M:%S%.9f")}, "")},
      Q::MonthL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_month(item, true)}, "m")},
      Q::DateL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_date(item)}, "")},
      Q::DatetimeL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_timestamp(item, Q_0Nz, Q_0Wz, "%Y.%m.%dT%H:%M:%S%.3f")}, "")},
      Q::TimespanL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format_timespan(item)}, "")},
      Q::MinuteL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format!("{}", format_time(item, "%H:%M"))}, "")},
      Q::SecondL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format!("{}", format_time(item, "%H:%M:%S"))}, "")},
      Q::TimeL(ql) => {write_enlist!(f, ql); write_simple_qlist!(f, ql, |item|{format!("{}", format_time(item, "%H:%M:%S%.3f"))}, "")},
      Q::MixedL(ql) => {
        write_enlist!(f, ql); 
        write!(f, "(");
        for (i, q) in ql.get_vec().iter().enumerate(){
          if i!=0{
            write!(f, ";");
          }
          q.fmt(f)?;
        }
        write!(f, ")")
      },
      Q::Table(table) => {write!(f, "+")?; table.col.fmt(f)?; write!(f, "!")?; table.value.fmt(f)},
      Q::Dictionary(dict) => {dict.key.fmt(f)?; write!(f, "!")?; dict.value.fmt(f)},
      Q::KeyedTable(table) => {write!(f, "(")?; table.keytab.fmt(f)?; write!(f, ")!")?; table.valuetab.fmt(f)},
      Q::GeneralNull(_) => write!(f, "::")
    }
  }
}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                 Trait Implementation                  //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Conversion from Q to Rust Native Type %%//vvvvvvvvvv/
impl Q{
  /// Convert `Q::Bool` object into `bool`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qbool=QGEN::new_bool(false);
  /// let rust_bool=qbool.into_bool()?;
  /// assert_eq!(rust_bool, false);
  /// ```
  pub fn into_bool(self) -> io::Result<bool>{
    match self{
      Q::Bool(b) => Ok(b),
      _ => Err(io::Error::from(QError::ConversionError(&self, "bool")))
    }
  }

  /// Convert `Q::GUID` object into `[u8; 16]`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qGUID=QGEN::new_GUID([0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]);
  /// let rust_guid=qGUID.into_GUID()?;
  /// ```
  pub fn into_GUID(self) -> io::Result<[u8; 16]>{
    match self{
      Q::GUID(g) => Ok(g),
      _ => Err(io::Error::from(QError::ConversionError(&self, "GUID")))
    }
  }

  /// Convert `Q::Byte` object into `u8`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qbyte=QGEN::new_byte(0x3c);
  /// let rust_byte=qbyte.into_u8()?;
  /// assert_eq!(rust_byte, 0x3c);
  /// ```
  pub fn into_u8(self) -> io::Result<u8>{
    match self{
      Q::Byte(b) => Ok(b),
      _ => Err(io::Error::from(QError::ConversionError(&self, "u8")))
    }
  }

  /// Convert `Q::Short` object into `i16`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qshort=QGEN::new_short(Q_0Wh);
  /// let rust_short=qbyte.into_u8()?;
  /// assert_eq!(rust_short, Q_0Wh);
  /// ```
  pub fn into_i16(self) -> io::Result<i16>{
    match self{
      Q::Short(s) => Ok(s),
      _ => Err(io::Error::from(QError::ConversionError(&self, "i16")))
    }
  }

  /// Convert `Q` object into `i32`. Original `Q` object is consumed.
  ///  There are six compatible types with `i32`:
  /// - `Q::Int`: returns underlying `i32` object.
  /// - `Q::Month`: returns the number of months from `1970.01.01`
  /// - `Q::Date`: returns the number of days from `1970.01.01`
  /// - `Q::Minute`: returns the elapsed time in minutes from `00:00:00`
  /// - `Q::Second`: returns the elapsed time in seconds from `00:00:00`
  /// - `Q::Time`: returns the elapsed time in milliseconds from `00:00:00.000`
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qsecond=QGEN::new_second_hms(15, 3, 36);
  /// let rust_second=qsecond.into_i32()?;
  /// assert_eq!(rust_second, 54216);
  /// ```
  pub fn into_i32(self) -> io::Result<i32>{
    match self{
      Q::Int(i) => Ok(i),
      Q::Date(d) => {
        if d.eq(&Q_0Nd){
          Ok(Q_0Ni)
        }
        else if d.eq(&Q_0Wd){
          Ok(Q_0Wi)
        }
        else{
          Ok(Date::signed_duration_since(d, Utc.ymd(1970, 1, 1)).num_days() as i32)
        }
      },
      Q::Month(m) => {
        if m.eq(&Q_0Nm){
          return Ok(Q_0Ni);
        }
        else if m.eq(&Q_0Wm){
          return Ok(Q_0Wi);
        }
        else{
          return Ok((m.year() - 1970) * 12 + m.month0() as i32);
        }
      },
      Q::Minute(m) => {
        match m{
          QTime::Time(time) => Ok(NaiveTime::signed_duration_since(time, NaiveTime::from_hms(0, 0, 0)).num_minutes()as i32),
          QTime::Inf(i) | QTime::Null(i) => Ok(i)
        }
      },
      Q::Second(s) => {
        match s{
          QTime::Time(time) => Ok(NaiveTime::signed_duration_since(time, NaiveTime::from_hms(0, 0, 0)).num_seconds()as i32),
          QTime::Inf(i) | QTime::Null(i) => Ok(i)
        }
      },
      Q::Time(t) => {
        match t{
          QTime::Time(time) => Ok(NaiveTime::signed_duration_since(time, NaiveTime::from_hms(0, 0, 0)).num_milliseconds() as i32),
          QTime::Inf(i) | QTime::Null(i) => Ok(i)
        }
      },
      _ => Err(io::Error::from(QError::ConversionError(&self, "i32")))
    }
  }

  /// Convert `Q` object into `i64`. Original `Q` object is consumed.
  ///  There are three compatible types with `i64`:
  /// - `Q::Long`: returns underlying `i64` object.
  /// - `Q::Timestamp`: returns the elapsed time in nanoseconds from `1970.01.01D00:00:00.000000000`
  /// - `Q::Timespan`: returns nanoseconds
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qtimestamp=QGEN::new_timestamp_ymd_hms_nanos(2014, 11, 3, 3, 25, 9, 50072305);
  /// let rust_timestamp=qtimestamp.into_i64()?;
  /// assert_eq!(rust_second, 1414985109050072305_i64);
  /// ```
  pub fn into_i64(self) -> io::Result<i64>{
    match self{
      Q::Long(l) => Ok(l),
      Q::Timestamp(t) => {
        if t.eq(&Q_0Np){
          Ok(Q_0Nj)
        }
        else if t.eq(&Q_0Wp){
          Ok(Q_0Wj)
        }
        else{
          Ok(t.timestamp_nanos())
        }
      },
      Q::Timespan(t) => Ok(t.num_nanoseconds().expect("overflow happened for timespan")),
      _ => Err(io::Error::from(QError::ConversionError(&self, "i64")))
    }
  }

  /// Convert `Q::Real` object into `f32`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// #[macro_use]
  /// extern crate float_cmp;
  /// 
  /// use rustkdb::qtype::*;
  /// 
  /// let qreal=QGEN::new_real(30.4_f32));
  /// let rust_real=qreal.into_f32()?;
  /// assert!(approx_eq!(f32, rust_real, 30.4, epsilon=0.1))
  /// ```
  pub fn into_f32(self) -> io::Result<f32>{
    match self{
      Q::Real(r) => Ok(r),
      _ => Err(io::Error::from(QError::ConversionError(&self, "f32")))
    }
  }

  /// Convert `Q` object into `f64`. Original `Q` object is consumed.
  ///  There are two compatible types with `f64`:
  /// - `Q::Float`: returns underlying `f64` object.
  /// - `Q::Datetime`: returns the number of days from `1970.01.01` measured with millisecond.
  /// # Example
  /// ```
  /// #[macro_use]
  /// extern crate float_cmp;
  /// 
  /// use rustkdb::qtype::*;
  /// 
  /// let qdatetime=QGEN::new_datetime_ymd_hms_millis(2005, 3, 3, 4, 1, 43, 28);
  /// let rust_datetime=qdatetime.into_f64()?;
  /// assert!(approx_eq!(f64, rust_datetime, 12845.17, epsilon=0.01));
  /// ```
  pub fn into_f64(self) -> io::Result<f64>{
    match self{
      Q::Float(f) => Ok(f),
      Q::Datetime(t) => {
        if t.eq(&Q_0Nz){
          Ok(Q_0n)
        }
        else if t.eq(&Q_0Wz){
          Ok(Q_0w)
        }
        else{
          Ok(t.timestamp_millis() as f64 / ONE_DAY_MILLIS as f64)
        }
      },
      _ => Err(io::Error::from(QError::ConversionError(&self, "f64")))
    }
  }

  /// Convert `Q::Char` object into `f32`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qchar=QGEN::new_char('k');
  /// let rust_char=qchar.into_char()?;
  /// assert_eq!(rust_char, 'k');
  /// ```
  pub fn into_char(self) -> io::Result<char>{
    match self{
      Q::Char(c) => Ok(c),
      _ => Err(io::Error::from(QError::ConversionError(&self, "char")))
    }
  }

  /// Convert `Q::Symbol` object into `String`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qsymbol=QGEN::new_symbol("KxSystems");
  /// let rust_symbol=qsymbol.into_string()?;
  /// assert_eq!(rust_symbol, String::from("KxSystems"));
  /// ```
  pub fn into_string(self) -> io::Result<String>{
    match self{
      Q::Symbol(s) => Ok(s),
      _ => Err(io::Error::from(QError::ConversionError(&self, "String")))
    }
  }

  /// Convert `Q` object into `chrono::DateTime<Utc>`. Original `Q` object is consumed.
  ///  There are two compatible types with `DateTime<Utc>`:
  /// - `Q::Timestamp`: returns underlying datetime
  /// - `Q::Datetime`: returns underlying datetime
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// let qtimestamp=QGEN::new_timestamp_ymd_hms_nanos(2011, 5, 20, 11, 9, 7, 3078);
  /// let rust_timestamp=qtimestamp.into_datetime()?;
  /// assert_eq!(rust_timestamp, Utc.ymd(2011, 5, 20).and_hms_nano(9, 7, 3078));
  /// ```
  pub fn into_datetime(self) -> io::Result<DateTime<Utc>>{
    match self{
      Q::Timestamp(t) | Q::Datetime(t) => Ok(t),
      _ => Err(io::Error::from(QError::ConversionError(&self, "DateTime<Utc>")))
    }
  }

  /// Convert `Q` object into `chrono::Date<Utc>`. Original `Q` object is consumed.
  ///  There are two compatible types with `Date<Utc>`:
  /// - `Q::Month`: returns underlying `Date<Utc>` object
  /// - `Q::Date`: returns underlying `Date<Utc>` object
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// let qmonth=QGEN::new_month_hm(2020, 8);
  /// let rust_month=qmonth.into_date()?;
  /// assert_eq!(rust_month, Utc.ymd(2020, 8, 1));
  /// ```
  pub fn into_date(self) -> io::Result<Date<Utc>>{
    match self{
      Q::Month(m) | Q::Date(m) => Ok(m),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Date<Utc>")))
    }
  }

  /// Convert `Q` object into `chrono::NaiveTime`. Original `Q` object is consumed.
  ///  There are three compatible types with `NaiveTime`:
  /// - `Q::Minute`: returns underlying `%H:%M` time
  /// - `Q::Second`: returns underlying `%H:%M:%S` time
  /// - `Q::Time`: returns underlying `%H:%M:%S%.n` time
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// let qminute=QGEN::new_minute_hm(2, 57);
  /// let rust_minute=qminute.into_naivetime()?;
  /// assert_eq!(rust_minute, NaiveTime::from_hms(2, 57, 0));
  /// ```
  pub fn into_naivetime(self) -> io::Result<NaiveTime>{
    match self{
      // Need to separate all types to catch `self`
      Q::Minute(m) => {
        match m{
          QTime::Time(time) => Ok(time),
          _ => Err(io::Error::from(QError::ConversionError(&self, "NaiveTime")))
        }
      },
      Q::Second(s) => {
        match s{
          QTime::Time(time) => Ok(time),
          _ => Err(io::Error::from(QError::ConversionError(&self, "NaiveTime")))
        }
      },
      Q::Time(t) => {
        match t{
          QTime::Time(time) => Ok(time),
          _ => Err(io::Error::from(QError::ConversionError(&self, "NaiveTime")))
        }
      },
      _ => Err(io::Error::from(QError::ConversionError(&self, "NaiveTime")))
    }
  }

  /// Convert `Q::Char` object into `chrono::Duration`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::Duration;
  /// 
  /// let qtimespan=QGEN::new_timespan_nanos(ONE_DAY_NANOS);
  /// let rust_timespan=qtimespan.into_duration()?;
  /// assert_eq!(rust_timespan, Duration::nanoseconds(ONE_DAY_NANOS));
  /// ```
  pub fn into_duration(self) -> io::Result<Duration>{
    match self{
      Q::Timespan(t) => Ok(t),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Duration")))
    }
  }

  /// Convert `Q::BoolL` object into a tuple of `(Attribute, Vec<bool>)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let bool_vec=vec![true, true, false, false, false];
  /// let qbool_list=QGEN::new_bool_list(Attribute::Parted, vec![true, true, false, false, false]);
  /// let (attribute, rust_bool_vec)=qbool_list.into_bool_vec()?;
  /// assert_eq!(attribute, Attribute::Parted);
  /// assert_eq!(rust_bool_vec, bool_vec);
  /// ```
  pub fn into_bool_vec(self) -> io::Result<(Attribute, Vec<bool>)>{
    match self{
      Q::BoolL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<bool>")))
    }
  }

  /// Similar to `into_bool_vec` but get a reference to underlying `Attribute` and `Vec<bool>` from `Q::BoolL` object.
  pub fn get_bool_vec(&self) -> io::Result<(Attribute, &Vec<bool>)>{
    match self{
      Q::BoolL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<bool>")))
    }
  }

  /// Similar to `into_bool_vec` but get a mutable reference to underlying `Attribute` and `Vec<bool>` from `Q::BoolL` object.
  pub fn get_bool_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<bool>)>{
    match self{
      Q::BoolL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<bool>")))
    }
  }

  /// Convert `Q::GUIIDL` object into a tuple of `(Attribute, Vec<[u8; 16]>)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let GUID_vec=vec![[0x8c, 0x6b, 0x8b, 0x64, 0x68, 0x15, 0x60, 0x84, 0x0a, 0x3e, 0x17, 0x84, 0x01, 0x25, 0x1b, 0x68], [0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]];
  /// let qGUID_list=QGEN::new_GUID_list(Attribute::None, vec![[0x8c, 0x6b, 0x8b, 0x64, 0x68, 0x15, 0x60, 0x84, 0x0a, 0x3e, 0x17, 0x84, 0x01, 0x25, 0x1b, 0x68], [0x5a, 0xe7, 0x96, 0x2d, 0x49, 0xf2, 0x40, 0x4d, 0x5a, 0xec, 0xf7, 0xc8, 0xab, 0xba, 0xe2, 0x88]]);
  /// let (_, rust_GUID_vec)=qGUID_list.into_GUID_vec()?;
  /// assert_eq!(rust_GUID_vec, GUID_vec);
  /// ```
  pub fn into_GUID_vec(self) -> io::Result<(Attribute, Vec<[u8; 16]>)>{
    match self{
      Q::GUIDL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<[u8; 16]>")))
    }
  }

  /// Similar to `into_GUID_vec` but get a reference to underlying `Attribute` and `Vec<[u8; 16]>` from `Q::GUIDL` object.
  pub fn get_GUID_vec(&self) -> io::Result<(Attribute, &Vec<[u8; 16]>)>{
    match self{
      Q::GUIDL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<[u8; 16]>")))
    }
  }

  /// Similar to `into_GUID_vec` but get a mutable reference to underlying `Attribute` and `Vec<[u8; 16]>` from `Q::GUIDL` object.
  pub fn get_GUID_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<[u8; 16]>)>{
    match self{
      Q::GUIDL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<[u8; 16]>")))
    }
  }

  /// Convert `Q::ByteL` object into a tuple of `(Attribute, Vec<u8>)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let byte_vec=vec![0x1a, 0xfc, 0x5e, 0xbb];
  /// let qbyte_list=QGEN::new_byte_list(Attribute::None, vec![0x1a, 0xfc, 0x5e, 0xbb]);
  /// let (_, rust_byte_vec)=qbyte_list.into_u8_vec()?;
  /// assert_eq!(rust_byte_vec, byte_vec);
  /// ```
  pub fn into_u8_vec(self) -> io::Result<(Attribute, Vec<u8>)>{
    match self{
      Q::ByteL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<u8>")))
    }
  }

  /// Similar to `into_u8_vec` but get a reference to underlying `Attribute` and `Vec<u8>` from `Q::ByteL` object.
  pub fn get_u8_vec(&self) -> io::Result<(Attribute, &Vec<u8>)>{
    match self{
      Q::ByteL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<u8>")))
    }
  }

  /// Similar to `into_u8_vec` but get a mutable reference to underlying `Attribute` and `Vec<u8>` from `Q::ByteL` object.
  pub fn get_u8_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<u8>)>{
    match self{
      Q::ByteL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<u8>")))
    }
  }

  /// Convert `Q::ShortL` object into a tuple of `(Attribute, Vec<i16>)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let short_vec=vec![20_i16, 99, -127];
  /// let qshort_list=QGEN::new_short_list(Attribute::None, vec![20_i16, 99, -127]);
  /// let (_, rust_short_vec)=qshort_list.into_i16_vec()?;
  /// assert_eq!(short_vec, rust_short_vec);
  /// ```
  pub fn into_i16_vec(self) -> io::Result<(Attribute, Vec<i16>)>{
    match self{
      Q::ShortL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<i16>")))
    }
  }

  /// Similar to `into_i16_vec` but get a reference to underlying `Attribute` and `Vec<i16>` from `Q::ShortL` object.
  pub fn get_i16_vec(&self) -> io::Result<(Attribute, &Vec<i16>)>{
    match self{
      Q::ShortL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<i16>")))
    }
  }

  /// Similar to `into_i16_vec` but get a mutable reference to underlying `Attribute` and `Vec<i16>` from `Q::ShortL` object.
  pub fn get_i16_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<i16>)>{
    match self{
      Q::ShortL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<i16>")))
    }
  }

  /// Convert `Q` object into a tuple of `(Attribute, Vec<i32>)`. Original `Q` object is consumed.
  ///  There are six compatible types with `i32`: 
  /// - `Q::IntL`: returns underlying `i32` objects
  /// - `Q::MonthL`: returns the number of months from `1970.01.01`
  /// - `Q::DateL`: returns the number of days from `1970.01.01`
  /// - `Q::MinuteL`: returns the elapsed time in minutes from `00:00:00`
  /// - `Q::SecondL`: returns the elapsed time in seconds from `00:00:00`
  /// - `Q::TimeL`: returns the elapsed time in milliseconds from `00:00:00.000`
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// let int_vec=vec![17838, 14351];
  /// let qdate_list=QGEN::new_date_list(Attribute::None, vec![Utc.ymd(2018, 11, 3), Utc.ymd(2009, 4, 17)]);
  /// let (_, rust_int_vec)=qdate_list.into_i32_vec()?;
  /// assert_eq!(rust_int_vec, int_vec);
  /// ```
  pub fn into_i32_vec(self) -> io::Result<(Attribute, Vec<i32>)>{
    match self{
      Q::IntL(l) => Ok((l.get_attribute(), l.into_vec())),
      Q::MonthL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(|month|{
        if month.eq(&Q_0Nm){
          Q_0Ni
        }
        else if month.eq(&Q_0Wm){
          Q_0Wi
        }
        else{
          (month.year() - 1970) * 12 + month.month0() as i32
        }
      }).collect())),
      Q::DateL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(|&date| {
        if date.eq(&Q_0Nd){
          Q_0Ni
        }
        else if date.eq(&Q_0Wd){
          Q_0Wi
        }
        else{
          Date::signed_duration_since(date, Utc.ymd(1970, 1, 1)).num_days() as i32
        }
      }).collect())),
      Q::MinuteL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(|&minute|{ 
        match minute{
          QTime::Time(t) => NaiveTime::signed_duration_since(t, NaiveTime::from_hms(0, 0, 0)).num_minutes() as i32,
          QTime::Inf(i) | QTime::Null(i) => i
        }
      }).collect())),
      Q::SecondL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(|&second|{ 
        match second{
          QTime::Time(t) => NaiveTime::signed_duration_since(t, NaiveTime::from_hms(0, 0, 0)).num_seconds() as i32,
          QTime::Inf(i) | QTime::Null(i) => i
        }
      }).collect())),
      Q::TimeL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(|&time|{ 
        match time{
          QTime::Time(t) => NaiveTime::signed_duration_since(t, NaiveTime::from_hms(0, 0, 0)).num_milliseconds() as i32,
          QTime::Inf(i) | QTime::Null(i) => i
        }
      }).collect())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<i32>")))
    }
  }

  /// Similar to `into_i32_vec` but get a reference to underlying `Attribute` and `Vec<i32>` from `Q::IntL` object.
  pub fn get_i32_vec(&self) -> io::Result<(Attribute, &Vec<i32>)>{
    match self{
      Q::IntL(l) => Ok((l.get_attribute(),l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<i32>")))
    }
  }

  /// Similar to `into_i32_vec` but get a mutable reference to underlying `Attribute` and `Vec<i32>` from `Q::IntL` object.
  pub fn get_i32_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<i32>)>{
    match self{
      Q::IntL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<i32>")))
    }
  }

  /// Convert `Q` object into a tuple of `(Attribute, Vec<i64>)`. Original `Q` object is consumed.
  ///  There are three compatible types with `i64`:
  /// - `Q::LongL`: returns underlying `i64` objects
  /// - `Q::TimestampL`: returns the elapsed time in nanoseconds from `1970.01.01D00:00:00.000000000`
  /// - `Q::TimespanL`: returns nanoseconds
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// let long_vec=vec![1200804374178753408, 1057967265849827968, 1135258660145492480];
  /// let qtimestamp_list=QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2008, 1, 20, 4, 46, 14, 17853408), (2003, 7, 11, 23, 47, 45, 849827968), (2005, 12, 22, 13, 37, 40, 145492480)]);
  /// let (_, rust_long_vec)=qtimestamp_list.into_i64_vec()?;
  /// assert_eq!(rust_long_vec, long_vec);
  /// ```
  pub fn into_i64_vec(self) -> io::Result<(Attribute, Vec<i64>)>{
    match self{
      Q::LongL(l) => Ok((l.get_attribute(), l.into_vec())),
      Q::TimestampL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(|&timestamp| {
        if timestamp.eq(&Q_0Np){
          Q_0Nj
        }
        else if timestamp.eq(&Q_0Wp){
          Q_0Wj
        }
        else{
          timestamp.timestamp_nanos()
        }
      }).collect())),
      Q::TimespanL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(|&timespan| timespan.num_nanoseconds().expect("overflow happened for timespan")).collect())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<i64>")))
    }
  }

  /// Similar to `into_i64_vec` but get a reference to underlying `Attribute` and `Vec<i64>` from `Q::LongL` object.
  pub fn get_i64_vec(&self) -> io::Result<(Attribute, &Vec<i64>)>{
    match self{
      Q::LongL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<i64>")))
    }
  }

  /// Similar to `into_i64_vec` but get a mutable reference to underlying `Attribute` and `Vec<i64>` from `Q::LongL` object.
  pub fn get_i64_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<i64>)>{
    match self{
      Q::LongL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<i64>")))
    }
  }

  /// Convert `Q::RealL` object into a tuple of `(Attribute, Vec<f32>)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// #[macro_use]
  /// extern crate float_cmp;
  /// 
  /// use rustkdb::qtype::*;
  /// 
  /// let qreal_list=QGEN::new_real_list(Attribute::None, vec![104.52_f32]);
  /// let (_, rust_real_vec)=qreal_list.into_f32_vec()?;
  /// assert!(approx_eq!(f32, rust_real_vec[0], 104.52, epsilon=0.01));
  /// ```
  pub fn into_f32_vec(self) -> io::Result<(Attribute, Vec<f32>)>{
    match self{
      Q::RealL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<f32>")))
    }
  }

  /// Similar to `into_f32_vec` but get a reference to underlying `Attribute` and `Vec<f32>` from `Q::RealL` object.
  pub fn get_f32_vec(&self) -> io::Result<(Attribute, &Vec<f32>)>{
    match self{
      Q::RealL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<f32>")))
    }
  }

  /// Similar to `into_f32_vec` but get a mutable reference to underlying `Attribute` and `Vec<f32>` from `Q::RealL` object.
  pub fn get_f32_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<f32>)>{
    match self{
      Q::RealL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<f32>")))
    }
  }

  /// Convert `Q` object into a tuple of `(Attribute, Vec<f64>)`. Original `Q` object is consumed.
  ///  There are two compatible types with `f64`:
  /// - `Q::FloatL`: returns underlying `f64` objects
  /// - `Q::DatetimeL`: returns the number of days from `1970.01.01` measured with millisecond
  /// # Example
  /// ```
  /// #[macro_use]
  /// extern crate float_cmp;
  /// 
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// let float_vec=vec![16360.84, 14084.52];
  /// let qdatetime_list=QGEN::new_datetime_list_ymd_hms_millis(Attribute::None, vec![(2014, 10, 17, 20, 6, 23, 096), (2008, 7, 24, 12, 28, 16, 261)]);
  /// let (_, rust_datetime_vec)=qdatetime_list.into_f64_vec()?;
  /// for (&v1, &v2) in rust_datetime_vec.iter().zip(float_vec.iter()){
  ///   assert!(approx_eq!(f64, v1, v2, epsilon=0.01));
  /// }
  /// ```
  pub fn into_f64_vec(self) -> io::Result<(Attribute, Vec<f64>)>{
    match self{
      Q::FloatL(l) => Ok((l.get_attribute(), l.into_vec())),
      Q::DatetimeL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(|datetime| {
        if datetime.eq(&Q_0Nz){
          Q_0n
        }
        else if datetime.eq(&Q_0Wz){
          Q_0w
        }
        else{
          datetime.timestamp_millis() as f64 / ONE_DAY_MILLIS as f64
        }
      }).collect())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<f64>")))
    }
  }

  /// Similar to `into_f64_vec` but get a reference to underlying `Attribute` and `Vec<f64>` from `Q::FloatL` object.
  pub fn get_f64_vec(&self) -> io::Result<(Attribute, &Vec<f64>)>{
    match self{
      Q::FloatL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<f64>")))
    }
  }

  /// Similar to `into_f64_vec` but get a mutable reference to underlying `Attribute` and `Vec<f64>` from `Q::FloatL` object.
  pub fn get_f64_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<f64>)>{
    match self{
      Q::FloatL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<f64>")))
    }
  }

  /// Convert `Q::CharL` object into a tuple of `(Attribute, String)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qchar_list=QGEN::new_char_list(Attribute::Parted, "OOOXXXX");
  /// let (attribute, rust_char_vec)=qchar_list.into_char_vec()?;
  /// assert_eq!(attribute, Attribute::Parted);
  /// assert_eq!(rust_char_vec, String::from("OOOXXXX"));
  /// ```
  pub fn into_char_vec(self) -> io::Result<(Attribute, String)>{
    match self{
      Q::CharL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "String")))
    }
  }

  /// Similar to `into_char_vec` but get a reference to underlying `Attribute` and `String` from `Q::CharL` object.
  pub fn get_char_vec(&self) -> io::Result<(Attribute, &String)>{
    match self{
      Q::CharL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "String")))
    }
  }

  /// Similar to `into_char_vec` but get a mutable reference to underlying `Attribute` and `String` from `Q::CharL` object.
  pub fn get_char_vec_mut(&mut self) -> io::Result<(Attribute, &mut String)>{
    match self{
      Q::CharL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "String")))
    }
  }

  /// Convert `Q::SymbolL` object into a tuple of `(Attribute, Vec<String>)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let string_vec=vec!["Shimeji", "Matsutake", "Enoki", "Nameko"].iter().map(|&sym| String::from(sym)).collect::<Vec<_>>();
  /// let qsymbol_list=QGEN::new_symbol_list(Attribute::Unique, vec!["Shimeji", "Matsutake", "Enoki", "Nameko"]);
  /// let (attribute, rust_symbol_vec)=qsymbol_list.into_string_vec()?;
  /// assert_eq!(attribute, Attribute::Unique);
  /// assert_eq!(rust_symbol_vec, string_vec);
  /// ```
  pub fn into_string_vec(self) -> io::Result<(Attribute, Vec<String>)>{
    match self{
      Q::SymbolL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<String>")))
    }
  }

  /// Similar to `into_string_vec` but get a reference to underlying `Attribute` and `Vec<String>` from `Q::SymbolL` object.
  pub fn get_string_vec(&self) -> io::Result<(Attribute, &Vec<String>)>{
    match self{
      Q::SymbolL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<String>")))
    }
  }

  /// Similar to `into_string_vec` but get a mutable reference to underlying `Attribute` and `Vec<String>` from `Q::SymbolL` object.
  pub fn get_string_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<String>)>{
    match self{
      Q::SymbolL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<String>")))
    }
  }

  /// Convert `Q` object into a tuple of `(Attribute, Vec<chrono::DateTime<Utc>>)`. Original `Q` object is consumed.
  ///  There are two compatible types with `DateTime<Utc>`:
  /// - `Q::TimestampL`: returns underlying `chrono::DateTime<Utc>` objects
  /// - `Q::DatetimeL`: returns underlying `chrono::DateTime<Utc>` objects
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// let datetime_vec=vec![Utc.ymd(2008, 1, 20).and_hms_nano(4, 46, 14, 17853408), Utc.ymd(2003, 7, 11).and_hms_nano(23, 47, 45, 849827968)];
  /// let qtimestamp_list=QGEN::new_timestamp_list_ymd_hms_nanos(Attribute::None, vec![(2008, 1, 20, 4, 46, 14, 17853408), (2003, 7, 11, 23, 47, 45, 849827968)]);
  /// let (_, rust_timestamp_vec)=qtimestamp_list.into_datetime_vec()?;
  /// assert_eq!(rust_timeestamp_vec, datetime_vec);
  /// ```
  pub fn into_datetime_vec(self) -> io::Result<(Attribute, Vec<DateTime<Utc>>)>{
    match self{
      Q::TimestampL(l) | Q::DatetimeL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<DateTime<Utc>>")))
    }
  }

  /// Similar to `into_datetime_vec` but get a reference to underlying `Attribute` and `Vec<DateTime<Utc>>` from `Q` object.
  ///  There are two compatible types with `DateTime<Utc>`:
  /// - `Q::TimestampL`: returns underlying `chrono::DateTime<Utc>` objects
  /// - `Q::DatetimeL`: returns underlying `chrono::DateTime<Utc>` objects
  pub fn get_datetime_vec(&self) -> io::Result<(Attribute, &Vec<DateTime<Utc>>)>{
    match self{
      Q::TimestampL(l) | Q::DatetimeL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<DateTime<Utc>>")))
    }
  }

  /// Similar to `into_datetime_vec` but get a mutable reference to underlying `Attribute` and `Vec<DateTime<Utc>>` from `Q` object.
  ///  There are two compatible types with `DateTime<Utc>`:
  /// - `Q::TimestampL`: returns underlying `chrono::DateTime<Utc>` objects
  /// - `Q::DatetimeL`: returns underlying `chrono::DateTime<Utc>` objects
  pub fn get_datetime_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<DateTime<Utc>>)>{
    match self{
      Q::TimestampL(l) | Q::DatetimeL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<DateTime<Utc>>")))
    }
  }

  /// Convert `Q` object into a tuple of `(Attribute, Vec<chrono::Date<Utc>>)`. Original `Q` object is consumed.
  ///  There are two compatible types with `Date<Utc>`:
  /// - `Q::MonthL`: returns underlying `Date<Utc>` objects
  /// - `Q::DateL`: returns underlying `Date<Utc>` objects
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::prelude::*;
  /// 
  /// let date_vec=vec![Utc.ymd(2004, 2, 1), Utc.ymd(2005, 6, 6), Utc.ymd(2017, 6, 5)];
  /// let qdate_list=QGEN::new_date_list_ymd(Attribute::None, vec![(2004, 2, 1), (2005, 6, 6), (2017, 6, 5)]);
  /// let (_, rust_date_vec)=qdate_list.into_date_vec()?;
  /// assert_eq!(rust_date_vec, date_vec);
  /// ```
  pub fn into_date_vec(self) -> io::Result<(Attribute, Vec<Date<Utc>>)>{
    match self{
      Q::MonthL(l) | Q::DateL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<Date<Utc>>")))
    }
  }

  /// Similar to `into_date_vec` but get a reference to underlying `Attribute` and `Vec<Date<Utc>>` from `Q` object.
  ///  There are two compatible types with `Date<Utc>`:
  /// - `Q::MonthL`: returns underlying `Date<Utc>` objects
  /// - `Q::DateL`: returns underlying `Date<Utc>` objects
  pub fn get_date_vec(&self) -> io::Result<(Attribute, &Vec<Date<Utc>>)>{
    match self{
      Q::MonthL(l) | Q::DateL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<Date<Utc>>")))
    }
  }

  /// Similar to `into_date_vec` but get a mutable reference to underlying `Attribute` and `Vec<Date<Utc>>` from `Q` object.
  ///  There are two compatible types with `Date<Utc>`:
  /// - `Q::MonthL`: returns underlying `Date<Utc>` objects
  /// - `Q::DateL`: returns underlying `Date<Utc>` objects
  pub fn get_date_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<Date<Utc>>)>{
    match self{
      Q::MonthL(l) | Q::DateL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<Date<Utc>>")))
    }
  }

  /// Convert `Q::Timespan` object into a tuple of `(Attribute, Vec<chrono::Duration>)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::Duration;
  /// 
  /// let duration_vec=vec![Duration::nanoseconds(106055166332423), Duration::nanoseconds(91494278061389)];
  /// let qtimespan_list=QGEN::new_timespan_list_nanos(Attribute::None, vec![106055166332423_i64, 91494278061389]);
  /// let (_, rust_timespan_vec)=qtimespan_list.into_duration_vec()?;
  /// assert_eq!(rust_duration_vec, duration_vec);
  /// ```
  pub fn into_duration_vec(self) -> io::Result<(Attribute, Vec<Duration>)>{
    match self{
      Q::TimespanL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<Duration>")))
    }
  }

  /// Similar to `into_duration_vec` but get a reference to underlying `Attribute` and `Vec<chrono::Duration>` from `Q::Timespan` object.
  pub fn get_duration_vec(&self) -> io::Result<(Attribute, &Vec<Duration>)>{
    match self{
      Q::TimespanL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<Duration>")))
    }
  }

  /// Similar to `into_duration_vec` but get a mutable reference to underlying `Attribute` and `Vec<chrono::Duration>` from `Q::Timespan` object.
  pub fn get_duration_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<Duration>)>{
    match self{
      Q::TimespanL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<Duration>")))
    }
  }

  /// Convert `Q` object into a tuple of `(Attribute, Vec<chrono::NaiveTime>)`. Original `Q` object is consumed.
  ///  There are three compatible types with `NaiveTime`:
  /// - `Q::MinuteL`: returns underlying `chrono::NaiveTime` object
  /// - `Q::SecondL`: returns underlying `chrono::NaiveTime` object
  /// - `Q::TimeL`: returns underlying `chrono::NaiveTime` object
  /// If underlying value is null or infinity this function returns an error.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// use chrono::NaiveTime;
  /// 
  /// let naivetime_vec=vec![NaiveTime::from_hms_milli(5, 18, 45, 828), NaiveTime::from_hms_milli(2, 25, 54, 221), NaiveTime::from_hms_milli(11, 32, 19, 305)];
  /// let qtime_list=QGEN::new_time_list_millis(Attribute::None, vec![19125828, 8754221, 41539305]);
  /// let (_, rust_time_vec)=qtime_list.into_naivetime_vec()?;
  /// assert_eq!(rust_time_vec, naivetime_vec);
  /// ```
  pub fn into_naivetime_vec(self) -> io::Result<(Attribute, Vec<NaiveTime>)>{
    match self{
      Q::MinuteL(l) | Q::SecondL(l) | Q::TimeL(l) => Ok((l.get_attribute(), l.into_vec().iter().map(
        |time| time.into_time().or_else::<NaiveTime, _>(|err| {
          eprintln!("{}. Supress time into 00:00:00", err);
          Ok(NaiveTime::from_hms(0, 0, 0))
        }).unwrap()
      ).collect())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<NaiveTime>")))
    }
  }
 
  /// Convert `Q` object into a tuple of `(Attribute, Vec<QTime>)`. Original `Q` object is consumed.
  ///  There are three compatible types with `QTime`:
  /// - `Q::MinuteL`: returns underlying `QTime` objects
  /// - `Q::SecondL`: returns underlying `QTime` objects
  /// - `Q::TimeL`: returns underlying `QTime` objects
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qtime_vec=vec![QTimeGEN::new_minute(NaiveTime::from_hms(5, 18, 0)), Q_0Wu, QTimeGEN::new_minute(NaiveTime::from_hms(11, 32, 0))];
  /// let qtime_list=QGEN::new_minute_list_min(Attribute::None, vec![318, Q_0Wi, 692]);
  /// let (_, rust_qtime_vec)=qtime_list.into_qtime_vec()?;
  /// assert_eq!(rust_qtime_vec, qtime_vec);
  /// ```
  pub fn into_qtime_vec(self) -> io::Result<(Attribute, Vec<QTime>)>{
    match self{
      Q::MinuteL(l) | Q::SecondL(l) | Q::TimeL(l) => Ok((l.get_attribute(), l.into_vec())),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<QTime>")))
    }
  }

  /// Similar to `into_qtime_vec` but get a reference to underlying `Attribute` and `Vec<QTime>` from `Q` object.
  ///  There are three compatible types with `QTime`:
  /// - `Q::MinuteL`: returns underlying `QTime` objects
  /// - `Q::SecondL`: returns underlying `QTime` objects
  /// - `Q::TimeL`: returns underlying `QTime` objects
  pub fn get_qtime_vec(&self) -> io::Result<(Attribute, &Vec<QTime>)>{
    match self{
      Q::MinuteL(l) | Q::SecondL(l) | Q::TimeL(l) => Ok((l.get_attribute(), l.get_vec())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<QTime>")))
    }
  }

  /// Similar to `into_qtime_vec` but get a mutable reference to underlying `Attribute` and `Vec<QTime>` from `Q` object.
  ///  There are three compatible types with `QTime`:
  /// - `Q::Minute`: returns underlying `QTime` object
  /// - `Q::Second`: returns underlying `QTime` object
  /// - `Q::Time`: returns underlying `QTime` object
  pub fn get_qtime_vec_mut(&mut self) -> io::Result<(Attribute, &mut Vec<QTime>)>{
    match self{
      Q::MinuteL(l) | Q::SecondL(l) | Q::TimeL(l) => Ok((l.get_attribute(), l.get_vec_mut())),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<QTime>")))
    }
  }

  /// Convert `Q::MixedL` object into a tuple of `(Attribute, Vec<Q>)`. Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let q_vec=vec![QGEN::new_month_list_ym(Attribute::None, vec![(2003, 7), (2009, 3)]), QGEN::new_real(0.19_f32)];
  /// let q_list=QGEN::new_mixed_list(vec![QGEN::new_month_list_ym(Attribute::None, vec![(2003, 7), (2009, 3)]), QGEN::new_real(0.19_f32)]);
  /// let rust_q_vec=q_list.into_q_vec()?;
  /// assert_eq!(rust_q_vec, q_vec);
  /// ```
  pub fn into_q_vec(self) -> io::Result<Vec<Q>>{
    match self{
      Q::MixedL(l) => Ok(l.into_vec()),
      _ => Err(io::Error::from(QError::ConversionError(&self, "Vec<Q>")))
    }
  }

  /// Similar to `into_q_vec` but get a reference to underlying `Attribute` and `Vec<Q>` from `Q` object.
  pub fn get_q_vec(&self) -> io::Result<&Vec<Q>>{
    match self{
      Q::MixedL(l) => Ok(l.get_vec()),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<Q>")))
    }
  }

  /// Similar to `into_q_vec` but get a reference to underlying `Attribute` and `Vec<Q>` from `Q` object.
  pub fn get_q_vec_mut(&mut self) -> io::Result<&mut Vec<Q>>{
    match self{
      Q::MixedL(l) => Ok(l.get_vec_mut()),
      _ => Err(io::Error::from(QError::ConversionError(self, "Vec<Q>")))
    }
  }

  /// Decompose `Q` object into a pair of `(Q, Q)`. Original `Q` object is consumed.
  ///  There are three types compatible with `(Q, Q)` objects:
  /// - `Q::Table`: returns underlying header (`Q::SymbolL`) and column values (`Q::MixedL`)
  /// - `Q::Dictionary`: returns underlying key and value
  /// - `Q::KeyedTable`: returns underlying key table (`Q::Table`) and value table (`Q::Table`)
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qdict=QGEN::new_dictionary(
  ///   QGEN::new_symbol_list(Attribute::Sorted, vec!["customer", "os"]),
  ///   QGEN::new_mixed_list(vec![QGEN::new_int_list(Attribute::None, vec![10, 20]), QGEN::new_symbol_list(Attribute::None, vec!["Windows", "Linux", "Mac"])])
  /// );
  /// let (key, value) = qdict.into_key_value()?;
  /// assert_eq!(key, QGEN::new_symbol_list(Attribute::Sorted, vec!["customer", "os"]));
  /// assert_eq!(value, QGEN::new_mixed_list(vec![QGEN::new_int_list(Attribute::None, vec![10, 20]), QGEN::new_symbol_list(Attribute::None, vec!["Windows", "Linux", "Mac"])]));
  /// ```
  pub fn into_key_value(self) -> io::Result<(Q, Q)>{
    match self{
      Q::Table(t) => Ok((*t.col, *t.value)),
      Q::Dictionary(d) => Ok((*d.key, *d.value)),
      Q::KeyedTable(kt) => Ok((*kt.keytab, *kt.valuetab)),
      _ => Err(io::Error::from(QError::OtherError("Cannot decompose into (key, value)")))
    }
  }

  /// Decompose `Q::Table` into a header (`Vec<String>`) and column values (`Vec<Q>`). Original `Q` object is consumed.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qtable=QGEN::new_table(
  ///   vec!["time", "sym", "price", "size"],
  ///   vec![
  ///     QGEN::new_timestamp_list_nanos(Attribute::None, vec![1585735201000000001_i64, 1585735202000000002, 1585735203000000003]),
  ///     QGEN::new_symbol_list(Attribute::Grouped, vec!["USD/JPY", "GBP/JPY", "USD/JPY"]),
  ///     QGEN::new_float_list(Attribute::None, vec![105.64_f64, 135.82, 105.63]),
  ///     QGEN::new_long_list(Attribute::None, vec![1000000_i64, 2000000, 2000000])
  ///   ]
  /// );
  /// let (header, body) = qtable.into_header_body()?;
  /// assert_eq!(header, vec![String::from("time"), String::from("sym"), String::from("price"), String::from("size")]);
  /// assert_eq!(body, vec![
  ///     QGEN::new_timestamp_list_nanos(Attribute::None, vec![1585735201000000001_i64, 1585735202000000002, 1585735203000000003]),
  ///     QGEN::new_symbol_list(Attribute::Grouped, vec!["USD/JPY", "GBP/JPY", "USD/JPY"]),
  ///     QGEN::new_float_list(Attribute::None, vec![105.64_f64, 135.82, 105.63]),
  ///     QGEN::new_long_list(Attribute::None, vec![1000000_i64, 2000000, 2000000])
  ///   ]);
  pub fn into_header_body(self) -> io::Result<(Vec<String>, Vec<Q>)>{
    match self{
      Q::Table(t) => Ok((t.col.into_string_vec()?.1, t.value.into_q_vec()?)),
      _ => Err(io::Error::from(QError::ConversionError(&self, "(Vec<String>, Vec<Q>)")))
    }
  }

  /// Decompose `Q::Keyedtable` into a header of key-table (`Vec<String>`), column values of key-table (`Vec<Q>`),
  ///  a header of value-table (`Vec<String>`) and column values of value-table (`Vec<Q>`).
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qkeyed_table=QGEN::new_keyed_table(
  ///   vec!["id"],
  ///   vec![
  ///     QGEN::new_int_list(Attribute::None, vec![1, 2, 3])
  ///   ],
  ///   vec!["sex", "age", "os"],
  ///   vec![
  ///     QGEN::new_symbol_list(Attribute::None, vec!["F", "M", "M"]),
  ///     QGEN::new_short_list(Attribute::None, vec![12_i16, 31, 18]),
  ///     QGEN::new_symbol_list(Attribute::None, vec!["Mac", "Linux", "Windows"])
  ///   ]
  /// );
  /// let (khead, kval, vhead, vval) = qkeyed_table.into_keyedtable_components()?;
  /// assert_eq!(khead, vec!["id"]);
  /// assert_eq!(kval, vec![
  ///   QGEN::new_int_list(Attribute::None, vec![1, 2, 3])
  ///   ]);
  /// assert_eq!(vhead, vec!["sex", "age", "os"]);
  /// assert_eq!(vval, vec![
  ///   QGEN::new_symbol_list(Attribute::None, vec!["F", "M", "M"]),
  ///   QGEN::new_short_list(Attribute::None, vec![12_i16, 31, 18]),
  ///   QGEN::new_symbol_list(Attribute::None, vec!["Mac", "Linux", "Windows"])
  ///  ]);
  /// ```
  pub fn into_keyedtable_components(self) -> io::Result<(Vec<String>, Vec<Q>, Vec<String>, Vec<Q>)>{
    match self{
      Q::KeyedTable(kt) => {
        let (kheader, kbody) = kt.keytab.into_header_body()?;
        let (valueheader, valuebody) = kt.valuetab.into_header_body()?;
        Ok((kheader, kbody, valueheader, valuebody))
      },
      _ => Err(io::Error::from(QError::ConversionError(&self, "(Vec<String>, Vec<Q>, Vec<String>, Vec<Q>)")))
    }
  }
}
