use crate::game::entity::{Player, PlayerId};
use crate::game::MainPlayer;
use crate::input::InputHandler;
use crate::network::{NetworkHandler, RemoteInfo};
use crate::render::renderer::Renderer;

use core::events::{ClientEvent, ServerEvent};
use core::world::{World, WorldCoordinate};
use std::collections::HashMap;
use std::io;
use std::time::Instant;

pub const NETWORK_UPDATE_TIMEOUT: u128 = 50;

#[derive(Debug, Clone)]
pub enum GameType {
    Local,
    Remote { info: RemoteInfo },
}

pub struct Game {
    game_type: GameType,
    world: Option<World>,
    player: MainPlayer,
    players: HashMap<PlayerId, Player>,
    renderer: Renderer,
    network: Option<NetworkHandler>,
    last_network_update: Instant,
}

impl Game {
    pub fn new(game_type: GameType) -> io::Result<Self> {
        let player = MainPlayer::new(WorldCoordinate {
            x: 0.0,
            y: 150.0,
            z: 0.0,
        });

        Ok(match game_type.clone() {
            GameType::Local => Self {
                game_type,
                world: Some(World::new()),
                player,
                players: HashMap::new(),
                renderer: Renderer::new(0, 0),
                network: None,
                last_network_update: Instant::now(),
            },
            GameType::Remote { info } => Self {
                game_type,
                world: None,
                player,
                players: HashMap::new(),
                renderer: Renderer::new(0, 0),
                network: (|| {
                    let network = NetworkHandler::new(info).ok()?;
                    network.send(ClientEvent::PlayerConnect);
                    Some(network)
                })(),
                last_network_update: Instant::now(),
            },
        })
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.renderer.resize(width, height);
        self.player
            .camera
            .set_aspect_ratio(width as f32 / height as f32);
    }

    pub fn render(&self) {
        let pals = self.players.values().collect::<Vec<&Player>>();
        self.renderer.draw(&self.player.camera, &pals)
    }

    pub fn update(&mut self, time_delta: f64, input_handler: &InputHandler) {
        self.poll_network();
        self.update_world(time_delta, input_handler);
        self.update_network();
    }

    fn update_world(&mut self, time_delta: f64, input_handler: &InputHandler) {
        self.player.update(time_delta, &input_handler);

        for (_, player) in self.players.iter_mut() {
            player.update();
        }

        if let Some(world) = self.world.as_mut() {
            world.load_around(vec![self.player.position()]);
            self.renderer.update(&world, &input_handler);
        }
    }

    fn update_network(&mut self) {
        if let Some(network) = self.network.as_ref() {
            if self.last_network_update.elapsed().as_millis() >= NETWORK_UPDATE_TIMEOUT {
                network.send(ClientEvent::PlayerMove {
                    position: self.player.position(),
                });

                self.last_network_update = Instant::now();
            }
        }
    }

    fn poll_network(&mut self) {
        if self.network.is_none() {
            return;
        }

        if let Ok(events) = self.network.as_ref().unwrap().process() {
            for event in events {
                match event {
                    ServerEvent::PlayerConnected { id } => {
                        self.players.insert(id, Player::new(id));
                    }
                    ServerEvent::PlayerDisconnected { id } => {
                        self.players.remove(&id);
                    }
                    ServerEvent::PlayerMoved { id, position } => {
                        if let Some(player) = self.players.get_mut(&id) {
                            player.set_position(position);
                        }
                    }
                    ServerEvent::ServerInfo { seed, player_ids } => {
                        self.world = Some(World::from_seed(seed));

                        for id in player_ids.iter() {
                            self.players.insert(*id, Player::new(*id));
                        }
                    }
                };
            }
        } else {
            println!("<game> could not process network events");
        }
    }

    fn send_event(&self, event: ClientEvent) {
        if let Some(network) = self.network.as_ref() {
            network.send(event);
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        self.send_event(ClientEvent::PlayerDisconnect);
    }
}
