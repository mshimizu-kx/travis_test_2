// connection.rs

//! This module provides methods to connect and send a query to kdb+ process.
//!
//! ## Connection
//! Following ways are supported to connect to kdb+:
//! - TCP
//! - TLS
//! - Unix Domain Socket
//! 
//! Compression and decompression of IPC message follows [the manner of kdb+](https://code.kx.com/q/basics/ipc/#compression).
//! 
//! ## Send Query
//! There are two kinds of query functions, sending a text query and sending a
//!  functional query which is expressed in general list of q language, each of
//!  which can be sent synchronously or asynchronously.
//! 
//! While TCP connection and TLS connection can be dealt in the same manner to send queries,
//!  sending queries with Unix Domain Socket is handled separately (`send_*_query_*_uds`).

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Load Library                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use super::qtype;
use super::error;
use super::compression;
use super::serialization;
use super::deserialization;
use std::error::Error as stdError;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::fs;
use std::net::{SocketAddr, Shutdown};
use native_tls::TlsConnector;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, AsyncBufRead, BufReader, BufWriter};
use tokio::time;
use tokio_native_tls::TlsStream;
use trust_dns_resolver::AsyncResolver;
use unix_socket::UnixStream;
use chrono::Utc;

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Define Struct                     //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% MessageType %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/
/// How the message should be processed on kdb+ side.
#[derive(Clone, Copy, Debug)]
pub(crate) enum MessageType{
  // `Async` is used to send a query to kdb+ asynchronously
  Async=0,
  // `Sync` is used to send a query to kdb+ synchronously
  Sync=1,
  // `Response` is used by kdb+ to send back the result to a client
  Response=2
}

impl From<u8> for MessageType{
  fn from(enc: u8) -> Self{
    match enc{
      0 => MessageType::Async,
      1 => MessageType::Sync,
      2 => MessageType::Response,
      _ => unreachable!()
    }
  }
}

//%% Encode %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// Passed to query functions to tell if the message should be sent in Big Endian or Little Endian.
#[derive(Clone, Copy, Debug)]
pub(crate) enum Encode{
  BigEndian=0,
  LittleEndian=1
}

impl From<u8> for Encode{
  fn from(enc: u8) -> Self{
    match enc{
      0 => Encode::BigEndian,
      1 => Encode::LittleEndian,
      _ => unreachable!()
    }
  }
}

//%% MsgHeader %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

// Header of q IPC data frame
#[derive(Clone, Copy, Debug)]
pub(crate) struct MsgHeader{
  encode: u8,
  msg_type: u8,
  compressed: u8,
  unused: u8,
  length: u32
}

impl MsgHeader{
  // Construct a header specifying encoding, message type (e.g. async or sync), compression and frame length
  pub(crate) fn new(enc: Encode, msg_t: MessageType, comp: u8, len: u32) -> Self{
    MsgHeader{
      encode: enc as u8,
      msg_type: msg_t as u8,
      compressed: comp,
      unused: 0,
      length: len
    }
  }

  // Default value of message header.
  // * Encode: Little Endian
  // * Message Type: Synchronous
  // * Compressed: 0
  // * Length: 0
  #[allow(dead_code)]
  pub(crate) fn default() -> Self{
    MsgHeader{
      encode: 1,
      msg_type: 1,
      compressed: 0,
      unused: 0,
      length: 0
    }
  }

  pub(crate) async fn from_bytes(raw: &[u8]) -> io::Result<MsgHeader>{

    let mut reader=BufReader::new(raw);

    // Read encoding
    let enc=reader.read_u8().await.expect("Failed to parse encoding");

    // Read message type
    let msg_t=reader.read_u8().await.expect("Failed to parse mesasage type");

    // Read compression flag
    let comp=reader.read_u8().await.expect("Failed to parse compression flag");

    // Read unused bytes
    let _=reader.read_u8().await.expect("Failed to parse unused bytes");

    // Read length
    let len=match enc{
      0 => reader.read_u32().await,
      _ => reader.read_u32_le().await
    }.expect("Failed to parse message length");

    // Build header
    let header=MsgHeader::new(enc.into(), msg_t.into(), comp, len);

    Ok(header)
  }
  
  // Get encoding from the eader
  #[allow(dead_code)]
  pub(crate) fn get_encode(&self) -> u8{
    self.encode
  }

  // Set encoding to the header
  #[allow(dead_code)]
  pub(crate) fn encode(mut self, enc: Encode) -> Self{
    self.encode = enc as u8;
    self
  }

  // Get encoding from the header
  #[allow(dead_code)]
  pub(crate) fn get_msg_type(&self) -> u8{
    self.msg_type
  }

  // Set message type to the header
  #[allow(dead_code)]
  pub(crate) fn msg_type(mut self, msg_t: MessageType) -> Self{
    self.msg_type = msg_t as u8;
    self
  }

  // Get compression flag from the eader
  #[allow(dead_code)]
  pub(crate) fn get_compressed(&self) -> u8{
    self.compressed
  }

  // Set compression flag to the header
  #[allow(dead_code)]
  pub(crate) fn compressed(mut self, comp: u8) -> Self{
    self.compressed = comp;
    self
  }

  // Get length from the eader
  #[allow(dead_code)]
  pub(crate) fn get_length(&self) -> u32{
    self.length
  }

