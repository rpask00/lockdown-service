-- Add migration script here
Alter TABLE users ADD COLUMN salt VARCHAR(255) NOT NULL default '';
