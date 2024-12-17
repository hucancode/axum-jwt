# Hello Axum!
This is an example web application build with `axum` and `surrealdb`
# Run the app
Rename `.env.example` to `.env`, default values work just fine but you can adjust the variables if needed.
Then start `surrealdb` with
```bash
# cp .env.example .env
docker compose up -d
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
```bash
curl 'http://0.0.0.0:8080/api/profile' \
  -H 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJiZWZjM2NiNC00MTVjLTRmNjUtYTRhOS0zNzM4MDFiMzNiZTciLCJpYXQiOjE2ODI2Njg0MDMsImV4cCI6MTY4MjY3MjAwM30.xH2D5vRXAHFe17fRnDWJD4vGAm8IWAMNi-1bHCVtzRc'
```
_The token after `Bearer` is the result of the previous `login` command_

When you are done, shutdown server instance with
```bash
docker compose down
```
