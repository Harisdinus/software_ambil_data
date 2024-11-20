use reqwest::{Client, Response};
use serde::Serialize;
use crate::shared::{Globals, Result};

pub struct Servo;

impl Servo {
    pub async fn geterin(global: &Globals) -> Result<()> {
        reqwest::get(global.use_url("/geterin")).await?;
        Ok(())
    }

    pub async fn reset(global: &Globals) -> Result<()> {
        let data = ServoReset {
            perintah: "servo_reset".into(),
        };
        Servo::fetch(&global.use_url("/perintah"), &data).await?;
        Ok(())
    }

    pub async fn up(value: i32, global: &Globals) -> Result<String> {
        let data = ServoMove {
            perintah: "servo_up".into(),
            unit: 0,
            value,
        };
        let response = Servo::fetch(&global.use_url("/perintah"), &data).await?;
        Ok(response.text().await?)
    }

    pub async fn down(value: i32, global: &Globals) -> Result<String> {
        let data = ServoMove {
            perintah: "servo_down".into(),
            unit: 0,
            value,
        };
        let response = Servo::fetch(&global.use_url("/perintah"), &data).await?;
        Ok(response.text().await?)
    }

    async fn fetch<D: Serialize>(href: &str, data: &D) -> Result<Response> {
        Client::new()
            .post(href)
            .json(data)
            .send()
            .await
            .map_err(Into::into)
    }
}

#[derive(Serialize)]
struct ServoMove {
    perintah: String,
    unit: i32,
    value: i32,
}

#[derive(Serialize)]
struct ServoReset {
    perintah: String,
}

