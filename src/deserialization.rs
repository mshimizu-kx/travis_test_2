// deserialization.rs

// This module provides methods to parse bytes into `Q` object.

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Load Library                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use super::qtype::*;
use super::error;
use std::io;
use chrono::prelude::*;
use chrono::Duration;
use tokio::io::{AsyncReadExt, AsyncBufReadExt, BufReader};
use async_recursion::async_recursion;

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                    Define Functions                   //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Parser %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Parse bytes into q onject
#[async_recursion]
pub async fn parse_q(reader: &mut BufReader<&[u8]>, vectype: i8, encode: u8) -> Q{
  //let vectype=reader.read_i8().await.expect("Failed to parse vec tor type");
  if vectype == Q_GENERAL_NULL{
    reader.read_u8().await.expect("Failed to read unused (::) value");
    Q::GeneralNull(QGeneralNull{})
  }
  else if vectype == Q_DICTIONARY || vectype == Q_SORTED_DICTIONARY{
    parse_dictionary(reader, encode).await
  }
  else if vectype == Q_TABLE{
    parse_table(reader, encode).await
  }
  else if vectype < 0{
    parse_atom(reader, vectype, encode).await
  }
  else if vectype > 0{
    parse_simple_list(reader, vectype, encode).await
  }
  else if vectype == 0{
    parse_mixed_list(reader, encode).await
  }
  else{
    unimplemented!()
  }
}

// Atom Parser //------------------------------/

// Parse atom q object
async fn parse_atom(reader: &mut BufReader<&[u8]>, vectype: i8, encode: u8) -> Q{
  match -vectype{
    Q_BOOL => Q::Bool(parse_bool(reader).await),
    Q_GUID => Q::GUID(parse_guid(reader).await),
    Q_BYTE => Q::Byte(parse_byte(reader).await),
    Q_SHORT => Q::Short(parse_short(reader, encode).await),
    Q_INT => Q::Int(parse_int(reader, encode).await),
    Q_LONG => Q::Long(parse_long(reader, encode).await),
    Q_REAL => Q::Real(parse_real(reader, encode).await),
    Q_FLOAT => Q::Float(parse_float(reader, encode).await),
    Q_CHAR => Q::Char(parse_char(reader).await),
    Q_SYMBOL => Q::Symbol(parse_symbol(reader).await),
    Q_TIMESTAMP => Q::Timestamp(parse_timestamp(reader, encode).await),
    Q_MONTH => Q::Month(parse_month(reader, encode).await),
    Q_DATE => Q::Date(parse_date(reader, encode).await),
    Q_DATETIME => Q::Datetime(parse_datetime(reader, encode).await),
    Q_TIMESPAN => Q::Timespan(parse_timespan(reader, encode).await),
    Q_MINUTE => Q::Minute(parse_minute(reader, encode).await),
    Q_SECOND => Q::Second(parse_second(reader, encode).await),
    Q_TIME => Q::Time(parse_time(reader, encode).await),
    _ => unimplemented!()
  }
}

async fn parse_bool(reader: &mut BufReader<&[u8]>) -> bool{
  match reader.read_u8().await.expect("Failed to parse bool"){
    0 => false,
    _ => true
  }  
}

async fn parse_guid(reader: &mut BufReader<&[u8]>) -> [u8; 16]{
  let mut guid=[0u8; 16];
  reader.read_exact(&mut guid).await.expect("Failed to parse byte");
  guid
}

async fn parse_byte(reader: &mut BufReader<&[u8]>) -> u8{
  reader.read_u8().await.expect("Failed to parse byte")
}

async fn parse_short(reader: &mut BufReader<&[u8]>, encode: u8) -> i16{
  match encode{
    0 => reader.read_i16().await.expect("Failed to parse short"),
    _ => reader.read_i16_le().await.expect("Failed to parse short"),
  }
}

async fn parse_int(reader: &mut BufReader<&[u8]>, encode: u8) -> i32{
  match encode{
    0 => reader.read_i32().await.expect("Failed to parse int"),
    _ => reader.read_i32_le().await.expect("Failed to parse int")
  }
}

async fn parse_long(reader: &mut BufReader<&[u8]>, encode: u8) -> i64{
  match encode{
    0 => reader.read_i64().await.expect("Failed to parse long"),
    _ => reader.read_i64_le().await.expect("Failed to parse long")
  }
}

async fn parse_real(reader: &mut BufReader<&[u8]>, encode: u8) -> f32{
  let mut real_holder=[0u8;4];
  reader.read_exact(&mut real_holder).await.expect("Failed to read real");
  match encode{
    0 => f32::from_be_bytes(real_holder),
    _ => f32::from_le_bytes(real_holder)
  }
}

