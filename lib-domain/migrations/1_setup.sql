create table claw
(
    id        text    not null
        primary key,
    expiry_at bigint  not null,
    data      text    not null,
    pem       text    not null,
    sha256    text    not null,
    validity  integer not null
);
