INSERT INTO optic.users(name, email)
VALUES ($1, $2)
RETURNING $table_fields;