/* シーケンス作成 */
CREATE SEQUENCE public.product_category_seq
  INCREMENT 1
  MINVALUE 1
  MAXVALUE 999999999
  START 1;
ALTER TABLE public.product_category_seq
  OWNER TO postgres;
CREATE SEQUENCE public.product_seq
  INCREMENT 1
  MINVALUE 1
  MAXVALUE 999999999
  START 1;
ALTER TABLE public.product_seq
  OWNER TO postgres;
CREATE SEQUENCE public.user_seq
  INCREMENT 1
  MINVALUE 1
  MAXVALUE 999999999
  START 1;
ALTER TABLE public.user_seq
  OWNER TO postgres;

/* カテゴリテーブル作成 */
CREATE TABLE public.product_category
(
  id integer NOT NULL DEFAULT nextval('product_category_seq'::regclass),
  name character varying(20),
  CONSTRAINT product_category_pk PRIMARY KEY (id)
)
WITH (
  OIDS=FALSE
);
ALTER TABLE public.product_category
  OWNER TO postgres;
/* 商品テーブル作成 */
CREATE TABLE public.product
(
  id integer NOT NULL DEFAULT nextval('product_seq'::regclass),
  name character varying(30),
  price integer,
  category_id integer,
  CONSTRAINT product_pk PRIMARY KEY (id),
  CONSTRAINT product_category_fk FOREIGN KEY (category_id)
      REFERENCES public.product_category (id) MATCH SIMPLE
      ON UPDATE NO ACTION ON DELETE NO ACTION
)
WITH (
  OIDS=FALSE
);
ALTER TABLE public.product
  OWNER TO postgres;

/* ユーザーテーブル */
CREATE TABLE public."user"
(
  id integer NOT NULL DEFAULT nextval('user_seq'::regclass),
  user_id character varying(40) NOT NULL,
  user_name character varying(30) NOT NULL,
  password character varying(130) NOT NULL,
  mail character varying(50) NOT NULL,
  CONSTRAINT user_pk PRIMARY KEY (id)
)
WITH (
  OIDS=FALSE
);
ALTER TABLE public."user"
  OWNER TO postgres;
