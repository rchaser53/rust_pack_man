use std::{path, fmt, process, result};

use sdl2::video::WindowBuildError;
use sdl2::messagebox::{show_simple_message_box, MESSAGEBOX_ERROR};

use constants::FILE_PATHS;
use mixer_music::play_sound_effect;

#[derive(Debug)]
pub enum GameOverError {
    OtherError(&'static str),
    HitEnemy(&'static str)
}
impl fmt::Display for GameOverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameOverError::OtherError(ref e) => write!(f, "{}", e.to_string()),
            GameOverError::HitEnemy(e) => write!(f, "{}", e.to_string())
        }
    }
}
impl Error for GameOverError {
    fn description(&self) -> &str {
        match *self {
            GameOverError::OtherError(error_message) => error_message,
            GameOverError::HitEnemy(error_message) => error_message
        }
    }
    fn cause(&self) -> Option<&Error> { None }
}

pub type Result<T> = result::Result<T, CustomError>;
trait Error: fmt::Debug + fmt::Display {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&Error>;
}

#[derive(Debug)]
pub enum CustomError {
    ParseWindowBuildError(WindowBuildError),
    ParseString(String),
    ParseGameOverError(GameOverError)
}

impl From<GameOverError> for CustomError {
    fn from(err: GameOverError) -> CustomError {
        play_sound_effect(path::Path::new(FILE_PATHS.get("HIT_EFFECT_PATH").unwrap()));
        show_message("title", err.description());
        CustomError::ParseGameOverError(err)
    }
}

impl From<WindowBuildError> for CustomError {
    fn from(err: WindowBuildError) -> CustomError {
        CustomError::ParseWindowBuildError(err)
    }
}

impl From<String> for CustomError {
    fn from(err: String) -> CustomError {
        CustomError::ParseString(err)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result<> {
        match *self {
            CustomError::ParseWindowBuildError(_) => write!(f, "nya-n"),
            CustomError::ParseGameOverError(_) => { process::exit(1); },
            CustomError::ParseString(ref e) => panic!("{}", e),
        }
    }
}

pub fn show_message(title: &str, message: &str) {
  let _ = show_simple_message_box(
              MESSAGEBOX_ERROR, title,
              message, None
          );
}