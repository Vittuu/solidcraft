# Relatórios de falha

Quando o aplicativo para desktop falha abruptamente ou encontra um erro interno, ele grava um relatório de falha neste
computador. Cada relatório contém o suficiente para reproduzir e corrigir o problema: na maioria dos casos, o projeto
e os comandos exatos que o provocam. Relatórios nunca são enviados a qualquer lugar. Para compartilhar um, use
**Relatar…** em Ajuda ▸ Relatórios de falha…, que abre um rascunho de issue do GitHub. Nada é compartilhado
até que você revise e envie, e o projeto não faz parte do rascunho.

Para enviar um relatório:

1. Revise título, texto e captura no SolidCraft e clique em **Abrir rascunho no GitHub**.
   Para relatórios longos, SolidCraft copia o texto; cole-o no rascunho do GitHub.
2. Anexe a captura se quiser compartilhá-la: use **Copiar captura de tela** e cole no GitHub,
   ou **Salvar captura de tela…** e envie o PNG. Capturas não são anexadas automaticamente.
3. Clique em **Submit new issue** no GitHub para publicar. Apenas abrir o rascunho não o envia.

## Onde ficam

`<data dir>/solidcraft/crashes/`, ao lado da pasta de recuperação:

| Sistema | Pasta |
|---|---|
| Linux | `~/.local/share/solidcraft/crashes` (ou `$XDG_DATA_HOME/solidcraft/crashes`) |
| Windows | `%LOCALAPPDATA%\SolidCraft\crashes` |
| macOS | `~/Library/Application Support/SolidCraft/crashes` |

`SOLIDCRAFT_CRASH_DIR` substitui a pasta. `solidcraft --crash-log` e
`solidcraft-cli crashes dir` a exibem.

## Para agentes

Todos estes leem a mesma pasta:

| Origem | Listar | Um relatório | Excluir |
|---|---|---|---|
| Comando do mecanismo (MCP `execute`, canal de controle, scripts) | `diagnostics.crash_reports {limit?}` | `diagnostics.crash_report {id}` | `diagnostics.crash_reports.clear {id?}` |
| Ferramenta MCP | `crash_reports {limit?}` | `crash_reports {id}` | — |
| CLI | `solidcraft-cli crashes [--json]` | `solidcraft-cli crashes show <id>` | `solidcraft-cli crashes clear [<id>]` |
| Canal de controle (interface) | `ui.help {"item": "crashes"}` abre a janela | | |

**Reproduzir:** `solidcraft-cli crashes replay <id>` abre o projeto do relatório, executa seus
comandos e informa se o mesmo erro interno ocorre novamente (mesmo local e mensagem).
Sem a CLI, execute `solidcraft-cli run <report>/replay.json --in <report>/design.solidcraft`.
Uma falha corrigida se torna um teste de regressão com os mesmos dois arquivos.

**Confira se os relatórios funcionam:** com `SOLIDCRAFT_CRASH_TEST=1` definido para o aplicativo, o comando
`diagnostics.crash_test` tem três modos:
- `panic`: um erro interno em um comando
- `kernel`: um erro interno dentro do núcleo de geometria
- `abort`: o aplicativo termina imediatamente, e o relatório aparece na próxima inicialização

## O que um relatório contém

Uma pasta `crashes/<YYYYMMDD-HHMMSS>-<signature>/`:

