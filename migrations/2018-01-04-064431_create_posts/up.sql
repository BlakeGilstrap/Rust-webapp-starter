CREATE TABLE users (
  id SERIAL NOT NULL PRIMARY KEY,
  email TEXT NOT NULL,
  username TEXT NOT NULL,
  password TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (email, username)
);

 INSERT INTO users (id, email, username, password, created_at) VALUES
  (1, 'admin@163.com', 'admin', '$2y$12$yXTjrGePVLBPUH6YVs2f5OsUEGSotZxdL5Uu/70r63I5GtynVVjkK', '2017-09-08 13:00:26.353041'),
  (2, 'aaaa@163.com', 'aaaa', '$2y$12$3lOwd/qun2g.KBQpYz7DQu4HgreLODO4aJgYwFAQNj2AqgS14DAMK', '2017-09-08 13:00:28.353041'),
  (3, 'zzzz@163.com', 'zzzz', '$2y$12$6ofSZ3hpsGtDt6bM0WU0geDgZLLETFUVB6FpMXI61SbAvuQD5RiWK', '2017-09-08 13:00:38.353041');
 SELECT setval('users_id_seq', 3, true);

CREATE TABLE  article (
  id SERIAL NOT NULL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  category TEXT NOT NULL,
  title TEXT NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

 INSERT INTO article (id, user_id, category, title, body, created_at) VALUES
 (1, 1, 'Topic',  'Rust Article', 'Rust 2017 Survey Results', '2017-07-24 23:41:45.672805609'),
 (2, 2, 'Article', 'The Rust Libz Blitz','This post covers the library team’s major initiative: raising a solid core of the Rust crate ecosystem to a consistent level of completeness and quality. ', '2017-07-23 23:41:45.672805609'),
 (3, 23, 'FAQ', 'Rust 2017 roadmap', 'This year, the overarching theme is productivity, especially for early-stage Rust users. ', '2017-07-23 23:41:45.672805609'),
 (4, 1, 'Share', 'Incremental Compilation', 'One of the projects that is building on these foundations, and that should help improve compile times a lot for typical workflows, is incremental compilation. ', '2017-07-24 23:41:45.672805609'),
 (5, 2, 'Job', 'Rust jobs','Today we are announcing an alpha version of incremental compilation', '2017-07-23 23:41:45.672805609'),
 (6, 3, 'Announcement', 'Introducing MIR','MIR is the key to ticking off a number of our highest priorities for Rust', '2017-07-23 23:41:45.672805609');
 SELECT setval('article_id_seq', 6, true);