  // Set length of data frame to the header
  #[allow(dead_code)]
  pub(crate) fn length(mut self, len: u32) -> Self{
    self.length = len;
    self
  }

  // Return size of MsgHeader
  pub(crate) fn size() -> usize{
    return 1  // encode
          +1  // msg_type
          +1  // compressed
          +1  // unused
          +4;  // length
  }
}

//%% UnixStreamH %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/// Handle to unix domain socket. Socket file is automatically created and removed.
pub struct UnixStreamH{
  handle: UnixStream,
  sockfile: String
}

impl Drop for UnixStreamH{
  fn drop(&mut self){
    // Remove soket file if it exists
    if Path::new(&self.sockfile).exists(){
      match fs::remove_file(&self.sockfile){
        Ok(_) => (),
        Err(err) => eprintln!("{}", err)
      }
    }
  }
}

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Define Functions                  //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

//%% Connect %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/*
* Implementation of actual connection attempt with the specified timeout configuration.
* @param
* addr: Socket address to try to connect.
* @param
* timeout_millis: Try to connect for this period (millisecond). If this value is set `0`, timeout is disabled
* and response is returned immediately.
* @param
* tral_interval: While trying to connect to q process, each attempt is done in this interval (millisecond).
*/
async fn try_connect(addr: &SocketAddr, timeout_millis: u64, trial_interval: u64) -> io::Result<TcpStream>{
  if timeout_millis > 0{
    // With timeout
    let mut interval = time::interval(time::Duration::from_millis(trial_interval));
    let now=Utc::now();
    loop{
      if let Ok(h) = TcpStream::connect(addr).await{
        // Successfully connected
        return Ok(h);
      }
      else{
        eprintln!("retry to connect...");
        if (Utc::now() - now).num_milliseconds() as u64 > timeout_millis{
          // Timeout
          return Err(io::Error::new(io::ErrorKind::TimedOut, "Connection timeout"));
        }
        interval.tick().await;
      }
    }
  }
  else{
    // Without timeout (immediate response)
    Ok(TcpStream::connect(addr).await.expect("Failed to connect"))
  }
}

/*
* @brief
* Inner function of `connect` to establish TCP connection with the sepcified endpoint with
* specified timeout configuration. The hostname is resolved system DNS resolver to IP address.
* Try to connect to multiple resolved IP addresses until it first succeeds to connect. Error is
* returned if none of them are valid.
* @param
* host: Hostname
* @param
* port: Port number of target q process
* @param
* timeout_millis: Try to connect for this period (millisecond). If this value is set `0`, timeout is disabled
* and response is returned immediately.
* @param
* tral_interval: While trying to connect to q process, each attempt is done in this interval (millisecond).
*/
async fn connect_tcp(host: &str, port: i32, timeout_millis: u64, trial_interval: u64) -> io::Result<TcpStream>{

  // DNS system resolver (should not fail)
  let resolver=AsyncResolver::tokio_from_system_conf().await.expect("Failed to create a resolver");

  // Resolve the given hostname
  let response=resolver.ipv4_lookup(format!("{}.", host).as_str()).await?;
  for ans in response{
    // For DEBUG
    // println!("Got IP adress: {}", ans);
    let hostport=format!("{}:{}", ans, port);
    // Propagate parse error if any
    if let Ok(addr)=hostport.parse::<SocketAddr>(){
      // Return if this IP address is valid
      if let Ok(h)=try_connect(&addr, timeout_millis, trial_interval).await{
        return Ok(h);
      }
    }
    else{
      return Err(io::Error::new(io::ErrorKind::Other, format!("Could not parse host port: {}", hostport)));
    }    
  }

  Err(io::Error::new(io::ErrorKind::ConnectionRefused, format!("Could not find any available endpoint for TCP connection for {}.", host)))
}

/// Connect to q process running on specified `host` and `port` and credential `username:password`.
///  Returned handle is used to send/receive a message to and from the connected q process.
/// # Parameters
/// - `host`: Hostname
/// - `port`: Port number of target q process
/// - `credential`: Credential used to connect to the target q process expressed in `username:password`
/// - `timeout_millis`: Try to connect for this period (millisecond). If this value is set `0`, timeout is disabled
///  and response is returned immediately.
/// - `trial_interval`: While trying to connect to q process, each attempt is done in this interval (millisecond).
/// # Example
/// ```
/// use rustkdb::connection::*;
/// 
/// // Timeout is set 1 second (1000 millisecond) and connection is attempted every 200 millisecond
/// let mut handle=connect("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");
/// ```
pub async fn connect(host: &str, port: i32, credential: &str, timeout_millis: u64, trial_interval: u64) -> Result<TcpStream, Box<dyn stdError>>{

  // Connect to kdb+
  let mut handle=connect_tcp(host, port, timeout_millis, trial_interval).await?;
  
  // Send credential
  let credential=credential.to_string()+"\x03\x00";
  if let Err(err)=handle.write_all(credential.as_bytes()).await{
    return Err(Box::new(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send handshake: {}", err))));
  }
  if let Err(err)=handle.flush().await{
    return Err(Box::new(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err))));
  }

  // Placeholder of common capablility
  let mut cap= [0u8;1];
  if let Err(_)=handle.read_exact(&mut cap).await{
    // Connection is closed in case of authentication failure
    return Err(Box::new(io::Error::new(tokio::io::ErrorKind::ConnectionAborted, "Authentication failure.")));
  }

  Ok(handle)
}

