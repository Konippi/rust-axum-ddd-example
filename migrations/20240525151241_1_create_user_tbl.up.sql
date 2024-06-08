/* 
    In PostgreSQL, user is a reserved word, so give it double quotations.
    Also, table names should not be plural because SeaORM automatically generates Entity.
*/
CREATE TABLE IF NOT EXISTS "user" (
    id SERIAL,
    email VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT users_id_pk PRIMARY KEY (id)
);
