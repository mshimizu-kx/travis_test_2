// serialization.rs

// This module provides methods to convert `Q` object into Rust native types.

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Load Library                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use std::io;
use super::qtype::*;
use async_recursion::async_recursion;

//%% Serializer %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// Serialize q object and write into byte vector
#[async_recursion]
pub(crate) async fn serialize_q(message: &mut Vec<u8>, qobj: Q, encode: u8) -> io::Result<()>{

  match qobj{
    Q::Bool(b) => serialize_bool(message, b),
    Q::GUID(g) => serialize_guid(message, g),
    Q::Byte(b) => serialize_byte(message, b),
    Q::Short(s) => serialize_short(message, s, encode),
    Q::Int(i) => serialize_int(message, i, encode),
    Q::Long(j) => serialize_long(message, j, encode),
    Q::Real(r) => serialize_real(message, r, encode),
    Q::Float(f) => serialize_float(message, f, encode),
    Q::Char(c) => serialize_char(message, c),
    Q::Symbol(s) => serialize_symbol(message, s),
    Q::Timestamp(_) => serialize_timestamp(message, qobj.into_i64()?, encode),
    Q::Month(_) => serialize_month(message, qobj.into_i32()?, encode),
    Q::Date(_) => serialize_date(message, qobj.into_i32()?, encode),
    Q::Datetime(_) => serialize_datetime(message, qobj.into_f64()?, encode),
    Q::Timespan(_) => serialize_timespan(message, qobj.into_i64()?, encode),
    Q::Minute(_) => serialize_minute(message, qobj.into_i32()?, encode),
    Q::Second(_) => serialize_second(message, qobj.into_i32()?, encode),
    Q::Time(_) => serialize_time(message, qobj.into_i32()?, encode),
    Q::BoolL(_) => serialize_bool_list(message, qobj, encode)?,
    Q::GUIDL(_) => serialize_guid_list(message, qobj, encode)?,
    Q::ByteL(_) => serialize_byte_list(message, qobj, encode)?,
    Q::ShortL(_) => serialize_short_list(message, qobj, encode)?,
    Q::IntL(_) => serialize_int_list(message, qobj, encode)?,
    Q::LongL(_) => serialize_long_list(message, qobj, encode)?,
    Q::RealL(_) => serialize_real_list(message, qobj, encode)?,
    Q::FloatL(_) => serialize_float_list(message, qobj, encode)?,
    Q::CharL(_) => serialize_char_list(message, qobj, encode)?,
    Q::SymbolL(_) => serialize_symbol_list(message, qobj, encode)?,
    Q::TimestampL(_) => serialize_timestamp_list(message, qobj, encode)?,
    Q::MonthL(_) => serialize_month_list(message, qobj, encode)?,
    Q::DateL(_) => serialize_date_list(message, qobj, encode)?,
    Q::DatetimeL(_) => serialize_datetime_list(message, qobj, encode)?,
    Q::TimespanL(_) => serialize_timespan_list(message, qobj, encode)?,
    Q::MinuteL(_) => serialize_minute_list(message, qobj, encode)?,
    Q::SecondL(_) => serialize_second_list(message, qobj, encode)?,
    Q::TimeL(_) => serialize_time_list(message, qobj, encode)?,
    Q::MixedL(_) => serialize_mixed_list(message, qobj, encode).await?,
    Q::Table(_) => serialize_table(message, qobj, encode).await?,
    Q::Dictionary(_) => serialize_dictionary(message, qobj, encode).await?,
    Q::KeyedTable(_) => serialize_keyed_table(message, qobj, encode).await?,
    Q::GeneralNull(_) => serialize_general_null(message)?
  }

  Ok(())

}

fn serialize_bool(message: &mut Vec<u8>, obj: bool){
  // -1 (bool atom) and object
  message.extend(&[0xff, obj as u8]);
}

fn serialize_guid(message: &mut Vec<u8>, obj: [u8; 16]){
  // -2 (GUID atom)
  message.push(0xfe); 
  message.extend(&obj);
}

fn serialize_byte(message: &mut Vec<u8>, obj: u8){
  // -4 (byte atom) and object
  message.extend(&[0xfc, obj]);
}

fn serialize_short(message: &mut Vec<u8>, obj: i16, encode: u8){
  // -5 (short atom)
  message.push(0xfb); 
  let short=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&short)
}

fn serialize_int(message: &mut Vec<u8>, obj: i32, encode: u8){
  // -7 (int atom)
  message.push(0xfa); 
  let int=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&int);
}

