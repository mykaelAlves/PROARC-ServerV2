#![allow(dead_code)]

// ===================== Usuário =====================
pub static GET_HASH_AND_SALT: &str = "SELECT hash_and_salt, salt FROM usuario WHERE username = $1";
pub static GET_ALL_USUARIOS: &str = "SELECT * FROM usuario";
pub static GET_USUARIO_BY_ID: &str = "SELECT * FROM usuario WHERE usuario_id = $1";
pub static GET_USUARIO_BY_USERNAME: &str = "SELECT * FROM usuario WHERE username = $1";

// ===================== Reclamado =====================
pub static GET_RECLAMADO_ID_POR_ADDR: &str = "SELECT reclamado_id FROM Reclamados WHERE numero_addr = $1 AND logradouro_addr = $2 AND bairro_addr = $3 AND cidade_addr = $4 AND uf_addr = $5 AND cep = $6 LIMIT 1";
pub static GET_RECLAMADO_ID_POR_CNPJ: &str = "SELECT reclamado_id FROM Reclamados WHERE cnpj = $1";
pub static GET_ALL_RECLAMADOS: &str = "SELECT * FROM Reclamados";
pub static INSERT_RECLAMADO: &str = "INSERT INTO Reclamados (nome, cpf, cnpj, numero_addr, logradouro_addr, bairro_addr, cidade_addr, uf_addr, telefone, email, cep) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)";
pub static DELETE_RECLAMADOS_POR_RECLAMACAO: &str = "DELETE FROM RelacaoProcessoReclamado WHERE reclamacao_id = $1";
pub static GET_RECLAMADOS_POR_RECLAMACAO_ID: &str = "SELECT rd.* FROM Reclamados rd JOIN RelacaoProcessoReclamado rpr ON rd.reclamado_id = rpr.reclamado_id WHERE rpr.reclamacao_id = $1";

// ===================== Reclamação =====================
pub static GET_RECLAMACAO_ID_POR_TITULO: &str = "SELECT reclamacao_id FROM Reclamacoes WHERE titulo = $1";
pub static GET_RECLAMACAO_POR_TITULO: &str = "SELECT * FROM reclamacoes WHERE titulo = $1";
pub static GET_RECLAMACAO_SITUACAO_POR_TITULO: &str = "SELECT situacao FROM Reclamacoes WHERE titulo = $1";
pub static GET_ALL_RECLAMACOES: &str = "SELECT * FROM Reclamacoes";

// ===================== Reclamante =====================
pub static GET_RECLAMANTE_POR_CPF: &str = "SELECT reclamante_id FROM Reclamantes WHERE cpf = $1";
pub static GET_RECLAMANTE_ID_POR_CPF: &str = "SELECT reclamante_id FROM Reclamantes WHERE cpf = $1";
pub static GET_ALL_RECLAMANTES: &str = "SELECT * FROM Reclamantes";
pub static INSERT_RECLAMANTE: &str = "INSERT INTO Reclamantes (nome, rg, cpf, telefone, email) VALUES ($1, $2, $3, $4, $5)";
pub static COUNT_RECLAMANTES: &str = "SELECT COUNT(*) FROM Reclamantes";

// ===================== Procurador =====================
pub static GET_PROCURADOR_POR_CPF: &str = "SELECT procurador_id FROM Procuradores WHERE cpf = $1";
pub static GET_ALL_PROCURADORES: &str = "SELECT * FROM Procuradores";
pub static INSERT_PROCURADOR: &str = "INSERT INTO Procuradores (nome, rg, cpf, telefone, email) VALUES ($1, $2, $3, $4, $5)";

// ===================== Motivo =====================
pub static GET_MOTIVO_ID_POR_NOME: &str = "SELECT motivo_id FROM Motivos WHERE nome = $1";
pub static GET_ALL_MOTIVOS: &str = "SELECT * FROM Motivos";
pub static INSERT_MOTIVO: &str = "INSERT INTO Motivos (nome) VALUES ($1)";
pub static DELETE_MOTIVO_POR_NOME: &str = "DELETE FROM Motivos WHERE nome = $1";
pub static UPDATE_MOTIVO_POR_ID: &str = "UPDATE Motivos SET nome = $1 WHERE nome = $2";
pub static COUNT_MOTIVOS: &str = "SELECT COUNT(*) FROM Motivos";

