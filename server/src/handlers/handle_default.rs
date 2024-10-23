use super::WebsocketHandlingError;
use crate::handlers::utils::get_channel_addr;
use artnet::ArtNetClient;
use config::{Binding, Fixture, KeyframesMode};
use logger::log;
use std::{collections::HashMap, sync::Arc};

pub fn handle(
    msg: &str,
    client: Arc<ArtNetClient>,
    fixtures: Arc<HashMap<String, Fixture>>,
    bindings: Arc<HashMap<String, Binding>>,
) -> Result<(), WebsocketHandlingError> {
    let binding = bindings
        .get(msg)
        .ok_or_else(|| WebsocketHandlingError::UnknownMessage(msg.to_string()))?;

    match binding {
        Binding::WithActions(bnd) => {
            for action in bnd.get_actions() {
                let parts: Vec<&str> = action[0].split('.').take(2).collect();
                let [fixture_name, channel_name] = match parts.as_slice() {
                    [fixture_name, channel_name] => [fixture_name, channel_name],
                    _ => {
                        panic!("Invalid action format: {}", action[0]);
                    }
                };
                let addr = get_channel_addr(fixture_name, channel_name, &fixtures)?;
                let value: u8 = action[1]
                    .parse()
                    .map_err(|e| WebsocketHandlingError::ParseVariableToNumError(e))?;
                log!("Setting channel {} to value {}", addr, value);
                client.set_single(addr, value);
                return client.commit().map_err(WebsocketHandlingError::IoError);
            }
        }
        Binding::WithKeyframes(bnd) => match bnd.get_mode() {
            KeyframesMode::Once => {
                for step in bnd.get_steps().iter() {
                    std::thread::sleep(std::time::Duration::from_millis(step.get_delay()));
                    for action in step.get_actions() {
                        let parts: Vec<&str> = action[0].split('.').take(2).collect();
                        let [fixture_name, channel_name] = match parts.as_slice() {
                            [fixture_name, channel_name] => [fixture_name, channel_name],
                            _ => {
                                panic!("Invalid action format: {}", action[0]);
                            }
                        };
                        let addr = get_channel_addr(fixture_name, channel_name, &fixtures)?;
                        let value: u8 = action[1]
                            .parse()
                            .map_err(|e| WebsocketHandlingError::ParseVariableToNumError(e))?;
                        log!("Setting channel {} to value {}", addr, value);
                        client.set_single(addr, value);
                    }
                    client.commit().map_err(WebsocketHandlingError::IoError)?;
                }
            }
            KeyframesMode::Alernate => {
                for step in bnd.get_steps().iter() {
                    std::thread::sleep(std::time::Duration::from_millis(step.get_delay()));
                    for action in step.get_actions() {
                        let parts: Vec<&str> = action[0].split('.').take(2).collect();
                        let [fixture_name, channel_name] = match parts.as_slice() {
                            [fixture_name, channel_name] => [fixture_name, channel_name],
                            _ => {
                                panic!("Invalid action format: {}", action[0]);
                            }
                        };
                        let addr = get_channel_addr(fixture_name, channel_name, &fixtures)?;
                        let value: u8 = action[1]
                            .parse()
                            .map_err(|e| WebsocketHandlingError::ParseVariableToNumError(e))?;
                        log!("Setting channel {} to value {}", addr, value);
                        client.set_single(addr, value);
                    }
                    client.commit().map_err(WebsocketHandlingError::IoError)?;
                }

                for step in bnd.get_steps().iter().rev().skip(1) {
                    std::thread::sleep(std::time::Duration::from_millis(step.get_delay()));
                    for action in step.get_actions() {
                        let parts: Vec<&str> = action[0].split('.').take(2).collect();
                        let [fixture_name, channel_name] = match parts.as_slice() {
                            [fixture_name, channel_name] => [fixture_name, channel_name],
                            _ => {
                                panic!("Invalid action format: {}", action[0]);
                            }
                        };
                        let addr = get_channel_addr(fixture_name, channel_name, &fixtures)?;
                        let value: u8 = action[1]
                            .parse()
                            .map_err(|e| WebsocketHandlingError::ParseVariableToNumError(e))?;
                        log!("Setting channel {} to value {}", addr, value);
                        client.set_single(addr, value);
                    }
                    client.commit().map_err(WebsocketHandlingError::IoError)?;
                }
            }
        },
    }

    Ok(())
}
