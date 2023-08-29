use deno_core::error::AnyError;
use deno_core::op;
use deno_core::op2;
use deno_core::Extension;
use deno_core::Op;
use deno_core::{serde_json, serde_json::json};
// use deno_core::Snapshot;
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::path;
use std::rc::Rc;
use std::time::SystemTime;

pub mod cli;

#[op2]
#[string]
fn op_get_version() -> Result<String, AnyError> {
  Ok(env!("CARGO_PKG_VERSION").to_string())
}

#[op2]
#[string]
fn op_get_v8_version() -> Result<String, AnyError> {
  Ok(deno_core::v8_version().to_string())
}

#[op]
fn op_get_args() -> Result<Vec<String>, AnyError> {
  Ok(env::args().collect::<Vec<String>>())
}

#[op2]
#[serde]
fn op_get_env() -> Result<serde_json::Value, AnyError> {
  Ok(json!(env::vars().collect::<HashMap<String, String>>()))
}

#[op2(fast)]
#[smi]
fn op_get_time_in_nanos() -> Result<u128, AnyError> {
  let duration_since_epoch = SystemTime::now()
    .duration_since(SystemTime::UNIX_EPOCH)
    .unwrap();

  Ok(duration_since_epoch.as_nanos())
}

#[op]
async fn op_read_file(path: String) -> Result<String, AnyError> {
  let contents = tokio::fs::read_to_string(path).await?;
  Ok(contents)
}

#[op]
async fn op_write_file(path: String, contents: String) -> Result<(), AnyError> {
  tokio::fs::write(path, contents).await?;
  Ok(())
}

#[op]
fn op_remove_file(path: String) -> Result<(), AnyError> {
  std::fs::remove_file(path)?;
  Ok(())
}

#[op]
fn op_read_dir(dir: Option<String>) -> Result<Vec<String>, AnyError> {
  let folders = std::fs::read_dir(dir.unwrap_or(".".to_string()))?;
  let folder_names = folders
    .filter_map(|entry| {
      entry.ok().and_then(|e| {
        e.path()
          .file_name()
          .and_then(|n| n.to_str().map(|s| String::from(s)))
      })
    })
    .collect::<Vec<String>>();

  Ok(folder_names)
}

// static RUNTIME_SNAPSHOT: &[u8] = include_bytes!("../cache/MICASCRIPT_SNAPSHOT.bin");

async fn execute_file(file_path: path::PathBuf) -> Result<(), AnyError> {
  let current_dir = env::current_dir()?;
  let main_module = deno_core::resolve_path(file_path.to_str().unwrap(), current_dir.as_path())?;

  // Extensions
  let info_extension = Extension {
    name: "information",
    ops: Cow::Borrowed(&[
      op_get_version::DECL,
      op_get_v8_version::DECL,
      op_get_args::DECL,
      op_get_env::DECL,
      op_get_time_in_nanos::DECL,
    ]),
    ..Default::default()
  };
  let fs_extension = Extension {
    name: "file-system",
    ops: Cow::Borrowed(&[
      op_read_file::DECL,
      op_write_file::DECL,
      op_remove_file::DECL,
      op_read_dir::DECL,
    ]),
    ..Default::default()
  };

  // Runtime
  let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
    module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
    // startup_snapshot: Some(Snapshot::Static(RUNTIME_SNAPSHOT)),
    extensions: vec![info_extension, fs_extension],
    ..Default::default()
  });

  js_runtime
    .execute_script(
      "runtime",
      deno_core::FastString::Static(include_str!("./runtime/runtime.js")),
    )
    .unwrap();

  let mod_id = js_runtime.load_main_module(&main_module, None).await?;
  let result = js_runtime.mod_evaluate(mod_id);
  js_runtime.run_event_loop(false).await?;
  result.await?
}

fn main() {
  // let args = cli::parse_args(std::env::args());
  let args = cli::parse_args();

  match &args.command {
    Some(cli::Commands::Run(args)) => {
      let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

      // TODO: add esm support
      #[allow(unused_variables)]
      let is_esm = args.es_module || args.file_path.extension().unwrap_or_default().eq("mjs");

      if let Err(error) = runtime.block_on(execute_file(args.file_path.to_owned())) {
        eprintln!("{}", error);
      }
    }
    None => {}
  }

  // let command_name = args.arguments.into_iter().find_map(|arg| match arg {
  //   cli::ArgumentType::Argument { value } => Some(value),
  //   _ => None,
  // });

  // if let Some(command_name) = command_name {
  //   // There is a command

  //   if let Some(command) = cli::get_command(command_name) {
  //     let parameters = command
  //       .arguments
  //       .parse_from(args.arguments[1..].into_iter());

  //     if command_name.eq("help") {
  //       println!(
  //         "{}",
  //         cli::get_help(
  //           parameters
  //             .optional
  //             .iter()
  //             .find_map(|param| param
  //               .name
  //               .eq("command")
  //               .then_some(param.to_owned().value.unwrap()))
  //             .unwrap_or(command_name)
  //         )
  //         .unwrap()
  //       )
  //     } else if command_name.eq("run") {
  //       if let Some(file_path) = parameters
  //         .required
  //         .iter()
  //         .find(|param| param.name == "file")
  //       {
  //         let runtime = tokio::runtime::Builder::new_current_thread()
  //           .enable_all()
  //           .build()
  //           .unwrap();

  //         // FIXME: Check file is exists

  //         if let Err(error) =
  //           runtime.block_on(execute_file(file_path.value.clone().unwrap().as_str()))
  //         {
  //           eprintln!("{}", error);
  //         }
  //       } else {
  //         println!("{}", cli::get_help(command_name).unwrap())
  //       }
  //     }
  //   } else {
  //     // There is not a known command
  //     println!(
  //       "No command named \"{command_name}\" found!\n\
  //       \n\
  //       Type \"mica help\" for get command list."
  //     )
  //   }
  // } else {
  //   // There is not a command
  //   println!("{}", cli::get_help("help".to_string()).unwrap())
  // }

  std::process::exit(0);
}
