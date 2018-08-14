/**
 * Since the slack API only sends one single post command, we catch them all here
 * and process them.
 */
use rocket::request::LenientForm;
use rocket_contrib::Json;
use rocket::State;
use super::state::dictionary::ScrabbleDictionary;
use super::guards::db_guard::DbConn;

// These are the modules for the different commands that can be used
pub mod help;
pub mod play;
pub mod quit;
pub mod start;
pub mod hand;
pub mod board;
pub mod dict;

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

impl SlackResponse {
  fn new(text: String, in_channel: bool) -> SlackResponse {
    SlackResponse {
      text: text,
      response_type: if in_channel { Some("in_channel".to_owned()) } else { None }
    }
  }
}


#[post("/", format="application/x-www-form-urlencoded", data="<data>")]
pub fn post(data: LenientForm<SlackCommand>, db: DbConn, dict: State<ScrabbleDictionary>) -> Json<SlackResponse> {
  let command = data.get();

  println!("{:#?}", command);

  let arguments: Vec<&str> = command.text.split(" ").collect();

  if arguments.len() == 0 {
    return Json(help::help().unwrap());
  }

  // refers to the sub command. Eg: for /scrabbler play, the sub command
  // would be play
  let sub_command = arguments[0];

  let response = match sub_command {
    "help" => help::help(),
    "play" => play::play(command, &*db, &*dict),
    "quit" => quit::quit(command, &*db),
    "start" => start::start(command, &*db),
    "hand" => hand::hand(command, &*db),
    "board" => board::board(command, &*db),
    "dict" => dict::dict(command, &*dict),
    _ => help::help(),
  };

  match response {
    Ok(r) => Json(r),
    Err(message) => Json(SlackResponse::new(format!("error: {}", message), false))
  }
}
