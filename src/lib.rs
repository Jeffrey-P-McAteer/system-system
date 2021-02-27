
use proc_macro;
use proc_macro::TokenStream;

use std::path::{PathBuf};
use std::process::{Command, Stdio};

use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{
  parse_macro_input,
  braced,
  parse::ParseBuffer,
  ExprArray,
  ExprBlock,
  Block,
  Expr,
  Lit,
  LitStr,
  Ident,
  Token,
  PatWild,
  Type,
};

// These follow the system!() arguments
// for different embedded program types.
#[derive(Debug)]
enum Program {
  Java {
    build_cwd: String,
    build_cmd: Vec<String>,

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
  },
}

mod kw {
  syn::custom_keyword!(java);
}

struct ProgramDef {
  type_name: String,
  programs: Vec<Program>
}

impl Parse for ProgramDef {
  fn parse(input: ParseStream) -> Result<Self> {
      
    let type_name: Ident = input.parse()?;

    input.parse::<Token![,]>()?;

    let mut programs = vec![];

    loop {
      if input.is_empty() {
        break;
      }

      let lookahead = input.lookahead1();
      if lookahead.peek(kw::java) {
        input.parse::<kw::java>()?;

        let block_content: ParseBuffer;
        let brace_token = braced!(block_content in input);

        let build_cwd = block_content.parse::<LitStr>()?;

        block_content.parse::<Token![,]>()?;

        let build_cmd_tokens = block_content.parse::<ExprArray>()?;
        let mut build_cmd = vec![];
        for elm in build_cmd_tokens.elems {
          if let Expr::Lit(expr_lit) = elm {
            if let Lit::Str(l_str) = expr_lit.lit {
              build_cmd.push(l_str.value());
            }
          }
        }

        block_content.parse::<Token![,]>()?;
        
        let classpath_tokens = block_content.parse::<ExprArray>()?;
        let mut classpath = vec![];
        for elm in classpath_tokens.elems {
          if let Expr::Lit(expr_lit) = elm {
            if let Lit::Str(l_str) = expr_lit.lit {
              classpath.push(l_str.value());
            }
          }
        }

        block_content.parse::<Token![,]>()?;

        let classname = block_content.parse::<LitStr>()?;

        if let Err(_e) = block_content.parse::<Token![,]>() {
          // ignore a trailing comma, those are fine.
        }

        let mut p = Program::Java {
          build_cwd: build_cwd.value(),
          build_cmd: build_cmd,
          classpath: classpath,
          classname: classname.value(),
        };
        programs.push(p);
      }
      else {
        return Err(syn::Error::new(input.span(), "expected one of `java`, `python`, or `bin`"));
      }
    }

    Ok(ProgramDef {
        type_name: type_name.to_string(),
        programs: programs,
    })
  }
}

#[proc_macro]
pub fn system(item: TokenStream) -> TokenStream {
    let prog: ProgramDef = parse_macro_input!(item as ProgramDef);

    // Iterate programs, build app directory
    for p in prog.programs {
      println!("[BUILD] got program {:?}", p);
      match p {
        Program::Java{build_cwd, build_cmd, classpath, classname} => {
          let cmd_s = Command::new(&build_cmd[0])
                        .args(&build_cmd[1..])
                        .stdin(Stdio::inherit())
                        .stdout(Stdio::inherit())
                        .stderr(Stdio::inherit())
                        .current_dir(&build_cwd)
                        .status()
                        .expect("Could not run process to compile java classes");

          if !cmd_s.success() {
            eprintln!("[BUILD] Error running: {:?} process returned {:?}", &build_cmd, cmd_s.code());
            panic!("system-system cannot continue because a sub-process failed");
          }


        }
        unk => {
          println!("[BUILD] Error, unhandled program p={:?}", unk);
        }
      }
    }

    

    let struct_def = format!(r#"
      pub struct {type_name};

      impl {type_name} {{
        pub fn extract(&mut self) {{
          println!("[RUN] TODO extract() data");
        }}
        pub fn run(&mut self) {{
          println!("[RUN] TODO run()");
        }}
      }}

      impl Default for {type_name} {{
        fn default() -> Self {{
          {type_name}
        }}
      }}
    "#,
      type_name=prog.type_name
    );

    struct_def.as_str().parse().unwrap()
}
