use reqwest::{Client, multipart};
use strum_macros::EnumIter;
use serde_json::Value;
use std::io::{Error as IoError, ErrorKind};

// Washer 1 is closest to the door, then 2, 3 and so on
#[derive(Debug, Clone, Copy, EnumIter, PartialEq, Eq)]
pub enum LaundryMachine {
    Washer1,
    Washer2,
    Washer3,
    Washer4,
    Washer5,
    Dryer,
}

impl LaundryMachine {
    pub fn goods_id(&self) -> &'static str {
        match self {
            LaundryMachine::Washer1 => "1101161253",
            LaundryMachine::Washer2 => "1100860858",
            LaundryMachine::Washer3 => "1101161254",
            LaundryMachine::Washer4 => "1100860895",
            LaundryMachine::Washer5 => "1101161255",
            LaundryMachine::Dryer => "1101161256",
        }
    }

    
pub async fn is_free(
    &self,
    client: &Client,
) -> Result<bool, Box<dyn std::error::Error>> {
    let form = multipart::Form::new().text("goodsId", self.goods_id());

    let response = client
        .post("https://userapi.qiekj.com/goods/normal/details")
        .header("Accept", "application/json, text/plain, */*")
        .header("Origin", "https://h5.qiekj.com")
        .header("Referer", "https://h5.qiekj.com/")
        .header("phoneBrand", "DEFAULT")
        .header("uid", "undefined")
        .multipart(form)
        .send()
        .await?
        .error_for_status()?;

    let body = response.text().await?;
    let json: Value = serde_json::from_str(&body)?;

    let invalid_response = || {
        IoError::new(ErrorKind::InvalidData, "Unexpected laundry API response")
    };

    // The API itself must report success.
    if json.get("code").and_then(Value::as_i64) != Some(0) {
        return Err(invalid_response().into());
    }

    let data = json.get("data").ok_or_else(invalid_response)?;

    // Make sure we received information about the machine we requested.
    let expected_id: u64 = self.goods_id().parse()?;
    if data.get("goodsId").and_then(Value::as_u64) != Some(expected_id) {
        return Err(invalid_response().into());
    }

    let error_message = data
        .get("deviceErrorMsg")
        .ok_or_else(invalid_response)?;

    // A known occupied-state message.
    if error_message.as_str()
        == Some("设备工作中，请更换设备使用")
    {
        return Ok(false);
    }

    let error_code = data
        .get("deviceErrorCode")
        .ok_or_else(invalid_response)?;

    let sold_state = data
        .get("soldState")
        .and_then(Value::as_i64);

    // A known free-state pattern from the response we inspected.
    if error_message.is_null()
        && error_code.is_null()
        && sold_state == Some(1)
    {
        return Ok(true);
    }

    // An unfamiliar state must never be silently labeled free.
    Err(invalid_response().into())
}

}
