-- Your SQL goes here
CREATE TABLE url (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	short_url TEXT NOT NULL UNIQUE,
	long_url TEXT NOT NULL,
	created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  expires_at DATETIME,
  access_count INTEGER DEFAULT 0
)