-- Add down migration script here

-- Revert "CREATE EXTENSION IF NOT EXISTS "uuid-ossp";"
DROP EXTENSION IF EXISTS "uuid-ossp";
