
use proc_macro;
use proc_macro::TokenStream;

use std::path::{PathBuf};

enum Program {
  Java {
    classpath: Vec<String>,
    classname: String,
  },
  Python {
    pythonpath: Vec<String>,
    modulename: String,
  },
  Binary {
    libpath: Vec<String>,
    binary: PathBuf,
  }
}


#[proc_macro]
pub fn system(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}
