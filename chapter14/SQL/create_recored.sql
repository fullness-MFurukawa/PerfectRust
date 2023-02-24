/* カテゴリデータ追加 */
INSERT INTO product_category (name) VALUES('文房具');
INSERT INTO product_category (name) VALUES('雑貨');
INSERT INTO product_category (name) VALUES('パソコン周辺機器');
/* 商品データ追加 */
insert into product (name , price , category_id) values('水性ボールペン(黒)',120,1);
insert into product (name , price , category_id) values('水性ボールペン(赤)',120,1);
insert into product (name , price , category_id) values('水性ボールペン(青)',120,1);
insert into product (name , price , category_id) values('油性ボールペン(黒)',100,1);
insert into product (name , price , category_id) values('油性ボールペン(赤)',100,1);
insert into product (name , price , category_id) values('油性ボールペン(青)',100,1);
insert into product (name , price , category_id) values('蛍光ペン(黄)',130,1);
insert into product (name , price , category_id) values('蛍光ペン(赤)',130,1);
insert into product (name , price , category_id) values('蛍光ペン(青)',130,1);
insert into product (name , price , category_id) values('蛍光ペン(緑)',130,1);
insert into product (name , price , category_id) values('鉛筆(黒)',100,1);
insert into product (name , price , category_id) values('鉛筆(赤)',100,1);
insert into product (name , price , category_id) values('色鉛筆(12色)',400,1);
insert into product (name , price , category_id) values('色鉛筆(48色)',1300,1);
insert into product (name , price , category_id) values('レザーネックレス',300,2);
insert into product (name , price , category_id) values('ワンタッチ開閉傘',3000,2);
insert into product (name , price , category_id) values('金魚風呂敷',500,2);
insert into product (name , price , category_id) values('折畳トートバッグ',600,2);
insert into product (name , price , category_id) values('アイマスク',900,2);
insert into product (name , price , category_id) values('防水スプレー',500,2);
insert into product (name , price , category_id) values('キーホルダ',800,2);
insert into product (name , price , category_id) values('ワイヤレスマウス',900,3);
insert into product (name , price , category_id) values('ワイヤレストラックボール',1300,3);
insert into product (name , price , category_id) values('有線光学式マウス',500,3);
insert into product (name , price , category_id) values('光学式ゲーミングマウス',4800,3);
insert into product (name , price , category_id) values('有線ゲーミングマウス',3800,3);
insert into product (name , price , category_id) values('USB有線式キーボード',1400,3);
insert into product (name , price , category_id) values('無線式キーボード',1900,3);
/* ユーザーデータ追加 */
/* password = pass001 */
INSERT INTO "user" (user_id,user_name,password,mail) VALUES('5772a800-fef1-40bf-888b-68fddd29d881','user001','a034408b78dfee92cdbfc6e5247cf0ece119f30e6ba7653f4b7a6f2f384f92a3c7cd4a0ec914ae3fb1ea93684b46f8ff2644ec0198d67be2fd2cbf68587f07b8','yamada@sample.com');
/* password = pass002 */
INSERT INTO "user" (user_id,user_name,password,mail) VALUES('5ca87702-a40a-4f08-85c3-534e92e36c0e','user002','51ca7a5622b4a5bcebc96c523dd464da5a62af27fa8ac0ba2d9d2a3efa46426424408865a980d5c71e770936b17b3502fa68993286ac958eff5bee0d7ec3ac3b','suzuki@sample.com');
