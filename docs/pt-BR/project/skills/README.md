# Skills para agentes

Skills para agentes de IA que modelam no SolidCraft por seu [servidor MCP](../../mcp.md). Cada pasta é
uma skill no formato [Agent Skills](https://agentskills.io): um `SKILL.md` com nome e
descrição que informam ao agente quando carregá-la, e um arquivo de referência opcional de apoio.

| Skill | Carregada quando o agente |
|---|---|
| [`solidcraft-modeling`](solidcraft-modeling/SKILL.md) | cria uma nova peça (carcaça, suporte, placa, flange, …) |
| [`solidcraft-editing`](solidcraft-editing/SKILL.md) | altera, corrige ou amplia um projeto existente |
| [`solidcraft-checking`](solidcraft-checking/SKILL.md) | cria, altera ou adapta um projeto, junto com as duas anteriores: intervalos de parâmetros, todos os valores permitidos, comparação STEP exata, testes de regressão, projetos mantidos no git |
| [`solidcraft-mechanisms`](solidcraft-mechanisms/SKILL.md) | cria, aciona ou anima uma montagem: componentes, juntas, vínculos de movimento, ciclos fechados como atuadores, conjuntos de contato e estudos de movimento |

Elas ensinam o agente a:
- planejar a peça em números
- criá-la em poucas chamadas `batch` com posições paramétricas
- verificar cada etapa com `measure` comparando com valores que calculou
- omitir respostas grandes de que não precisa, como o catálogo de comandos sem filtro e listagens completas de topologia

Também listam as armadilhas que, caso contrário, custam chamadas com falha. Exemplos são os eixos de esboço do plano XZ, seleções de aresta que se ajustam à mais próxima e cortes que não atingem o corpo, mas ainda informam sucesso.

Elas foram escritas para SolidCraft 0.8 e posteriores; `solidcraft-mechanisms`, para `main` após
0.11.1, que conta os graus de liberdade de ciclos fechados. A tradução de mecanismos acompanha
as respostas de juntas, as listas compactas e `snapshot --in` disponíveis na base `0872eb9`.

## Instalação

Claude Code, para todos os projetos:

```sh
git clone --depth 1 https://github.com/bherbruck/solidcraft
cp -r solidcraft/skills/solidcraft-* ~/.claude/skills/
```

Windows (PowerShell):

```powershell
git clone --depth 1 https://github.com/bherbruck/solidcraft
Copy-Item -Recurse solidcraft\skills\solidcraft-* $HOME\.claude\skills\
```

- **Somente um projeto:** copie as pastas para `.claude/skills/` desse projeto.
- **Outros agentes:** para agentes que leem o formato Agent Skills, copie as pastas para o diretório de skills indicado em sua documentação.
- **Atualização:** copie as pastas novamente após atualizar o SolidCraft.

## Manutenção

- O exemplo desenvolvido em `solidcraft-modeling/reference.md` é executado em CI. `skill_reference_example_builds_as_documented` em `crates/mcp/src/tests.rs` o cria, verifica o volume documentado, altera um parâmetro e verifica novamente.
- A tabela de planos de esboço é coberta por `sketch_plane_axes_match_the_skill_reference` no mesmo arquivo.
- O exemplo de `solidcraft-mechanisms/reference.md` também tem um teste de CI:
  `mechanisms_skill_example_moves_as_documented` o cria, aciona a dobradiça e compara o curso do
  atuador, o painel vinculado e a parada com valores analíticos.
- `skills_name_only_real_tools_and_commands` verifica que todas as ferramentas e comandos chamados pelas skills existem.
- `solidcraft-checking/scripts/tests` verifica os auxiliares dessa skill, suas armadilhas e convenções, e os fatos por trás das correções em `solidcraft-modeling`: `PYTHONDONTWRITEBYTECODE=1 uv run --no-project --with pytest --with build123d pytest -p no:cacheprovider skills/solidcraft-checking/scripts/tests`. Os testes executam `solidcraft-cli` encontrado em `PATH`, não a compilação do workspace, e CI não os executa.
- Quando um comando, nome de parâmetro ou comportamento descrito nesses arquivos mudar, atualize-os na mesma PR.
- Fatos que descrevem limitações atuais, como seleções de arestas sem limite de distância, devem ser removidos quando a limitação for corrigida.
