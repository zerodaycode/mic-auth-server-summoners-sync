--! Note that sql script file is intented to be used only in a local-dev enviroment!

-- Switch to the zerodaycode database
\connect zerodaycode;

-- Create the schema
CREATE SCHEMA IF NOT EXISTS summonerssync;

-- Set the schema search path
SET search_path TO summonerssync;

-- Create the roles table
CREATE TABLE IF NOT EXISTS roles (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) UNIQUE NOT NULL,
    description TEXT
);

-- Create the users table
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Create the user_roles table
CREATE TABLE IF NOT EXISTS user_roles (
    user_id INT NOT NULL,
    role_id INT NOT NULL,
    PRIMARY KEY (user_id, role_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
);

-- Insert default roles only if the table is empty
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM roles) THEN
        INSERT INTO roles (name, description) VALUES 
        ('ADMIN', 'Full access to the system'),
        ('USER', 'Limited access to the system'),
        ('DEVELOPER', 'External contributor with restricted access');
    END IF;
END $$;
