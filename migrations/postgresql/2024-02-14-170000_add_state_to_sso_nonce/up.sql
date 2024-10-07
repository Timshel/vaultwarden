DROP TABLE sso_nonce;

CREATE TABLE sso_nonce (
	state               TEXT NOT NULL PRIMARY KEY,
  	nonce               TEXT NOT NULL,
  	redirect_uri 		TEXT NOT NULL,
  	created_at          TIMESTAMP NOT NULL DEFAULT now()
);