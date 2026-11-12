import "dotenv/config";
import bcrypt from "bcryptjs";
import cors from "cors";
import Database from "better-sqlite3";
import express, { type ErrorRequestHandler } from "express";
import rateLimit from "express-rate-limit";
import helmet from "helmet";
import jwt from "jsonwebtoken";
import { mkdirSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { z } from "zod";

const port = Number(process.env.PORT ?? 3000);
const jwtSecret = process.env.JWT_SECRET;
if (!jwtSecret || jwtSecret.length < 32) {
  throw new Error("JWT_SECRET must be configured with at least 32 characters.");
}

const databasePath = resolve(process.env.DATABASE_PATH ?? "./data/accounts.sqlite");
mkdirSync(dirname(databasePath), { recursive: true });
const db = new Database(databasePath);
db.pragma("journal_mode = WAL");
db.pragma("foreign_keys = ON");
db.exec(`
  CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE COLLATE NOCASE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
  );
`);

const app = express();
app.disable("x-powered-by");
app.use(helmet());
app.use(cors({
  origin: process.env.CLIENT_ORIGIN || false,
  methods: ["GET", "POST"],
  allowedHeaders: ["Content-Type", "Authorization"],
}));
app.use(express.json({ limit: "16kb" }));

const authLimiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  limit: 20,
  standardHeaders: "draft-8",
  legacyHeaders: false,
  message: { error: "Too many authentication attempts. Try again later." },
});

const credentialsSchema = z.object({
  email: z.string().trim().email().max(254),
  password: z.string().min(8).max(128),
}).strict();

type UserRow = { id: string; email: string; password_hash: string; created_at: string };
type TokenPayload = { sub: string };

function issueToken(user: UserRow): string {
  return jwt.sign({}, jwtSecret!, { subject: user.id, expiresIn: "7d", issuer: "eai-auth" });
}

function publicUser(user: UserRow) {
  return { id: user.id, email: user.email, createdAt: user.created_at };
}

function parseCredentials(body: unknown, res: express.Response) {
  const result = credentialsSchema.safeParse(body);
  if (!result.success) {
    res.status(400).json({ error: "Enter a valid email and a password of 8–128 characters." });
    return null;
  }
  return { email: result.data.email.toLowerCase(), password: result.data.password };
}

app.get("/health", (_req, res) => res.json({ status: "ok" }));

app.post("/auth/signup", authLimiter, async (req, res, next) => {
  try {
    const credentials = parseCredentials(req.body, res);
    if (!credentials) return;

    const existing = db.prepare("SELECT id FROM users WHERE email = ?").get(credentials.email);
    if (existing) {
      res.status(409).json({ error: "An account with this email already exists." });
      return;
    }

    const user: UserRow = {
      id: crypto.randomUUID(),
      email: credentials.email,
      password_hash: await bcrypt.hash(credentials.password, 12),
      created_at: new Date().toISOString(),
    };
    try {
      db.prepare("INSERT INTO users (id, email, password_hash) VALUES (?, ?, ?)")
        .run(user.id, user.email, user.password_hash);
    } catch (error) {
      // The unique index also handles two simultaneous signups for one address.
      if (error instanceof Error && error.message.includes("UNIQUE constraint failed")) {
        res.status(409).json({ error: "An account with this email already exists." });
        return;
      }
      throw error;
    }
    res.status(201).json({ user: publicUser(user), token: issueToken(user) });
  } catch (error) {
    next(error);
  }
});

app.post("/auth/login", authLimiter, async (req, res, next) => {
  try {
    const credentials = parseCredentials(req.body, res);
    if (!credentials) return;
    const user = db.prepare("SELECT * FROM users WHERE email = ?")
      .get(credentials.email) as UserRow | undefined;
    // Keep the response identical for unknown email and wrong password.
    if (!user || !(await bcrypt.compare(credentials.password, user.password_hash))) {
      res.status(401).json({ error: "Invalid email or password." });
      return;
    }
    res.json({ user: publicUser(user), token: issueToken(user) });
  } catch (error) {
    next(error);
  }
});

app.get("/auth/me", (req, res) => {
  const authorization = req.header("authorization");
  const token = authorization?.startsWith("Bearer ") ? authorization.slice(7) : "";
  try {
    const payload = jwt.verify(token, jwtSecret!, { issuer: "eai-auth" }) as TokenPayload;
    const user = db.prepare("SELECT * FROM users WHERE id = ?").get(payload.sub) as UserRow | undefined;
    if (!user) {
      res.status(401).json({ error: "Invalid or expired session." });
      return;
    }
    res.json({ user: publicUser(user) });
  } catch {
    res.status(401).json({ error: "Invalid or expired session." });
  }
});

const errors: ErrorRequestHandler = (error, _req, res, _next) => {
  console.error("Request failed:", error);
  res.status(500).json({ error: "Internal server error." });
};
app.use(errors);

app.listen(port, "0.0.0.0", () => {
  console.log(`eAI auth API listening on http://localhost:${port}`);
});
