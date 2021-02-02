CREATE TABLE public.usuarioregistro
(
  idusuarioregistro integer NOT NULL DEFAULT nextval('usuarioregistro_idusuarioregistro_seq'::regclass),
  identificacion character varying(20) NOT NULL,
  nombre character varying(50) NOT NULL,
  genero character(1) NOT NULL,
  estadocivil character varying(20) NOT NULL,
  fechanacimiento date NOT NULL,
  telefono character varying(5) NOT NULL,
  direccion character varying(70),
  email character varying(50),
  CONSTRAINT usuarioregistro_pkey PRIMARY KEY (idusuarioregistro),
  CONSTRAINT usuarioregistro_nombre_key UNIQUE (nombre)
)
WITH (
  OIDS=FALSE
);
ALTER TABLE public.usuarioregistro
  OWNER TO postgres;

  CREATE TABLE public.persona
(
  idpersona integer NOT NULL DEFAULT nextval('persona_idpersona_seq'::regclass),
  idusuarioregistro integer NOT NULL,
  estado bit(1) NOT NULL,
  CONSTRAINT persona_pkey PRIMARY KEY (idpersona)
)