/// TLS version of `connect`.
///  Returned handle is used to send/receive a message to and from the connected q process.
/// # Parameters
/// - `host`: Hostname
/// - `port`: Port number of target q process
/// - `credential`: Credential used to connect to the target q process expressed in `username:password`
/// - `timeout_millis`: Try to connect for this period (millisecond). If this value is set `0`, timeout is disabled
///  and response is returned immediately.
/// - `trial_interval`: While trying to connect to q process, each attempt is done in this interval (millisecond).
/// # Example
/// ```
/// use rustkdb::connection::*;
/// 
/// // Timeout is set 1 second (1000 millisecond) and connection is attempted every 200 millisecond
/// let mut handle=connect_tls("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");
/// ```
pub async fn connect_tls(host: &str, port: i32, credential: &str, timeout_millis: u64, trial_interval: u64) -> Result<TlsStream<TcpStream>, Box<dyn stdError>>{

  // Connect to kdb+
  let handle=connect_tcp(host, port, timeout_millis, trial_interval).await?;
  // Use TLS
  let cx = TlsConnector::builder().build()?;
  let cx = tokio_native_tls::TlsConnector::from(cx);
  let mut handle = cx.connect(host, handle).await?;
  
  // Send credential
  let credential=credential.to_string()+"\x03\x00";
  if let Err(err)=handle.write_all(credential.as_bytes()).await{
    return Err(Box::new(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send handshake: {}", err))));
  }
  if let Err(err)=handle.flush().await{
    return Err(Box::new(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err))));
  }

  // Placeholder of common capablility
  let mut cap= [0u8;1];
  if let Err(_)=handle.read_exact(&mut cap).await{
    // Connection is closed in case of authentication failure
    return Err(Box::new(io::Error::new(tokio::io::ErrorKind::ConnectionAborted, "Authentication failure.")));
  }

  Ok(handle)
}

/// Connect to q process running on specified `port` with Unix Domain Socket using a credential `username:password`.
///  Returned handle is used to send/receive a message to and from the connected q process.
/// # Parameters
/// - `port`: Port number of target q process
/// - `credential`: Credential used to connect to the target q process expressed in `username:password`
/// - `timeout_millis`: Try to connect for this period (millisecond). If this value is set `0`, timeout is disabled
///  and response is returned immediately.
/// # Example
/// ```
/// use rustkdb::connection::*;
/// 
/// // Timeout is set 1 second (1000 millisecond)
/// let mut handle=connect_uds(5000, "kdbuser:pass", 1000).await.expect("Failed to connect");
/// ```
pub async fn connect_uds(port: i32, credential: &str, timeout_millis: u64) -> io::Result<UnixStreamH>{

  // Create file path
  let udspath=match env::var("QUDSPATH"){
    Ok(dir) => format!("{}/kx.{}", dir, port),
    Err(_) => format!("/tmp/kx.{}", port)
  };
  let udspath=udspath;
  let sockfile=Path::new(&udspath);

  // Create the file if necessary
  if !sockfile.exists() {
    println!("Create {}", sockfile.display());
    fs::OpenOptions::new().read(true).write(true).create_new(true).open(&sockfile)?;
  }

  // Bind to the file
  let abs_sockfile=format!("\x00{}", udspath);
  let abs_sockfile=Path::new(&abs_sockfile);
  let mut handle = if timeout_millis > 0{
    UnixStream::connect_timeout(&abs_sockfile, std::time::Duration::from_millis(timeout_millis))?
  }else{
    UnixStream::connect(&abs_sockfile)?
  };

  // Send credential
  let credential=credential.to_string()+"\x06\x00";
  if let Err(err)=handle.write_all(credential.as_bytes()){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send handshake: {}", err)));
  }
  if let Err(err)=handle.flush(){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err)));
  }

  // Placeholder of common capablility
  let mut cap= [0u8;1];
  if let Err(_)=handle.read_exact(&mut cap){
    // Connection is closed in case of authentication failure
    return Err(io::Error::new(tokio::io::ErrorKind::ConnectionAborted, "Authentication failure."));
  }

  Ok(UnixStreamH{handle: handle, sockfile: udspath})
}

/// Close a handle to a q process.
/// # Example
/// ```
/// use rustkdb::connection::*;
/// 
/// // Open connection to a q process
/// let mut handle=connect("localhost", 5000, "kdbuser:pass", 0, 0).await.expect("Failed to connect");
/// 
/// // Close the handle
/// close(&mut handle).await?;
/// ```
pub async fn close(handle: &mut TcpStream) -> io::Result<()>{
  handle.shutdown().await
}

/// Close a handle to a q process which is connected over TLS.
/// # Example
/// ```
/// use rustkdb::connection::*;
/// 
/// // Open connection to a q process
/// let mut handle=connect_tls("localhost", 5000, "kdbuser:pass", 0, 0).await.expect("Failed to connect");
/// 
/// // Close the handle
/// close_tls(&mut handle).await?;
/// ```
pub async fn close_tls(handle: &mut TlsStream<TcpStream>) -> io::Result<()>{
  handle.shutdown().await
}

