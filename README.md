# API

Ce projet implémente un serveur HTTP en Rust.
Le serveur écoute sur un port configurable, défini par la variable d'environnement PING_LISTEN_PORT (par défaut : 8080).

## Fonctionnalités


**GET /ping**

Cette route retourne les entêtes HTTP de la requête sous forme de JSON. 

Exemple de réponse :

```json
{
    "headers": {
        "sec-ch-ua": "\"Chromium\";v=\"134\", \"Not:A-Brand\";v=\"24\", \"Google Chrome\";v=\"134\"",
        "Connection": "keep-alive",
        "Sec-Fetch-Dest": "document",
        "Upgrade-Insecure-Requests": "1",
        "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36",
    }
}
```

**Requête non supportée** : Toute autre requête retourne une erreur 404.

## Prérequis

Avant de commencer, assurez-vous d'avoir Rust installé sur votre machine. Si ce n'est pas encore fait, vous pouvez suivre les instructions d'installation sur le [site officiel](https://www.rust-lang.org/tools/install).

## Installation

Pour installer le serveur, exécutez la commande suivante :

1. Cloner le repository  :
```bash
git clone https://github.com/CesarVictor/WIK-DPS-TP01.git
cd WIK-DPS-TP01
```

2. Lancer le serveur :
```bash
cargo run
``` 

Par défaut, le serveur écoute sur le port 8080. Pour changer le port, vous pouvez définir la variable d'environnement PING_LISTEN_PORT :

```bash
export PING_LISTEN_PORT=8081
cargo run
```

ou directement :

```bash
PING_LISTEN_PORT=8081 cargo run
```

3. Tester le serveur :

Une fois le serveur lancé, vous pouvez accéder à la route /ping en utilisant un navigateur ou un outil comme curl :

```bash
curl http://localhost:8080/ping
``` 

Requête non supportée :

```bash
curl http://localhost:8080/autre
```

Sortie attendue :

```bash
➜  ~ curl -v http://localhost:3000/pong
*   Trying 127.0.0.1:3000...
* Connected to localhost (127.0.0.1) port 3000 (#0)
> GET /pong HTTP/1.1
> Host: localhost:3000
> User-Agent: curl/7.81.0
> Accept: */*
>
* Mark bundle as not supporting multiuse
< HTTP/1.1 404 Not Found
< Content-Length: 0
<
* Connection #0 to host localhost left intact
```