// ===================== Histórico =====================
pub static INSERT_SITUACAO_MUDANCA_HISTORICO: &str = "INSERT INTO HistoricoMudancaSituacao (reclamacao_id, situacao_old, situacao_new) VALUES ($1, $2, $3)";
pub static GET_HISTORICO_SITUACAO_POR_RECLAMACAO_ID: &str = "SELECT * FROM HistoricoMudancaSituacao WHERE reclamacao_id = $1 ORDER BY changed_at DESC";

// ===================== Estatísticas =====================
pub static ESTATISTICA_MAIS_RECLAMADOS: &str = "SELECT r.nome, COUNT(rpr.reclamacao_id) AS total_reclamacoes FROM RelacaoProcessoReclamado AS rpr JOIN Reclamados AS r ON rpr.reclamado_id = r.reclamado_id GROUP BY r.reclamado_id, r.nome ORDER BY total_reclamacoes DESC LIMIT $1";
pub static ESTATISTICA_MOTIVOS_MAIS_USADOS: &str = "SELECT m.nome AS motivo, COUNT(r.reclamacao_id) AS total_reclamacoes FROM Reclamacoes r JOIN Motivos m ON r.motivo_id = m.motivo_id GROUP BY m.nome ORDER BY total_reclamacoes DESC";
pub static ESTATISTICA_RECLAMACOES_POR_MES_ANO_ATUAL: &str = "SELECT EXTRACT(MONTH FROM data_abertura) AS mes, COUNT(reclamacao_id) AS total_reclamacoes FROM Reclamacoes WHERE EXTRACT(YEAR FROM data_abertura) = EXTRACT(YEAR FROM CURRENT_DATE) GROUP BY mes ORDER BY mes";
pub static ESTATISTICA_RECLAMACOES_POR_MES_ANO: &str = "SELECT EXTRACT(MONTH FROM data_abertura) AS mes, COUNT(reclamacao_id) AS total_reclamacoes FROM Reclamacoes WHERE EXTRACT(YEAR FROM data_abertura) = $1 GROUP BY mes ORDER BY mes";
pub static ESTATISTICA_CIDADES_COM_MAIS_RECLAMACOES: &str = "SELECT r.cidade_addr AS cidade, COUNT(rpr.reclamacao_id) AS total_reclamacoes FROM RelacaoProcessoReclamado AS rpr JOIN Reclamados AS r ON rpr.reclamado_id = r.reclamado_id GROUP BY r.cidade_addr ORDER BY total_reclamacoes DESC";
pub static ESTATISTICA_RECLAMACOES_POR_CRIADOR: &str = "SELECT criador, COUNT(*) AS total FROM Reclamacoes GROUP BY criador ORDER BY total DESC";
pub static ESTATISTICA_RECLAMACOES_POR_SITUACAO: &str = "SELECT situacao, COUNT(*) AS total FROM Reclamacoes GROUP BY situacao ORDER BY total DESC";

// ===================== Contagens =====================
pub static COUNT_RECLAMACOES: &str = "SELECT COUNT(*) FROM Reclamacoes";
pub static COUNT_RECLAMACOES_ENEL: &str = "SELECT COUNT(*) FROM ReclamacoesEnel";
pub static COUNT_RECLAMACOES_GERAL: &str = "SELECT COUNT(*) FROM ReclamacoesEnel";
pub static COUNT_RECLAMACOES_ENEL_ANO: &str = "SELECT MAX(reclamacao_id) AS ultimo_reclamacao_id FROM Reclamacoes";
pub static COUNT_RECLAMACOES_GERAL_ANO: &str = "SELECT MAX(reclamacao_id) AS ultimo_reclamacao_id FROM Reclamacoes";

// ===================== Atualizações específicas =====================
pub static UPDATE_RECLAMACAO_ENEL: &str = "UPDATE ReclamacoesEnel SET atendente = COALESCE($1, atendente), contato_enel_telefone = COALESCE($2, contato_enel_telefone), contato_enel_email = COALESCE($3, contato_enel_email), observacao = COALESCE($4, observacao) WHERE reclamacao_id = $5";
pub static UPDATE_RECLAMACAO_GERAL: &str = "UPDATE ReclamacoesGeral SET data_audiencia = COALESCE($1, data_audiencia), conciliador = COALESCE($2, conciliador) WHERE reclamacao_id = $3";