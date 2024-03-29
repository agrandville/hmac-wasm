// Copyright 2024 GRANDVILLE Arnaud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// The wasm-pack uses wasm-bindgen to build and generate JavaScript binding file.
// Import the wasm-bindgen crate.
use wasm_bindgen::prelude::*;

use sha2::Sha256;
use hmac::{Hmac, Mac};

extern crate web_sys;

pub fn to_hex_string(bytes: Vec<u8>) -> String {
  let strs: Vec<String> = bytes.iter()
                               .map(|b| format!("{:02X}", b))
                               .collect();
  strs.join(" ")
}


#[wasm_bindgen]
pub fn sha256(secret: String, data: String) -> String {  

  let mut output = format!("data(\"{}\")={}",data,to_hex_string(data.to_owned().into_bytes()));
  web_sys::console::log_1(&output.into());
  
  output = format!("secret(\"{}\")={}",secret,to_hex_string(secret.to_owned().into_bytes()));
  web_sys::console::log_1(&output.into());

  type HmacSha256 = Hmac<Sha256>; 
  let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
    .expect("HMAC can take key of any size");
    
  mac.update(data.as_bytes());
  let result = mac.finalize();

  output = format!("hmac={:x?}", result.to_owned().into_bytes());
  web_sys::console::log_1(&output.to_owned().into());

  return format!("{:x}", result.to_owned().into_bytes());
}