/// Close a handle to a q process which is connected with Unix Domain Socket.
///  Socket file is removed.
/// # Example
/// ```
/// use rustkdb::connection::*;
/// 
/// // Open connection to a q process
/// let mut handle=connect_uds(5000, "kdbuser:pass", 0).await.expect("Failed to connect");
/// 
/// // Close the handle
/// close_uds(&mut handle).await?;
/// ```
pub async fn close_uds(handle: &mut UnixStreamH) -> io::Result<()>{
  handle.handle.shutdown(Shutdown::Both)?;
  if Path::new(&handle.sockfile).exists(){
    match fs::remove_file(&handle.sockfile){
      Ok(_) => (),
      Err(err) => eprintln!("{}", err)
    }
  }
  Ok(())
}

//%% Send Data %%//vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv/

/*
* @brief
* Receive response from q process with decompression if necessary.
* @param
* reader: Expected to be `BufReader<&mut S>` where S should be `TcpStream` or `TlsStream<TcpStream>`.
* @param
* buf: buffer to read header. This will be shadowed to read body.
*/ 
async fn recieve_response<T>(reader: &mut T, buf: &mut Vec<u8>) -> io::Result<(MsgHeader, Vec<u8>)>
where T: AsyncReadExt + AsyncBufRead + Unpin{

  // Read header
  if let Err(err)=reader.read_exact(buf).await{
    // The expected message is header or EOF (close due to q process failureresulting from bad query)
    return Err(io::Error::new(tokio::io::ErrorKind::ConnectionAborted, format!("Connection dropped: {}", err)));
  }

  // Parse message header (should not fail)
  let msg_header=MsgHeader::from_bytes(buf).await?;

  // Read body
  let body_length=msg_header.get_length() as usize-MsgHeader::size();
  let mut buf=vec![0u8; body_length];
  if let Err(err)=reader.read_exact(&mut buf).await{
    // Fails if q process fails before reading the body
    return Err(io::Error::new(tokio::io::ErrorKind::UnexpectedEof, format!("Failed to read body of message: {}", err)));
  }

  match msg_header.get_compressed(){
    0x01 => Ok((msg_header, compression::decompress(buf.as_slice(), msg_header.get_encode()).await)),
    _ => Ok((msg_header, buf)) 
  }

}

/*
* @brief
* Receive response from q process with decompression if necessary with Unix Domain Socket (`AsyncRead` is not supported).
* @param
* reader: Buffer reader with `UnixStream` an underlying handle
* @param
* buf: buffer to read header. This will be shadowed to read body.
*/ 
async fn recieve_response_uds(reader: &mut std::io::BufReader<&mut UnixStream>, buf: &mut Vec<u8>) -> io::Result<(MsgHeader, Vec<u8>)>{

  // Read header
  if let Err(err)=reader.read_exact(buf){
    // The expected message is header or EOF (close due to q process failureresulting from bad query)
    return Err(io::Error::new(tokio::io::ErrorKind::ConnectionAborted, format!("Connection dropped: {}", err)));
  }

  // Parse message header (should not fail)
  let msg_header=MsgHeader::from_bytes(buf).await?;

  // Read body
  let body_length=msg_header.get_length() as usize-MsgHeader::size();
  let mut buf=vec![0u8; body_length];
  if let Err(err)=reader.read_exact(&mut buf){
    // Fails if q process fails before reading the body
    return Err(io::Error::new(tokio::io::ErrorKind::UnexpectedEof, format!("Failed to read body of message: {}", err)));
  }

  match msg_header.get_compressed(){
    0x01 => Ok((msg_header, compression::decompress(buf.as_slice(), msg_header.get_encode()).await)),
    _ => Ok((msg_header, buf)) 
  }

}

/*
* @brief
* Check the contents of response bytes which were received by `receive_response`. If it is `Err`
* returns the `Err`; otherwise parse the bytes into `Q` and return `Ok(Q)`.
* @param
* reader: BufReader to read underlying bytes response
* @param
* header: Header of response. Used to get encode information in it.
*
* The underlying buffer is the bytes read from a handle and so is independent of connection
*/
async fn inspect_response(reader: &mut BufReader<&[u8]>, header: MsgHeader) -> io::Result<qtype::Q>{

    // Pick up the first byte and see if it is error
    let vectype=reader.read_i8().await.expect("Failed to parse vector type");

    if vectype == qtype::Q_ERROR{
      // Return q process error
      let mut err=String::new();
      reader.read_to_string(&mut err).await?;
      return Err(error::QError::QProcessError(Box::leak(err.into_boxed_str())).into());
    }
    else{
      // Return parsed q object
      let response=deserialization::parse_q(reader, vectype, header.get_encode()).await;
      // For DEBUG - Display q object
      // println!("{}", response);
      Ok(response)
    }
}