async fn parse_float(reader: &mut BufReader<&[u8]>, encode: u8) -> f64{
  let mut float_holder=[0u8;8];
  reader.read_exact(&mut float_holder).await.expect("Failed to read float");
  match encode{
    0 => f64::from_be_bytes(float_holder),
    _ => f64::from_le_bytes(float_holder)
  }
}

async fn parse_char(reader: &mut BufReader<&[u8]>) -> char{
  reader.read_u8().await.expect("Failed to parse character") as char
}

async fn parse_symbol(reader: &mut BufReader<&[u8]>) -> String{
  let mut symbol=Vec::new();
  reader.read_until(0u8, &mut symbol).await.expect("Failed to parse symbol");
  // Eliminate null character
  String::from_utf8(symbol.split_at(symbol.len()-1).0.to_vec()).expect("Failed to build string from bytes")
}

async fn parse_timestamp(reader: &mut BufReader<&[u8]>, encode: u8) -> DateTime<Utc>{
  let timestamp=match encode{
    0 => reader.read_i64().await,
    _ => reader.read_i64_le().await
  }.expect("Failed to parse timestamp");

  match timestamp{
    Q_0Wj => Q_0Wp,
    Q_0Nj => Q_0Np,
    _ => Utc.timestamp_nanos(timestamp + KDB_TIMESTAMP_OFFSET)
  }
}

async fn parse_month(reader: &mut BufReader<&[u8]>, encode: u8) -> Date<Utc>{
  let month_count=match encode{
    0 => reader.read_i32().await,
    _ => reader.read_i32_le().await
  }.expect("Failed to parse month count");

  match month_count{
    Q_0Wi => Q_0Wm,
    Q_0Ni => Q_0Nm,
    _ => {
      let year=2000 + month_count / 12;
      let month=1 + month_count % 12;
      Utc.ymd(year, month as u32, 1)
    }
  }
}

async fn parse_date(reader: &mut BufReader<&[u8]>, encode: u8) -> Date<Utc>{
  let day_count=match encode{
    0 => reader.read_i32().await,
    _ => reader.read_i32_le().await
  }.expect("Faield to parse day count");

  match day_count{
    Q_0Wi => Q_0Wd,
    Q_0Ni => Q_0Nd,
    _ => {
      let (year, year_day)=year_from_days(day_count).expect("Could not determine proper date from given day count");
      Utc.yo(year, year_day)
    }
  }
}

// Return tuple of (year, year day) from given day count
fn year_from_days(days: i32) -> io::Result<(i32, u32)> {
  // Assume days is positive value
  // 1461 represents days in 4 years
  let nth_year=days / 1461;
  let mut lower_year=2000 + 4 * nth_year;
  let mut lower_day=nth_year * 1461;
  for i in 0..4{
    let one_year=match i{
      0 => 366,
      1 | 2 | 3 => 365,
      _ => unreachable!()
    };
    if days < lower_day + one_year{
      return Ok((lower_year, (days - lower_day + 1) as u32));
    }
    else{
      lower_year += 1;
      lower_day += one_year;
    }
  }
  Err(io::Error::from(error::QError::ParseError(Q_DATE)))
}

async fn parse_datetime(reader: &mut BufReader<&[u8]>, encode: u8) -> DateTime<Utc>{
  let mut float_holder=[0u8;8];
  reader.read_exact(&mut float_holder).await.expect("Failed to read datetime");
  let datetime=match encode{
    0 => f64::from_be_bytes(float_holder),
    _ => f64::from_le_bytes(float_holder)
  };

  if datetime.is_nan(){
    Q_0Nz
  }
  else if datetime.is_infinite(){
    Q_0Wz
  }
  else{
    // Add 30 years for kdb+ offset
    Utc.timestamp_millis((ONE_DAY_MILLIS as f64 * (KDB_DAY_OFFSET as f64 + datetime)) as i64)
  }
}

async fn parse_timespan(reader: &mut BufReader<&[u8]>, encode: u8) ->Duration{
  let timespan=match encode{
    0 => reader.read_i64().await,
    _ => reader.read_i64_le().await
  }.expect("Failed to parse timespan");

  match timespan{
    Q_0Wj => *Q_0Wn,
    Q_NEG_0Wj => *Q_NEG_0Wn,
    Q_0Nj => *Q_0Nn,
    _ => Duration::nanoseconds(timespan)
  }
}

async fn parse_minute(reader: &mut BufReader<&[u8]>, encode: u8) ->QTime{
  let minute=match encode{
    0 => reader.read_i32().await,
    _ => reader.read_i32_le().await
  }.expect("Failed to parse minute");
  match minute{
    Q_0Ni => QTime::Null(Q_0Ni),
    Q_0Wi => QTime::Inf(Q_0Wi),
    Q_NEG_0Wi => QTime::Inf(Q_NEG_0Wi),
    _ => {
      let (hour, minute) = (minute / 60, minute % 60);
      QTime::Time(NaiveTime::from_hms(hour as u32, minute as u32, 0))
    }
  }
}

