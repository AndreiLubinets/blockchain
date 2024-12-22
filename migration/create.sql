create table blocks(
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  'from' TEXT,
  'to' TEXT,
  value TEXT,
  hash TEXT UNIQUE
);

