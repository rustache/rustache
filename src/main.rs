extern crate rustache;

use std::os;
use std::io::stdio::stdout;

#[allow(dead_code)]
fn main() {
  let args = os::args();
  if args.len() < 3 {
  	println!("usage: rustache <template file> <json file>");
  	return;
  }

  let path = Path::new(args[2].clone());
  let rv = rustache::render_file(args[1].as_slice(), path);
  match rv {
  	Ok(mut reader) => { 

  		match reader.read_to_string() {
  			Err(err) => { println!(" rustache: {}", err); },
  			Ok(text) => { 
  				match stdout().write_str(text.as_slice()) {
  				_ => { /* don't care */ }
  				}
  			}
  		}
  	},
  	Err(err) => { println!(" rustache: {}", err); }
  }
}
