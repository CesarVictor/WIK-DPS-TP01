use crate::handler::handle_client;
use std::env;
use std::io::Result;
use std::net::TcpListener;

pub(crate) struct Api {
    addr: String,
}

impl Api {
    pub fn new() -> Self {
        let port: String = env::var("PING_LISTEN_PORT").unwrap_or_else(|_| "8080".to_string());
        let addr: String = format!("0.0.0.0:{}", port);
        Self { addr }
    }

    pub fn start(&self) -> Result<()> {
        let listener: TcpListener =
            TcpListener::bind(&self.addr).expect("Impossible de démarrer le serveur");

        println!("Serveur en écoute sur {}", self.addr);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Erreur lors du traitement de la requête : {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Erreur de connexion : {}", e);
                }
            }
        }

        Ok(())
    }
}