- `report.json` (`"format": "solidcraft-crash/1"`):
  - `kind`: um dos seguintes
    - `command_panic`: um comando encontrou um erro interno; ele foi capturado e o projeto foi mantido inalterado
    - `kernel_panic_recovered`: o núcleo de geometria de terceiros gerou um panic que foi capturado. Isso é inofensivo no desktop, mas faria a compilação web falhar abruptamente.
    - `panic`: um erro interno fora de qualquer comando
    - `unclean_exit`: o aplicativo terminou sem que um hook o detectasse (um abort, estouro de pilha, falha de segmentação, falha em um driver gráfico ou encerramento do processo). Isso é detectado na próxima inicialização. Quando o aplicativo morre ao iniciar, a mensagem e `location` identificam a etapa (veja abaixo). No Windows, falhas de código nativo também indicam exceção, módulo e deslocamento nele.
  - `message`, `location` (arquivo:linha), `thread`
  - `backtrace`: quadros `symbol (file:line)`, sem a infraestrutura de panic, os componentes internos de std ou a pasta pessoal do usuário
  - `app` (versão, commit, data de compilação), `system` (sistema operacional, arquitetura, renderizador e GPU), `session.uptime_s`
  - `command`: o comando que falhou, com seus parâmetros
  - `recent_commands`: até 32, com sucesso ou erro
  - `base`: o ponto de partida da reprodução (o projeto antes do comando, um salvamento automático, um arquivo salvo ou um novo projeto)
  - `count`, `first_seen`, `last_seen`: repetições da mesma falha apenas aumentam `count`
  - `notes`: o que está ausente e por quê, por exemplo um projeto grande demais para copiar
  - `startup_phase`: `{step, ms}` quando o aplicativo morre antes do primeiro quadro: a etapa
    (`window`, `graphics instance`, `window surface`, `adapter request`, `adapter chosen`,
    `device creation`, `egui renderer (shader compile)`, `surface configure`, `app setup`,
    `viewport pipelines (shader compile)`, `first frame`, `first frame (present)`) e quantos
    milissegundos após iniciar ela começou. `system.environment` identifica renderizador
    (backend, GPU, driver) assim que o adaptador é escolhido, mesmo se o dispositivo não iniciou.
  - `native` (Windows): o que o filtro de exceções capturou: `code`, `name`, `address`, `module`,
    `offset` (no módulo), `detail` (endereço lido ou escrito em violação de acesso) e `thread`.
    O `backtrace` mostra a mesma informação em duas linhas.

- `design.solidcraft`: o projeto usado como ponto de partida da reprodução
- `replay.json`: `{"commands": [{command, params}, …]}`

- `minidump.dmp` (Windows, falhas nativas): pilhas das threads e módulos carregados no momento
  da falha, para WinDbg ou Visual Studio. Contém apenas memória de pilha, não o projeto, e
  fica neste computador como o restante do relatório; compartilhe apenas quando solicitado.

Uma falha que encerra o processo sem exceção (fail-fast, como um abort termina no Windows)
ainda registra a etapa de inicialização. Windows também pode guardar dumps de todas as falhas:
defina `DumpFolder` sob `HKLM\SOFTWARE\Microsoft\Windows\Windows Error Reporting\LocalDumps`.

## Como mantém custo e tamanho baixos

- **Antes de uma falha:** o aplicativo mantém um diário dos comandos de nível superior executados no projeto
  na tela (`crashes/sessions/*.jsonl`, no máximo 512 KB). Cada comando adiciona duas pequenas gravações,
  sem flush para o disco. Prévias e outras sessões temporárias não são registradas. Cada salvamento automático
  reinicia o diário a partir do projeto salvo automaticamente. Um encerramento normal o exclui. O arquivo do diário
  fica bloqueado enquanto o aplicativo está em execução, portanto, na próxima inicialização, um diário não utilizado pertence a um
  aplicativo que foi interrompido.
- **Um panic** captura seu backtrace apenas na primeira vez em que sua assinatura é detectada. A
  assinatura é o local mais a mensagem, ignorando números.
- **Limites:**
  - no máximo 5 novos relatórios por execução
  - 20 relatórios
  - 50 MB no total
  - 90 dias
  - um projeto acima de 10 MB não é copiado; o relatório informa isso

  Ao exceder um limite, panics recuperados do núcleo são excluídos antes de falhas reais, começando pelos mais antigos.
- **Onde está desativado:** a CLI, os testes e a compilação web nunca instalam o hook. Nesses casos,
  o único custo para os comandos é verificar um sinalizador atômico.

Compilações de versão mantêm tabelas de linhas (`debug = "line-tables-only"`), para que backtraces identifiquem o arquivo
e a linha. Remover símbolos de um binário (`strip`) elimina essas tabelas e também os nomes das funções.
