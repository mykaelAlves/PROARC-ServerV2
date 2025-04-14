CREATE TABLE HistoricoMudancaSituacao(
    reclamacao_id INT,
    situacao_old VARCHAR(75),
    situacao_new VARCHAR(75) NOT NULL,
    changed_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (reclamacao_id) REFERENCES Reclamacoes(reclamacao_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
