USE NS Dispatcher;
USE DB DispatcherDB;

INFO FOR DB;

SELECT * FROM role;



CREATE session:2;
RELATE human:2->participateIn->session:2;
SELECT id, <-participateIn<-human.name, <-participateIn<-interroagtor.name, <-participateIn<-computer.model, ->includes->answer.text, ->includes->question.text, ->includes->verdict.correct FROM session;
