#![feature(duration_constants)]

extern crate thrussh;
extern crate thrussh_keys;
extern crate tokio;

mod colours;
mod users;

use crate::colours::Colours;
use crate::users::User;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thrussh::server;
use thrussh::server::{Auth, Session};
use thrussh::{ChannelId, CryptoVec};
use thrussh_keys::key;

const BANNER: &'static str = r#"
Welcome to the Rustacean SSH Chat.

If you get a SSH Permission denied or asked for a password you must generate a ssh key pair.

"#;

#[tokio::main]
async fn main() {
    let client_key = thrussh_keys::key::KeyPair::generate_ed25519().unwrap();
    let client_pubkey = Arc::new(client_key.clone_public_key());

    let mut config = thrussh::server::Config::default();

    config.connection_timeout = Some(std::time::Duration::from_secs(3));
    config.auth_banner = Some(BANNER);
    config.auth_rejection_time = std::time::Duration::from_secs(3);
    config
        .keys
        .push(thrussh_keys::key::KeyPair::generate_ed25519().unwrap());

    let config = Arc::new(config);

    let sh = Server {
        client_pubkey,
        clients: Arc::new(Mutex::new(HashMap::new())),
        id: 0,
    };

    let host = "0.0.0.0:2222";
    println!("Staring server on ssh://{}", host);
    let _ = tokio::time::timeout(
        std::time::Duration::MAX,
        thrussh::server::run(config, host, sh),
    )
    .await
    .unwrap_or(Ok(()));
}

#[derive(Clone)]
struct Server<'a> {
    client_pubkey: Arc<thrussh_keys::key::PublicKey>,
    clients: Arc<Mutex<HashMap<User<'a>, thrussh::server::Handle>>>,
    id: usize,
}

impl<'a> server::Server for Server<'a> {
    type Handler = Self;
    fn new(&mut self, s: Option<std::net::SocketAddr>) -> Self {
        println!("New connection from tcp://{}", s.unwrap());
        let s = self.clone();
        self.id += 1;
        s
    }
}

impl<'a> server::Handler for Server<'a> {
    type Error = anyhow::Error;
    type FutureAuth = futures::future::Ready<Result<(Self, server::Auth), anyhow::Error>>;
    type FutureUnit = futures::future::Ready<Result<(Self, Session), anyhow::Error>>;
    type FutureBool = futures::future::Ready<Result<(Self, Session, bool), anyhow::Error>>;

    fn finished_auth(self, auth: Auth) -> Self::FutureAuth {
        futures::future::ready(Ok((self, auth)))
    }

    fn finished_bool(self, b: bool, s: Session) -> Self::FutureBool {
        futures::future::ready(Ok((self, s, b)))
    }

    fn finished(self, s: Session) -> Self::FutureUnit {
        futures::future::ready(Ok((self, s)))
    }

    fn channel_open_session(self, channel: ChannelId, session: Session) -> Self::FutureUnit {
        {
            let mut clients = self.clients.lock().unwrap();

            let colour: Colours = rand::random();
            let colour = colour.value();
            let user = User::new(self.id, channel, &"none", colour);

            clients.insert(user, session.handle());
        }

        self.finished(session)
    }

    fn auth_publickey(self, user: &str, _key: &key::PublicKey) -> Self::FutureAuth {
        self.finished_auth(server::Auth::Accept)
    }

    fn data(self, channel: ChannelId, data: &[u8], mut session: Session) -> Self::FutureUnit {
        {
            let mut clients = self.clients.lock().unwrap();
            for (user, ref mut s) in clients.iter_mut() {
                if user.id != self.id {
                    s.data(user.channel, CryptoVec::from_slice(data));
                }
            }
        }
        session.data(channel, CryptoVec::from_slice(data));
        self.finished(session)
    }
}
