drop table if exists course;

create table course
(
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default now()
);


-- insert into course
--     (id, teacher_id, name, time)
-- values(1, 1, 'First course', '2022-01-17 05:40:40');


-- insert into course
--     (id, teacher_id, name, time)
-- values(2, 1, 'First course', '2022-01-17 05:40:40');

ALTER TABLE course ADD COLUMN description varchar(2000) default null;
ALTER TABLE course ADD COLUMN format varchar(30) default null;
ALTER TABLE course ADD COLUMN structure varchar(200) default null;
ALTER TABLE course ADD COLUMN duration varchar(30) default null;
ALTER TABLE course ADD COLUMN price integer default null;
ALTER TABLE course ADD COLUMN language varchar(30) default null;
ALTER TABLE course ADD COLUMN level varchar(30) default null;


DROP SEQUENCE IF EXISTS course_id_seq;
CREATE SEQUENCE course_id_seq START 100;

ALTER TABLE course ALTER COLUMN id TYPE integer default nextval('course_id_seq'::regclass)





