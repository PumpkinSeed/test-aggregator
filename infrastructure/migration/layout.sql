--
-- Table structure for table api_keys
--

DROP TABLE IF EXISTS api_keys;
CREATE TABLE api_keys (
  id SERIAL PRIMARY KEY,
  key varchar(255) UNIQUE DEFAULT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  expires_at timestamp DEFAULT CURRENT_TIMESTAMP
);

--
-- Table structure for table custom_filters
--

DROP TABLE IF EXISTS custom_filters;
CREATE TABLE custom_filters (
  id SERIAL PRIMARY KEY,
  name varchar(255) DEFAULT NULL,
  user_id varchar(255) DEFAULT NULL,
  triggered_by varchar(255) DEFAULT NULL,
  commit_hash varchar(255) DEFAULT NULL,
  status varchar(255) DEFAULT NULL,
  started_after bigint DEFAULT NULL,
  started_before bigint DEFAULT NULL,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--
-- Table structure for table results
--

DROP TABLE IF EXISTS results;
CREATE TABLE results (
  id varchar(50) NOT NULL,
  triggered_by varchar(50) DEFAULT NULL,
  branch_name varchar(250) DEFAULT NULL,
  start_timestamp bigint DEFAULT NULL,
  end_timestamp bigint DEFAULT NULL,
  commit_hash varchar(50) DEFAULT NULL,
  status varchar(50) DEFAULT NULL,
  error_message varchar(250) DEFAULT NULL,
  short_description varchar(250) DEFAULT NULL,
  payload_data text DEFAULT NULL,
  payload_text text,
  sequence_number SERIAL,
  invalid boolean DEFAULT '0',
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);

--
-- Table structure for table statistics
--

DROP TABLE IF EXISTS statistics;
CREATE TABLE statistics (
  id SERIAL PRIMARY KEY,
  result_id varchar(50) DEFAULT NULL,
  start_timestamp bigint DEFAULT NULL,
  method varchar(255) DEFAULT NULL,
  endpoint varchar(255) DEFAULT NULL,
  measurement bigint DEFAULT NULL,
  invalid boolean DEFAULT '0',
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP
);

--
-- Table structure for table users
--

DROP TABLE IF EXISTS users;
DROP TYPE IF EXISTS user_role;
CREATE TYPE user_role AS ENUM('admin','user');
CREATE TABLE users (
  id char(36) NOT NULL,
  password_hash varchar(255) DEFAULT NULL,
  email varchar(255) unique DEFAULT NULL,
  git_user varchar(255) DEFAULT NULL,
  role user_role DEFAULT 'user',
  notification boolean DEFAULT NULL,
  updated_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  created_at timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);
