# SolidCraft — instruções para agentes

SolidCraft é um aplicativo de CAD 3D paramétrico de código aberto, desenvolvido de forma independente e inteiramente em Rust:
esboços com um solucionador de restrições, uma linha do tempo paramétrica de operações, um núcleo B-rep
e uma interface de desktop que agentes podem controlar. Projetos publicados por
[storytold](https://github.com/storytold) podem servir de referência para convenções de implementação.
Qualquer código adaptado de outros projetos (MIT OR Apache-2.0) é indicado em ATTRIBUTION.md.

## Nome do produto
SolidCraft em todo lugar: crates `solidcraft-*`, binários `solidcraft` e `solidcraft-cli`,
projetos `.solidcraft`, a etiqueta de formato `solidcraft/N`, variáveis de ambiente `SOLIDCRAFT_*`,
pastas de configurações e recuperação, o nome do servidor MCP, documentação, CI e empacotamento. O nome antigo
permanece apenas nas camadas explícitas de compatibilidade listadas em `docs/compatibility.md`; não adicione
novos usos dele em outros lugares. Implantações públicas de documentação/web/Flatpak usam o caminho do Pages
`/solidcraft/`; não adicione aliases permanentes do nome antigo no Pages.

## Comece toda sessão aqui
1. Leia `plan/STATUS.md` (marco atual e próxima tarefa), depois a tarefa em `plan/execution-plan.md`
   e a seção relevante de `plan/architecture.md`.
2. Decisões sobre núcleo e bibliotecas: `plan/adr/0001-geometry-kernel.md` (do que podemos depender, o que
   é rejeitado por motivos de licença).
3. Siga o protocolo de operação autônoma (`plan/execution-plan.md` §6). Não pare para perguntar
   a menos que ele classifique a decisão como responsabilidade do proprietário.

`plan/` é ignorado pelo git (apenas local).

## Nunca falhar abruptamente
As pessoas confiam seus projetos ao SolidCraft; uma falha abrupta perde seu trabalho. **Isso tem prioridade sobre novas funcionalidades.**
- **Nenhum panic em código fora dos testes:** nenhum `unwrap()`, `expect()`, `panic!`, `unreachable!`, `todo!`,
  `unimplemented!`; nenhum `unsafe` (`unsafe_code = "forbid"`). As três exceções, na crate do
  aplicativo (que usa deny), não têm API segura: duas funções em
  `apps/solidcraft/src/open_documents.rs` (o macOS entrega arquivos do Finder apenas a um método
  Objective-C) e `apps/solidcraft/src/native_crash.rs`, exclusivo do Windows (o filtro de
  exceções que registra falhas em código nativo, como drivers gráficos) e
  `apps/solidcraft/src/native_frame.rs`, exclusivo do Windows (o procedimento de janela que
  responde ao teste de detecção das bordas de redimensionamento da janela sem decorações do sistema).
- **Erros são `Result<T, E>`** por meio do tipo de erro da crate e de `?`. Uma funcionalidade inacabada
  retorna um erro ("ainda não suportado: …"); nunca gera um panic.
- **Números derivados da entrada são hostis** (parâmetros de comandos, arquivos de projeto, entrada do canal
  de controle): use `get()`, não `[i]`, verificações de finitude e limites para tamanhos e contagens de iterações.
- **Chamadas a núcleos de terceiros são protegidas:** tudo que chama `truck` passa por
  `solidcraft_kernel::guard`, que transforma um panic em `KernelError::Internal`.
- **Proteção de último recurso:** `Session::execute` executa todos os comandos sob `catch_unwind`; um
  panic que escape restaura o projeto e informa `EngineError::Internal`.
- **Relatórios de falha:** o aplicativo para desktop grava cada panic e cada encerramento anormal em uma pasta local de falhas,
  com o projeto e os comandos que o reproduzem (`solidcraft_engine::crash`, docs/crash-reports.md).
  Quando alguém disser que o aplicativo falhou, procure ali primeiro: `solidcraft-cli crashes`, `crashes show <id>`,
  `crashes replay <id>` (ou o comando `diagnostics.crash_reports` / ferramenta MCP `crash_reports`).
- **Comprove:** toda correção de falha inclui um teste de regressão (veja `hostile_params_never_panic`);
  o `design.solidcraft` + `replay.json` de um relatório de falha são o ponto de partida do teste.
- Toda raiz de crate de produção contém
  `#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]`.

## Regras inegociáveis
- **Independente.** SolidCraft é independente. O comportamento é projetado a partir de documentação pública,
  convenções gerais de CAD e comentários dos usuários. Os testes de referência usam valores analíticos e ferramentas de código aberto.
  Nunca copie o código, os ícones, a arte, os textos de ajuda, as dicas, os modelos ou os
  materiais de outro produto. Os formatos de arquivo vêm de especificações públicas.
- **Nenhum produto de CAD comercial.** Agentes não devem operar, criar scripts, automatizar ou extrair dados de
  produtos de CAD comerciais (Fusion, SolidWorks, Onshape, …) para este projeto: nenhum dado de teste,
  medição, lista de comandos, ID ou arquivo produzido com eles.
- **Nunca leia, copie ou vincule código GPL/LGPL/AGPL**: OpenCASCADE, FreeCAD (incluindo planegcs),
  SolveSpace, CadQuery/OCP, LibreCAD, QCAD — nem mesmo leia seus códigos-fonte. (`tools/freecad-oracle/`
  executa o FreeCAD como programa separado por meio de sua API pública de scripts para calcular valores de referência;
  nada do FreeCAD é vinculado, incorporado ou distribuído.) O solucionador de esboços e as operações
  de transição são nossos.
- **Recursos — absolutamente essencial.** SolidCraft não contém **nenhum ícone, imagem, fonte ou
  arte de produtos de CAD comerciais — nunca.** Todos os ícones da interface são desenhados em código reunido
  (`assets/icons/ui/source/`, compilado por `crates/ui-egui/src/icons.rs`). Qualquer recurso em arquivo deve ser original ou ter licença aberta e **deve ter uma linha em ATTRIBUTION.md**
  (`cargo xtask assets` impõe essa regra). Capturas de tela de software de CAD comercial nunca são incluídas no repositório.
  Violar essa regra é o erro mais grave que se pode cometer neste repositório.
- **Tudo é um comando.** Comportamento visível ao usuário = um comando em `crates/engine/src/cmd/*`
  (`CommandSpec`: id = nosso próprio ID em letras minúsculas e separado por pontos `<area>.<thing>[.<variant>]`, como
  `solid.extrude` ou `sketch.rectangle.two_point`, nunca o ID de comando de outro produto; rótulo;
  aba e painel da barra de ferramentas; ícone; atalho; documentação dos parâmetros; `enabled`; JSON `run`) + testes. A barra de ferramentas,
  os diálogos, as ferramentas de esboço, a paleta, os scripts, a CLI e o canal de controle acessam os mesmos comandos.
- **Chamadas programáticas nunca abrem diálogos.** `engine.execute` executa a forma JSON; somente cliques
  na barra de ferramentas (`ui.start`) iniciam ferramentas interativas e diálogos, que então executam comandos.
- **As camadas** são impostas por `cargo xtask layers`: L0 `geom` → L1 `sketch`, `kernel`, `render`
  → L2 `doc` → L3 `io` → L4 `engine` → L5 `ui-egui` → apps. Nenhuma camada abaixo de L5 depende de
  egui/eframe/winit/wgpu/rfd. **A crate da interface é substituível.** Somente `crates/kernel` pode usar `truck`.
- **A interface é enxuta**: os painéis leem o estado do mecanismo e agem por meio de `app.run(id, params)` /
  `app.start(id)`. As cores vêm de `theme::Tokens`.
- **Um único conjunto de controles de câmera**: toda página ou tela com câmera (área de visualização, prévia de
  Renderizar, futuras páginas de planificação ou desenho) chama `navigation::handle` com suas `NavCaps`
  (órbita desativada em páginas 2D planas). Novas telas devem usar `navigation::handle`; nunca implemente
  entrada de câmera por conta própria (um teste rejeita isso).
- **Somente Rust.** As unidades internas são milímetros e radianos; Z aponta para cima. A única exceção é
  o JavaScript exigido pelo navegador: o service worker `apps/solidcraft-web/sw.js` (navegadores só
  executam service workers como JavaScript) e os pequenos scripts inline em `apps/solidcraft-web/index.html`.
  Mantenha-os mínimos; `cargo xtask web-offline-test` cobre o service worker.
- **Verificações de qualidade** antes de cada commit: `cargo xtask ci` (fmt, clippy -D warnings, testes,
  recursos, camadas). Faça um commit após cada etapa de funcionalidade que compile.
- **Publicação**: `origin` é github.com/bherbruck/solidcraft. As alterações entram por **pull requests**:
  envie uma branch, abra uma PR e deixe a CI do GitHub executar. Os mantenedores (e os agentes que trabalham para eles) podem
  integrar suas próprias PRs assim que a CI estiver verde (`gh pr merge --squash`); uma correção trivial (documentação, erros de digitação)
  pode ser integrada imediatamente. Mantenha cada PR em um único assunto, aplique rebase ou integre `main` nela quando
  houver conflitos e nunca force um push para `main`. Nunca inclua `plan/`, diretórios de compilação (`target*/`) ou
  dados de referência gerados; confira `git ls-files` antes de enviar.
- **Nunca quebre o wasm**: tudo abaixo dos aplicativos deve compilar para `wasm32-unknown-unknown`
  (`cargo xtask wasm`, parte de `cargo xtask ci`).

## Mostrar e acompanhar as alterações até a conclusão

- Ajude os revisores a visualizar o que está quebrado ou alterado. Para comportamentos observáveis de CAD, interface ou modelo,
  issues e PRs devem incluir capturas de tela descritivas reais ou uma gravação curta de uma
  pequena reprodução. Isso inclui alterações no núcleo, na geometria, em montagens e no backend cujos
  efeitos possam ser mostrados no aplicativo. Agentes de programação devem executar o aplicativo e capturar essas evidências
  por conta própria; não peça a uma pessoa evidências que o agente possa produzir. Mostre a falha ou o resultado real,
  de preferência antes e depois, com passos breves de reprodução, o comportamento esperado/real
  e anotações que apontem o detalhe relevante.
- Fluxogramas e diagramas Mermaid não substituem capturas de tela ou gravações de comportamentos
  visíveis. Use diagramas para raciocínio arquitetural quando capturas de tela reais não puderem ilustrar o
  ponto. Alterações sem comportamento observável útil não precisam de capturas de tela ou vídeos.
- Use um pequeno exemplo seguro para publicação e confira projetos privados, caminhos, detalhes de conta ou
  outros conteúdos confidenciais antes de compartilhar. Anexe a mídia à issue/PR ou forneça um link para uma
  gravação acessível; não inclua no repositório capturas de tela ou vídeos gerados apenas para servir de evidência de revisão.
- Ao longo do trabalho e antes da entrega, verifique os commits atuais do upstream e o estado de integração
  da PR relevante. Mantenha sua branch atualizada com as integrações do upstream, resolva conflitos sem
  descartar o trabalho de outras pessoas e repita a validação quando a integração alterar o resultado. Use
  `git pull --rebase origin main` antes de enviar, como indicado acima; nunca force o push. Informe o estado
  do upstream/PR verificado, os conflitos resolvidos e a validação realizada. Se uma PR continuar aberta,
  acompanhe-a enquanto a tarefa estiver ativa e deixe seu estado pendente claro na entrega;
  não afirme que foi integrada sem verificar.

## Reservar uma issue = abrir uma PR em rascunho
- Antes de trabalhar na issue #N, verifique se ninguém a reservou: `gh pr list -R bherbruck/solidcraft --state open --search "#N"`.
- Se não houver uma reserva, crie uma branch a partir do `origin/main` mais recente, execute
  `git commit --allow-empty -m "claim: #N"`, envie a branch e abra imediatamente uma PR em
  **rascunho** com o título `<summary> (#N)`. **A PR aberta com o menor número para uma issue
  tem prioridade**; se a sua perdeu, feche-a. Não reserve uma issue por comentários nem editando arquivos.
- Envie o trabalho em andamento pelo menos uma vez por dia. Um rascunho sem pushes por 72 horas
  pode ser assumido por outra pessoa: comente nele primeiro e depois abra sua própria reserva
  (você pode começar a partir da branch dele).
- Em uma máquina de compilação compartilhada, verifique a carga antes de uma compilação grande
  (`cut -d' ' -f1 /proc/loadavg`) e espere enquanto ela estiver acima de 24; compile com `nice`.

## Formato dos arquivos de projeto
Uma alteração que salva algo novo em um projeto adiciona UM arquivo, `crates/doc/src/format/pending/<name>.rs`
(uma nota `//!` e `pub fn step` se projetos antigos precisarem ser reescritos). Nunca edite `FORMAT`,
`JSON_STEPS`, o histórico em `format.rs`, o arquivo de outra alteração nem uma pasta `v<N>/`: a
compilação determina o número (o próximo enquanto houver alterações pendentes), e a versão publicada
incorpora as alterações pendentes em `v<N>/` com `scripts/fold-format-pending.sh`. Veja CONTRIBUTING.md.

## Versões do aplicativo
Pushes de desenvolvimento não são versões do aplicativo. Siga `docs/releasing.md` quando for solicitado a
preparar ou distribuir uma versão: escolha a versão a partir das alterações reais, atualize Cargo.toml e
Cargo.lock, passe pelas verificações de qualidade, avance `release` sem forçar o push e verifique o
rascunho resultante e seus artefatos. Quando a publicação estiver autorizada, publique esse rascunho verificado
e confirme sua tag e a implantação posterior. Não publique automaticamente a cada push para `main`, não deduza
incrementos de versão apenas pelos prefixos de commits, não sobrescreva versões publicadas nem envie tags de versão
antes de verificar o rascunho.

## Executar e visualizar o aplicativo
- `cargo run --release -p solidcraft -- --sample --control PORT` (projeto de exemplo + canal de controle).
  Escolha uma porta livre. Sem monitor, execute sob `Xvfb :99` com `DISPLAY=:99`.
- Controle-o com linhas JSON em `127.0.0.1:PORT` (veja `docs/control-protocol.md`). A primeira
  solicitação em uma conexão contém o token da inicialização como `"token"`: leia-o de
  `~/.local/share/solidcraft/control/PORT.token` ou inicie o aplicativo com
  `SOLIDCRAFT_CONTROL_TOKEN=$(openssl rand -hex 32)` e envie esse token.
  - `{"id":1,"method":"engine.execute","params":{"command":"solid.extrude","params":{"distance":20}}}`
  - `{"id":2,"method":"ui.start","params":{"command":"sketch.create"}}` depois `ui.click {x, y}`
  - `{"id":3,"method":"ui.screenshot","params":{"path":"/tmp/shot.png"}}` depois leia o PNG.
- **Para alterações na interface, veja o resultado** (captura de tela, leia o PNG). Sem janela, use
  `ui.render` ou `solidcraft-cli snapshot`.
- Sem interface gráfica: `solidcraft-cli run script.json --out part.step`, `solidcraft-cli eval design.solidcraft`,
  `solidcraft-cli snapshot script.json --out shot.png`, `solidcraft-cli commands`.
- Testes de referência: `cargo xtask oracle` reproduz `tests/oracle/*/recipe.json` e compara com
  o `expected.json` de cada caso (valores analíticos ou do FreeCAD; grava `docs/oracle.md`). Os valores esperados
  nunca vêm de um produto de CAD comercial.
- Compile com `CARGO_TARGET_DIR=<repo>/target`. Windows: `cargo xwin build --release --target x86_64-pc-windows-msvc -p solidcraft`.

## Roteiro
`ROADMAP.md` (versionado) acompanha o estado, os marcos e as estimativas. Atualize-o sempre que uma
tarefa de marco for integrada. `docs/features.md` (comandos do SolidCraft por área, a partir do registro) e
`THIRD-PARTY-LICENSES.txt` são gerados novamente em `main` após cada integração pelo workflow Generated files;
nunca os gere novamente em uma PR.

## Listas que recebem itens de muitas PRs
Evite conflitos nelas (CONTRIBUTING.md, "Listas que recebem itens de muitas alterações"): catálogos de
mensagens ordenados (`cargo xtask lists --fix`), linhas `mod` em seu grupo ordenado, a lista de idiomas
com inglês primeiro e depois por ID, arquivos gerados deixados para `main` e uma alteração de formato
como um novo arquivo.