async fn parse_second(reader: &mut BufReader<&[u8]>, encode: u8) ->QTime{
  let second=match encode{
    0 => reader.read_i32().await,
    _ => reader.read_i32_le().await
  }.expect("Failed to parse second");
  match second{
    Q_0Ni => QTime::Null(Q_0Ni),
    Q_0Wi => QTime::Inf(Q_0Wi),
    Q_NEG_0Wi => QTime::Inf(Q_NEG_0Wi),
    _ => {
      let (hour, minute, second) = (second / 3600, (second % 3600) / 60, second % 60);
      QTime::Time(NaiveTime::from_hms(hour as u32, minute as u32, second as u32))
    }
  }
}

async fn parse_time(reader: &mut BufReader<&[u8]>, encode: u8) ->QTime{
  let time=match encode{
    0 => reader.read_i32().await,
    _ => reader.read_i32_le().await
  }.expect("Failed to parse time");
  match time{
    Q_0Ni => QTime::Null(Q_0Ni),
    Q_0Wi => QTime::Inf(Q_0Wi),
    Q_NEG_0Wi => QTime::Inf(Q_NEG_0Wi),
    _ => {
      let (hour, minute, second, milli) = (time / 3600000, (time % 3600000) / 60000, (time % 60000) / 1000, time % 1000);
      QTime::Time(NaiveTime::from_hms_milli(hour as u32, minute as u32, second as u32, milli as u32))
    }
  }
}

// List Parser //------------------------------/

// Parse simple list q object
// Cannot reuse parse_atom due to the limitation of using mutable borrow inside loop
async fn parse_simple_list(reader: &mut BufReader<&[u8]>, vectype: i8, encode: u8) -> Q{
  let attribute=reader.read_u8().await.expect("Failed to parse list attribute");
  let length=match encode{
    0 => reader.read_u32().await,
    _ => reader.read_u32_le().await
  }.expect("Failed to parse list length");

  match vectype{
    Q_BOOL => QGEN::new_bool_list(attribute.into(), parse_bool_list(reader, length).await),
    Q_GUID => QGEN::new_GUID_list(attribute.into(), parse_guid_list(reader, length).await),
    Q_BYTE => QGEN::new_byte_list(attribute.into(), parse_byte_list(reader, length).await),
    Q_SHORT => QGEN::new_short_list(attribute.into(), parse_short_list(reader, encode, length).await),
    Q_INT => QGEN::new_int_list(attribute.into(), parse_int_list(reader, encode, length).await),
    Q_LONG => QGEN::new_long_list(attribute.into(), parse_long_list(reader, encode, length).await),
    Q_REAL => QGEN::new_real_list(attribute.into(), parse_real_list(reader, encode, length).await),
    Q_FLOAT => QGEN::new_float_list(attribute.into(), parse_float_list(reader, encode, length).await),
    Q_CHAR => QGEN::new_char_list(attribute.into(), parse_char_list(reader, length).await),
    Q_SYMBOL => QGEN::new_symbol_list(attribute.into(), parse_symbol_list(reader, length).await),
    Q_TIMESTAMP => QGEN::new_timestamp_list(attribute.into(), parse_timestamp_list(reader, encode, length).await),
    Q_MONTH => QGEN::new_month_list(attribute.into(), parse_month_list(reader, encode, length).await),
    Q_DATE => QGEN::new_date_list(attribute.into(), parse_date_list(reader, encode, length).await),
    Q_DATETIME => QGEN::new_datetime_list(attribute.into(), parse_datetime_list(reader, encode, length).await),
    Q_TIMESPAN => QGEN::new_timespan_list(attribute.into(), parse_timespan_list(reader, encode, length).await),
    Q_MINUTE => QGEN::new_minute_list(attribute.into(), parse_minute_list(reader, encode, length).await),
    Q_SECOND => QGEN::new_second_list(attribute.into(), parse_second_list(reader, encode, length).await),
    Q_TIME => QGEN::new_time_list(attribute.into(), parse_time_list(reader, encode, length).await),
    _ => unimplemented!()
  }
}

async fn parse_bool_list(reader: &mut BufReader<&[u8]>, length: u32) -> Vec<bool>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_bool(reader).await);
  }
  
  res
}

async fn parse_guid_list(reader: &mut BufReader<&[u8]>, length: u32) -> Vec<[u8; 16]>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_guid(reader).await);
  }
  
  res
}

async fn parse_byte_list(reader: &mut BufReader<&[u8]>, length: u32) -> Vec<u8>{
  let mut res=Vec::new();
  for _ in 0..length{
    // Prefer not to use parse_byte function for its performance
    res.push(reader.read_u8().await.expect("Failed to parse byte"));
  }
  
  res
}

