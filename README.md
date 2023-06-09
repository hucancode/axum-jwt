# Hello Axum!
This is an example web application build with `axum`
# Run the app
Rename `.env.example` to `.env`, default values work just fine by you can adjust the variables if needed.
Then start `surrealdb` with
```bash
docker compose up -d # to spin up a surrealdb instance
```
Start web server
```bash
cargo run
```
# Use the app
Register new user with
```bash
curl -X POST 'http://0.0.0.0:8080/api/register' \
  -d '{"email":"admin@admin.com","name":"Admin","password":"123456"}' \
  -H 'Content-Type: application/json'
```
Login with
```bash
curl -X POST 'http://0.0.0.0:8080/api/login' \
  -d '{"email":"admin@admin.com","password":"123456"}' \
  -H 'Content-Type: application/json'
```
Access protected resource (for example user profile) with
The token after `Bearer` is the result of the previous `login` command
```bash
curl 'http://0.0.0.0:8080/api/profile' \
  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiZWZjM2NiNC00MTVjLTRmNjUtYTRhOS0zNzM4MDFiMzNiZTciLCJpYXQiOjE2ODI2Njg0MDMsImV4cCI6MTY4MjY3MjAwM30.xH2D5vRXAHFe17fRnDWJD4vGAm8IWAMNi-1bHCVtzRc'
```
When you are done, shutdown `surrealdb` instance with
```bash
docker compose down
```

