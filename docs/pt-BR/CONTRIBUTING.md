# Como contribuir com o SolidCraft

SolidCraft é um aplicativo de CAD 3D paramétrico de código aberto escrito inteiramente em Rust.
Você desenha em um plano, aplica restrições ao esboço, transforma perfis em sólidos e
altera um parâmetro para ver a linha do tempo ser reconstruída. Ele funciona como aplicativo para desktop (Windows, Linux), no
navegador (wasm + WebGPU/WebGL2), sem interface gráfica por uma CLI e como servidor MCP com o qual agentes de IA podem
modelar.

Sua ajuda é bem-vinda, seja corrigindo uma linha ou assumindo uma área inteira. Esta página explica como
compilar o projeto, como o código está organizado, as poucas regras inegociáveis e onde a ajuda é
mais necessária.

## Coloque para funcionar (cerca de 10 minutos em uma máquina rápida)

Você precisa do Rust estável 1.95 ou mais recente (exigido pelo egui/eframe 0.36). Para a compilação
web, você também precisa de `rustup target add wasm32-unknown-unknown` e [`trunk`](https://trunkrs.dev).

```sh
git clone https://github.com/bherbruck/solidcraft && cd solidcraft
cargo run --release -p solidcraft -- --sample      # desktop app with a sample part
cargo xtask test                                   # unit, engine and UI-scenario tests
cargo xtask ci                                     # everything a PR must pass (see below)
```

SolidCraft é o nome do produto, do repositório, do pacote Cargo e do executável.
Compile o aplicativo para desktop com `cargo build --release -p solidcraft` ou a CLI com
`cargo build --release -p solidcraft-cli`. Execute a CLI com
`cargo run --release -p solidcraft-cli -- commands` (ou `target/release/solidcraft-cli commands`
após compilar). O nome antigo aparece apenas nas camadas de compatibilidade listadas em
[docs/compatibility.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/compatibility.md); não adicione novos usos dele.

Web: `cd apps/solidcraft-web && trunk serve --release`, depois abra a URL exibida (`?sample`
abre a peça de exemplo; `?webgl` força o WebGL2). Salvar e abrir arquivos no navegador exige
HTTPS ou localhost, pois utiliza o Origin Private File System.

Windows a partir do Linux: `cargo xwin build --release --target x86_64-pc-windows-msvc -p solidcraft`.

### NixOS: `nix develop`

O NixOS não tem `/usr/lib`, por isso nada está em `LD_LIBRARY_PATH` e um simples `cargo run` não encontra
nenhuma das bibliotecas que o aplicativo carrega em tempo de execução: as bibliotecas Wayland pelas quais o winit abre a janela, o carregador EGL
pelo qual o wgpu renderiza e a libdbus-1, pela qual os diálogos de arquivo do rfd acessam o xdg-desktop-portal
(nesse caso, o diálogo nunca aparece e a seleção não retorna um caminho):

```
ERROR solidcraft: winit EventLoopError: ... The wayland library could not be loaded
ERROR eframe::native::run: Exiting because of error: WGPU error: Failed to create surface for any enabled backend: {}
ERROR rfd::backend::xdg_desktop_portal::portal::libdbus: Can't connect to a portal: libdbus-1.so not found
```

`nix develop` (o [`flake.nix`](https://github.com/bherbruck/solidcraft/blob/main/flake.nix) na raiz do repositório) fornece as bibliotecas de execução
Wayland, X11, EGL, Vulkan-loader e D-Bus. Também expõe os manifestos dos drivers do NixOS
em `/run/opengl-driver/share`, para que o Vulkan possa descobrir os drivers
do sistema. Habilite `hardware.graphics.enable` no NixOS; outras distribuições Linux
podem precisar de um [wrapper nixGL](https://github.com/nix-community/nixGL) compatível.
O shell de desenvolvimento padrão mantém a cadeia de ferramentas Rust/C já disponível em `PATH`;
`nix develop .#rust` fornece a cadeia de ferramentas fixada e compila em `target/nix`.

Para iniciar, use `nix run .#solidcraft -- --sample`. Para instalar os comandos de terminal
do aplicativo, a entrada no menu do desktop e os ícones gerados, use `nix profile install .#solidcraft`.
`nix build` sozinho cria um resultado de compilação; ele não instala um lançador no
menu do desktop. O comando `solidcraft` instalado executa o aplicativo com seu wrapper; `solidcraft-cli` também está
incluído. Consulte
[Instalação e solução de problemas com Nix](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/nix.md) para comandos de flake remota,
integração com o desktop e como identificar um executável antigo em `PATH`.

## Como o código está organizado

O [README](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/README.md#arquitetura) tem a tabela de camadas. A versão curta:

- `crates/geom`: vetores, planos, malhas e medidas.
- `crates/sketch`: nosso próprio solucionador de restrições, além da identificação de perfis.
- `crates/kernel`: a fronteira do núcleo B-rep. Encapsula o [truck](https://github.com/ricosjp/truck)
  e é a única crate autorizada a acessar o truck. Arredondamentos, chanfros, casca/deslocamento, Excluir face,
  Dividir face, leitura e gravação STEP/IGES e rastreamento da origem das faces são implementados aqui por nós. Cópias
  corrigidas de três crates do truck ficam em `vendor/` e estão excluídas de nossas regras de fmt/clippy.
- `crates/doc`: o documento. Contém parâmetros e expressões, a linha do tempo de operações e sua
  reconstrução incremental, componentes e ocorrências, juntas, aparências e nomeação persistente.
- `crates/io`: arquivos de projeto e formatos de malha (3MF, STL, OBJ).
- `crates/engine`: a sessão e o registro de comandos. **Tudo que o usuário pode fazer é um
  comando**, e a barra de ferramentas, os diálogos, os scripts, a CLI, o canal de controle e o MCP executam os mesmos
  comandos.
- `crates/ui-egui`: a interface de desktop/web (egui + wgpu). É substituível; nenhuma camada abaixo dela
  depende de egui.
- `crates/mcp`, `apps/solidcraft`, `apps/solidcraft-cli`, `apps/solidcraft-web`.

`cargo xtask layers` faz a compilação falhar se uma camada inferior depender de uma superior.

## Regras inegociáveis

1. **Independente.** SolidCraft é independente. O comportamento é projetado a partir de documentação
   pública, convenções gerais de CAD e comentários dos usuários. Os testes de referência usam valores analíticos
   e ferramentas de código aberto. Não operamos, criamos scripts, automatizamos ou extraímos dados de produtos de CAD
   comerciais e nunca copiamos o código, os ícones, a arte, os textos de ajuda, as dicas,
   os modelos ou os materiais de outro produto. Os IDs dos comandos são nossos (`solid.fillet`).
2. **Nenhum código GPL/LGPL/AGPL, nem mesmo lê-lo.** Isso exclui OpenCASCADE, FreeCAD
   (incluindo planegcs), SolveSpace, CadQuery/OCP, LibreCAD e QCAD. SolidCraft é
   MIT OR Apache-2.0, e seu solucionador e suas transições são escritos a partir da matemática. (Executar um
   aplicativo de código aberto para obter valores de referência para um teste é permitido; ler seu código-fonte
   não é.) Dependências
   permissivas são permitidas. Indique na PR qual está adicionando e por quê. THIRD-PARTY-LICENSES.txt
   passa a listá-la quando a alteração é integrada: o workflow Generated files o reescreve em `main`.
   A CI noturna/manual também verifica avisos de segurança, proibições e fontes com cargo-deny; consulte
   [o comando local e os limites de cobertura](https://bherbruck.github.io/solidcraft/pt-BR/dev/testing.html#alertas-de-dependências).
3. **Todo recurso é original ou tem licença aberta e possui uma linha em
   [ATTRIBUTION.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/ATTRIBUTION.md).** Os ícones são desenhados em código (`crates/ui-egui/src/icons.rs`).
   `cargo xtask assets` impõe essa regra.
4. **Nunca falhar abruptamente.** As pessoas confiam seu trabalho a um aplicativo de CAD.
   - O código fora dos testes nunca usa `unwrap`, `expect`, `panic!` ou indexação de dados derivados da entrada.
     Não há `unsafe` fora de `apps/solidcraft/src/open_documents.rs` (eventos de abertura de arquivos no macOS)
     e `apps/solidcraft/src/native_crash.rs` (captura de falhas nativas do Windows), e o compilador
     o proíbe.
   - Um caso não suportado retorna um erro ("ainda não suportado: …"); nunca gera um panic.
   - Toda chamada ao truck passa por `solidcraft_kernel::guard`.
   - Toda correção de falha abrupta acompanha um teste de regressão.
5. **Tudo é um comando, com um teste.** Uma nova funcionalidade significa um `CommandSpec` em
   `crates/engine/src/cmd/`, testes do mecanismo que medem o resultado (volume, área,
   contagens de faces/arestas/vértices comparadas com um valor que você possa derivar) e, para alterações na interface, um cenário (abaixo).

[AGENTS.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/AGENTS.md) contém a versão extensa dessas regras. Foi escrito para agentes de IA, mas é
a descrição mais completa do estilo do projeto.

## Testes

| O quê | Onde | Executar |
|---|---|---|
| Testes unitários e do mecanismo | junto do código ou em arquivos `*_tests.rs` | `cargo xtask test` (ou `cargo test --workspace --profile ci`) |
| Cenários de interface | `crates/ui-egui/tests/scenarios/*.json` | parte de `cargo test` |
| Testes de referência | `cargo xtask oracle` | `tests/oracle/` (veja abaixo) |

**Os cenários de interface** controlam o aplicativo real sem interface gráfica: iniciam uma ferramenta, clicam em um plano ou ponto de esboço,
pressionam teclas e então verificam o resultado.

```json
[
  {"start": "sketch.create"},
  {"click": {"plane": "XY"}},
  {"start": "sketch.rectangle.two_point"},
  {"click": {"sketch": [0, 0]}},
  {"click": {"sketch": [40, 30]}},
  {"expect": {"errors": 0}}
]
```

Um novo arquivo `.json` nessa pasta é detectado automaticamente. Para registrar um defeito antes de corrigi-lo,
coloque `{"pending": "why"}` no cenário e remova-o na PR que corrige o defeito.

**Um teste que falha sem sua correção não pode ser integrado como está:** deixaria a CI de `main` vermelha para todos.
Inclua a correção na mesma PR ou marque o teste com `#[ignore = "fails until #123 is fixed"]`
(assim ele pode ser integrado imediatamente) e remova essa linha na PR que corrige o defeito.

**Use o perfil `ci`.** A CI executa os testes com `--profile ci`: otimização leve, tabelas de linhas,
sem LTO e com as verificações de depuração desativadas (o truck tem verificações exclusivas de depuração que rejeitam geometrias válidas). Ele
compila várias vezes mais rápido que `--release`. Se um teste falhar somente em uma compilação de depuração comum, informe isso
na PR. `cargo xtask test` compila os testes com esse perfil e divide uma quantidade limitada de CPUs entre
os binários comuns, os testes comuns do engine e os testes de documentação. Depois desse trabalho,
quatro regressões do engine que medem tempo ou recursos do processo rodam, cada uma, em seu próprio
processo isolado, e a matriz de ações roda sem testes concorrentes. Seus limites existentes não medem
a contenção do runner. Os argumentos após `--` são enviados a cada binário, por exemplo,
`cargo xtask test -- shell`; com qualquer argumento desse tipo, toda a suíte do engine roda em sequência
e isolada para preservar o significado exato dos filtros e opções solicitados. Os testes comuns
continuam executados em paralelo e não devem depender do tempo do relógio.

**As PRs testam o que alteram; main e a execução noturna testam tudo.** Em uma pull request, a CI
mapeia os arquivos alterados para os pacotes do workspace e executa Clippy, a compilação dos testes
e os testes desses pacotes e daqueles que dependem deles (`scripts/ci/affected.py`, com autotestes
no job `checks`). Uma alteração em `Cargo.lock`, no `Cargo.toml` da raiz, em `.cargo/`, `.config/`,
`.github/`, `vendor/`, `tests/`, `scripts/` ou em qualquer arquivo fora de um pacote executa todo o
workspace, assim como cada push para `main`, a execução noturna e a opção Run workflow.

**A CI distribui os testes.** Um job os compila em um arquivo cargo-nextest e quatro jobs executam um
quarto cada um, de modo que cada teste roda em seu próprio processo: não dependa do estado deixado por outro teste.
As pull requests pulam alguns testes pesados (de um a quatro minutos cada), que são executados em cada push para
`main` e à noite; confira a execução de `main` após a integração. Eles estão listados em
`.config/nextest.toml` e na [página de testes](https://bherbruck.github.io/solidcraft/pt-BR/dev/testing.html):

- `solidcraft-engine`: `tests::hostile_params_never_panic`, tudo em `tests::robustness` e
  `tests::robustness_goldens` (pull requests que alteram `crates/kernel` executam sua parte `sample_*`),
  `pattern_suppress_tests::enclosure_arc_extinction_keeps_a_real_sharp_inner_lip_after_resize_and_reopen`,
  `pattern_suppress_tests::enclosure_datums_follow_wall_height_and_board_depth_with_material_proofs`,
  `cmd::plastic::tests::enclosure_snap_fit_follows_resized_front_wall`
- `solidcraft-kernel`: `tests::boolean_identity_on_tangent_primitives`
- cenários de `solidcraft-ui-egui`: `acceptance_sensor_enclosure`

**A matriz de ações** testa cada ação comum em cada face, aresta, vértice e corpo de um
conjunto de projetos contra uma referência progressiva versionada ([docs/action-matrix.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/action-matrix.md)).
Uma pull request que faça uma ação passar deve registrá-la com `cargo xtask action-matrix --bless`;
uma que faça falhar uma ação que passava reprova a amostra verificada em cada execução da CI.

**Os testes de referência** são 76 peças em [`tests/oracle/`](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/project/tests/oracle/README.md), cada uma com um `recipe.json`
em nosso próprio formato e um `expected.json` ao lado. O SolidCraft reconstrói a receita e compara
contagem de corpos, volume, área, centro de massa e caixa delimitadora (e, quando há um `reference.stl`,
a própria superfície). Os valores esperados são calculados em forma fechada
(`tools/oracle-analytic/`) ou construindo a mesma receita no FreeCAD, executado como programa
separado (`tools/freecad-oracle/`); cada arquivo informa qual método foi usado. [docs/oracle.md](https://bherbruck.github.io/solidcraft/pt-BR/dev/oracle.html) apresenta
os resultados atuais, e a CI executa `cargo xtask oracle` em cada pull request. Um caso que falha
por um defeito conhecido indica sua issue em `expected.json` (`"known_failure": "#123"`); nunca
altere um valor esperado para coincidir com a saída do SolidCraft.

## Alterar o formato dos arquivos de projeto

Um arquivo de projeto informa seu formato (`"format": "solidcraft/N"`). Se sua alteração salva
algo novo em um projeto (um campo, um tipo de operação, um novo tipo de nome) ou muda o significado
dos projetos antigos, ela precisa de uma alteração de formato. **Adicione um arquivo e não edite nada compartilhado:**

1. Crie `crates/doc/src/format/pending/<what-it-is>.rs` (letras minúsculas e hífens). Comece com uma
   nota `//!`: o que é salvo, o que os projetos antigos recebem na ausência disso e que a última
   versão publicada pede uma atualização em vez de tentar processar a novidade.
2. Se os projetos antigos precisarem ser reescritos ao abrir, coloque `pub fn step(v: &mut Value) ->
   crate::Result<()>` no mesmo arquivo. As etapas pendentes são executadas na ordem dos nomes dos
   arquivos, depois das etapas dos formatos anteriores.
3. Não altere `FORMAT`, `JSON_STEPS`, o histórico em `crates/doc/src/format.rs`, o arquivo de outra
   alteração nem qualquer pasta `v<N>/`: eles se tornam definitivos quando publicados.

Isso basta. A compilação conta os arquivos: enquanto houver algum pendente, grava o próximo número
de formato, que nenhuma versão publicada lê; assim, uma versão publicada recusa um projeto com sua
alteração em vez de perdê-la. O arquivo de projeto também lista suas alterações pendentes (`format_pending`),
para que uma compilação de desenvolvimento posterior execute apenas as etapas que ainda faltam nele.
Duas pull requests que adicionam uma alteração cada são integradas sem conflito.
`crates/doc/src/format/pending/README.md` contém um exemplo.

**Ao preparar uma versão**, o mantenedor executa `scripts/fold-format-pending.sh`, que move os arquivos
pendentes para `crates/doc/src/format/v<N>/` e torna esse número definitivo (veja
[docs/releasing.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/releasing.md)).
Nunca adicione um arquivo a uma pasta `v<N>/` depois disso.

## Listas que recebem itens de muitas alterações

Alguns arquivos recebem um item em muitas pull requests ao mesmo tempo. Eles são organizados para
que duas alterações que adicionam um item cada só se encontrem quando seus itens forem vizinhos,
o que poupa conflitos e novas execuções da CI:

- **Catálogos de mensagens** (`assets/locales/*.json`): uma mensagem por linha, ordenadas por ID.
  Adicione sua mensagem em qualquer posição e execute `cargo xtask lists --fix`; `cargo xtask lists`
  (em `cargo xtask ci` e no workflow Lists) falha se um catálogo não estiver ordenado ou tiver um ID repetido.
- **Linhas `mod`**: uma por linha, no grupo ordenado a que pertencem (`cargo fmt` mantém cada grupo
  ordenado). Adicione um novo módulo ao grupo existente, sem criar um grupo no fim do arquivo.
- **Idiomas** (`crates/engine/src/i18n.rs`, reexportados pela interface em `crates/ui-egui/src/i18n.rs`):
  um por linha, inglês primeiro e depois por ID (um teste verifica isso).
- **Arquivos gerados**: `THIRD-PARTY-LICENSES.txt` e `docs/features.md` são reescritos em `main`
  após cada integração pelo workflow Generated files (`cargo xtask licences`, `cargo xtask features`).
  Não os gere novamente nem os edite em uma pull request.
- **Formato dos projetos**: um arquivo por alteração, sem editar um número compartilhado (veja acima).

## Enviando uma alteração

1. O trabalho é acompanhado em [issues do GitHub](https://github.com/bherbruck/solidcraft/issues), identificadas
   por área (`area:kernel`, `area:sketch`, `area:doc`, `area:ui`, `area:infra`). Para qualquer trabalho
   maior que uma correção de defeito que ainda não tenha uma issue, abra uma primeiro. **Reserve a
   issue com uma PR em rascunho** antes de começar, para que duas pessoas não construam a mesma coisa:
   - Verifique se ninguém a reservou: `gh pr list -R bherbruck/solidcraft --state open --search "#N"`.
   - Crie uma branch a partir da `main` mais recente, faça o commit `git commit --allow-empty -m "claim: #N"`,
     envie a branch e abra uma PR em **rascunho** com o título `<summary> (#N)`, por exemplo,
     `Fillet: variable radius along a chain (#123)`.
   - **A PR aberta com o menor número para uma issue tem prioridade.** Se a sua perdeu, feche-a
     (ou ofereça ajuda na outra).
   - Envie o trabalho em andamento pelo menos uma vez por dia. Um rascunho sem pushes por 72 horas
     pode ser assumido por outra pessoa: comente nele primeiro e depois abra sua própria reserva.
   - Marque a PR como pronta para revisão quando terminar.
2. Crie uma branch a partir de `main` e mantenha a PR em um único assunto.
3. Execute `cargo xtask ci` antes de enviar. Ele executa fmt, as verificações de recursos, licenças e camadas,
   clippy (`-D warnings`) e então os testes, junto com a verificação de compilação wasm. A PR precisa passar.
4. Referencie a issue (`Fixes #12`). Na PR, diga o que testou e como. Para alterações de geometria, inclua os números medidos (e
   os esperados).

Comente `/claim` em uma issue para reservá-la: você recebe a atribuição, ela recebe a etiqueta
`in progress`, e o bot responde com o prazo. Abra uma PR em rascunho que referencie a issue em até
72 horas; caso contrário, a reserva expira e a issue fica disponível novamente. Uma reserva com PR
é liberada após 14 dias sem pushes. `/unclaim` a libera antes disso (somente você ou um mantenedor
pode fazer isso). Issues assumidas por um mantenedor nunca são liberadas automaticamente.

Ajude os revisores a visualizar o que as palavras descrevem. Para comportamentos observáveis de CAD, interface ou modelo, inclua
capturas de tela descritivas reais ou uma gravação curta na issue ou PR. Essa evidência
é obrigatória para alterações ou defeitos demonstráveis no aplicativo, incluindo trabalhos no núcleo,
na geometria, em montagens e no backend. Agentes de programação devem executar uma pequena reprodução e
capturar as evidências por conta própria; não peça a uma pessoa evidências que o agente possa produzir.
Mostre a falha ou o resultado real, de preferência antes e depois, com passos breves e
o comportamento esperado/real. Uma seta ou anotação pode ajudar a identificar o detalhe relevante.

Fluxogramas e diagramas Mermaid não substituem evidências de comportamento visível. Diagramas são
úteis para raciocínio arquitetural quando capturas de tela reais não conseguem ilustrar o ponto.
Alterações sem comportamento observável útil não precisam de capturas de tela ou vídeos. Use um
pequeno exemplo sem projetos privados ou detalhes confidenciais; anexe a mídia ou forneça um link para uma
gravação acessível em vez de incluir no repositório os recursos gerados para a revisão.

Continue verificando os commits do upstream e o estado de integração da PR relevante ao longo do trabalho e
antes da entrega. Incorpore as integrações do upstream, resolva conflitos sem perder alterações de outras pessoas
e valide o resultado integrado. Antes de enviar, execute `git pull --rebase origin main`, execute novamente
a CI se ela trouxer alterações e envie sem forçar. Informe o estado do upstream/PR verificado,
os conflitos resolvidos e os resultados da validação. Enquanto uma tarefa estiver ativa, acompanhe uma PR aberta
pelas mudanças de estado; na entrega, identifique claramente tudo que ainda aguarda integração.

Grande parte do SolidCraft até agora foi escrita por agentes de programação de IA (Claude) trabalhando em paralelo, cada um
responsável por uma área, com uma pessoa definindo a direção e relatando defeitos. Suas notas de trabalho estão em
[AGENTS.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/AGENTS.md). Colaboradores humanos são igualmente bem-vindos. Use as ferramentas que preferir,
desde que a alteração cumpra as regras acima.

## Onde a ajuda é mais necessária

Veja as etiquetas [`help wanted`](https://github.com/bherbruck/solidcraft/labels/help%20wanted) e
[`good first issue`](https://github.com/bherbruck/solidcraft/labels/good%20first%20issue).
Em ordem de quanto ajudariam os usuários:

1. **Robustez dos arredondamentos e das transições.** Os arredondamentos são código nosso (o truck não tem operação de arredondamento).
   Arredondamentos de raio constante, raio variável e por corda funcionam, assim como cantos em que se encontram arredondamentos de
   raios diferentes, incluindo três arredondamentos encontrando-se em um vértice. Os próximos são cantos com recuo, arredondamentos ao longo de arestas curvas entre
   faces curvas e arredondamentos que consomem faces vizinhas.
2. **Robustez das operações booleanas.** Um fuzzer de projetos aleatórios atualmente consegue concluir cerca da metade de seus projetos
   sem erros do núcleo. Cada semente que falha é um defeito reproduzível.
3. **Y para cima / Z para cima.** SolidCraft só admite Z para cima, então a maioria dos arquivos STEP (do SolidWorks, Onshape ou
   da configuração padrão do Fusion) abre deitada de costas. O Fusion resolve isso com uma preferência de "orientação padrão
   de modelagem".
4. **Aparências importadas.** As cores dos arquivos STEP se perdem quando uma operação altera o
   corpo importado. Elas devem se tornar aparências do projeto durante a importação.
5. **Desenhos técnicos, CAM e ferramentas de malha.** Estão fora do escopo por enquanto (veja
   [ROADMAP.md](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/ROADMAP.md)), mas quem quiser assumir uma dessas áreas é bem-vindo.

## Licença

Ao contribuir, você concorda que sua contribuição é licenciada sob MIT OR Apache-2.0, assim como o restante
do SolidCraft. SolidCraft não tem afiliação com a Autodesk. Fusion é uma marca registrada da Autodesk, Inc.
