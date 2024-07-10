DROP SEQUENCE IF EXISTS teacher_id_seg;
CREATE SEQUENCE teacher_id_seg START 100;



drop table if exists teacher;

create table teacher
(
    id integer default nextval('teacher_id_seg'::regclass),
    name varchar(100) not null,
   	picture_url varchar(200) not null,
	profile varchar(2000) not null
)