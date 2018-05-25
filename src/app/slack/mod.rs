/**
 * Since the slack API only sends one single post command, we catch them all here
 * and process them.
 */
use rocket::request::LenientForm;
use rocket_contrib::Json;

// These are the modules for the different commands that can be used
mod help;
mod play;
mod quit;
mod start;

#[derive(FromForm, Debug)]
pub struct SlackCommand {
  pub token: String,
  pub team_id: String,
  pub team_domain: String,
  pub enterprise_id: Option<String>,
  pub enterprise_name: Option<String>,
  pub channel_id: String,
  pub channel_name: String,
  pub user_id: String,
  
  // This field is being fazed out and thus not guaranteed
  pub user_name: Option<String>,

  // This will pretty much always be "/scrabbler"
  pub command: String,

  // This is the arguments to the command
  pub text: String,
  pub response_url: String,
  pub trigger_id: String,
}

#[derive(Serialize)]
pub struct SlackResponse {
  response_type: Option<String>,
  text: String,
}


#[post("/", format="application/x-www-form-urlencoded", data="<data>")]
pub fn post(data: LenientForm<SlackCommand>) -> Json<SlackResponse> {
  let command = data.get();

  let arguments: Vec<&str> = command.text.split(" ").collect();

  if arguments.len() == 0 {
    return Json(help::help(command));
  }

  // Argument type referes the the first arg eg: /scrabbler <play>
  let argument_type = arguments[0];

  let response = match argument_type {
    "help" => help::help(command),
    "play" => play::play(command),
    "quit" => quit::quit(command),
    "start" => start::start(command),
    _ => help::help(command),
  };

  return Json(response)
}