/*
* @brief
* Prepare string query with header to send to q process.
* @param
* msg_type: Enum value indicating synchronous query or asynchronous query
* @param
* msg: string query
* @param
* encode: Enum value denoting Big edian or Little Endian
*/ 
async fn send_string_query_prepare_data(msg_type: MessageType, msg: &str, encode: Encode) -> Vec<u8>{

  //  Build header //--------------------------------/
  // Message header + (vector type + vector header) + data size
  let size=(MsgHeader::size()+6+msg.as_bytes().len()) as u32;
  let size_info=match encode{
    Encode::BigEndian => size.to_be_bytes(),
    Encode::LittleEndian => size.to_le_bytes()
  };

  // encode, message type, 0x00 for compression and 0x00 for reserved
  let mut message=vec![encode as u8, msg_type as u8, 0, 0];
  // total body length
  message.extend(&size_info);

  //  Build body //---------------------------------/
  let length_info=match encode{
    Encode::BigEndian => (msg.len() as u32).to_be_bytes(),
    Encode::LittleEndian => (msg.len() as u32).to_le_bytes()
  };

  // vector type and 0x00 for attribute
  message.extend(&[10 as u8, 0]);
  // length of vector(message)
  message.extend(&length_info);
  // message
  message.extend(msg.as_bytes());
 
  message
}

/*
* @brief
* Send a string query to q process synchronously.
* @param
* `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
* @param
* `msg`: String query.
* @param
* `encode`: Enum value denoting Big Endian or Little Endian.
*/
async fn send_string_query<T>(handle: &mut T, msg: &str, encode: Encode) -> io::Result<qtype::Q>
  where T: AsyncReadExt + AsyncWriteExt + Unpin{
  
  // Send string query synchronously
  let message=send_string_query_prepare_data(MessageType::Sync, msg, encode).await;

  let mut writer = BufWriter::new(handle);
  
  // Send data
  if let Err(_)=writer.write_all(&message).await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to send a text query"));
  }
  // Flush
  if let Err(_) = writer.flush().await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to flush a sender handle."));
  }
  
  // Receive data
  let mut reader=BufReader::new(writer.into_inner());
  let mut body: Vec<u8>=vec![0u8; MsgHeader::size()];
  let (msg_header, body) = recieve_response(&mut reader, &mut body).await?;

  // Prepare a new reader of response
  let mut reader=BufReader::new(body.as_slice());

  // Inspect response if it is a kdb+ error; otherwise return teh result
  inspect_response(&mut reader, msg_header).await
  
}

/// Send a string query to q process synchronously in Little Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `msg`: `String` query.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
/// # Eaxmple
/// ```
/// use rustkdb::connection::*;
/// 
/// // Connect to q process
/// let mut handle=connect("localhost", 5000, "kdbuser:pass", 0, 0).await.expect("Failed to connect");
/// // Get a value by a synchronous query
/// let res_int=send_string_query_le(&mut handle, "prd 1 -3 5i").await?;
/// ```
pub async fn send_string_query_le<T>(handle: &mut T, msg: &str) -> io::Result<qtype::Q>
  where T: AsyncReadExt + AsyncWriteExt + Unpin{
  send_string_query(handle, msg, Encode::LittleEndian).await
}

/// Send a string query to q process synchronously in Big Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `msg`: `String` query.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
pub async fn send_string_query_be<T>(handle: &mut T, msg: &str) -> io::Result<qtype::Q>
  where T: AsyncReadExt + AsyncWriteExt + Unpin{
  send_string_query(handle, msg, Encode::BigEndian).await
}

/*
* @brief
* Send a string query to q process synchronously with Unix Domain Socket.
* @param
* `handle`: Handle to q connection. `UnixStreamH`.
* @param
* `msg`: `String` query.
* @param
* `encode`: Enum value denoting Big Endian or Little Endian.
*/
async fn send_string_query_uds(handle: &mut UnixStreamH, msg: &str, encode: Encode) -> io::Result<qtype::Q>{
  
  // Send string query synchronously
  let message=send_string_query_prepare_data(MessageType::Sync, msg, encode).await;

  let mut writer = std::io::BufWriter::new(&mut handle.handle);
  
  // Send data
  if let Err(_)=writer.write_all(&message){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to send a text query"));
  }
  // Flush
  if let Err(_) = writer.flush(){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to flush a sender handle."));
  }
  
  // Receive data
  let mut reader=std::io::BufReader::new(writer.into_inner()?);
  let mut body: Vec<u8>=vec![0u8; MsgHeader::size()];
  let (msg_header, body) = recieve_response_uds(&mut reader, &mut body).await?;

  // Prepare a new reader of response
  let mut reader=BufReader::new(body.as_slice());

  // Inspect response if it is a kdb+ error; otherwise return teh result
  inspect_response(&mut reader, msg_header).await
  
}

/// Send a string query to q process synchronously in Little Endian with Unix Domain Socket.
/// # Parameters
/// - `handle`: Handle to q connection. `UnixStreamH`.
/// - `msg`: String query.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
/// # Eaxmple
/// ```
/// use rustkdb::connection::*;
/// 
/// // Connect to q process
/// let mut handle=connect_uds(5000, "kdbuser:pass", 1000).await.expect("Failed to connect");
/// 
/// // Get a value by a synchronous query
/// let res_int=send_string_query_le_uds(&mut handle, "prd 1 -3 5i").await?;
/// ```
pub async fn send_string_query_le_uds(handle: &mut UnixStreamH, msg: &str) -> io::Result<qtype::Q>{
  send_string_query_uds(handle, msg, Encode::LittleEndian).await
}

