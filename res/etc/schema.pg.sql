DROP DATABASE IF EXISTS db1;
CREATE DATABASE db1 WITH ENCODING 'utf-8';
-- ********************************
-- *            table             *
-- ********************************
DROP TABLE IF EXISTS public.user;
CREATE TABLE public.user
(
    user_id  bigserial,
    nickname   varchar(64)  NOT NULL,
    password   varchar(32)  NOT NULL,
    email      varchar(128) NOT NULL,
    created_at timestamp    NOT NULL DEFAULT current_timestamp,
    updated_at timestamp    NOT NULL DEFAULT current_timestamp,
    deleted_at timestamp,
    PRIMARY KEY (user_id)
);
DROP TABLE IF EXISTS public.district;
CREATE TABLE public.district
(
    district_id bigserial,
    code        int         NOT NULL,
    name        varchar(32) NOT NULL,
    created_at  timestamp   NOT NULL DEFAULT current_timestamp,
    updated_at  timestamp   NOT NULL DEFAULT current_timestamp,
    deleted_at  timestamp,
    PRIMARY KEY (district_id)
);
-- ********************************
-- *          function            *
-- ********************************
CREATE OR REPLACE FUNCTION refresh_updated_at() RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = current_timestamp;
    RETURN NEW;
END
$$ LANGUAGE 'plpgsql';
-- ********************************
-- *        add trigger           *
-- ********************************
DO
$$
    DECLARE
        row record;
    BEGIN
        FOR row IN SELECT tablename FROM pg_catalog.pg_tables WHERE schemaname = 'public'
            LOOP
                EXECUTE format(
                        'CREATE TRIGGER trigger_refresh_updated_at BEFORE UPDATE ON %I FOR EACH ROW EXECUTE FUNCTION refresh_updated_at() ',
                        row.tablename);
            END LOOP;
    END
$$;
-- ********************************
-- *            data              *
-- ********************************
INSERT INTO public.user (nickname, password, email) VALUES ('忘机', '123456', 'wj@gmail.com');
INSERT INTO public.user (nickname, password, email) VALUES ('白鹤', '123456', 'bh@gmail.com');
INSERT INTO public.user (nickname, password, email) VALUES ('花咲', '123456', 'hx@gmail.com');
INSERT INTO public.user (nickname, password, email) VALUES ('明月', '123456', 'my@gmail.com');
--
INSERT INTO public.district (code, name) VALUES (110000, '北京市');
INSERT INTO public.district (code, name) VALUES (110101, '东城区');
INSERT INTO public.district (code, name) VALUES (120000, '天津市');
INSERT INTO public.district (code, name) VALUES (120101, '和平区');
