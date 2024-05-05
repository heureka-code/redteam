# Redteam-demo application
This application demonstrates how sql injections work by providing an unsafe page to play with.

## Setup
Only docker compose is required to run the application.
The api needs to know where to locate the backend. Therefore you have to set the environment variable "HOSTNAME" at build time, so when running ``docker compose build``. You can do this for example by adding a file called ``.env`` next to ``docker-compose.yml``:
```env
HOSTNAME=...
IP=...
```
(IP is optional)

This name will also be added as allowed origin to the backend (if IP is given it's also added) so this hostname must be used to access the frontend, e. g. https://YOUR-HOSTNAME.
The hostname is used instead of an IP or a domain.

Also you have to generate the needed files for TLS support. In the following the commands for generating a self-signed certificate from within a linux shell are displayed (executed from the dir that contains ``docker-compose.yml`` and the other directories):
```shell
mkdir nginx-tls # if it doesn't already exist

openssl req -x509 -nodes -days 365 -newkey rsa:2048 -keyout nginx-tls/nginx-selfsigned.key -out nginx-tls/nginx-selfsigned.crt

openssl dhparam -out nginx-tls/dhparam.pem 4096
```
## Building and running the app
Now (and everytime the environment variables change), you have to build the application with:
```shell
docker compose build
```
This will take a while and needs access to the internet for installing everything that's needed.

From now on you can run the app with:
```shell
docker compose up
```
and stop it with:
```shell
docker compose down
```

## Good to know!
Everytime the backend container boots (after crashed or on ``docker compose up``) the tables of the database will be recreated and the passwords of all users will change.

The app will listen on port 443 (HTTPS). Further explainations and the exercises to complete are integrated with the app and details about the database schema.
