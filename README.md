# Jukebox

An open jukebox to control music from a local network

## Setup instructions :

### Docker :

Docker is the recommended way to run this project.

The attached docker compose file has to be filled with the appropriate api keys to be used. The docker container must be run in privileged mode to use the sound card with the container.

`git clone https://github.com/pinsarda/jukebox-backend.git`
`cd jukebox-backend`
`docker build . -t jukebox`
`cd ..`

This repository only provides the server side of the project. A compatible UI is required for practical use, see [jukebox-frontend](https://github.com/pinsarda/jukebox-frontend). 

`git clone https://github.com/pinsarda/jukebox-frontend.git`
`cd jukebox-frontend`
`docker build . -t jukebox-frontend`

Now that the docker images are built, make sure to edit `docker-compose.yaml` with your own configuration. Without proper api keys, the server won't start.

`docker compose up`

#### Run only the api container :

```
docker run --privileged --device /dev/snd --network=jukebox -p 8080:8080 --volume /path/to/storage:/jukebox/Storage --name jukebox \
-e DATABASE_URL=postgres://postgres:mysecretpassword@jukebox-postgres:5432/diesel_demo \
-e RUST_LOG=debug \
-e RUST_BACKTRACE=0 \
-e YOUTUBE_API_KEY="YOUR KEY" \
-e YOUTUBE_MUSIC_COOKIE="YOUR COOKIE" \
-e MUSICAPI_TOKEN="Token YOUR TOKEN" \
-e PLAYER_DISABLED=0 \
jukebox
```

### Natively

#### Building from source

Alternatively, you can build from source.
yt-dlp, ffmpeg and a postgres database are required. A .env file can contain the environment variables at the root of the project.

`git clone https://github.com/pinsarda/jukebox-backend.git`
`cd jukebox-backend`
`cargo build --release`

To build [jukebox-frontend](https://github.com/pinsarda/jukebox-frontend) sources :

`git clone https://github.com/pinsarda/jukebox-frontend.git`
`cd jukebox-frontend`
`npm install`
`npm run build`

Then you can now host the `dist/` folder statically using a server of your choice.

## Documentation

API docs are accessible at [https://localhost:8080/swagger-ui/](https://localhost:8080/swagger-ui/)

### Nginx configuration

For a fully working deployment, see the docker-compose file. To run the web interface, http://yourwebinterface.domain/api must resolve to the port 8080 of your docker container. In the proposed docker compose, it is achieved through nginx with this configuration at ./nginx/nginx.conf :

```
events {
    worker_connections  1024;
}

http {
    access_log  /etc/nginx/nginx.log;
    server {
        listen 80;
        server_name jukebox;

        location / {
            proxy_redirect  http://jukebox-frontend:80/  /;
            proxy_pass http://jukebox-frontend:80;
        }
        
        location /api/ {
            proxy_redirect  http://jukebox:8080/  /api/;
            proxy_pass http://jukebox:8080/;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header Upgrade $http_upgrade;
            proxy_set_header Connection "upgrade";
        }

    }
}
```