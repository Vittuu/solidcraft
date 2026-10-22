# SolidCraft e agentes de IA (MCP)

O SolidCraft usa o [Model Context Protocol](https://modelcontextprotocol.io) por stdio
(JSON-RPC 2.0, uma mensagem por linha; revisões de protocolo 2025-06-18, 2025-03-26 e 2024-11-05).
Um agente modela exatamente como uma pessoa: cada ferramenta executa comandos do mecanismo, os mesmos usados pela
barra de ferramentas, paleta, scripts e canal de controle, e nunca abre um diálogo.

Sessões persistentes sem GUI e conectadas expõem imagens assíncronas de produto pela ferramenta
`execute`: `render.preflight`, `render.start`, `render.status`, `render.cancel` e `render.export`.
Mantenha o ID completo retornado; os bytes ficam na sessão até a exportação PNG explícita.
Veja [trabalhos de renderização](render-jobs.md).

Para saber como essa mesma interface de comandos deve funcionar no navegador (workers, armazenamento em camadas,
agentes na aba), veja [web-runtime.md](https://github.com/bherbruck/solidcraft/blob/main/docs/web-runtime.md). Para plugins isolados e operações paramétricas
da linha do tempo que também usam `run(command, args)`, veja [plugins.md](https://github.com/bherbruck/solidcraft/blob/main/docs/plugins.md). O MCP para desktop descrito neste
documento continua sendo a implementação de referência atual.

## Inicialização

```sh
# Headless: an in-process design session, no window (screenshots are drawn on the GPU).
solidcraft-cli mcp

# Headless, starting from a design file or a command script.
solidcraft-cli mcp --in part.solidcraft

# Bridged to the running app, so you can watch the agent model:
solidcraft --control 7878 &
solidcraft-cli mcp --connect 127.0.0.1:7878
```

No modo conectado, o servidor encontra o token de controle do aplicativo no arquivo de token gravado pelo aplicativo para essa porta
([control-protocol.md](https://bherbruck.github.io/solidcraft/pt-BR/dev/control-protocol.html#token)); `--token T` ou `SOLIDCRAFT_CONTROL_TOKEN` o fornecem
explicitamente quando o aplicativo funciona em um ambiente isolado cuja pasta de dados não é visível ao servidor (prefira a
variável: argumentos de linha de comando são visíveis a outros usuários na lista de processos).

Um servidor conectado inicia mesmo sem um aplicativo escutando na porta. Assim, um cliente MCP
que o inicie antes do aplicativo ou depois que ele fechar mantém suas ferramentas. As instruções
do servidor informam que o aplicativo não estava aberto; cada chamada retorna um erro dizendo
que não há aplicativo SolidCraft escutando em HOST:PORT (em inglês, `no SolidCraft app is listening at HOST:PORT`)
e como iniciá-lo (`solidcraft --control PORT`). A primeira chamada depois da inicialização se
conecta e lê o token daquela execução. Se o aplicativo encerrar ou falhar durante a sessão, a
próxima chamada informa o mesmo. Uma chamada que já estava em andamento informa que o aplicativo
não respondeu e pode ter encerrado ou falhado, antes ou depois de executá-la: inspecione o projeto
antes de reenviar uma alteração. Um `batch` que não pôde ser enviado informa que o aplicativo
não está aberto; nesse caso, pode ser reenviado com segurança.

stdout transporta apenas mensagens do protocolo; erros vão para stderr. Capturas no modo conectado são gravadas em um
arquivo temporário pelo aplicativo e lidas pelo servidor; por isso, ambos devem funcionar na mesma máquina.

### Quais arquivos o agente pode acessar

Chamadas de ferramentas leem, gravam e excluem arquivos apenas dentro das pastas do servidor:

- a pasta do projeto aberto (o projeto de `--in`, ou o último que o agente abriu ou salvou),
  mas somente quando estiver estritamente dentro da pasta pessoal (ou da pasta temporária) e não em uma
  pasta oculta. A própria pasta pessoal, uma pasta acima dela (`/`, `/home`, `/Users`, a raiz de uma unidade),
  pastas ocultas como `~/.config` e pastas em outros locais não contam. Arquivos e pastas ocultos
  dentro da pasta do projeto (`.env`, `.git`) também ficam fora de alcance;
- a pasta de recuperação do próprio SolidCraft, para que `doc.recovery_list`, `doc.recover` e
  `doc.recovery_discard` funcionem diretamente (descartar exclui apenas entradas reais de recuperação);
- a pasta de trabalho temporária do próprio servidor (uma nova pasta privada `solidcraft-mcp-*` na pasta
  temporária), onde `screenshot` sem `path` é gravado;
- cada `--allow-path DIR` (repita para adicionar mais). Uma pasta nomeada aqui é acessível por inteiro,
  incluindo arquivos ocultos.

A pasta de trabalho onde o servidor foi iniciado não conta por si só, então `~/.ssh`, `~/.aws`,
`~/.bashrc` e outros arquivos iniciados por ponto nunca são acessíveis por padrão. stderr lista as pastas na
inicialização e informa quando a pasta do projeto de `--in` foi excluída; `--allow-path` a adiciona de volta.

Caminhos são resolvidos antes de comparados, então `..` e links simbólicos que levam para fora são recusados
(salvar por um link grava o arquivo para o qual ele aponta, portanto o destino do link é o que conta), e
um novo arquivo deve ser gravado em uma pasta existente (uma pasta ausente dentro de uma permitida é informada como ausente; não é criada e não exige ampliar `--allow-path`). No Windows, um caminho deve estar em uma unidade local
(`C:\…` ou `\\?\C:\…`): caminhos UNC, de dispositivo e NT em qualquer grafia (`\\server\share`,
`//server/share`, `\\?\UNC\…`, `\??\…`, `\\.\…`), caminhos relativos a unidade (`C:file`), `:` em um
nome de arquivo (fluxos de dados alternativos), nomes reservados de dispositivos com qualquer extensão (`NUL`, `CON`,
`COM1.txt`, …) e nomes curtos 8.3 (`PROGRA~1`) são recusados antes de qualquer chamada ao sistema de arquivos, e o
caminho resolvido é verificado novamente; por isso, um link ou junção que leva a um compartilhamento também é recusado. Isso
inclui uma fonte de `sketch.text` fornecida por caminho e o arquivo de fonte indicado pelo texto de um projeto ao abri-lo; fontes instaladas identificadas pela família continuam funcionando.
O servidor usa nomes longos para suas próprias pastas (temporária e de recuperação), para que funcionem quando
o caminho da pasta temporária tem um nome curto (`…\NAME~1.XYZ\AppData\Local\Temp`, como o Windows frequentemente
fornece quando o nome da pasta do usuário é longo).
Um caminho recusado é um erro de ferramenta que explica o motivo.

Um link físico dentro de uma pasta permitida não pode ser distinguido de um arquivo comum; portanto, um link físico
para um arquivo em outro local permite acessá-lo por esse link. Não mantenha esses links em uma pasta que você permita.

```sh
solidcraft-cli mcp --allow-path ~/designs --allow-path ~/exports
solidcraft-cli mcp --allow-any-path   # no limit (the behaviour before this option existed)
```

No modo conectado, o servidor envia suas pastas em cada solicitação (`allow_paths`, veja
[control-protocol.md](https://bherbruck.github.io/solidcraft/pt-BR/dev/control-protocol.html)), e o aplicativo adiciona sua própria pasta de recuperação e a
pasta do projeto aberto nele, sob as mesmas regras. Por isso, forneça caminhos absolutos: um caminho relativo
é resolvido a partir da pasta de trabalho do aplicativo, não do servidor. Os diálogos de arquivo do próprio
aplicativo nunca são limitados.

### Anotações de ferramentas

Cada ferramenta informa o que pode fazer, para clientes que pedem autorização antes de executá-la (ou aprovam automaticamente com base
nessas indicações). `list_commands`, `inspect_design`, `measure`, `body_topology` e `crash_reports`
são somente leitura. `set_parameter`, `undo` e `redo` alteram o projeto de forma reversível. `new_design`
o substitui (ação destrutiva). `execute`, `batch`, `open`, `save`, `export` e `screenshot` (cujo
`path` grava um arquivo) podem ler, gravar, sobrescrever ou excluir arquivos: não são somente leitura,
e `destructiveHint` e `openWorldHint` são true.

### Configuração do cliente

Claude Code (`.mcp.json` do projeto, ou `claude mcp add solidcraft -- solidcraft-cli mcp`; uma configuração
adicionada sob o nome anterior continua funcionando, veja as [notas de compatibilidade](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/compatibility.md#6-mcp)):

```json
{
  "mcpServers": {
    "solidcraft": { "command": "solidcraft-cli", "args": ["mcp"] }
  }
}
```

Claude Desktop (`claude_desktop_config.json`), conectado a um aplicativo em execução:

```json
{
  "mcpServers": {
    "solidcraft": { "command": "/path/to/solidcraft-cli", "args": ["mcp", "--connect", "127.0.0.1:7878"] }
  }
}
```

### Skills para agentes

[`skills/`](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/project/skills/README.md) contém quatro [Agent Skills](https://agentskills.io) para agentes que modelam por este servidor:
- `solidcraft-modeling` cobre a criação de uma nova peça.
- `solidcraft-editing` cobre a alteração de um projeto existente.
- `solidcraft-checking` acrescenta verificações de erros que o SolidCraft não informa, testes de regressão e adaptação entre sistemas, junto com as outras duas.
- `solidcraft-mechanisms` cobre montagens: juntas por coordenadas globais, vínculos de movimento,
  ciclos fechados como atuadores, conjuntos de contato e estudos de movimento, comparados com valores analíticos.

Elas cobrem o fluxo de trabalho, os formatos dos comandos, como verificar com `measure` e as armadilhas conhecidas. Com elas, um agente cria uma peça em poucas chamadas `batch` e lê muito menos saída. Para instalá-las no Claude Code, copie as pastas para `~/.claude/skills/`; [skills/README.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/project/skills/README.md) contém os detalhes.

## Ferramentas

| Ferramenta | Argumentos | O que faz |
|---|---|---|
| `list_commands` | `filter?` | O catálogo de comandos: ID, rótulo, aba/painel, atalho, documentação de parâmetros, disponibilidade atual. `filter` corresponde a ID, rótulo, aba e painel. |
| `execute` | `command`, `params?`, `verbose?` (padrão true) | Executa qualquer comando com parâmetros JSON (`list_commands` os mostra). Falhas deixam o projeto inalterado. Um comando de operação responde com a operação (`feature`, `name`), todos os corpos (`bodies`) e os corpos criados, alterados e removidos (`created_bodies`, `changed_bodies`, `removed_bodies`). `verbose: false` usa a mesma resposta compacta de `batch`. |
| `batch` | `commands: [{command, params}]`, `rollback?` (padrão true), `verbose?` (padrão true) | Executa etapas em ordem, para no primeiro erro e desfaz as etapas concluídas. A resposta nomeia a etapa que falhou (`failed.index`, `failed.command`, `failed.error`) e cada resultado anterior. `verbose: false` mantém a resposta pequena em projetos grandes: etapas de operação omitem a lista completa de corpos (`created_bodies`, `changed_bodies`, `removed_bodies` permanecem) e etapas de esboço omitem os IDs repetidos fora de `created`. Conectado a um aplicativo que tenha `engine.batch`, o lote inteiro é uma solicitação de controle (um quadro do aplicativo). Aplicativos antigos e aplicativos que mostram o cursor do agente recebem as etapas uma a uma. |
| `inspect_design` | `measure?` (padrão true), `sketches?` (padrão true) | Propriedades do projeto (`metadata`: projeto, autor… e chaves de outras ferramentas; altere com `execute` `document.metadata.set`), parâmetros, linha do tempo (tipo, suprimido, retorno, erro, aviso, tempo), corpos (volume, área, centro de massa, limites, contagens de faces/arestas/vértices, visível), esboços (curvas, pontos, restrições, perfis, graus de liberdade, estado de solução, visível), seleção, profundidade de desfazer. Oculte esboços auxiliares, corpos ou planos por `execute` `browser.visibility` (`{items, visible}` ou `{folder, visible}`), e símbolos de juntas por `{joints: [id or name], visible}` ou `{folder: "joints", visible}` (uma junta oculta ainda restringe e aciona; `joint.list` informa `visible` de cada uma). |
| `measure` | `body?` | Volume (mm³), área (mm²), centro de massa, caixa delimitadora e contagens topológicas por corpo, com totais. |
| `body_topology` | `body` | Arestas (índice, ponto médio, comprimento, extremidades) e faces (índice, nome, área, centroide, normal, `point` e, para uma face cilíndrica, `cylinder`: `origin`, `axis`, `radius`). O ponto médio de uma aresta é o `[x,y,z]` que a seleciona para arredondamento e chanfro. O `point` de uma face fica sobre ela e a seleciona (seu centroide pode não ficar: o de uma face com furo fica no furo); o `name` a seleciona nos comandos que recebem faces (`face.delete`, `solid.shell`, `solid.draft`, `solid.offset_face`, `solid.split_face`, `solid.replace_face` e `face` de `sheet.convert`). |
| `set_parameter` | `name`, `value` (número ou expressão), `unit?`, `comment?` | Cria ou altera um parâmetro do usuário e recalcula; informa `recomputed` (operações reconstruídas), `restored` (operações cujo resultado anterior para as mesmas entradas foi reutilizado, por exemplo um valor restaurado) e operações que agora falham. |
| `screenshot` | `view?` (iso, top, front, back, bottom, left, right, home, fit, ou uma vista nomeada salva com `view.save`), `camera?` (`{target, yaw, pitch, distance, fov?}`, como em `view.apply`), `path?`, `width?`, `height?`, `source?` (window, model), `grid?`, `axes?` | Um bloco de conteúdo de imagem PNG. Sem interface gráfica: renderização com os mesmos pipelines da área de visualização em um dispositivo GPU sem janela (veja `solidcraft-cli gpu`; sem adaptador, o rasterizador por CPU, sem cores de análise), ajustada ao modelo, ou a partir da vista nomeada ou câmera; o campo `renderer` da resposta informa qual foi usado (`gpu`, `gpu-software`, `cpu`, com um `warning` para `cpu`). Conectado: a janela do aplicativo (padrão) ou uma renderização do modelo com a câmera do aplicativo; uma vista nomeada ou câmera primeiro move a área de visualização do aplicativo para lá (`execute` `view.apply` faz o mesmo sem uma captura). `grid: false` e `axes: false` omitem da renderização a grade em Z 0 e os eixos da origem. Eixos ocultos por `browser.visibility` continuam ocultos, independentemente de `axes`. No modo conectado, fornecer `grid` ou `axes` seleciona uma renderização do modelo, que segue os controles Grade e Origem do aplicativo para o que não for informado; combinar qualquer um com `source: "window"` é erro. |
| `export` | `path`, `format?` (step, 3mf, 3mf-tagged, stl, stla, obj, urdf), `bodies?`, `mesh_prefix?`, `effort?`, `velocity?` | Exportação STEP, 3MF ou de malha; URDF grava a montagem inteira com as malhas ao lado (`docs/urdf.md`). |
| `undo`, `redo` | — | Desfaz / refaz uma alteração. |
| `crash_reports` | `id?`, `limit?` | Os relatórios de falha neste computador, mais recentes primeiro; com `id`, o relatório inteiro (rastreamento de pilha, comando, comandos recentes, `design_path`/`replay_path`). Veja [crash-reports.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/crash-reports.md). |
| `new_design` | `name?` | Inicia um projeto vazio. |
| `open` | `path`, `up?`, `names?` | Abre um projeto `.solidcraft`, um arquivo STEP (`.step`/`.stp`) como um novo projeto contendo seus corpos (uma operação base de Importação), ou um arquivo 3MF/STL como corpos de malha. Para STEP e IGES, `up` e `names` escolhem o eixo vertical e a origem dos nomes dos corpos; veja [Importação STEP](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/step-import.md). |
| `save` | `path?` | Salva o projeto (no arquivo atual quando `path` não é fornecido). |

Para alterar uma operação depois de criada, leia sua definição com `execute` `timeline.inspect`
(`{feature}`: ID ou nome) e altere campos com `timeline.edit` (`{feature, set}`; por exemplo,
`{"extent": {"distance": "30"}}`). Os campos por tipo de operação estão listados no capítulo
*Operações de sólidos* do livro.

Recursos: `solidcraft://design` (inspeção JSON com medições) e `solidcraft://commands`
(o catálogo).

Erros: uma ferramenta desconhecida ou argumentos que não correspondam ao esquema da ferramenta são erros JSON-RPC
`-32602`; uma falha do mecanismo (parâmetros de comando inválidos, operação que falha) é um resultado
de ferramenta com `isError: true` e a mensagem do mecanismo, que o modelo pode ler e corrigir.

O retorno de `batch` volta a um checkpoint do mecanismo (`edit.checkpoint`) criado antes da primeira
etapa: o projeto, o esboço ativo e o histórico de desfazer são restaurados exatamente como estavam,
independentemente do tamanho do histórico. Lotes que chamam `file.new` ou `doc.open` recuperam o projeto,
não seu caminho de arquivo. (Um aplicativo antigo sem checkpoints recorre a desfazer tantas etapas quanto
o lote adicionou ao histórico de desfazer.)

## Câmera e roteiro para vistas de detalhe

A `camera` de `screenshot` (o mesmo objeto aceito por `view.apply`, `view.save` e
`solidcraft-cli snapshot --camera`) é uma câmera de órbita,
`{target, yaw, pitch, distance, fov?}`, em milímetros e **radianos**. Ao contrário de `angle` em
`solid.move`, um número sem unidade aqui não representa graus. O observador está em
`target + distance * (-sin(yaw) * cos(pitch), -cos(yaw) * cos(pitch), sin(pitch))`, olhando para
`target`, com Z para cima na tela. Portanto, `yaw: 0, pitch: 0` é a vista frontal: o observador
em -Y, olhando ao longo de +Y, com +X à direita. Um yaw de π/2 olha a partir de -X; π, a partir de
+Y; e -π/2, a partir de +X. O pitch é o ângulo de elevação a partir do plano XY; π/2 olha diretamente
para baixo, com +X à direita e +Y para cima. `fov` é o campo de visão vertical em radianos.
`0`, ou a ausência de `fov`, produz uma vista ortográfica cuja metade da altura visível é
0.4 × `distance`; com um `fov` em perspectiva, a metade da altura no alvo é
`distance * tan(fov / 2)`. `inspect.section` remove o que estiver no lado para o qual a normal
aponta: mantém os pontos `p` com `n · p < d`, em que `d = n · origin` (`flip: true` mantém o
outro lado). Para olhar para dentro do corte, posicione o observador no lado removido.

Para observar de perto uma parte de um modelo grande, como uma montagem STEP, trabalhe em uma
sessão sem interface gráfica. Estas etapas alteram apenas o projeto aberto em memória:

1. Abra o modelo com `solidcraft-cli mcp --in assembly.step` (ou a ferramenta `open`, que também
   aceita `up` e `names`; veja a etapa 4 e [Importação STEP](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/step-import.md)).
   `inspect_design` lista os corpos com seus nomes e caixas delimitadoras.
2. Remova o que estiver no caminho com `solid.remove`, por exemplo `{"bodies": ["Cover"]}`.
3. Separe a região do restante com `solid.box`, por exemplo `{"corner": [40, 0, -45], "length": 20, "width": 50, "height": 45, "operation": "intersect", "targets": ["Housing", "Plate"]}`.
   Cada corpo listado é recortado pela caixa; um corpo inteiramente fora dela é removido.
   A própria caixa não é mantida.
4. Se o modelo estiver deitado (muitos arquivos STEP têm +Y para cima), abra-o com `up: "y"`
   (`open`, `doc.open` ou `file.insert_step`), que gira os corpos um quarto de volta em X
   (Y se torna Z). Para um modelo já aberto, coloque-o com Z para cima usando `solid.move`,
   listando todos os corpos mantidos, por exemplo `{"bodies": ["Housing", "Plate"], "axis": [1, 0, 0], "angle": 90}`.
   Os corpos giram em torno da origem global; um ângulo sem unidade está em graus.
   Uma rotação de +90° em torno de X leva +Y a +Z.
5. Colora as peças relevantes com `appearance.assign`, por exemplo `{"bodies": ["Housing"], "color": "#3a7bd5"}`,
   e oculte corpos auxiliares ou esboços com `browser.visibility`, por exemplo `{"items": ["Plate"], "visible": false}`.
6. Faça uma seção com `inspect.section`, por exemplo `{"plane": {"origin": [50, 30, 0], "normal": [0, 1, 0]}}` ou `{"plane": "XY", "at": 5}`.
7. Capture a imagem com `screenshot`: uma `camera` cujo `target` é o centro da região e cujo
   observador fica no lado cortado, `source: "model"`, `grid: false` quando a região cruza Z 0
   (a grade a atravessaria), e `path`, `width` e `height` conforme
   necessário. Uma imagem ortográfica tem altura de 0.8 × `distance`, portanto uma `distance`
   de aproximadamente 1.5 vez o tamanho da região a enquadra. Observe a imagem, ajuste `yaw`,
   `pitch` e `distance` e capture novamente. Sem interface gráfica, a imagem é sempre uma
   renderização do modelo; conectado ao aplicativo, `source: "model"` solicita essa renderização
   em vez da janela do aplicativo.

## Respostas compactas

`execute` e `batch` usam `verbose: true` por padrão, preservando os resultados completos dos comandos.
Para sessões longas de modelagem, defina `verbose: false` no nível dos argumentos da ferramenta:

```json
{"name": "execute", "arguments": {"command": "solid.box", "params": {"length": 10, "width": 10, "height": 10}, "verbose": false}}
```

Respostas de operações omitem a lista completa `bodies`, mantendo `feature`, `name`,
`created_bodies`, `changed_bodies` e `removed_bodies`. Assim, um corte pode informar um
corpo alterado mesmo sem adicionar ou remover nomes de corpos. Respostas de esboço mantêm novos
IDs sob `created` e omitem apenas listas idênticas `points`, `curves`,
`constraints` e `dimensions` do nível superior. Outros campos e erros permanecem iguais;
comandos de inspeção ainda retornam os resultados completos. A compactação usa diretamente a
resposta do mecanismo, sem uma solicitação adicional de inspeção do projeto.

## Demonstração: uma carcaça criada por MCP

`examples/demo/enclosure_mcp.py` é um pequeno cliente MCP. Ele controla um aplicativo em execução (`solidcraft
--control PORT`) por `solidcraft-cli mcp --connect` e cria uma carcaça para eletrônica com uma
chamada `execute` por vez (`enclosure_steps.json`): uma caixa arredondada, transformada em casca aberta no topo, uma
porta USB cortada em uma parede lateral, quatro ressaltos para parafusos e, depois, uma alteração do parâmetro
`width` que reconstrói a linha do tempo. `examples/demo/record.sh` grava a execução com ffmpeg,
com a legenda de cada etapa incorporada à imagem.

`examples/demo/enclosure_ui_mcp.py` (v2) cria a mesma peça como uma pessoa faria: a
modelagem passa pela interface do aplicativo através do canal de controle. O ponteiro desliza até botões da barra de ferramentas
e faces e clica neles; os diálogos reais abrem, os esboços ficam nas faces do modelo
e a câmera acompanha. Os parâmetros, as cotas de esboço vinculadas a eles e duas extrusões passam
pelo MCP. Com `--story v3` (o padrão), um apoio é repetido em padrão 2 × 2 no diálogo Padrão
retangular com espaçamentos dos parâmetros, e depois largura e parede são alteradas no diálogo
Parâmetros. `record_ui.sh` grava a execução e desenha o ponteiro e as legendas.

## Uma sessão típica

```text
→ {"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"me","version":"1"}}}
→ {"jsonrpc":"2.0","method":"notifications/initialized"}
→ {"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"batch","arguments":{"commands":[
     {"command":"sketch.create","params":{"plane":"XY"}},
     {"command":"sketch.rectangle.two_point","params":{"p0":[0,0],"p1":[40,30]}},
     {"command":"sketch.finish"},
     {"command":"solid.extrude","params":{"distance":20}},
     {"command":"solid.fillet","params":{"edges":[[0,0,10]],"radius":3}},
     {"command":"sketch.create","params":{"plane":"XY"}},
     {"command":"sketch.circle.center","params":{"center":[20,15],"radius":5}},
     {"command":"solid.extrude","params":{"distance":20,"operation":"cut"}}]}}}
← {"jsonrpc":"2.0","id":2,"result":{"content":[{"type":"text","text":"{\"ok\": true, \"completed\": 8, \"results\": [...]}"}]}}
→ {"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"measure","arguments":{}}}
← … "total": {"volume_mm3": 22390.6, …}
→ {"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"screenshot","arguments":{"view":"iso"}}}
← {"jsonrpc":"2.0","id":4,"result":{"content":[{"type":"image","mimeType":"image/png","data":"iVBORw0…"},{"type":"text","text":"{\"source\":\"model\",…}"}]}}
→ {"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"export","arguments":{"path":"/tmp/part.step"}}}
```

Os testes em `crates/mcp/src/tests.rs` executam sessões como essa (caixa → arredondamento → furo, parâmetros,
retorno de lote, argumentos hostis) e verificam os resultados apenas por MCP.