/// Send a string query to q process synchronously in Big Endian with Unix Domain Socket.
/// # Parameters
/// - `handle`: Handle to q connection. `UnixStreamH`.
/// - `msg`: String query.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
pub async fn send_string_query_be_uds(handle: &mut UnixStreamH, msg: &str) -> io::Result<qtype::Q>{
  send_string_query_uds(handle, msg, Encode::BigEndian).await
}

/// Send a string query to q process asynchronously in Little Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `msg`: String query.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
/// # Eaxmple
/// ```
/// use rustkdb::connection::*;
/// 
/// // Connect to q process over TLS
/// let mut handle=connect_tls("locahost", 5000, "kdbuser:pass", 1000, 100).await.expect("Failed to connect");
/// 
/// // Set a value 'a' by an asynchronous query
/// send_string_query_async_le(&mut handle, "a:1+2").await?;
/// 
/// // Get a value associated with 'a' by a synchronous query.
/// let res_short=send_string_query_le(&mut handle, "type a").await?;
/// 
/// // -7h
/// println!("{}", res_short);
/// ```
pub async fn send_string_query_async_le<T>(handle: &mut T, msg: &str) -> io::Result<()>
  where T: AsyncWriteExt + Unpin{
  // Send string query asynchronously
  let message=send_string_query_prepare_data(MessageType::Async, msg, Encode::LittleEndian).await;
  
  let mut writer = BufWriter::new(handle);
  
  // Send data
  if let Err(_)=writer.write_all(&message).await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to send a text query"));
  }
  // Flush
  if let Err(_) = writer.flush().await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to flush a sender handle."));
  }

  Ok(())
}

/// Send a string query to q process asynchronously in Big Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `msg`: String query.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
pub async fn send_string_query_async_be<T>(handle: &mut T, msg: &str) -> io::Result<()>
  where T: AsyncWriteExt + Unpin{
  // Send string query asynchronously
  let message=send_string_query_prepare_data(MessageType::Async, msg, Encode::BigEndian).await;

  let mut writer = BufWriter::new(handle);
  
  // Send data
  if let Err(_)=writer.write_all(&message).await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to send a text query"));
  }
  // Flush
  if let Err(_) = writer.flush().await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to flush a sender handle."));
  }

  Ok(())
}

/// Send a string query to q process asynchronously in Little Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `msg`: String query.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
/// # Eaxmple
/// ```
/// use rustkdb::connection::*;
/// 
/// // Connect to q process with Unix Domain Socket
/// let mut handle=connect_uds(5000, "kdbuser:pass", 1000).await.expect("Failed to connect");
/// 
/// // Set a value 'a' by an asynchronous query
/// send_string_query_async_le_uds(&mut handle, "a:1+2").await?;
/// 
/// // Get a value associated with 'a' by a synchronous query.
/// let res_short=send_string_query_le_uds(&mut handle, "type a").await?;
/// 
/// // -7h
/// println!("{}", res_short);
/// ```
pub async fn send_string_query_async_le_uds(handle: &mut UnixStreamH, msg: &str) -> io::Result<()>{
  // Send string query asynchronously
  let message=send_string_query_prepare_data(MessageType::Async, msg, Encode::LittleEndian).await;

  let mut writer = std::io::BufWriter::new(&mut handle.handle);
  
  // Send data
  if let Err(_)=writer.write_all(&message){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to send a text query"));
  }
  // Flush
  if let Err(_) = writer.flush(){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to flush a sender handle."));
  }

  Ok(())
}

/// Send a string query to q process asynchronously in Big Endian with Unix Domain Socket.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `msg`: String query.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
pub async fn send_string_query_async_be_uds(handle: &mut UnixStreamH, msg: &str) -> io::Result<()>{
  // Send string query asynchronously
  let message=send_string_query_prepare_data(MessageType::Async, msg, Encode::BigEndian).await;

  let mut writer = std::io::BufWriter::new(&mut handle.handle);
  
  // Send data
  if let Err(_)=writer.write_all(&message){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to send a text query"));
  }
  // Flush
  if let Err(_) = writer.flush(){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, "Failed to flush a sender handle."));
  }

  Ok(())
}

/*
* @brief
* Prepare a query to q process which is expressed in a mixed list.
* @param
* msg_type: Enum value indicating synchronous query or asynchronous query
* @param
* query: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
* @param
* encode: Enum value denoting Big edian or Little Endian
*/ 
async fn send_query_prepare_data(msg_type: MessageType, query: qtype::Q, encode: Encode) -> io::Result<Vec<u8>>{

  //  Build body //---------------------------------/

  // Serialize Q object
  let mut data: Vec<u8>=Vec::new();
  serialization::serialize_q(&mut data, query, encode as u8).await?;

  //  Build header //-------------------------------/

  let size_info=match encode{
    Encode::BigEndian => (MsgHeader::size() as u32 + data.len() as u32).to_be_bytes(),
    Encode::LittleEndian => (MsgHeader::size() as u32 + data.len() as u32).to_le_bytes()
  };

  let mut message;
  // Compression is trigerred when entire message size is more than 2000 bytes.
  if data.len() > 1992{
    // encode, message type, 0x00 for compression, 0x00 for reserved and 0x00000000 for total size
    message=vec![encode as u8, msg_type as u8, 0, 0, 0, 0, 0, 0];
    message.extend(&data);
    // Try to encode entire message.
    let compressed_message=compression::compress(message.as_slice(), encode as u8).await;
    if compressed_message.len() < message.len() / 2{
      message=compressed_message;
    }
    else{
      // Write total data size
      message[4..8].copy_from_slice(&size_info);
    }
  }
  else{
    // encode, message type, 0x00 for compression and 0x00 for reserved
    message=vec![encode as u8, msg_type as u8, 0, 0];
    // Total length of body
    message.extend(&size_info);
    message.extend(&data);
  }

  Ok(message)
}

