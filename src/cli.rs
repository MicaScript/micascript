#[derive(Clone)]
pub struct Parameter {
  pub name: String,
  pub value: Option<String>,
}

impl Default for Parameter {
  fn default() -> Parameter {
    Parameter {
      name: format!(""),
      value: None,
    }
  }
}

#[derive(Clone)]
pub struct Parameters {
  pub required: Vec<Parameter>,
  pub optional: Vec<Parameter>,
  pub options: Vec<Parameter>,
}

impl Default for Parameters {
  fn default() -> Parameters {
    Parameters {
      required: Vec::new(),
      optional: Vec::new(),
      options: Vec::new(),
    }
  }
}

#[derive(Clone)]
pub struct Args {
  pub mica_path: String,
  pub command_name: Option<String>,
  pub parameters: Vec<Parameter>,
}

#[derive(Clone)]
pub struct CommandParameter {
  pub name: String,
  pub description: String,
  pub aliases: Vec<String>,
}

impl Default for CommandParameter {
  fn default() -> CommandParameter {
    CommandParameter {
      name: format!(""),
      description: format!(""),
      aliases: Vec::new(),
    }
  }
}

#[derive(Clone)]
pub struct CommandParameters {
  pub required: Vec<CommandParameter>,
  pub optional: Vec<CommandParameter>,
  pub options: Vec<CommandParameter>,
}

impl Default for CommandParameters {
  fn default() -> CommandParameters {
    CommandParameters {
      required: Vec::new(),
      optional: Vec::new(),
      options: Vec::new(),
    }
  }
}

impl CommandParameters {
  pub fn parse_from(self, params: Vec<Parameter>) -> Parameters {
    let mut required: Vec<Parameter> = Vec::new();
    let mut optional: Vec<Parameter> = Vec::new();

    params
      .iter()
      .cloned()
      .filter(|param| !param.name.starts_with("-"))
      .enumerate()
      .for_each(|(i, param)| {
        if self.required.len() > i {
          required.push(Parameter {
            name: self.required[i].name.clone(),
            value: Some(param.name),
          });
        } else if self.optional.len() + self.required.len() > i {
          let j = i - self.required.len();

          optional.push(Parameter {
            name: self.optional[j].name.clone(),
            value: Some(param.name),
          });
        }
      });

    Parameters {
      required: required.to_owned(),
      optional: optional.to_owned(),
      options: params
        .iter()
        .cloned()
        .filter(|p| {
          if p.name.starts_with("-") {
            self
              .options
              .iter()
              .any(|param| param.name.eq(&p.name.clone()) || param.aliases.contains(&p.name))
          } else {
            false
          }
        })
        .collect(),
    }
  }
}

#[derive(Clone)]
pub struct Command {
  pub name: String,
  pub description: String,
  pub aliases: Vec<String>,
  pub message: Option<String>,
  pub parameters: CommandParameters,
}

impl Default for Command {
  fn default() -> Command {
    Command {
      name: format!(""),
      description: format!(""),
      aliases: Vec::new(),
      message: None,
      parameters: CommandParameters {
        ..Default::default()
      },
    }
  }
}

pub fn get_commands() -> Vec<Command> {
  vec![
    Command {
      name: format!("help"),
      description: format!("Display the help message"),
      aliases: vec![format!("-h"), format!("--help")],
      message: Some(format!("MicaScript: a JavaScript compiler and runtime.\n\n    help [command] - Display the help message\n    run <file> - Run the file with MicaScript\n\n<> - required | [] - optional")),
      parameters: CommandParameters {
        optional: vec![CommandParameter {
          name: format!("command"),
          description: format!("Specify a command to get information about it"),
          ..Default::default()
        }],
        ..Default::default()
      },
    },
    Command {
      name: format!("run"),
      description: format!("Run the file with MicaScript"),
      parameters: CommandParameters {
        required: vec![CommandParameter {
          name: format!("file"),
          description: format!("Specify a file for MicaScript to execute"),
          ..Default::default()
        }],
        ..Default::default()
      },
      ..Default::default()
    },
  ]
}

pub fn get_command(command_name: String) -> Option<Command> {
  get_commands()
    .iter()
    .find(|cmd| cmd.name.eq(&command_name) || cmd.aliases.contains(&command_name))
    .cloned()
}

pub fn get_help(command_name: String) -> Option<String> {
  if let Some(command) = get_command(command_name) {
    if let Some(message) = command.message {
      Some(message)
    } else {
      let required_parameters = if command.parameters.required.is_empty() {
        "".to_string()
      } else {
        format!(
          " <{}>",
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
          "{}[{}]",
          if required_parameters.is_empty() {
            " ".to_string()
          } else {
            "".to_string()
          },
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
    }
  } else {
    None
  }
}

fn parse_option(option: String) -> Parameter {
  // let is_option = option.starts_with("--");
  // let is_short_option = !is_option && option.starts_with("-");

  // let slice_index = 0 + is_option as usize + is_short_option as usize;

  if let Some(eq_index) = option.find("=") {
    Parameter {
      name: option[..eq_index].to_string(),
      value: Some(option[eq_index + 1..].to_string()),
    }
  } else {
    Parameter {
      name: option.to_string(),
      value: None,
    }
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
    parameters: args[if command_name.is_some() { 2 } else { 1 }..]
      .iter()
      .map(|param| {
        if param.starts_with("-") {
          parse_option(param.to_owned())
        } else {
          Parameter {
            name: param.to_owned(),
            ..Default::default()
          }
        }
      })
      .collect(),
  }
}
