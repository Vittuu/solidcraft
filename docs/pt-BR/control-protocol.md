# Protocolo de controle

`solidcraft --control PORT` (ou `SOLIDCRAFT_CONTROL_PORT=PORT`) escuta em `127.0.0.1:PORT` por
linhas JSON. Cada solicitação é uma linha `{"id": …, "method": "…", "params": {…}}`; cada resposta é uma
linha `{"id": …, "ok": true, "result": …}` ou `{"id": …, "ok": false, "error": "…"}`. Solicitações são
processadas na thread da interface entre quadros; entradas de ponteiro e teclado são injetadas como eventos egui
reais, um evento de ponteiro por quadro.

A preferência de idioma do aplicativo seleciona os rótulos de comandos, a ajuda e os diagnósticos
próprios nas respostas processadas pela interface. Nomes de métodos, chaves JSON, IDs, valores de
enums e dados criados no modelo mantêm sua grafia canônica. Respostas de autenticação e detalhes
do parser no transporte, antes de a solicitação chegar à interface, permanecem literais.

### Token

Cada conexão precisa se autenticar. Na inicialização, o aplicativo cria um token aleatório de 256 bits para esta
execução e o grava, como uma linha hexadecimal, em um arquivo que somente o usuário pode ler:

| Sistema | Arquivo de token |
|---|---|
| Linux | `$XDG_DATA_HOME/solidcraft/control/PORT.token` (`~/.local/share/solidcraft/control/PORT.token`) |
| macOS | `~/Library/Application Support/SolidCraft/control/PORT.token` |
| Windows | `%LOCALAPPDATA%\SolidCraft\control\PORT.token` |

No Unix, a pasta tem permissões `0700` e o arquivo `0600`; no Windows, as permissões herdadas do arquivo são
substituídas por uma única entrada para o usuário. `SOLIDCRAFT_CONTROL_DIR` muda a pasta. O arquivo é
removido quando o aplicativo termina, e a próxima inicialização grava um novo token. O caminho do arquivo de token é
exibido em stderr na inicialização.

A primeira solicitação de uma conexão deve conter o token como um membro `"token"` no nível superior
(`{"id": 1, "method": "ui.inspect", "params": {}, "token": "…"}`); solicitações posteriores na mesma
conexão podem omiti-lo. Uma primeira linha sem o token correto recebe a resposta
`{"ok": false, "error": "unauthorized: …"}`, e a conexão é fechada sem executar
nada. Tokens são comparados em tempo constante.

`solidcraft-cli mcp --connect 127.0.0.1:PORT` lê automaticamente o arquivo de token dessa porta
(`--token T` ou `SOLIDCRAFT_CONTROL_TOKEN` fornecem um explicitamente). Para uma estrutura de testes ou um
pacote isolado cuja pasta de dados o cliente não consegue ver, inicie o aplicativo com
`SOLIDCRAFT_CONTROL_TOKEN` definido com pelo menos 32 caracteres hexadecimais (por exemplo,
`openssl rand -hex 32`), e ele usa esse token em vez de criar um; o arquivo ainda é
gravado. Um cliente de uma linha em Python:

```python
token = (Path.home() / ".local/share/solidcraft/control/7878.token").read_text().strip()
sock.sendall((json.dumps({"id": 1, "method": "ui.inspect", "params": {}, "token": token}) + "\n").encode())
```

### Limites

O canal é uma entrada não confiável. Uma linha de solicitação pode ter no máximo 16 MiB; uma maior recebe
`{"ok": false, "error": "request too large"}`, e a conexão é fechada, assim como
conexões que enviam oito linhas que não são JSON. No máximo 16 conexões são atendidas simultaneamente
(as seguintes recebem `{"ok": false, "error": "too many connections"}`); no máximo 64 solicitações enfileiradas
são executadas por quadro para que um excesso não impeça o desenho; uma solicitação que a thread da interface não
responde em 60 segundos (3600 segundos para `engine.batch`) recebe
`{"ok": false, "error": "timeout"}`. O mesmo limite de linha de 16 MiB se aplica ao servidor MCP.

