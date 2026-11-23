-- Add migration script here
create table apartments(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    price INTEGER NOT NULL CHECK (price>=0)
);

insert into apartments (price) VALUES
    (50),
    (200),
    (70)
