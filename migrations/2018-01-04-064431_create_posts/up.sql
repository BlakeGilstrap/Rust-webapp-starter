CREATE TABLE users (
  id varchar NOT NULL PRIMARY KEY,
  email varchar NOT NULL,
  username varchar NOT NULL,
  password varchar NOT NULL,
  UNIQUE (email, username)
);

INSERT INTO users (id, email, username, password) VALUES
('2c224422-cfec-4c90-b0be-a6a544f3c6fd', 'zz@163.com', 'zz', 'zz');