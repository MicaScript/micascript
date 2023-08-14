#[derive(Clone)]
pub struct Args {
  pub mica_path: String,
  pub command_name: Option<String>,
  pub options: Vec<String>,
}

#[derive(Clone)]
pub struct CommandParameter {
  pub name: String,
  pub description: String,
}

#[derive(Clone)]
pub struct CommandParameters {
  pub required: Vec<CommandParameter>,
  pub optional: Vec<CommandParameter>,
}

#[derive(Debug)]
pub struct Parameter {
  pub name: String,
  pub value: String,
}

impl CommandParameters {
  pub fn parse_from(self, params: Vec<String>) {
    #[allow(unused_variables)]
    let named_params: Vec<Parameter> = params
      .iter()
      .filter_map(|p| {
        p.starts_with("--").then(|| {
          if let Some(eq_index) = p.find("=") {
            Parameter {
              name: p[2..eq_index].to_string(),
              value: p[eq_index + 1..].to_string(),
            }
          } else {
            Parameter {
              name: p[2..].to_string(),
              value: "true".to_string(),
            }
          }
        })
      })
      .collect();

    // TODO: Continue from here
  }
}

impl Default for CommandParameters {
  fn default() -> CommandParameters {
    CommandParameters {
      required: vec![],
      optional: vec![],
    }
  }
}

#[derive(Clone)]
pub struct Command {
  pub name: String,
  pub description: String,
  pub message: String,
  pub parameters: CommandParameters,
}

pub fn get_commands() -> Vec<Command> {
  vec![
    Command {
      name: format!("help"),
      description: format!("Display the help message"),
      message: format!("MicaScript: a JavaScript compiler and runtime.\n    help [command] - Display the help message\n    help [command] - Display the help message\n    run <file> - Run the file with micascript"),
      parameters: CommandParameters {
        optional: vec![CommandParameter {
          name: format!("command"),
          description: format!("Specify a command to get information about it"),
        }],
        ..Default::default()
      },
    },
    Command {
      name: format!("run"),
      description: format!("Run the file with micascript"),
      message: format!(""),
      parameters: CommandParameters {
        required: vec![CommandParameter {
          name: format!("file"),
          description: format!("Specify a file for MicaScript to execute"),
        }],
        ..Default::default()
      },
    },
  ]
}

pub fn get_command(command_name: String) -> Option<Command> {
  get_commands()
    .iter()
    .find(|cmd| cmd.name.eq(&command_name))
    .cloned()
}

pub fn get_help(command_name: String) -> Option<String> {
  if let Some(command) = get_command(command_name) {
    let required_parameters = if command.parameters.required.is_empty() {
      " ".to_string()
    } else {
      format!(
        "<{}>",
        command
          .parameters
          .required
          .iter()
          .map(|param| param.name.clone())
          .collect::<Vec<String>>()
          .join("|")
      )
    };
    let optional_parameters = if command.parameters.optional.is_empty() {
      "".to_string()
    } else {
      format!(
        "[{}]",
        command
          .parameters
          .optional
          .iter()
          .map(|param| param.name.clone())
          .collect::<Vec<String>>()
          .join("|")
      )
    };

    Some(format!(
      "{}{}{} - {}",
      command.name, required_parameters, optional_parameters, command.description
    ))
  } else {
    None
  }
}

pub fn parse_args(args: Vec<String>) -> Args {
  let args_len = args.len();
  let command_name: Option<String> = if args_len > 1 && !args[1].starts_with("--") {
    Some(args[1].clone())
  } else {
    None
  };

  Args {
    mica_path: args[0].to_owned(),
    command_name: command_name.clone(),
    options: args[if command_name.is_some() { 2 } else { 1 }..].to_vec(),
  }
}