async fn parse_short_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<i16>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_short(reader, encode).await)
  }

  res
}

async fn parse_int_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<i32>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_int(reader, encode).await);
  }

  res
}

async fn parse_long_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<i64>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_long(reader, encode).await);
  }

  res
}

async fn parse_real_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<f32>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_real(reader, encode).await);
  }

  res
}

async fn parse_float_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<f64>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_float(reader, encode).await);
  }

  res
}

async fn parse_char_list(reader: &mut BufReader<&[u8]>, length: u32) -> String{
  // Read as String for performance
  let mut string=vec![0_u8; length as usize];
  reader.read_exact(&mut string).await.expect("Failed to parse string");

  String::from_utf8(string).expect("Failed to buid String")
}

async fn parse_symbol_list(reader: &mut BufReader<&[u8]>, length: u32) -> Vec<String>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_symbol(reader).await);
  }

  res
}

async fn parse_timestamp_list<'a>(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<DateTime<Utc>>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_timestamp(reader, encode).await);
  }

  res
}

async fn parse_month_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<Date<Utc>>{
  let mut res=Vec::new();
  for _ in 0..length{res.push(parse_month(reader, encode).await);
  }

  res
}

async fn parse_date_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<Date<Utc>>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_date(reader, encode).await);
  }

  res
}

async fn parse_datetime_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<DateTime<Utc>>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_datetime(reader, encode).await);
  }

  res
}

async fn parse_timespan_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<Duration>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_timespan(reader, encode).await);
  }

  res
}

async fn parse_minute_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<QTime>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_minute(reader, encode).await);
  }

  res
}

async fn parse_second_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<QTime>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_second(reader, encode).await);
  }

  res
}

async fn parse_time_list(reader: &mut BufReader<&[u8]>, encode: u8, length: u32) -> Vec<QTime>{
  let mut res=Vec::new();
  for _ in 0..length{
    res.push(parse_time(reader, encode).await);
  }

  res
}

// Compound List Parser //--------------------/

// Parse compound list q object
async fn parse_mixed_list(reader: &mut BufReader<&[u8]>, encode: u8) -> Q{
  let _ =reader.read_u8().await.expect("Failed to parse unused list attribute");

  let length=match encode{
    0 => reader.read_u32().await,
    _ => reader.read_u32_le().await
  }.expect("Failed to parse list length");

  let mut res=Vec::new();
  for _ in 0..length{
    let vectype=reader.read_i8().await.expect("Failed to parse vector type");
    res.push(parse_q(reader, vectype, encode).await);
  }
  
  QGEN::new_mixed_list(res)
}

//%% Parse Table %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

async fn parse_table(reader: &mut BufReader<&[u8]>, encode: u8) -> Q{
  let _ =reader.read_i8().await.expect("Failed to parse unused table attribute");
  let _ = reader.read_i8().await.expect("Failed to parse unused dictionary indicator");

  let coltype=reader.read_i8().await.expect("Failed to parse key type");
  let cols=parse_simple_list(reader, coltype, encode).await;

  let _ =reader.read_i8().await.expect("Failed to parse unused general list indicator");
  let values = parse_mixed_list(reader, encode).await;

  Q::Table(QTable{
    col: Box::new(cols),
    value: Box::new(values)
  })
}

//%% Parse Dictionary %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

async fn parse_dictionary(reader: &mut BufReader<&[u8]>, encode: u8) -> Q{
  let keytype=reader.read_i8().await.expect("Failed to parse key type");
  let keys=match keytype{
    // Keyed table. Deligate processing to parse_keyed_table function. The result is returned early here
    Q_TABLE => return parse_keyed_table(reader, encode).await,
    // Normal dictionary key
    _ => parse_simple_list(reader, keytype, encode).await,
  };
  let valuetype=reader.read_i8().await.expect("Failed to parse value type");
  // Possiility of table type is gone already since it has been returned before reaching here
  let values = match valuetype{
    0 => parse_mixed_list(reader, encode).await,
    _ => parse_simple_list(reader, valuetype, encode).await,
  };

  QGEN::new_dictionary(keys, values)
}

//%% Parse Keyed Table %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

async fn parse_keyed_table(reader: &mut BufReader<&[u8]>, encode: u8) -> Q{
  // Byte indicating a table type has already been read in parse_dictionary function
  // Therefore bytes start from table attribute which will be parsed in parse_table function
  let keys= parse_table(reader, encode).await;

  let _ =reader.read_i8().await.expect("Failed to parse unused table type indicator");
  let values = parse_table(reader, encode).await;

  Q::KeyedTable(QKeyedTable{
    keytab: Box::new(keys),
    valuetab: Box::new(values)
  })
}