/*
* @brief
* Send a string query to q process synchronously.
* @param
* `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
* @param
* `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
* @param
* `encode`: Enum value denoting Big Endian or Little Endian.
*/
async fn send_query<T>(handle: &mut T, query: qtype::Q, encode: Encode) -> io::Result<qtype::Q>
  where T: AsyncReadExt + AsyncWriteExt + Unpin{
  // Send data
  let message=send_query_prepare_data(MessageType::Sync, query, encode).await?;

  // Prepare new buf writer
  let mut writer=BufWriter::new(handle);

  // Send data
  if let Err(err)=writer.write_all(&message).await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send a query: {}", err)));
  }
  // Flush
  if let Err(err) = writer.flush().await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err)));
  }
  
  // Receive data
  let mut reader=BufReader::new(writer.into_inner());
  let mut body: Vec<u8>=vec![0u8; MsgHeader::size()]; 
  let (msg_header, body) = recieve_response(&mut reader, &mut body).await?;

  // Prepare a new reader of response
  let mut reader=BufReader::new(body.as_slice());

  // Inspect response if it is a kdb+ error; otherwise return teh result
  inspect_response(&mut reader, msg_header).await
}

/// Send a string query to q process synchronously in Little Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
/// # Eaxmple
/// ```
/// #[macro_use]
/// extern crate rustkdb;
/// 
/// use rustkdb::qtype::*
/// use rustkdb::connection::*;
/// 
/// // Connect to q process
/// let mut handle=connect("localhost", 5000, "kdbuser:pass", 0, 0).await.expect("Failed to connect");
/// 
/// // Assign some function to 'init' by an asynchronous call.
/// send_string_query_async_be(&mut handle, "init:{[] i:6; while[i-:1; -1 string[i], \"...\"; system \"sleep 1\"]; `Done.}").await?;
/// 
/// // Call 'init' without arguments. This is equivalent to (`init; ::) in q language.
/// let response=send_query_le(&mut handle, q_mixed_list![q_symbol!["init"], q_general_null!["::"]]).await?;
/// 
/// // `Done.
/// println!("{}", response);
/// ```
pub async fn send_query_le<T>(handle: &mut T, query: qtype::Q) -> io::Result<qtype::Q>
  where T: AsyncReadExt + AsyncWriteExt + Unpin{
  send_query(handle, query, Encode::LittleEndian).await
}

/// Send a string query to q process synchronously in Big Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
pub async fn send_query_be<T>(handle: &mut T, query: qtype::Q) -> io::Result<qtype::Q>
  where T: AsyncReadExt + AsyncWriteExt + Unpin{
  send_query(handle, query, Encode::BigEndian).await
}

/*
* @brief
* Send a string query to q process synchronously with Unix Domain Socket.
* @param
* `handle`: Handle to q connection. `UnixStreamH`
* @param
* `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
* @param
* `encode`: Enum value denoting Big Endian or Little Endian.
*/
async fn send_query_uds(handle: &mut UnixStreamH, query: qtype::Q, encode: Encode) -> io::Result<qtype::Q>{
  // Send data
  let message=send_query_prepare_data(MessageType::Sync, query, encode).await?;

  // Prepare new buf writer
  let mut writer=std::io::BufWriter::new(&mut handle.handle);

  // Send data
  if let Err(err)=writer.write_all(&message){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send a query: {}", err)));
  }
  // Flush
  if let Err(err) = writer.flush(){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err)));
  }
  
  // Receive data
  let mut reader=std::io::BufReader::new(writer.into_inner()?);
  let mut body: Vec<u8>=vec![0u8; MsgHeader::size()]; 
  let (msg_header, body) = recieve_response_uds(&mut reader, &mut body).await?;

  // Prepare a new reader of response
  let mut reader=BufReader::new(body.as_slice());

  // Inspect response if it is a kdb+ error; otherwise return teh result
  inspect_response(&mut reader, msg_header).await
}

/// Send a string query to q process synchronously in Little Endian with Unix Domain Socket.
/// # Parameters
/// - `handle`: Handle to q connection. `UnixStreamH`
/// - `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
/// # Eaxmple
/// ```
/// #[macro_use]
/// extern crate rustkdb;
/// 
/// use rustkdb::qtype::*
/// use rustkdb::connection::*;
/// 
/// // Connect to q process
/// let mut handle=connect_uds(5000, "kdbuser:pass", 0).await.expect("Failed to connect");
/// 
/// // Assign some function to 'init' by an asynchronous call.
/// send_string_query_async_be_uds(&mut handle, "init:{[] i:6; while[i-:1; -1 string[i], \"...\"; system \"sleep 1\"]; `Done.}").await?;
/// 
/// // Call 'init' without arguments. This is equivalent to (`init; ::) in q language.
/// let response=send_query_le_uds(&mut handle, q_list![q_symbol!["init"], q_general_null!["::"]]).await?;
/// 
/// // `Done.
/// println!("{}", response);
/// ```
pub async fn send_query_le_uds(handle: &mut UnixStreamH, query: qtype::Q) -> io::Result<qtype::Q>{
  send_query_uds(handle, query, Encode::LittleEndian).await
}