fn serialize_long(message: &mut Vec<u8>, obj: i64, encode: u8){
  // -7 (long atom)
  message.push(0xf9); 
  let long=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&long);
}

fn serialize_real(message: &mut Vec<u8>, obj: f32, encode: u8){
  // -8 (real atom)
  message.push(0xf8); 
  let real=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&real);
}

fn serialize_float(message: &mut Vec<u8>, obj: f64, encode: u8){
  // -9 float atom
  message.push(0xf7); 
  let float=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&float);
}

fn serialize_char(message: &mut Vec<u8>, obj: char){
  // -10 (char atom)
  message.extend(&[0xf6, obj as u8]);
}

fn serialize_symbol(message: &mut Vec<u8>, obj: String){
  // -11 (symbol atom)
  message.push(0xf5); 
  message.extend(&(obj+"\x00").into_bytes());
}

fn serialize_timestamp(message: &mut Vec<u8>, obj: i64, encode: u8){
  // -12 (timestamp atom)
  message.push(0xf4); 
  let obj=match obj{
    Q_0Nj | Q_0Wj => obj,
    _ => obj - KDB_TIMESTAMP_OFFSET
  };
  let timestamp=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&timestamp);
}

fn serialize_month(message: &mut Vec<u8>, obj: i32, encode: u8){
  // -13 (month atom)
  message.push(0xf3); 
  let obj=match obj{
    Q_0Ni | Q_0Wi => obj,
    _ => obj - KDB_MONTH_OFFSET
  };
  let month=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&month);
}

fn serialize_date(message: &mut Vec<u8>, obj: i32, encode: u8){
  // -14 (date atom)
  message.push(0xf2); 
  let obj=match obj{
    Q_0Ni | Q_0Wi => obj,
    _ => obj - KDB_DAY_OFFSET as i32
  };
  let date=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&date);
}

fn serialize_datetime(message: &mut Vec<u8>, obj: f64, encode: u8){
  // -15 (datetime atom)
  message.push(0xf1); 
  let obj=if obj.is_nan() || obj.is_infinite(){
    obj
  }
  else{
    obj - KDB_DAY_OFFSET as f64
  };
  let datetime=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&datetime);
}

fn serialize_timespan(message: &mut Vec<u8>, obj: i64, encode: u8){
  // -16 (timespan atom)
  message.push(0xf0); 
  let timespan=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&timespan);
}

fn serialize_minute(message: &mut Vec<u8>, obj: i32, encode: u8){
  // -17 (minute atom)
  message.push(0xef); 
  let minute=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&minute);
}

fn serialize_second(message: &mut Vec<u8>, obj: i32, encode: u8){
  // -18 (second atom)
  message.push(0xee); 
  let second=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&second);
}

fn serialize_time(message: &mut Vec<u8>, obj: i32, encode: u8){
  // -19 (time atom)
  message.push(0xed); 
  let time=match encode{
    0 => obj.to_be_bytes(),
    _ => obj.to_le_bytes()
  };
  message.extend(&time);
}

fn serialize_bool_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(1_u8);
  let (attribute, list) = obj.into_bool_vec()?;
  message.push(attribute as u8);

  // Length of vector
  let length=match encode{
    0 => (list.len() as u32).to_be_bytes(),
    _ => (list.len() as u32).to_le_bytes(),
  };  
  message.extend(&length);

  for item in list{
    message.push(item as u8);
  }

  Ok(())
}

fn serialize_guid_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(2_u8);
  let (attribute, list) = obj.into_GUID_vec()?;
  message.push(attribute as u8);

  let length=match encode{
    0 => (list.len() as u32).to_be_bytes(),
    _ => (list.len() as u32).to_le_bytes(),
  };
  message.extend(&length);
  
  for item in list{
    message.extend(&item);
  }

  Ok(())
}

fn serialize_byte_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(4_u8);
  let (attribute, list) = obj.into_u8_vec()?;
  message.push(attribute as u8);

  let length=match encode{
    0 => (list.len() as u32).to_be_bytes(),
    _ => (list.len() as u32).to_le_bytes(),
  };
  message.extend(&length);
  
  for item in list{
    message.push(item);
  }

  Ok(())
}

fn serialize_short_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(5_u8);
  let (attribute, list) = obj.into_i16_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

fn serialize_int_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(6_u8);
  let (attribute, list) = obj.into_i32_vec()?;
  message.push(attribute as u8);
  
  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