Os `params` de qualquer solicitação podem conter `allow_paths: ["/abs/folder", …]`: cada arquivo que essa solicitação
lê, grava ou exclui (caminhos de comandos, como `file.export` com `path` ou `doc.recovery_*` com
`dir`,
`ui.render`, `ui.screenshot`) deve estar dentro de uma dessas pastas depois da resolução de `..` e
links simbólicos, ou a solicitação falha com `… outside the folders this agent may use …`.
O aplicativo adiciona sua própria pasta de recuperação e, quando ela fica seguramente dentro da pasta pessoal e não em
uma pasta oculta, a pasta do projeto aberto (cujos arquivos ocultos continuam fora de alcance). O servidor
MCP conectado envia suas pastas dessa forma em cada solicitação ([mcp.md](https://bherbruck.github.io/solidcraft/pt-BR/guide/mcp.html)). Sem
`allow_paths`, os caminhos de arquivos não são limitados: um cliente que tem o token pode fazer o que o usuário pode.

## Mecanismo

| Método | Parâmetros | Resultado |
|---|---|---|
| `engine.execute` | `command`: ID do comando, `params`: objeto JSON | o resultado do comando |
| `engine.script` | `commands`: `[{command, params}, …]` | resultados, para no primeiro erro |
| `engine.batch` | `commands`: `[{command, params}, …]` (no máximo 10.000), `rollback?`: bool (padrão true) | `{completed, results, undone}`, mais `failed: {index, command, error}` quando uma etapa falha. Executa as etapas em uma solicitação, portanto em um quadro, cada uma como `engine.execute` a executaria. A primeira falha interrompe o lote, e com `rollback` o projeto volta ao estado anterior (`undone`: as etapas desfeitas). Recusado antes de executar qualquer coisa, com um erro começando por `engine.batch unavailable`, enquanto o cursor do agente está visível ou quando uma etapa carrega um arquivo (`doc.open`, `file.insert_step`, `file.insert_mesh`). Envie essas etapas uma a uma. Respondido ao concluir, por até 3600 s |
| `engine.commands` | — | todos os comandos: ID, rótulo, aba, painel, ícone, atalho, documentação de parâmetros, disponibilidade |
| `document.inspect` | `measure?`: bool | parâmetros, linha do tempo (erros, tempos), corpos, esboços, seleção |

Comandos nunca abrem diálogos quando executados dessa forma. `solidcraft-cli commands` exibe o mesmo registro.

Imagens de produto usam `render.preflight/start/status/cancel/export` por `engine.execute`.
Mantenha o ID retornado, com escopo de sessão, nas chamadas seguintes; consultar estado não
bloqueia nem renderiza. Veja [trabalhos de renderização](render-jobs.md) para captura,
cancelamento e exportação.

Enquanto um seletor nativo interativo está aberto, `ui.inspect` informa `file_picker_busy: true`.
Comandos do mecanismo e solicitações `ui.file` e `ui.home` aguardam sua resposta;
redimensionamento, inspeção e capturas continuam disponíveis.

Cada comando `sketch.*` que altera um esboço adiciona `created: {points, curves, constraints,
dimensions}` ao resultado (e `wires` para curvas 3D): os IDs do que adicionou, em ordem de
criação, junto com qualquer outra informação retornada pelo comando (`curves`, `sides`, `corners`…). IDs de esboço são
estáveis e nunca reutilizados, mesmo após uma exclusão (`l1`, `l2`, depois `l3` quando `l2` já não existe; apenas desfazer
retorna a numeração junto com o projeto), então podem ser usados diretamente em comandos posteriores:

```text
{"id":7,"method":"engine.execute","params":{"command":"sketch.line","params":{"points":[[0,0],[40,0]]}}}
→ {"curves":["l1"],"created":{"points":["l1.start","l1.end"],"curves":["l1"],"constraints":["k1"],"dimensions":[]},…}
{"id":8,"method":"engine.execute","params":{"command":"sketch.dimension","params":{"entities":["l1"],"value":50}}}
```

Um `value` numérico sem unidade fornecido a `sketch.dimension` (pelo canal de controle, scripts ou MCP) é
interpretado nas unidades do projeto, exatamente como se fosse digitado na caixa de cotas: em um projeto em polegadas,
`"value": 2` deixa a linha com comprimento de 2 in (50.8 mm). Coordenadas de pontos permanecem em milímetros. Para fixar uma
unidade independentemente do projeto, passe uma expressão com ela: `"value": "50 mm"` ou
`"value": "0.5 in"`. Ângulos são em graus, a menos que a expressão indique outra unidade.

## Interface

| Método | Parâmetros | Observações |
|---|---|---|
| `ui.inspect` | — | Estado da interface, seletor pendente (`file_picker_busy`; alias de compatibilidade `file_dialog_open`, sem caminho), o que está oculto (`visibility`: corpos, origem e itens de construção, esboços, juntas (`hidden_joints`, IDs de juntas); defina com o comando `browser.visibility`), retângulo da área de visualização, câmera, ferramenta/diálogo ativo, linha de estado (`status`: texto e se é um erro), elemento sob o cursor, renderizador, tempos, prévia em tempo real (`active`, `busy`, `error`, `ms`, corpos que substitui) |
| `ui.set` | qualquer campo de `UiState` (`tab`, `perspective`, `showGrid`, `dark`, …), e `hiddenBodies`, `hiddenOrigin`, `hiddenSketches`, `shownSketches`, `hiddenJoints` (listas completas; `browser.visibility` altera itens individuais) | `dark: false` muda para o tema claro |
| `ui.view` | `view`: `front`, `back`, `top`, `bottom`, `left`, `right`, `iso`, `home`, `fit`; `animate` (bool, padrão false: mudança imediata) | |
| `ui.camera` | `target?`: `[x, y, z]` (mm), `yaw?`, `pitch?`, `roll?` (radianos), `distance?` (mm), `fov?`, `up?`; `fit?`: `true`, `false` ou `[ID ou nome de ocorrência…]` (`root`: corpos fora de qualquer componente) | Câmera de órbita da vista: olha para `target` a `distance` de distância. `yaw` 0 olha ao longo de +Y (pela frente), `pitch` é a elevação (±π/2: diretamente para baixo ou para cima), e `roll` gira a imagem. Os campos fornecidos são definidos primeiro; os demais mantêm seus valores. Depois, `fit` enquadra as ocorrências listadas ou tudo, mantendo a direção. Reenviar a resposta restaura a mesma vista, permitindo salvar um enquadramento e reproduzi-lo. `fov` e `up` seguem a perspectiva (`ui.set`) e a preferência de eixo vertical (`ui.prefs`); não podem mudar aqui. Um campo recusado não altera nada. |
| `ui.start` | `command`, `params?` | Como clicar na barra de ferramentas: inicia a ferramenta de esboço ou o diálogo. `params` define os valores iniciais; nada é executado antes de OK. `joint.drive` recebe `joint` (ID ou nome; sem ele, a junta selecionada no navegador ou a primeira que se move) e `values` (números em graus/milímetros ou expressões). Um comando que não aceita parâmetros é recusado antes de iniciar; um parâmetro não aceito pelo diálogo é recusado e o diálogo fecha. |
| `ui.click` | `x`, `y` (pontos na tela), `button?`: left/right/middle, `shift?`, `ctrl?`, `double?` | entrada real de ponteiro |
| `ui.move` | `x`, `y`, `shift?`, `alt?`, `ctrl?`, `cmd?` | movimento do ponteiro sem botão pressionado; os modificadores valem até o próximo movimento/clique sintético; um movimento sem modificadores os libera. Com `nav: "touchpad"`, Shift desloca a vista e Alt/Option orbita. |
| `ui.drag` | `x0`, `y0`, `x1`, `y1`, `button?`, `shift?`, `ctrl?`, `steps?`, `hold?`, `hold_end?` | pressionar, mover, soltar; um arraste esquerdo no modelo seleciona por caixa (da esquerda para a direita: janela, da direita para a esquerda: cruzamento); com navegação OpenSCAD, um arraste esquerdo simples em espaço vazio orbita, Shift+arraste esquerdo seleciona por caixa, e arrastes começando na geometria de esboço a editam; `hold` mantém o botão pressionado por essa quantidade de quadros primeiro (pressionar e segurar o botão direito abre o menu radial, soltar sobre uma direção a seleciona), `hold_end` faz o mesmo ao final (um arraste de esboço mostra seu ajuste) |
| `ui.menu` | `x?`, `y?`, `target?`, `close?` | com um ponto: abre um menu de contexto ali como um clique direito (o menu radial da área de visualização, ou um menu do navegador com `target: {"type": "body", "name"} \| {"type": "sketch", "id"} \| {"type": "component", "id"}`); sem ponto: o menu aberto (`radial` e `items`: ID, rótulo, atalho, disponibilidade) |
| `ui.menuPick` | `item`: ID ou rótulo | executa um item do menu aberto, como clicar nele |
| `ui.selectOther` | `x?`, `y?`, `pick?`, `add?`, `hover?`, `close?` | Selecionar outro: com um ponto, lista tudo selecionável ali (como segurar o botão esquerdo); `pick`: seleciona a linha `pick` (`add`: como Shift+clique; com um diálogo aberto, para sua entrada ativa); `hover`: realça uma linha; sem ponto: a lista aberta (`open`, `at`, `total`: tudo sob o cursor, `rows`: rótulo, ícone, sel, no máximo 20 por tipo, mais próximos primeiro) |
| `ui.rename` | `text?`, `commit?` (padrão true) | conclui a caixa de renomeação aberta por um item Renomear |
| `ui.selection` | | a seleção, as entradas do diálogo aberto, o item sob o cursor e as cotas desenhadas do esboço ativo (`dimensions`: ID e centro do texto, `dimension_selected`, `dimension_editing`) |
| `ui.worldToScreen` | `points`: pontos globais `[[x, y, z]…]` | seus pontos na tela |
| `ui.triad` | | pontos na tela das alças da tríade de movimento do esboço 3D (`x`, `y`, `z`, `plane`) |
| `ui.sketchToScreen` | `points`: `[[x, y]…]` em coordenadas do esboço ativo | seus pontos na tela (null quando atrás da câmera) |
| `ui.dialogInput` | `index` | o que clicar no botão Selecionar de uma entrada faz: seleções vão para essa entrada |
| `ui.at` | `world: [x,y,z]` \| `sketch: [x,y]` \| `plane: "XY"` \| `axis: "Z"` \| `dimension: "d1"` (o texto de seu valor) \| `handle: "arrow"` (uma alça de manipulador, veja `scenario::publish_handle`) | o ponto na tela (destino de uma etapa de cenário; veja `crates/ui-egui/src/scenario.rs`) |
| `ui.handles` | `prefix?` | as alças desenhadas no último quadro, `[{name, x, y}]`: botões da barra de ferramentas e itens suspensos `toolbar:<command id>` (apenas o botão: `button:<command id>`; uma linha suspensa: `dropdown:<command id>`, realçada `dropdown_hot:<command id>`, seu fixador `dropdown_pin:<command id>`; caixas de seleção de Personalizar barra de ferramentas `customize:<command id>`), nomes de painéis `panel:<tab>:<panel>`, linhas do navegador `row:<label>` e setas de expansão `fold:<label>`, alças de manipulador; pontas de eixos com sinal `axis_view:z`, `axis_view:-y`; faces opcionais do cubo `navigation_cube:top`, `navigation_cube:top_front_right`, controles `navigation_cube:control:home` (e outras ações), além de aliases de eixos com sinal para faces visíveis do cubo; seletor de navegador das Preferências `prefs:navigator`, opções `prefs:navigator:Signed axes` e `prefs:navigator:Cube` |
| `ui.editFeature` | `feature` (ID ou nome) | o que um clique duplo em um item da linha do tempo faz: retorna até ele e abre seu diálogo preenchido (esboços abrem no modo de esboço) |
| `ui.scroll` | `x`, `y`, `delta?` | zoom com a roda no cursor |
| `ui.key` | `key` (nome de tecla egui, `Enter`, `Escape`…), `cmd?`, `shift?` | |
| `ui.text` | `text` | texto digitado |
| `ui.screenshot` | `path?` | PNG da janela (exige um quadro apresentado) |
| `ui.render` | `path`, `width?`, `height?`, `grid?`, `axes?` | Renderização do modelo com a câmera atual (sem exigir janela), gerada no dispositivo GPU do aplicativo com os mesmos pipelines da área de visualização (o campo `renderer` da resposta: `gpu`, ou `gpu-software` em um adaptador por software), no tema da janela (claro ou escuro; `solidcraft-cli snapshot` mantém cores claras); cortada pela seção exibida na área de visualização (`inspect.section`, prévia de um diálogo aberto de Análise de seção, Fatiar de um esboço), com a face de corte tampada. Grade e eixos seguem a janela (`ui.set` `showGrid`/`showOrigin`, `hiddenOrigin`); `grid` e `axes` (booleanos) podem sobrescrever essa escolha apenas para esta renderização. |
| `ui.confirm` | `accept?`: bool | responde à confirmação de Excluir (mostrada quando uma exclusão leva outras operações junto ou faz algumas falharem); sem `accept`, informa o que ela lista |
| `ui.documents` | `action?`: `new`, `switch`, `close`, `quit`; `index?`; `force?` (fechar sem perguntar); `answer?`: `save`, `dont_save`, `cancel` (responde a "salvar alterações?") | os projetos abertos (abas): nomes, indicadores de alterações não salvas, o ativo, uma pergunta pendente "salvar alterações?", se uma saída está pedindo confirmação |
| `ui.file` | `path`; `insert?`: bool | inicia Abrir ou Inserir interativo em segundo plano; consulte `ui.inspect.file_busy` para verificar a conclusão; a janela continua respondendo |
| `ui.home` | `open?`: bool; `sample?`: índice | a página inicial: mostrar ou ocultar, abrir um exemplo integrado (depois de compilado); retorna `open` e os projetos recentes |
| `ui.shortcut` | `command`, `key?`, `replace?` | a tecla de um comando, ou a redefine (`""` remove; uma tecla usada por outro comando é recusada, a menos que se use `replace`) |
| `ui.help` | `item?`: `about`, `shortcuts` (somente leitura), `report` (abre um rascunho de relatório para revisão), `crashes` (a janela Relatórios de falha) | a versão, commit, data de compilação e texto de diagnóstico |
| `ui.prefs` | qualquer um de `default_units`, `nav` (`fusion`, `solidworks`, `inventor`, `alias`, `blender`, `tinkercad`, `openscad`, `touchpad`), `zoom_reverse`, `zoom_to_cursor`, `smooth_wheel_zoom` (padrão true), `trackpad_pan` (padrão true: rolagem com dois dedos desloca a vista, exceto no perfil Touchpad, que sempre aplica zoom; pinça aplica zoom), `orbit_around` (`view_centre` (padrão: o centro movido por Pan e enquadrado por Ajustar), `origin` (a origem global permanece imóvel na tela), `cursor` (o ponto do modelo sob o ponteiro quando o arraste começou); o booleano anterior `orbit_cursor` ainda define `cursor` quando true), `look_at_anim`, `look_at_speed`, `middle_double_click` (`home` (padrão), `fit`, `off`), `msaa`, `show_frame_rate` (padrão false: quadros por segundo no canto inferior direito da vista; a vista é redesenhada a cada quadro enquanto a opção está ativada), `length_decimals`, `angle_decimals`, `theme` (`dark`, `light`, `system`), `language` (`en`, `de`, `fr`, `ja`, `ko`, `pt-BR`, `ru`, `tr`, `uk`, `zh`; veja [localização](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/localization.md)), `ui_scale`, `ui_font` (uma família de fontes instalada ou um arquivo de fonte para a interface; vazio usa a fonte integrada), `up_axis` (`z`, `y`), `offer_stand_up`, `color_vision` (`off` (padrão), `protan`, `deutan`, `tritan`: modos de visão de cores, #778), `check_updates` (padrão true; [atualizações](book/src/guide/updates.md)); `open` (uma página: General, Navigation, Keyboard Shortcuts, Design, Graphics, Sketch, Selection, AI) abre o diálogo Preferências nela; `button`: `ok`, `cancel`, `apply`, `defaults`; `stand_up: true` coloca na vertical um arquivo Y-up recém-aberto | define preferências (mantidas entre execuções), retorna todas elas mais `dialog` (a página aberta ou null), `dark` e `stand_up_offer` |
| `ui.capture` | `action?`: `capture`, `revert`, `cancel` | responde à pergunta Capturar posição (feita quando um comando começa ou o projeto é salvo enquanto componentes foram movidos sem capturar); sem `action`, informa o que está aguardando |
| `ui.window` | `action`: minimize, maximize, restore, toggle, close | o que os botões de controle da barra de título fazem (no Windows e no Linux, a barra do aplicativo é a barra de título); fechar pergunta primeiro sobre projetos não salvos |
| `ui.agent_cursor` | `show` (bool), `speed`: um número de 1 a 5 (vezes a velocidade normal) ou `"instant"` (sem animação: o comando é executado imediatamente e o ponteiro vai ao local da ação), `follow_camera` (bool); todos opcionais | as configurações. Enquanto o cursor está visível, `engine.execute` primeiro desliza até o botão do comando na barra de ferramentas e até cada seleção, depois executa e responde; solicitações posteriores aguardam sua vez. `"animate": false` em uma chamada pula a animação |
| `ui.resize` | `width`, `height` | |
| `app.quit` | `force?` (sair sem perguntar sobre projetos não salvos) | `quitting` (a janela fecha), `asking` (uma pergunta "salvar alterações?" está aberta) |

`ui.workspace {workspace?: "design" | "render"}` lê ou altera o espaço ativo.
Entrar em Render exige concluir a operação de modelagem atual; voltar restaura a câmera
Projeto. Veja [o espaço Render integrado](render-workspace.md).

### Movimento contínuo da câmera

O movimento contínuo da câmera também está disponível por `engine.execute` com o comando
`view.navigate`: passe `camera` (de `ui.inspect`) e `motion: "pan" | "orbit" | "zoom"`.
Deslocamento e órbita recebem `delta: [dx,dy]` em pontos de tela (cada um entre ±10000); o deslocamento também exige
`viewport: [width,height]` (cada um de 1..1000000). A órbita aceita um `pivot` em coordenadas globais com o
tamanho da área de visualização. O zoom recebe `factor` (0.05..20, abaixo de 1 aproxima) e um `anchor`
opcional em coordenadas globais. A `camera` retornada é aplicada imediatamente pela interface, sem animação,
edições no projeto, etapas de desfazer ou uma entrada no registro de sucesso para cada movimento.

## MCP

`solidcraft-cli mcp --connect 127.0.0.1:PORT` conecta um cliente MCP (um agente de IA) a este
canal; veja [mcp.md](https://bherbruck.github.io/solidcraft/pt-BR/guide/mcp.html).

## Exemplo

```text
{"id":1,"method":"engine.execute","params":{"command":"sketch.create","params":{"plane":"XY"}}}
{"id":2,"method":"engine.execute","params":{"command":"sketch.rectangle.two_point","params":{"p0":[0,0],"p1":[40,30]}}}
{"id":3,"method":"engine.execute","params":{"command":"solid.extrude","params":{"distance":20}}}
{"id":4,"method":"engine.execute","params":{"command":"solid.fillet","params":{"edges":[[0,0,10]],"radius":3}}}
{"id":5,"method":"ui.screenshot","params":{"path":"/tmp/part.png"}}
```

Preferências → Navegação → Navegador de vistas oferece **Eixos com sinal** (o padrão original) e
**Cubo**. Essa escolha é salva independentemente do perfil do mouse. O canal de controle a seleciona
com `ui.prefs {"navigator_style":"axes"}` ou `ui.prefs {"navigator_style":"cube"}`.
Configurações existentes sem esse campo mantêm eixos com sinal. As pontas dos eixos escolhem vistas principais;
clicar novamente em uma ponta alinhada inverte a vista.

Segure Shift durante um gesto de órbita para ajustar à vista padrão ou diagonal mais próxima. Isso se aplica a
arrastar com o botão direito, à ferramenta de navegação Órbita e ao arraste de qualquer navegador de vistas. No perfil padrão,
Shift+botão do meio também orbita com ajuste; o botão do meio sozinho desloca a vista. Faces do cubo escolhem vistas principais,
e cantos escolhem vistas isométricas. O meio de cada aresta do cubo pertence à sua face. Os nomes de faces
seguem a preferência do eixo vertical; a pequena tríade X/Y/Z mostra eixos globais. Clicar na mesma face mantém essa vista.
As setas voltadas para dentro orbitam em 90 graus; as setas curvas no canto superior direito giram a
vista em 90 graus. O botão Inicial no canto superior esquerdo retorna à isométrica, e o círculo inferior direito
ajusta o modelo.

O cubo fica discreto quando inativo. Passar o cursor em sua área realça o cubo e a barra de navegação e
revela os controles ao redor; a face ou canto escolhido recebe preenchimento azul-claro e
contorno azul. Afastar o cursor restaura o estado discreto. Controles da barra de navegação também ficam realçados
quando o cursor passa sobre eles ou uma ferramenta de navegação está ativa.

O cubo tem seis alvos de face e oito de canto, sem alvos de aresta. As regiões de seleção
de canto ficam nas seis faces planas, preservando cantos vivos sem linhas permanentes de
subdivisão. Passar o cursor em um canto realça juntas suas partes nas faces visíveis adjacentes.

O widget de cubo ocupa um quadrado compacto de 112 pontos no canto da área de visualização. Uma face de
cubo vista de frente tem aproximadamente 55 pontos de largura; desenho e seleção escalam juntos. Nomes de faces
são esticados verticalmente. Cliques de vista passam suavemente por orientações intermediárias quando a animação
de mudanças de vista em Navegação está ativada (o padrão), na velocidade de animação configurada.

O cubo executa `view.navigate` pelo mecanismo de comandos. Chamadas programáticas passam `camera`
e `direction: [x,y,z]` no sistema de coordenadas do eixo vertical da câmera, ou `action` (`left`, `right`, `up`,
`down`, `clockwise`, `counterclockwise`, `home`, `fit`). O resultado contém `camera` e `fit`;
chamadas de interface aplicam a câmera com sua preferência de animação, sem editar o projeto.

O ajuste durante órbita com Shift inclui vistas de aresta a 45 graus e vistas isométricas de canto, com uma
transição de mola levemente amortecida e uma pequena margem nos limites para impedir oscilações.