/// Send a string query to q process synchronously in Big Endian with Unix Domain Socket.
/// # Parameters
/// - `handle`: Handle to q connection. `UnixStreamH`.
/// - `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
pub async fn send_query_be_uds(handle: &mut UnixStreamH, query: qtype::Q) -> io::Result<qtype::Q>{
  send_query_uds(handle, query, Encode::BigEndian).await
}

/// Send a string query to q process asynchronously in Little Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
/// # Eaxmple
/// ```
/// #[macro_use]
/// extern crate rustkdb;
/// 
/// use rustkdb::qtype::*
/// use rustkdb::connection::*;
/// 
/// // Connect to q process over TLS
/// let mut handle=connect_tls("localhost", 5000, "kdbuser:pass", 1000, 200).await.expect("Failed to connect");
///  
/// // Call 'set' with arguments `a and 42. This is equivalent to ("set"; `a; 42) in q language.
/// send_query_async_le(&mut handle, q_mixed_list![q_string!['*'; "set"], q_symbol!["a"], q_long![42_i64]]).await?;
/// ```
pub async fn send_query_async_le<T>(handle: &mut T, query: qtype::Q) -> io::Result<()>
  where T: AsyncWriteExt + Unpin{

  // Send data
  let message=send_query_prepare_data(MessageType::Async, query, Encode::LittleEndian).await?;

  // Prepare new buf writer
  let mut writer=BufWriter::new(handle);

  // Send data
  if let Err(err)=writer.write_all(&message).await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send a query: {}", err)));
  }
  // Flush
  if let Err(err) = writer.flush().await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err)));
  }

  Ok(())
}

/// Send a string query to q process asynchronously in Big Endian.
/// # Parameters
/// - `handle`: Handle to q connection. `TcpStream` or `TlsStream<TcpStream>`.
/// - `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
pub async fn send_query_async_be<T>(handle: &mut T, query: qtype::Q) -> io::Result<()>
  where T: AsyncWriteExt + Unpin{

  // Send data
  let message=send_query_prepare_data(MessageType::Async, query, Encode::BigEndian).await?;

  // Prepare new buf writer
  let mut writer=BufWriter::new(handle);

  // Send data
  if let Err(err)=writer.write_all(&message).await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send a query: {}", err)));
  }
  // Flush
  if let Err(err) = writer.flush().await{
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err)));
  }

  Ok(())
}

/// Send a string query to q process asynchronously in Little Endian with Unix Domain Socket.
/// # Parameters
/// - `handle`: Handle to q connection. `UnixStreamH`.
/// - `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
/// # Eaxmple
/// ```
/// #[macro_use]
/// extern crate rustkdb;
/// 
/// use rustkdb::qtype::*
/// use rustkdb::connection::*;
/// 
/// // Connect to q process over TLS
/// let mut handle=connect_uds(5000, "kdbuser:pass", 1000).await.expect("Failed to connect");
///  
/// // Call 'set' with arguments `a and 42. This is equivalent to ("set"; `a; 42) in q language.
/// send_query_async_le_uds(&mut handle, q_mixed_list![q_string!['*'; "set"], q_symbol!["a"], q_long![42_i64]]).await?;
/// ```
pub async fn send_query_async_le_uds(handle: &mut UnixStreamH, query: qtype::Q) -> io::Result<()>{

  // Send data
  let message=send_query_prepare_data(MessageType::Async, query, Encode::LittleEndian).await?;

  // Prepare new buf writer
  let mut writer=std::io::BufWriter::new(&mut handle.handle);

  // Send data
  if let Err(err)=writer.write_all(&message){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send a query: {}", err)));
  }
  // Flush
  if let Err(err) = writer.flush(){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err)));
  }

  Ok(())
}

/// Send a string query to q process asynchronously in Big Endian with Unix Domain Socket.
/// # Parameters
/// - `handle`: Handle to q connection. `UnixStreamH`.
/// - `query`: Query expressed in `Q::MixedL`, i.e. functional query in q terminology.
/// - `encode`: Enum value denoting Big Endian or Little Endian.
pub async fn send_query_async_be_uds(handle: &mut UnixStreamH, query: qtype::Q) -> io::Result<()>{

  // Send data
  let message=send_query_prepare_data(MessageType::Async, query, Encode::BigEndian).await?;

  // Prepare new buf writer
  let mut writer=std::io::BufWriter::new(&mut handle.handle);

  // Send data
  if let Err(err)=writer.write_all(&message){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to send a query: {}", err)));
  }
  // Flush
  if let Err(err) = writer.flush(){
    return Err(io::Error::new(tokio::io::ErrorKind::BrokenPipe, format!("Failed to flush a sender handle: {}", err)));
  }

  Ok(())
}