fn serialize_long_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(7_u8);
  let (attribute, list) = obj.into_i64_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

fn serialize_real_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(8_u8);
  let (attribute, list) = obj.into_f32_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

fn serialize_float_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(9_u8);
  let (attribute, list) = obj.into_f64_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

fn serialize_char_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(10_u8);
  let (attribute, list) = obj.into_char_vec()?;
  message.push(attribute as u8);

  let length=match encode{
    0 => (list.len() as u32).to_be_bytes(),
    _ => (list.len() as u32).to_le_bytes(),
  };
  message.extend(&length);

  // list is String. Write as it is.
  message.extend(&list.into_bytes());

  Ok(())
}

fn serialize_symbol_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(11_u8);
  let (attribute, list) = obj.into_string_vec()?;
  message.push(attribute as u8);

  let length=match encode{
    0 => (list.len() as u32).to_be_bytes(),
    _ => (list.len() as u32).to_le_bytes(),
  };
  message.extend(&length);

  for item in list{
    message.extend(&(item+"\x00").into_bytes());
  }

  Ok(())
}

fn serialize_timestamp_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(12_u8);
  let (attribute, list) = obj.into_i64_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&(item - KDB_TIMESTAMP_OFFSET).to_be_bytes())
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&(item - KDB_TIMESTAMP_OFFSET).to_le_bytes())
      }
    }
  };
 

  Ok(())
}

fn serialize_month_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(13_u8);
  let (attribute, list) = obj.into_i32_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&(item - KDB_MONTH_OFFSET).to_be_bytes())
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&(item - KDB_MONTH_OFFSET).to_le_bytes())
      }
    }
  };

  Ok(())
}

fn serialize_date_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(14_u8);
  let (attribute, list) = obj.into_i32_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&(item - KDB_DAY_OFFSET as i32).to_be_bytes())
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&(item - KDB_DAY_OFFSET as i32).to_le_bytes())
      }
    }
  };

  Ok(())
}

fn serialize_datetime_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(15_u8);
  let (attribute, list) = obj.into_f64_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&(item - KDB_DAY_OFFSET as f64).to_be_bytes())
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&(item - KDB_DAY_OFFSET as f64).to_le_bytes())
      }
    }
  };

  Ok(())
}

fn serialize_timespan_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(16_u8);
  let (attribute, list) = obj.into_i64_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

fn serialize_minute_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(17_u8);
  let (attribute, list) = obj.into_i32_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

fn serialize_second_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(18_u8);
  let (attribute, list) = obj.into_i32_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

fn serialize_time_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(19_u8);
  let (attribute, list) = obj.into_i32_vec()?;
  message.push(attribute as u8);

  match encode{
    0 => {
      message.extend(&(list.len() as u32).to_be_bytes());
      for item in list{
        message.extend(&item.to_be_bytes());
      }
    },
    _ => {
      message.extend(&(list.len() as u32).to_le_bytes());
      for item in list{
        message.extend(&item.to_le_bytes());
      }
    }
  }

  Ok(())
}

async fn serialize_mixed_list(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  // 0x00 for mixed list type and 0x00 for Attribute::None
  message.extend(&[0_u8, 0]);

  let list=obj.into_q_vec()?;
  let length=match encode{
    0 => (list.len() as u32).to_be_bytes(),
    _ => (list.len() as u32).to_le_bytes(),
  };
  message.extend(&length);

  for item in list{
    serialize_q(message, item, encode).await?;
  }

  Ok(())
}

async fn serialize_table(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.extend(&[98_u8, 0, 99]);
  let (header, value) = obj.into_key_value()?;
  // Write header
  serialize_symbol_list(message, header, encode)?;
  // Write column data
  serialize_mixed_list(message, value, encode).await?;

  Ok(())
}

async fn serialize_dictionary(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(99_u8);
  let (key, value) = obj.into_key_value()?;
  // Write key
  serialize_q(message, key, encode).await?;
  // Write value
  serialize_q(message, value, encode).await?;

  Ok(())
}

async fn serialize_keyed_table(message: &mut Vec<u8>, obj: Q, encode: u8) -> io::Result<()>{
  message.push(99_u8);
  let (keytab, valuetab) = obj.into_key_value()?;
  // Write key table
  serialize_table(message, keytab, encode).await?;
  // Write value table
  serialize_table(message, valuetab, encode).await?;
  Ok(())
}

fn serialize_general_null(message: &mut Vec<u8>) -> io::Result<()>{
  message.extend(&[101_u8, 0]);
  Ok(())
}