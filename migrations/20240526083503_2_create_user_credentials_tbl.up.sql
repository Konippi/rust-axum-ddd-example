CREATE TABLE IF NOT EXISTS user_credentials (
    user_id SERIAL NOT NULL,
    cerdenials VARCHAR(255) NOT NULL UNIQUE,
    CONSTRAINT user_credentials_user_id_pk PRIMARY KEY (user_id),
    CONSTRAINT user_credentials_user_id_fk FOREIGN KEY (user_id) 
        REFERENCES users(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
