use crate::shared::Result;

pub struct Servo;

impl Servo {
    pub async fn command(cmd: ServoCommand) -> Result<String> {
        let url = format!("http://localhost:3000{}", cmd.path());
        let response = reqwest::get(url).await?;
        Ok(response.text().await?)
    }
}

pub enum ServoCommand {
    Up,
    Down,
    Reset,
}

impl ServoCommand {
    fn path(&self) -> &'static str {
        match self {
            ServoCommand::Up => "/up",
            ServoCommand::Down => "/down",
            ServoCommand::Reset => "/reset"
        }
    }
}

