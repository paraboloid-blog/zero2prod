-- Add migration script here
INSERT INTO users (user_id, username, password_hash)
VALUES (
'9d8c91a7-e45d-4605-b513-ccbf68ff3b45',
'admin',
'$argon2id$v=19$m=16,t=2,p=1$Skk2Z1FtUWREYURVVUxNbA$GSIcZJAQ2w2/wyx7vsEkbA'
);
