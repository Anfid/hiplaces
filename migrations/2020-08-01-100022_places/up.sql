CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE places (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL,
    info TEXT NOT NULL,
    UNIQUE (name),
    created_by UUID NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY (created_by)
            REFERENCES users(id)
            ON DELETE SET NULL
);

SELECT diesel_manage_updated_at('places');
