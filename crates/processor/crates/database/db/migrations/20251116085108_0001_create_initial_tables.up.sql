CREATE TABLE Categories (
  category_id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  type TEXT NOT NULL CHECK (type IN ('expense', 'income'))
);

CREATE TABLE Accounts (
  account_id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  initial_balance INTEGER NOT NULL DEFAULT 0,
  currency TEXT NOT NULL DEFAULT 'JPY'
);

CREATE TABLE Transactions (
  transaction_id INTEGER PRIMARY KEY AUTOINCREMENT,
  date TEXT NOT NULL CHECK (
    date GLOB '[0-9][0-9][0-9][0-9]-[0-1][0-9]-[0-3][0-9]'
  ),
  amount INTEGER NOT NULL,
  currency TEXT NOT NULL DEFAULT 'JPY',
  memo TEXT,
  category_id INTEGER NOT NULL,
  account_id INTEGER NOT NULL,
  FOREIGN KEY (category_id) REFERENCES Categories (category_id),
  FOREIGN KEY (account_id) REFERENCES Accounts (account_id)
);
