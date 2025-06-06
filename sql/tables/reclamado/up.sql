CREATE TABLE Reclamado(
    reclamado_id INT PRIMARY KEY GENERATED BY DEFAULT AS IDENTITY,
    nome VARCHAR(150) NOT NULL,
    cpf CHAR(11) UNIQUE,
    cnpj CHAR(14) UNIQUE,
    numero_addr SMALLINT NOT NULL,
    logradouro_addr VARCHAR(100) NOT NULL,
    bairro_addr VARCHAR(100) NOT NULL,
    cidade_addr VARCHAR(100) NOT NULL,
    uf_addr CHAR(2) NOT NULL,
    telefone CHAR(11),
    email VARCHAR(100) CHECK (email IS NULL OR email LIKE '%_@__%.__%'),
    cep CHAR(8) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
