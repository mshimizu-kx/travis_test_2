// compression.rs

// This module provides a method to compress kdb+ IPC message.

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Load Library                      //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

use std::io;
use tokio::io::{AsyncReadExt, BufReader};

//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//
//                     Define Functions                  //
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++//

/*
* Compress body. The combination of serializing the data and compressing will result in
* the same output as shown in the q language by using the -18! function e.g.
* serializing 2000 bools set to true, then compressing, will have the same output as -18!2000#1b
*/
pub(crate) async fn compress(raw: &[u8], encode: u8) -> Vec<u8> {
  
  let mut i = 0_u8;
  let mut f = 0_u8;
  let mut h0 = 0_usize;
  let mut h = 0_usize;
  let mut g: bool;
  let mut compressed = vec![0_u8; (raw.len()) / 2];

  // Write size of raw bytes including a header
  let compressed_size=match encode{
    0 => (compressed.len() as u32).to_be_bytes(),
    _ => (compressed.len() as u32).to_le_bytes()
  };
  compressed[4..8].copy_from_slice(&compressed_size);
  
  // Start index of compressed body
  // 12 bytes are reserved for the header + size of raw bytes 
  let mut c = 12;
  let mut d = c;
  let e = compressed.len();
  let mut p = 0_usize;
  let mut q: usize;
  let mut r: usize;
  let mut s0 = 0_usize;
  
  // Body starts from index 8
  let mut s = 8_usize;
  let t = raw.len();
  let mut a =[0_i32; 256];

  // Copy encode, message type, compressed and reserved
  compressed[0..4].copy_from_slice(&raw[0..4]);
  // Set compressed flag
  compressed[2]=1;
  
  // Write size of raw bytes including a header
  let raw_size=match encode{
    0 => (t as u32).to_be_bytes(),
    _ => (t as u32).to_le_bytes()
  };
  compressed[8..12].copy_from_slice(&raw_size);

  while s < t {
    if i == 0 {
      if d > e-17 {
        // Early return when compressing to less than half failed
        return raw.to_vec();
      }
      i = 1;
      compressed[c] = f;
      c = d;
      d += 1;
      f = 0;
    }
    g = s > t-3;
    if !g {
      h = (raw[s] ^ raw[s+1]) as usize;
      p = a[h] as usize;
      g = (0 == p) || (0 != (raw[s] ^ raw[p]));
    }
    if 0 < s0 {
      a[h0] = s0 as i32;
      s0 = 0;
    }
    if g {
      h0 = h;
      s0 = s;
      compressed[d] = raw[s];
      d += 1;
      s += 1;
    }
    else {
      a[h] = s as i32;
      f |= i;
      p += 2;
      s += 2;
      r = s;
      q = if s+255 > t {t}else{s+255};
      while (s < q) && (raw[p] == raw[s]) {
        s += 1;
        if s < q {
          p += 1;
        }
      }
      compressed[d] = h as u8;
      d += 1;
      compressed[d] = (s - r) as u8;
      d += 1;
    }
    i=i.wrapping_mul(2);
  }
  compressed[c] = f;
  let compressed_size=match encode{
    0 => (d as u32).to_be_bytes(),
    _ => (d as u32).to_le_bytes()
  };
  compressed[4..8].copy_from_slice(&compressed_size);
  return compressed[0..d].to_vec();
}

/*
* Decompress body. The combination of decompressing and deserializing the data
* will result in the same output as shown in the q language by using the -19! function.
*/
pub(crate) async fn decompress(compressed: &[u8], encode: u8) -> Vec<u8>{

  let mut reader=BufReader::new(compressed);

  let mut n=0;
  let mut r: usize;
  let mut f=0_usize;

  // Header has already been removed.
  // Start index of decompressed bytes is 0
  let mut s=0_usize;
  let mut p = s;
  let mut i = 0_usize;

  // Reduce decoded bytes size by 8 bytes as 8 bytes are already taken as header
  let size=match encode{
    0 => reader.read_i32().await,
    _ => reader.read_i32_le().await
  }.expect("Failed to read size of compressed data")-8;
  let mut decompressed = vec![0u8; size as usize];

  // Start index of compressed body.
  // 8 bytes have already been removed as header
  let mut d=4;
  let mut aa= [0_i32; 256];
  while s < decompressed.len() {
    if i == 0 {
      f = (0xff & compressed[d]) as usize;
      d+=1;
      i = 1;
    }
    if (f & i) != 0{
      r = aa[(0xff & compressed[d]) as usize] as usize;
      d+=1;
      decompressed[s] = decompressed[r];
      s+=1;
      r+=1;
      decompressed[s] = decompressed[r];
      s+=1;
      r+=1;
      n = (0xff & compressed[d]) as usize;
      d+=1;
      for m in 0..n{
        decompressed[s+m] = decompressed[r+m];
      }
    }
    else{
      decompressed[s] = compressed[d];
      s+=1;
      d+=1;
    }
    while p < s-1 {
      aa[((0xff & decompressed[p])^(0xff & decompressed[p+1])) as usize] = p as i32;
      p+=1;
    }
    if (f & i) != 0 {
      s += n;
      p = s;
    }
    i *= 2;
    if i == 256 {
      i = 0;
    }
  }
  return decompressed;
}