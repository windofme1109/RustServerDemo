drop table if exist course;

create table course
(
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default now()
)


insert into course
    (id, teacher_id, name, time)
value(1, 1, 'First course', '2022-01-17 05:40:40')


insert into course
    (id, teacher_id, name, time)
value(2, 1, 'First course', '2022-01-17 05:40:40')