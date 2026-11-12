# eAI email auth API

Small TypeScript API for email and password signup/login. Accounts are stored in SQLite, passwords are hashed with bcrypt, and successful signup/login returns a signed seven-day bearer token.

## Start locally

```bash
cp .env.example .env
```

Set `JWT_SECRET` in `.env` to a private random value of at least 32 characters. For example, generate one with `openssl rand -base64 48`. Then run:

```bash
npm install
npm run dev
```

The API listens on port 3000 by default. For a production build, use `npm run build` and `npm start`. Keep `.env` and the SQLite database private, use HTTPS when exposed outside localhost, and set `CLIENT_ORIGIN` to the exact client origin when deploying.

## API

- `POST /auth/signup` — JSON `{ "email": "person@example.com", "password": "at-least-8-chars" }`; returns `{ user, token }`.
- `POST /auth/login` — same JSON; returns `{ user, token }`.
- `GET /auth/me` — requires `Authorization: Bearer <token>`; returns the current public user.
- `GET /health` — liveness check.

Example signup:

```bash
curl -X POST http://localhost:3000/auth/signup \
  -H 'Content-Type: application/json' \
  -d '{"email":"person@example.com","password":"correct-horse-battery"}'
```

This backend does not send verification or password-reset emails yet. It is a separate service and is not wired into the existing Rust desktop login screen.
