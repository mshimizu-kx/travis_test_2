//! This module provides custom errors which are consolidated under `QError` enum type.
//! 
//! When error happens it is converted into `io::Error`. 

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Load Library                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use std::{error, fmt};
use super::qtype::*;
use error::Error as stdError;
use std::io;

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Define Struct                     //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% QError %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

#[derive(Debug)]
pub enum QError<'a>{
  /// Indicates parse error from bytes into `Q` object.
  ParseError(i8),
  /// Indicates conversion error from `Q` object into Rust types.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qdate=QGEN::new_date_ymd(2020, 4, 17);
  /// // Conversion Error: Couldn't convert q object 2020.04.17 into Rust type: bool
  /// match qdate.into_bool(){
  ///   Ok(b) => println!("{}", b),
  ///   Err(e) => eprintln!("{}", e)
  /// }
  /// ```
  ConversionError(&'a Q, &'static str),
  /// Indicates that an error happened on kdb+ side when query was processed.
  /// # Example
  /// ```
  /// use rustkdb::connection;
  /// 
  /// let mut handle=connect("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");
  /// // q Error: Execution of query failed: length
  /// if let Err(e)=send_string_query(handle, "1 2 + 2 3 4", Encode::BigEndian).await{
  ///  eprintln!("{}", e);
  /// }
  QProcessError(&'static str),
  /// Miscellaneous error on Rust side.
  /// # Example
  /// ```
  /// use rustkdb::qtype::*;
  /// 
  /// let qfloat_list=QGEN::new_float_list(Attribute::None, vec![2.72, 37.734, 76.807, 6.18]);
  /// // General Error: Cannot decompose into (key, value)
  /// if let Err(e) = qfloat_list.into_key_value(){
  ///   eprintln!("{}", e);
  /// }
  OtherError(&'static str),
}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Define Trait                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//pub type Result<T> = result::Result<T, QError>;

impl<'a> fmt::Display for QError<'a>{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
    match *self{
      QError::ParseError(err) => write!(f, "Parse Error - [ Couldn't parse bytes into q object: {} ]", err),
      QError::ConversionError(from, to) => write!(f, "Conversion Error - [ Couldn't convert q object {} into Rust type: {} ]", from, to),
      QError::QProcessError(err) => write!(f, "q Error - [ Execution of query failed: {} ]", err),
      QError::OtherError(err) => write!(f, "General Error - [ {} ]", err)
    }
  }
}

impl<'a> stdError for QError<'a>{
  fn description(&self) -> &str{
    match *self{
      QError::ParseError(err) => Box::leak(format!("Failed to parse q object - type: {}", err).into_boxed_str()),
      QError::ConversionError(from, to) => Box::leak(format!("Failed to convert q object to Rust object: {} to {}", from, to).into_boxed_str()),
      QError::QProcessError(err) => Box::leak(format!("Failed to execute a query in q process: {}", err).into_boxed_str()),
      QError::OtherError(err) => Box::leak(format!("Failed to operate on q object: {}", err).into_boxed_str()),
    }
  }

  fn cause(&self) -> Option<&dyn error::Error>{
    match *self{
      QError::ParseError(_) => None,
      QError::ConversionError(_, _) => None,
      QError::QProcessError(_) => None,
      QError::OtherError(_) => None
    }
  }
}

impl<'a> From<QError<'a>> for io::Error{
  fn from(qerror: QError) -> Self{
    io::Error::new(io::ErrorKind::Other, qerror.to_string())
  }
}