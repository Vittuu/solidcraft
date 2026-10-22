# Roteiro do SolidCraft

Estado em **2026-10-14**. SolidCraft oferece modelagem paramétrica (esboço →
operações → linha do tempo paramétrica) como aplicativo aberto, inteiramente em Rust e controlável por agentes.

## Visão geral

| Pergunta | Resposta |
|---|---|
| Onde estamos? | **M0 e M13 concluídos; M1, M2, M5–M7, M9, M10, M14 em andamento.** Esboço com solucionador e o conjunto completo de criação/modificação de ESBOÇO; extrusão, revolução (dois lados, simétrica, até objeto), arredondamento/chanfro (raio variável, comprimento de corda, um raio por aresta, remendos de canto), furos, roscas (cosméticas e modeladas), padrões, espelhamento, casca (para dentro/para fora/ambos), ângulo de saída, loft, varredura, espiral, dividir corpo e dividir face, excluir face, planos, eixos e pontos de construção; nomeação persistente de faces e arestas; parâmetros e configurações; componentes, juntas, estudos de movimento, vistas explodidas; chapas metálicas e um subconjunto de caixas plásticas; STEP/IGES/3MF/STL/OBJ/DXF; salvamento automático e recuperação de falhas; aplicativo para desktop e navegador (wasm), CLI, servidor MCP. |
| Funcionalidades | Todos os comandos registrados por área — [docs/features.md](features.md) (`cargo xtask features`); a abrangência do registro não estabelece profundidade geométrica ou dos fluxos de trabalho |
| Testes de referência | **82 / 82 peças correspondem** — [docs/oracle.md](oracle.md): casos públicos em `tests/oracle/` com valores esperados analíticos (61) ou calculados pelo FreeCAD (21), executados pela CI em cada PR. |
| Testes | Solucionador, perfis, operações booleanas/transições/medidas do núcleo, expressões, linha do tempo, nomeação persistente, formatos de arquivo e migrações, montagens, chapas metálicas, mecanismo de ponta a ponta, fuzzing de entradas hostis em todos os comandos e cenários de interface sem janela. `cargo xtask ci` informa as contagens atuais de testes aprovados. |
| Verificações | `cargo xtask ci`: fmt, clippy -D warnings, testes, atribuição de recursos, camadas, compilação wasm32 — verde |
| Tempo até um alfa útil (M0–M6) | Restam ≈ **82 horas de agentes** (soma das estimativas de M1–M6 abaixo) |

## Escopo (decisão do proprietário, 2026-10-07)

O lote de issues adiciona preservação de cavidades cegas em cortes passantes, deslocamentos
projetados associativos e posicionamentos de plástico em faces, XZ (normal +Y) com uma migração v6 que preserva os
projetos mais antigos, posicionamento interativo de ângulos, importações de esboços por seleção de arquivo, controles de Início/recentes,
transbordamento da barra de ferramentas e o trabalho em andamento restante de sistemas de coordenadas de componentes e montagens. Os cenários de
aceitação de caixas agora incluem redimensionamentos da placa; a reabertura STEP é automatizada. Veja [Reconciliação do trabalho em andamento](wip-reconciliation.md).

No escopo, em ordem de prioridade:
1. SÓLIDO e ESBOÇO (agora).
2. Parâmetros / restrições funcionais (concluído: veja `parameters.*`, `expr.evaluate`; reconhecem unidades,
   ciclos são informados com seu caminho, renomeações acompanham referências, exclusões listam seus usos):
   parâmetros do usuário com unidades e expressões (+-*/, ^,
   parênteses, sin/cos/sqrt/min/max/floor/ceil/round, conversão de unidades), parâmetros de modelo
   d1, d2… para cada dimensão e entrada de operação, referências entre parâmetros com um
   grafo de dependências e detecção de ciclos, reavaliação ao mudar, comentários, favoritos,
   importação e exportação CSV/JSON.
3. Componentes e montagens com juntas (M9).
4. CHAPA METÁLICA: flange, flange de contorno, dobra, desdobrar/dobrar novamente, planificação com exportação DXF, regras de chapa
   metálica (espessura, fator K, raio de dobra) (M10).
5. Um pequeno subconjunto de PLÁSTICO para caixas, após as chapas metálicas: Ressalto, Nervura, Rede de nervuras, Rebordo/Ranhura, Encaixe por pressão.

Adiados até novo aviso: MALHA, FORMA (T-splines), SUPERFÍCIE além do necessário para sólidos (escopo proposto em #772), PCB / 3D
PCB / Encapsulamento, Renderização, Animação, Simulação, Fabricação/CAM, Desenhos técnicos.

## Marcos

| # | Marco | Estado | Horas restantes |
|---|---|---|---|
| M0 | Implementação de ponta a ponta | concluído | — |
| M1 | Profundidade de esboço: cobertura de comandos implementada — projeção/interseção/inclusão como geometria vinculada (projeção automática em faces, avisos de referências perdidas, desvincular), conjunto completo de criação/modificação de ESBOÇO (ranhuras, polígonos, arcos/círculos tangentes, elipse, splines por pontos de ajuste/controle, cônica, texto, recortar/estender/quebrar, arredondamento/chanfro, deslocamento, espelhamento, padrões, mover/escalar, curva de transição, linha de centro), restrições incluindo curvatura e polígono com rejeição de restrições excessivas, cotas de referência/comprimento de arco/diâmetro linear, Restrições automáticas, inserção DXF/SVG, curvas de esboço 3D (Projetar na superfície, Curva de interseção, Curva isoparamétrica, Perfil de revolução), perfis exatos de forma livre (arestas B-spline/NURBS, splines de qualquer grau, curvas cortadas por cruzamentos), varreduras e tubos ao longo de curvas de esboço 3D, imagens de referência e decalques, pentes de curvatura. Estender de forma livre exato e deslocamentos associativos certificados agora preservam geometria e IDs durante alterações, exclusão e reabertura. Fios 3D no sistema de coordenadas da ocorrência agora armazenam espaços de coordenadas explícitos e mantêm pontos e ajustes analíticos durante movimentos, atualização, desfazer e reabertura. Resolução construtiva de esboços (Owen 1991, Fudos & Hoffmann 1997) antes do solucionador numérico: pontos e agrupamentos rígidos são posicionados no ramo que o esboço tinha, para que alterações de dimensões e arrastes mantenham sua solução; alterações que inverteriam um esboço são rejeitadas; arrastes resolvem novamente apenas o esboço a cada quadro. Reparos restantes nos fluxos de trabalho: junções de deslocamentos mistos, emendas legadas de splines naturais e vínculos danificados de fios legados sem rastreamento da origem do sistema de coordenadas | em andamento | 4 |
| M2 | Modelagem robusta: operações booleanas coplanares no núcleo, furos (posicionados em faces e pontos de esboço), roscas (cosméticas e depois modeladas), padrões, espelhamento, casca, ângulo de saída, divisão, geometria de construção | em andamento | 16 |
| M3 | Arredondamentos e chanfros gerais (arestas curvas, cadeias, transições em vértices); superfícies em sela de esfera rolante agora usam seções contínuas de emendas e ajuste por comprimento de arco com contornos de contato compartilhados e verificações de precisão limitadas | em andamento | 30 |
| M4 | Arquivos: importação STEP (concluída — [docs/step-import.md](step-import.md)), exportação 3MF e importação 3MF/STL como corpos de malha (concluídas), 3MF com etiquetas de faces B-rep ([docs/formats/3mf-face-tags.md](formats/3mf-face-tags.md), concluído), reconstrução paramétrica de malha de seção constante, esboços DXF, importação OBJ | em andamento | 6 |
| M5 | Nomeação persistente de faces e arestas (docs/book/src/dev/naming.md): faces nomeadas por seu histórico exato informado pelo núcleo (laterais de curvas de esboço, tampas, seções de revolução, laterais de primitivas, transições, paredes internas de casca, cópias de padrão/espelhamento; peças divididas distinguidas pelas faces ao redor, volumes de um corpo cortado nomeados por suas faces), arestas por suas faces; arredondamentos/chanfros, esboços em faces, faces de casca/ângulo de saída/deslocamento/substituição, posicionamentos de furos, origens de juntas e aparências de faces resolvem primeiro pelo nome e avisam quando a entidade se dividiu ou desapareceu: concluído. Arestas de chapas metálicas pela linha do esboço-base e pelo lado da flange, faces internas de casca, funções de varredura e loft: concluído. Histórico de faces informado pelo núcleo: concluído (faces mescladas respondem a todos os nomes; operações ainda sem histórico usam nomes geométricos como alternativa, marcados como não verificados; o formato 14 mantém os nomes geométricos dos projetos antigos). Falta: histórico em operações de chapa metálica, plástico, superfície e relevo; certificados FaceSource ao longo do histórico | em andamento | 20 |
| M6 | Profundidade de interface: feedback de seleção e origem (concluído), edição de linha do tempo (concluída: passar o ponteiro e selecionar destaca faces modificadas, incluindo arredondamentos e chanfros; editar operação, arrastar marcador do histórico, reordenar um ou vários itens com verificação de dependências, suprimir, renomear, excluir com dependentes, resolver referências novamente; redefinir seleções atualiza referências persistentes de arestas/faces e mantém a identidade da operação e os nomes de dimensões; histórico opcional compacto de operações nomeadas à esquerda com barra de retorno, seleção múltipla e pastas persistentes acima da árvore de peças), diálogos ancorados à direita (concluído), prévias ao vivo para cada diálogo de operação (concluído), comandos orientados ao teclado com caixas de valores na área de desenho e cotas inline de esboço (concluído), manipuladores de arraste com união/corte automático (concluído: extrusão incluindo dois lados, arredondamento, casca, profundidade de furo, mover, espaçamento de padrão, manipulador de revolução; alça de inclinação pendente), tema escuro (concluído), menu do botão direito com Repetir (concluído), ferramenta de medição (concluído), análise de seção (concluído), filtro de seleção (concluído), resolução por arraste de pontos de esboço (concluído); controles de aparência agora preservam direcionamento de corpo/face/componente, aparências transparentes nomeadas, reutilização/busca e remoção de substituições; preferências | em andamento | 6 |
| M7 | Varredura, loft (incluindo até um ponto), nervura e rede de nervuras (até o próximo / profundidade), relevo (faces planas e relevo/gravação radial exato de perfis ortogonais limitados em cilindros externos completos/mangas), extrusão fina ao longo de cadeias abertas ou contornos fechados (incluindo paredes spline, espessuras bilaterais e cantos em meia-esquadria), rosca (cosmética e modelada M3–M20 externa e interna: o perfil ISO helicoidal como corte facetado), espiral (paredes helicoidais suaves B-spline), tubo, padrão no caminho, padrões de corpos e componentes, substituir face (paralela), alinhar, remover, copiar/colar operações: concluído. Alvos cilíndricos únicos elegíveis são inferidos para novas operações de Relevo. Falta: relevo geral em faces curvas e roscas modeladas exatas (não facetadas) | em andamento | 12 |
| M8 | Espaço de trabalho Superfície (#772). Concluído: Remendo (regiões de esboço; laços de arestas preenchidos G0), Costurar, Espessar (planos e cilindros), Estender (arestas retas de superfícies planas), Recortar (por um plano ou uma face), Excluir face (com reparo), Preenchimento de limites, análises de listras zebra/ângulo de saída/mapa de curvatura. Proposto, em ordem: Extrusão/Revolução/Varredura/Loft de superfície a partir de curvas abertas e fechadas (#757), Deslocar superfície (#758), Descosturar e Inverter normal (#766), Excluir face sem reparo (#768); Estender faces e arestas curvas (#760), recorte mútuo e recorte por curvas projetadas (#767), Desfazer recorte (#761), Espessar todos os tipos de superfície (#765); Superfície regrada (#762), Superfície por limites e redes de curvas com G0/G1/G2 (#763), Remendo com G1/G2 e laços gerais (#759), Superfície de transição (#764), trilhos de varredura, torção e escala (#771, trilhos de loft #201), arredondamento entre corpos de superfície (#769); análise de continuidade, pentes de curvatura de superfície e isocurvas (#770) | adiado | — |
| M9 | Componentes e montagens: Novo componente / Criar componentes a partir de corpos, componente ativo, origem/corpos/esboços/entradas de linha do tempo por componente, ocorrências com transformações, Copiar/Colar versus Colar como novo, árvore do modelo aninhada com visibilidade e isolamento; Fixar, Mover/copiar ocorrências, juntas (rígida, revoluta, deslizante, cilíndrica, pino-ranhura, plana, esférica) por origens de juntas com limites e Acionar juntas, ciclos cinemáticos fechados (quatro barras, biela-manivela: as outras juntas acompanham a acionada no mesmo ramo, e o movimento para onde o ciclo deixa de fechar), Junta na posição atual, Grupo rígido, Conjuntos de contato (acionamento e estudos de movimento param onde corpos de um conjunto atravessariam uns aos outros), Estudo de movimento (valores de juntas registrados ao longo de uma linha do tempo, reprodução, exportação de posições de ocorrências como JSON/CSV), Vistas explodidas (automáticas ou manuais, exibidas sem alterar posicionamentos); interferência, propriedades físicas por componente; STEP com estrutura de montagem; exportação URDF para simuladores de robôs (elos com malhas, massa e inércia; cadeias de juntas cujos valores correspondem a Acionar juntas; docs/urdf.md). As propriedades físicas agora medem posicionamentos de componentes/ocorrências considerando materiais, descendentes e instâncias repetidas, incluindo movimentos pendentes e deslocamentos de reatribuição de corpos; a geometria de contato e os centros de explosão automática respeitam esses deslocamentos. Base concluída: árvore de componentes, ocorrências com transformações (instâncias, Colar como novo), posicionamento global, componente ativo como sistema de coordenadas de criação, fixação, Registrar/Restaurar posição, mover corpos e esboços. Interface: diálogos do painel MONTAR para Novo componente, Junta (encaixes em faces, arestas, círculos e vértices; prévia animada de movimento; deslocamentos, inverter, limites), Junta na posição atual, Origem da junta, Grupo rígido, Acionar juntas (controle deslizante ao vivo), Vínculo de movimento e Interferência (sobreposições exibidas em vermelho); juntas na linha do tempo e na árvore do modelo | em andamento | 32 |
| M10 | Chapa metálica: flange (base, aresta, contorno), bainha, desdobrar/dobrar novamente, converter, planificação + DXF, regras: concluído. Subconjunto de caixas de PLÁSTICO: Ressalto, Nervura, Rede de nervuras, Rebordo/Ranhura, Encaixe por pressão, Suporte, Regras de plástico (biblioteca, editar, atribuir; ângulo de saída e folga da regra como padrões das operações): concluído, testado contra geometria calculada manualmente. Interface: abas CHAPA METÁLICA e PLÁSTICO no layout do Fusion com diálogos (prévia ao vivo, Editar operação, pré-seleção) para Flange, Bainha, Desdobrar/Dobrar novamente, Converter, Regras de chapa metálica, Criar planificação (vista plana com linhas de dobra, tabela de dobras CSV/JSON e exportação DXF com recortes exatos de círculos/arcos/linhas), Ressalto, Rebordo/Ranhura, Encaixe por pressão, Suporte, Gerenciar e Atribuir regras de plástico; Dobrar (dobra ao longo de uma linha de esboço); flanges que se encontram em um canto fechadas em qualquer ângulo com a folga da regra; flanges de aresta com ângulo e altura próprios por aresta em uma única operação; alívios de canto (entalhe, redondo, retangular, rasgo) escolhidos pela regra; Converter para chapa metálica a partir de corpo sólido ou com casca por suas arestas de dobra (faces planas, dobras agudas) | em andamento | 36 |
| M11 | Desenhos técnicos 2D | adiado | — |
| M12 | Versão: pacotes multiplataforma, verificação e publicação deliberada implementados; demonstração original com a interface atual renovada. Assinatura confiável de distribuição e trabalho mais amplo de desempenho ainda pendentes | em andamento | 16 |
| M14 | Configurações: tabela de linhas (expressões de parâmetros e supressões de operações por linha) alternada com config.activate (reversível, tudo ou nada), mantida durante renomeações de parâmetros e exclusões de operações (config.configure, config.table, config.column/row/activate). Materiais e aparências dos corpos por linha (aparências da biblioteca, cor/opacidade personalizada e remoção de substituições), ativação reversível e salvar/reabrir: concluído. Regras de configuração tipadas e ordenadas, validação atômica e edição estruturada de regras: concluído. Colunas de aparência de componente e estilo semântico de faces certificado, prévia/aplicação atômica e remapeamento de cópias independentes: concluído. A origem exata de laterais de esboço construídas e de faces importadas imutáveis agora preserva nomes legados e rejeita redirecionamentos. Falta: tampas deslocadas/bilaterais ambíguas | em andamento | 4 |
| M13 | Nunca perder trabalho: salvamentos atômicos (arquivo temporário, flush, renomeação; uma falha a qualquer momento deixa o arquivo antigo ou o novo) com a versão substituída mantida como `<file>.bak`; salvamento automático de alterações não salvas na pasta de recuperação a cada 5 minutos (preferência) e após operações grandes, somente quando algo mudou; "Recuperar projeto não salvo?" na próxima inicialização (`doc.recovery_list`, `doc.recover`, `doc.recovery_discard`); teste de interrupção durante o salvamento (processo encerrado enquanto salva um projeto de 0,5 MB, 25 vezes). Formato de arquivo: `"format": "solidcraft/12"`, formatos anteriores atualizados passo a passo (`doc::format`), formatos mais recentes rejeitados com uma mensagem, números fora da faixa e posicionamentos quebrados rejeitados; conjunto de projetos salvos pelas compilações M0, da época de componentes, da época de juntas e atuais (apps/solidcraft-cli/tests/corpus), que devem abrir e avaliar como foram salvos; variantes danificadas e hostis (truncamento, inversões de bits, valores hostis, chaves removidas, operações repetidas, aninhamento profundo) nunca geram panic | concluído | 6 |

As preferências de navegação de M6 incluem **Touchpad**: Shift + movimento do ponteiro desloca a vista, Alt/Option +
movimento do ponteiro orbita sem pressionar um botão do mouse, e a rolagem vertical aplica zoom. Os outros perfis
mantêm sua preferência salva de rolagem com dois dedos; o movimento da câmera está disponível por
`view.navigate` e pela automação `ui.move` com modificadores.

Precursor de M11: Arquivo → Exportar vista atual como DXF exporta os corpos visíveis em suas posições
pela câmera atual como um desenho 2D em milímetros, com arestas ocultas omitidas ou tracejadas.
A oclusão vetorial divide arestas parcialmente ocultas e inclui silhuetas curvas. As curvas são
aproximadas; folhas de desenho, anotações e curvas projetadas exatas continuam adiadas.
Veja [DXF da vista](view-dxf.md).

## Dados físicos de materiais

- A validação de fichas de materiais físicos (#923/#922) fornece dados mecânicos e
  térmicos tipados em SI, fontes/condições por propriedade e verificações de estabilidade
  isotrópica/ortotrópica pelo comando somente de leitura `material.properties.validate`.
  Ela não atribui nem persiste propriedades, fornece um solver ou habilita renderização; veja
  [o contrato e as próximas etapas](physical-material-cards.md).

- A elasticidade direcional (#994) consulta a ficha validada nos eixos locais do material
  pelo comando somente de leitura `material.properties.directional`, com revisão de fontes/condições.
  Ela não atribui sistemas de eixos a corpos, prevê comportamento estrutural nem habilita um solver.

## Fundamentos de bibliotecas (#785, #786, #787)

- Bibliotecas escalares locais (#846) inspecionam/exportam catálogos portáteis limitados e
  copiam revisões exatas selecionadas de aparência/material para o projeto em uma etapa de
  desfazer. Aparência oferece abertura/busca/exportação local; materiais físicos usam os
  comandos existentes. Esta etapa independe da pilha de renderização e não altera o formato.
  Veja [contrato e limites](book/src/dev/local-libraries.md).

- As fases propostas abrangem pacotes offline incluídos no aplicativo/do projeto/do usuário, materiais físicos e
  aparências, mapeamento editável de madeira e famílias de componentes/montagens parametrizadas.
  Identidade/revisão estável, definições copiadas do projeto, rastreamento de origem e limites de recursos
  se aplicam desde a primeira implementação. Os comandos existentes de material, aparência e peças padronizadas
  continuam sendo os caminhos do aplicativo. Veja [a proposta](book/src/dev/libraries.md)
  para revisão de fonte/licença e critérios de aceitação. Trata-se apenas de planejamento: carregamento de pacotes,
  renderização de texturas e famílias ampliadas de componentes não são implementados por esta entrada.

## Base de renderização

- PBR opaco opcional (#1046) conecta parâmetros persistidos de superfície à vista integrada
  de produto e ao PNG CPU com um estúdio fixo de luz direta. Geometria e massa não mudam.
  HDRI, transmissão e texturas são critérios separados. Veja [consumidor e limites](render-opaque-pbr.md).

- A integração combinada é acompanhada em #842, com bibliotecas escalares independentes em
  #863 e transporte PBR/fichas mais rico em #1086. Veja [fases e validação](render-integration.md).
  Consumidores de Render/biblioteca usam o seletor assíncrono integrado (#1127). UI de fichas
  (#1094), mapeamento de texturas e iluminação HDRI continuam como etapas explícitas futuras.

- R4a acrescenta trabalhos CPU limitados de imagem de produto, cancelamento e exportação PNG
  atômica por comandos, painel Render, CLI e MCP persistente. Capturas imutáveis e resultados
  ficam fora da persistência/desfazer; workers nativos e tarefas cooperativas do navegador
  compartilham o protocolo. Veja [trabalhos, limites e exportação](render-jobs.md).

- R3 acrescenta cena de produto sem sobreposições e espaço Render integrado, com capacidades
  do host GPU, rascunhos por documento e controles nomeados de câmera/fundo. Projeto mantém
  câmera e política de desenho. Transparência raster é aproximada; PBR e ambientes chegam
  em etapas posteriores. Veja [espaço e contratos de cena](render-workspace.md).

- R2 acrescenta configurações nomeadas limitadas no formato 21 e comandos de leitura/gravação
  `render.settings.get/set`. Alterações comprovadamente só de renderização preservam geometria
  e dados globais preparados em desfazer/refazer e restauração de checkpoint. Esta etapa não
  acrescenta espaço Render, PBR, ambientes ou trabalhos; veja [configurações e atualização](render-settings.md).
- Admissão limitada PNG/JPEG (#900) valida origens em memória e retorna pixels RGBA8 completos
  com papéis explícitos de cor/dados. É apenas utilitário de entrada: normalização ICC/16 bits,
  mapeamento local de madeira, persistência de recursos e consumidor de renderização são
  trabalhos separados. Veja [limites e arquivos licenciados](render-texture-input.md).

- O contrato numérico inicial de superfície opaca para #1004 oferece admissão limitada de
  cor linear/metallicidade/rugosidade/IOR e uma BRDF original testada de iluminação direta.
  O módulo numérico sozinho não anuncia capacidades; o consumidor instalado acima fornece
  renderização. Veja
  [o modelo, as evidências e os critérios restantes para os consumidores](render-surface.md).

- Um utilitário de entrada Radiance RGBE cooperativo e limitado está disponível em L1 render;
  ele ainda não habilita renderização HDRI/PBR nem um comando de importação de documento. Veja
  [o contrato de entrada](render-hdr-input.md) e #813 para o trabalho de integração.
- R1 adiciona metadados de backend/perfil independentes do renderizador, resolução rigorosa de solicitações e
  verificações limitadas de admissão de imagem/memória temporária, expostos pelo comando somente de leitura `render.backends`.
  Ele descreve o caminho atual de captura rasterizada pela CPU sem avaliar o modelo.
  O caminho legado de snapshot de Projeto permanece inalterado. Etapas posteriores acima fornecem
  espaço/trabalhos e PBR opaco; HDRI e mecanismos opcionais continuam futuros. Veja [o contrato](render-backends.md)
  e [a proposta em fases (#620)](https://github.com/bherbruck/solidcraft/issues/620).

## Dimensionamento e comunicação de erros

- O `execute` do MCP aceita `verbose: false`, compartilhando o auxiliar de resposta compacta usado por
  `batch`: alterações de corpos e IDs de esboços criados permanecem, enquanto listas repetidas são omitidas.
  As duas ferramentas mantêm respostas completas por padrão.

- Criar, em Esboço, agrupa as variantes de desenho em Linha, Retângulo, Círculo, Arco, Ranhura,
  Polígono e Spline. Cada peça aberta lembra sua última escolha interativa por família
  entre sessões de esboço e alternâncias de abas de documento; fechar a peça redefine essas escolhas da interface.
- As ferramentas de polígono inscrito, circunscrito e por aresta mostram uma prévia de seu contorno real antes do
  posicionamento, incluindo a orientação e a quantidade de lados selecionada.
- Construção (X) preserva a ferramenta ativa de desenho do esboço, as âncoras escolhidas e os valores
  de dimensões inline, para que o tipo de linha possa mudar durante o desenho de uma forma.
- Prévias de construção de esboços planos e feedback ao passar o ponteiro mantêm o mesmo padrão tracejado da
  geometria confirmada; linhas de centro mantêm seu padrão de traço e ponto.

- Ponto tem um botão padrão na barra de ferramentas CRIAR de Esboço junto às ferramentas de desenho existentes.
  Layouts personalizados salvos são preservados; Redefinir barra de ferramentas restaura o botão padrão de Ponto.
- Planos de construção não podem receber o nome de um plano de origem (XY, XZ, YZ, Top, Front, Right). Argumentos
  de plano rejeitam esse nome quando um plano de construção de um projeto antigo já o utiliza. Use
  o outro alias, o ID numérico da operação de construção ou um plano explícito; as referências
  já armazenadas mantêm seu significado salvo. Análise de seção usa a mesma verificação.
- O texto de esboço pode ser editado pelo menu de contexto do interior ou do contorno de uma letra, ou pelo comando Editar texto.
  A seleção do interior exige que todos os contornos do perfil pertençam à mesma fonte de texto;
  o editor destaca a fonte inteira sem substituir seleções de perfis.
  Conteúdo, X/Y da linha de base, altura e rotação atualizam a fonte original do texto em uma etapa de desfazer;
  cancelar ou rejeitar um valor inválido preserva o esboço. Os campos de posição e altura
  usam as unidades do documento capturadas ao iniciar a edição; contornos gerados permanecem associados
  à sua fonte de texto, e entradas não alteradas preservam seus valores internos salvos.
  Arrastar um contorno de texto move a fonte inteira em uma etapa de desfazer; Escape restaura o
  gesto. Mover/copiar em um contorno de texto selecionado fornece distâncias X/Y com reconhecimento de unidades e rotação
  em torno de sua linha de base; as cópias mantêm uma fonte de texto independente e editável.
- Repetir o ponto inicial de uma ferramenta de desenho de esboço plano com vários pontos é neutro: a ferramenta
  aguarda um ponto distinto sem erro de forma degenerada nem entrada no histórico. Entradas digitadas
  e comandos programáticos mantêm sua validação.
- Eixos de padrão circular e de plano inclinado rejeitam origens malformadas ou expressões fornecidas
  explicitamente, em vez de rotacionar silenciosamente em torno de zero. Origens numéricas continuam em milímetros;
  expressões associativas de eixo e referências a eixos de construção continuam não suportadas.
- O dimensionamento de linhas únicas e lados de retângulos permite selecionar uma linha ou suas duas extremidades.
  Digite um valor e pressione Enter para confirmar uma etapa de desfazer; valores rejeitados continuam editáveis e
  cancelar deixa o esboço inalterado. Ajustar mantém a geometria afastada da Paleta de esboço
  e dos controles de navegação, incluindo telas ampliadas e posicionamentos pendentes aninhados.
- Anotações de esboço, comprimentos durante o desenho e Medir usam as unidades do documento. Polegadas mostram
  três casas decimais e mantêm um quarto dígito significativo. Entradas de cotas de comprimento sem unidade capturam
  a unidade atual do projeto; alterar as unidades mantém a geometria e as expressões existentes intactas.
  Controles de distância de componente/imagem de referência, coordenadas do cursor de esboço, dimensões da planificação
  e profundidades de cavidades, volumes de interferência e comprimentos da biblioteca de peças também usam as unidades do projeto.
- As restrições Curvatura e Polígono aceitam operandos completos pré-selecionados. Fixar/liberar
  utiliza seleções completas de pontos/curvas de forma consistente; seleções mistas ou ambíguas
  mantêm o fluxo de seleção. Restrições rejeitadas preservam a seleção e o histórico de desfazer.
- Comunicação contextual de erros: uma ação **Criar issue** ao lado dos erros de operações abre um
  rascunho de issue do GitHub que pode ser revisado, com o erro, as configurações da operação, diagnósticos de compilação e
  contexto útil de reprodução. O rascunho é editável, com captura opcional da vista
  real e controles de copiar/salvar. Abrir o GitHub não envia a issue. A Ajuda também oferece
  relatos de resultados inesperados que não produziram erros.
- O cabeçalho oferece **Relatar um defeito**, incluindo um controle compacto acessível em
  janelas estreitas. Ele abre o mesmo relatório editável e a revisão da captura de tela sem
  alterar o modelo ativo ou uma dimensão pendente.
- Relatórios de falha: todo panic (em um comando, no núcleo ou em qualquer outro lugar) e todo encerramento
  anormal (abort, estouro de pilha, interrupção: detectados na próxima inicialização por um diário de comandos
  bloqueado por execução) deixa um relatório local com o backtrace, a versão e o commit, o comando e o
  projeto que o reproduzem (`solidcraft-cli crashes replay <id>`). Repetições são contadas,
  não regravadas; no máximo 20 relatórios, 50 MB e 90 dias. O aplicativo os mostra após uma falha (Ajuda >
  Relatórios de falha…) com um rascunho de issue revisado; agentes os leem por
  `diagnostics.crash_reports`, pela ferramenta MCP `crash_reports` ou pela CLI. Nada é enviado
  a qualquer lugar. Veja [relatórios de falha](crash-reports.md).
- Chat com agente (desktop): um painel lateral controla um agente de programação que o usuário
  já usa (Claude Code pelo adaptador ACP, Codex, Gemini CLI, OpenCode, Grok Build, …) pelo
  Agent Client Protocol, com a própria conta do usuário; o SolidCraft não armazena chaves. O agente trabalha
  pelas ferramentas MCP do SolidCraft em um endpoint próprio com controle de permissões: ações em arquivos aguardam Permitir ou
  Negar, relatórios de falha ficam fora de alcance, há 24 alterações por mensagem, uma etapa de desfazer por mensagem, e
  todo o grupo de processos do agente termina ao pressionar Parar ou encerrar o aplicativo. Os recursos de IA vêm ativados por padrão.
  O painel fica à esquerda ou à direita e pode ser minimizado em uma faixa ou ocultado sem parar o agente.
  A descoberta do Grok Build inclui `~/.grok/bin` e reutiliza o
  login do usuário no Grok. Veja [chat com agente](agent-chat.md).
- Scripts de comandos a partir de projetos (#886): `timeline.export_script` (CLI `solidcraft-cli script`)
  grava o histórico de um projeto como um script que `eval` e `run` reproduzem no mesmo projeto —
  parâmetros do usuário, unidades, regras de chapa metálica e plástico e materiais como seus próprios comandos,
  depois cada operação por `timeline.add`, preservando seus IDs, nomes e parâmetros de dimensão,
  para que todo tipo de operação seja reproduzido exatamente e uma operação que falha falhe da mesma forma. Remover as etapas
  do script após uma etapa fornece o projeto naquele ponto. Aparências, pastas, vistas e
  configurações são listadas como deixadas de fora; componentes e juntas ainda não são suportados.

## Lote da versão 0.2.0

- Inclui a nova navegação por eixos, desenho/seleção adaptativos de esboços, reparos de cantos na origem,
  referências duráveis de perfis/faces suportados, dimensionamento por extremidades e relatos contextuais.
- Junções de exibição suaves de paredes regradas são ocultadas apenas dentro do certificado geométrico suportado;
  divisões criadas pelo usuário e vincos reais permanecem protegidos. A vista em arame mantém arestas físicas.
- Colar como novo usa uma comparação limitada da geometria atual para prismas convexos suportados de linhas/círculos
  com furos circulares separados; formas não suportadas mantêm a verificação rigorosa existente.
- Novas tentativas de União com furo comum verificam curvas de contorno adicionais antes de cortar novamente o furo,
  evitando a perda reproduzida de parede fina. Recortes adicionais desconhecidos são rejeitados; recortes herdados
  de cilindros comuns mantêm a política existente, portanto isso não é uma prova geral de operações booleanas.
- A edição exata de juntas de montagem por caminhos completos continua sendo um trabalho separado e não faz parte deste lote.
- A versão estável 0.2.0 foi publicada: todas as somas de verificação dos arquivos da versão, a reabertura nativa sem cache, o
  aplicativo web implantado e as duas arquiteturas Flatpak foram verificados. A demonstração substituta do README usa
  a história original da caixa eletrônica e o ponteiro animado na interface real 0.2.0
  em inglês, com reprodução a 4×. A publicação segue [o fluxo de versões](releasing.md).

## Correções críticas da versão 0.2.1

- A vista de eixos e os controles de navegação permanecem ancorados à vista quando diálogos de operações ou
  a paleta de comandos são abertos, em escalas de tela normais e ampliadas. Ajustar ainda considera a
  área coberta pelo diálogo; controles cobertos não recebem cliques através dos diálogos.
- Faces planas elegíveis sobrevivem a operações de arredondamento, chanfro e casca com certificados
  exatos de origem. Esboços posicionados diretamente na parede da caixa e no piso interno acompanham
  alterações de largura e parede, inclusive após o corte USB e a reabertura sem cache.
- Remoção de subdivisões artificiais de paredes retas e cantos arredondados deixadas por casca e
  cortes de extrusão. Segmentos de parede gerados idênticos atravessam camadas adjacentes como uma única face
  física, com suportes exatos de altura total e costura de contornos. Divisões de faces criadas pelo usuário
  e aparências de faces são preservadas. Regressões permanentes abrangem material, STEP, reabertura e
  corpos não afetados ao longo do plano final da extrusão.
- Diálogos de operações usam a altura disponível da tela para seus campos, com OK e Cancelar
  fora da região de rolagem, inclusive em escalas de tela ampliadas.
- O formato de projeto 13 registra vinculações a faces de casca. Projetos antigos continuam legíveis;
  projetos recém-salvos exigem 0.2.1 ou posterior. Faces ambíguas ou não suportadas continuam rejeitando
  vinculações em vez de adivinhar um plano.
- Projeções salvas de uma faixa removida mantêm sua geometria e informam a referência
  perdida. Selecione explicitamente a parede novamente para reparar essa projeção.

## Navegação da vista

- No desktop, Ctrl/Cmd +/- e 0 alteram ou redefinem a escala salva da interface. O controle deslizante de escala mantém
  seu layout estável durante o arraste e aplica a escala selecionada ao soltá-lo.

- Predefinição de navegação OpenSCAD: arrastar com o botão esquerdo orbita, arrastar com o direito/do meio
  desloca, Shift+arrastar com o direito/do meio amplia e a roda amplia. Disponível nas preferências de Navegação compartilhadas de desktop/web.
- Pontas compactas e clicáveis dos eixos X/Y/Z com sinal, arraste suave para orbitar e encaixe de órbita com Shift
  em 26 vistas padrão e diagonais. O movimento do ponteiro se acumula independentemente de uma mola com amortecimento
  leve, com uma margem no contorno para evitar cintilação. Z para cima/Y para cima e a orientação quadrada mais próxima
  de cima/baixo são preservados. As diagonais incluem vistas de aresta a 45 graus e vistas isométricas de canto.

## Limitações conhecidas

- A CI agendada e manual verifica avisos de segurança, proibições e fontes de dependências fixadas com
  cargo-deny. Vulnerabilidades reprovam a verificação; crates sem manutenção geram avisos. Correções locais
  incorporadas exigem uma revisão separada do upstream porque as bases de avisos de segurança não conseguem corresponder
  as versões de suas dependências por caminho. As pull requests mantêm as verificações de licença existentes.

- Esboços associativos em faces de casca exigem uma fonte plana certificada única. Vinculações aninhadas
  a cascas e fontes coincidentes ambíguas continuam não suportadas.
- Arredondamentos G2: transições quínticas exatas de arestas retas entre faces planas, com uma face
  final plana perpendicular em cada extremidade. A interface e `solid.fillet` oferecem `curvature`
  (alias `g2`); o tamanho é um recuo circular equivalente. Faces/arestas curvas, extremidades
  inclinadas e junções de canto continuam não suportadas.
- Arredondamentos e chanfros: arredondamentos de raio constante, raio variável e comprimento de corda; um raio por aresta,
  arredondamentos próximos de arredondamentos anteriores de outro raio e remendos suaves de canto onde três arredondamentos
  se encontram (uma esfera para raios iguais); arestas retas (convexas ou côncavas) entre faces planas com faces
  finais planas perpendiculares (chanfros de distâncias iguais também encontram faces finais planas inclinadas, incluindo rebordos
  internos de bacias dentro da largura do rebordo); arredondamentos convexos também encontram extremidades planas inclinadas e cantos
  reentrantes ao redor de bordas de cavidades, com inclinações de parede diferentes e extensões confinadas à face;
  chanfros conectados de distâncias iguais também se recortam entre si ao longo de paredes inclinadas desiguais do rebordo;
  laços suaves completos de uma face plana, selecionados inteiros ou por uma aresta (cadeia
  tangente), incluindo círculos e paredes em qualquer ângulo (bases e topos de ressaltos, furos, pisos de cavidades,
  contornos de placas), laços com cantos agudos entre arestas retas (as transições se encontram em meia-esquadria), sequências
  de arestas vizinhas ao longo do laço de uma face plana (duas arestas superiores de uma caixa: em meia-esquadria, terminando em esquadro;
  também com a aresta entre elas, o canto se torna um octante de esfera),
  cantos arredondados arredondados novamente (octantes de esfera, toros em cantos côncavos); cada aresta de um
  corpo plano convexo ou de uma peça extrudada; arredondamentos dentro de padrões; cadeias fechadas de arestas
  curvas entre quaisquer dois lados suaves (um tubo secundário em um tubo principal), por uma esfera rolante (a
  transição é uma B-spline racional através de arcos exatos), fechadas ou terminando em faces planas perpendiculares às
  arestas, onde arestas retas ou curvas das tampas são recortadas até os contatos (a aresta reta
  entre dois arcos extrudados sobrepostos); arestas em lâmina, onde as duas faces têm normais
  opostas (uma parede que termina tangente a uma face: a esfera que cabe ali); uma aresta que
  termina ao longo de uma face final tangente a um lado (a aresta superior em arco dessa parede), cortada onde a
  transição a encontra; arredondamentos que terminam em um arredondamento anterior em faces curvas, em qualquer ordem (os arredondamentos
  de duas arestas em um vértice se encontram em um vinco, como em uma caixa), selecionados juntos ou um após o
  outro, exceto um arredondamento após um arredondamento da aresta entre a parede e uma face plana que ele
  atravessaria; as três arestas em um vértice de quaisquer faces suaves, com um único raio, terminam em um
  pedaço de esfera. Uma concordância no limite que colapsa uma parede lateral em curvas de contorno
  coincidentes é recusada por ser grande demais, em vez de retornar uma face que não pode ser triangulada (#1113).
- Operações booleanas: prismas ao longo de uma direção (placas com furos, ranhuras, padrões deles, seções
  iguais empilhadas de ponta a ponta) são combinados exatamente em 2D. Cortes axiais qualificados preservam
  furos anulares com ferramentas de corte que ultrapassam a peça, incluindo Pressionar/puxar para dentro após uma extrusão unida;
  a qualificação de paredes curvas rejeita falsos eixos transversais em vez de preencher furos.
  Corpos que se tocam ao longo de uma face inteira
  (uma peça e sua imagem espelhada) são costurados; faces planas coincidentes são afastadas quando
  suas vizinhas são planos ou cilindros. Outras configurações curvas coincidentes ainda podem
  falhar. A classificação limitada de contenção preserva cavidades totalmente internas e ilhas aninhadas
  de material independentemente da ordem das cascas, com regressões de ida e volta STEP e pertencimento
  ao material. Cascas curvas quase em contato usam o caminho booleano geral em vez de um
  atalho de contenção. Esferas são construídas com seis remendos sem polos.
- As propriedades de massa vêm de tesselação fina (faces curvas com precisão relativa de ~1e-4); triângulos
  que atravessam faces curvas (faces recortadas após operações booleanas) são primeiro divididos sobre a superfície.
- A importação STEP (nosso próprio leitor, crates/kernel/src/step_in) lê sólidos (B-rep com geometria analítica e B-spline), unidades,
  nomes de produtos, cores e montagens; deslocamentos analíticos em planos, cilindros, cones, esferas
  e toros regulares são lidos exatamente (distâncias com sinal/aninhadas, unidades e orientação da face). Superfícies
  deslocadas de forma livre e arestas somente de pcurve ainda não são lidas,
  e componentes de montagem são convertidos em corpos sem a hierarquia original. Faces periódicas fechadas (toro, faixas
  B-spline) são divididas em duas para gerar a malha; measure() usa uma tolerância mais fina para raios pequenos.
- A nomeação persistente acompanha o histórico exato de faces do núcleo (docs/book/src/dev/naming.md):
  referências são resolvidas primeiro pelo nome, acompanham divisões e mesclagens e avisam quando a entidade se dividiu,
  foi mesclada, desapareceu ou foi encontrada apenas pela geometria (operações que ainda não informam histórico).
  Novas vinculações de esboços planos usam certificados exatos de origem de caixas, tampas/laterais de extrusões e faces
  importadas imutáveis, com caminhos verificados de ocorrências de alvo e autoria. Uma fonte ausente ou alterada
  falha explicitamente; Redefinir plano do esboço mostra a prévia de uma nova vinculação ou de um plano fixo, mantendo
  a geometria atual e as identidades de dimensões. Faces modificadas não suportadas exigem um
  plano fixo explícito. Vinculações legadas mantêm o comportamento original de seu resolvedor.
- Roscas modeladas são cortadas como um corpo facetado (faces curvas com precisão de 1/200 do passo); Extrusão
  e Revolução até objeto em uma face curva recortam a ferramenta até a superfície do alvo.
- Quebras explícitas de esboço preservam elipses exatas, splines por pontos de ajuste/controle (incluindo graus altos)
  e cônicas; recorte/quebra analíticos também reconhecem ferramentas de corte de forma livre. Curvas de forma livre cortadas em um contato
  tangente (sem cruzamento claro) entram nesse perfil como polilinhas.
  Varreduras ao longo de curvas de esboço 3D seguem sua polilinha (paredes suaves quando o caminho é suave).
- Estender intersecta elipses exatas e splines racionais/de grau alto sem mover o suporte da linha ou do
  círculo. Deslocamentos associativos de forma livre têm um certificado de distância normal de 0,002 mm para
  todo o trecho e são refinados dentro de limites de nós, mantendo os IDs existentes.
  No limite de nós, subdivisões do certificado geométrico acomodam velocidades de parametrização
  diferentes das splines sem alterar os nós armazenados nem relaxar a tolerância de distância (#971).
  Os nós de ajuste de deslocamentos associados ficam ocultos na exibição, na seleção e no snap;
  arrastar a fonte editável atualiza o contorno derivado ao vivo. Romper associação restaura a edição dos pontos.
  Curvas singulares, autointersectantes e ambíguas são rejeitadas atomicamente; junções mistas e emendas não suaves
  de splines naturais fechadas continuam sendo limitações. Novas splines por pontos de ajuste explicitamente fechadas e saídas
  de deslocamentos fechadas usam interpolação C2 periódica, permitindo deslocamentos certificados repetidos; splines
  naturais antigas são preservadas. Clicar no ponto de ajuste inicial fecha deliberadamente um novo laço.
  Referências adaptativas de perfis recém-capturadas com cobertura completa das fontes mantêm as identidades externas
  e dos furos durante alterações da quantidade de trechos, incluindo alterações de parâmetros após a extrusão.
  Contornos parciais, intersectados ou ambíguos não se qualificam para essas referências de fontes inteiras.
  Multiconjuntos legados de trechos de curvas permanecem exatos e inalterados; seleções já perdidas podem exigir
  nova seleção. O formato 12 armazena as novas referências limitadas de contorno inteiro.
  Contornos analíticos projetados acompanham identidades de laços aninhados durante redimensionamentos, incluindo
  furos estacionários e cantos arredondados. Contornos convexos e tangentes suportados de linhas e arcos
  podem perder ou recuperar arcos de canto durante um deslocamento para dentro, preservando identidades reservadas de curvas
  e seleções de perfis vinculadas. O formato 12 salva essas associações; deslocamentos fechados antigos compatíveis
  são atualizados estruturalmente sem alterar seus IDs de curvas. Romper uma associação
  congela seu contorno atual e materializa referências de perfis posteriores. Deslocamentos derivados
  de um contorno com arcos desativados ainda exigem que sua associação seja rompida primeiro; junções mistas ou
  não convexas e referências mantidas a arcos individuais que desaparecem continuam limitadas.
- Os editores de Extrusão e Revolução mantêm direção original, expressões de extensão, alvos explícitos,
  eixos, entradas De/Até e a intenção de usar todos os perfis. Alterações sem efeito preservam os nomes de dimensões posteriores
  e os sistemas de coordenadas das ocorrências selecionadas; cancelar e editar com sucesso preservam o histórico de desfazer em uma etapa.
  Alterações de operações posicionadas exibem prévias até a entrada editada da linha do tempo, e as guias de Relevo cilíndrico
  usam o sistema de coordenadas do alvo real do esboço, da autoria ou de um terceiro componente com posicionamento único.
  Manipuladores de Revolução acompanham o eixo real com sinal e suportam extensões simétricas e bilaterais;
  arrastar uma extensão Até objeto converte seu valor derivado em um ângulo explícito. A geometria simétrica legada
  com eixo invertido é preservada pela convenção legada explícita do formato 11. Novas revoluções
  usam a direção normal do esboço para cada perfil selecionado, incluindo Até objeto e recorte de alvos
  curvos, independentemente do sinal do eixo selecionado.
- Relevo/gravação cilíndrico envolve perfis limitados de linhas ortogonais, incluindo furos, contornos
  côncavos e ilhas aninhadas, em cilindros externos completos ou mangas coaxiais com paredes radiais
  exatas. Novas operações inferem um único cilindro elegível quando o campo de face está vazio;
  alvos ambíguos exigem uma face explícita. Os caminhos salvos de ocorrências de alvo
  e autoria mantêm os posicionamentos escolhidos; faces ausentes/divididas são rejeitadas em vez de
  redirecionadas. Perfis gerais de forma livre, emendas, cilindros com degraus e outras superfícies curvas
  continuam não suportados. Testes de ida e volta STEP e verificações radiais/de volume independentes abrangem esse subconjunto.
- Casca e deslocamento de faces funcionam em corpos de planos, cilindros, cones e esferas (convexos ou não:
  caixas arredondadas, ressaltos, cúpulas, escareados; cantos onde quatro ou mais planos se encontram, como um
  piso inclinado para um ralo, desde que as arestas do canto sejam todas convexas ou todas côncavas); ângulo de saída em corpos planos convexos e em paredes (planos, cilindros → cones) entre tampas; lofts através de três ou mais seções são suaves (laterais B-spline através de cada seção), e entre duas
  seções são regrados.

- Ferramentas de cantos de esboço: arredondamento e os três modos de chanfro selecionam um canto conectado mesmo quando
  sua extremidade se sobrepõe à origem fixa ou a um ponto solto. O desenho comum ainda encaixa na
  origem; regressões de cliques na interface verificam geometria, perfis fechados e desfazer.
- Exibição de esboço: desenho, seleção, destaque, prévias de ferramentas e sombreamento de perfis compartilham
  contornos adaptativos de curvas projetadas. Ajustar, zoom, escala de tela e sistemas de coordenadas de esboços posicionados
  invalidam o cache de exibição sem alterar a geometria CAD ou o histórico da linha do tempo. Limites
  de trabalho e caminhos recortados impedem amostragem ilimitada ou arestas inventadas através do plano do observador.
  A Paleta de esboço permite rolar suas opções em janelas baixas e telas ampliadas, mantendo
  visíveis o estado das restrições e Finalizar esboço.
- Projeção de faces: contornos fechados têm uma ordem determinística com o exterior primeiro. Atualizar corresponde
  à geometria existente antes de atribuir posições, mantendo IDs de curvas e restrições salvos
  durante alterações da enumeração e redimensionamentos, incluindo círculos concêntricos. Identidades ambíguas
  mantêm a última geometria e informam uma referência perdida; a correspondência de permutações é limitada.
- Projetar aceita pré-seleção suportada completa em uma etapa de desfazer. Curvas e pontos visíveis de esboços
  externos mantêm a identidade de seu esboço de origem; geometria de modelo, eixos e planos usam
  os comandos existentes de projeção vinculada. A rejeição preserva o documento e a seleção;
  seleções mistas mantêm o fluxo comum de iniciar a ferramenta e depois selecionar.
- Cotas inline de esboço: Tab e Shift+Tab alternam entre as caixas de valores da forma ativa,
  selecionando os valores visitados para substituição. O preenchimento de expressões mantém a prioridade
  de Tab para avançar; regressões reais de teclado abrangem retângulos, linhas e círculos de valor único.
  Comprimentos, raios, ângulos de linhas, aberturas de arco com sinal e comprimentos de ranhura digitados persistem como restrições
  e expressões ao salvar/reabrir. A criação da forma e suas dimensões compõem uma etapa de desfazer;
  valores rejeitados mantêm o foco no campo editável.
- Componentes repetidos: seleção, visibilidade, propriedades, cópia e Mover/copiar na árvore do modelo usam
  caminhos verificados de ocorrências. Ocorrências aninhadas ativas criam esboços planos e sólidos em
  seu sistema de coordenadas selecionado. Movimentos pendentes são compostos através dos pais; cancelar um diálogo restaura
  seu movimento selecionado. Materiais de origem e aparências de corpos/faces acompanham as instâncias transformadas.
  Definições aninhadas compartilhadas continuam compartilhadas. Colar como novo em profundidade copia grafos de definições alcançáveis,
  históricos independentes de operações/entradas, estilos e registros internos de movimento, com verificações limitadas
  de geometria e avisos explícitos de vínculos externos. Colar como novo seleciona a cópia e preserva o contexto ativo
  de criação. Emendas periódicas STEP importadas sobrevivem ao posicionamento e à reexportação com topologia
  compartilhada e aparências intactas. Transferências registradas de corpos preservam o pertencimento histórico a União/Corte,
  material, posicionamentos e dados de usinagem durante alterações, cópia, desfazer e reabertura. Novos fios 3D
  registram seu sistema de coordenadas; fios legados migram a partir da origem conhecida sem alterar
  coordenadas. Vínculos legados quebrados sem rastreamento de origem podem exigir atualização ou nova vinculação explícita.
  Veja [fluxos de trabalho de ocorrências](occurrence-workflows.md).
- Soltar itens da barra de ferramentas sobre rótulos de painéis, espaços e margens adiciona comandos ao final com feedback de inserção;
  soltá-los sobre botões mantém a ordenação de inserir antes.
- Parâmetros e Configurações têm ações distintas no painel CONFIGURAR da barra de ferramentas SÓLIDO.
  Parâmetros abre o editor de variáveis do usuário/modelo; Configurações abre a tabela de configurações.
  Configurar continua disponível no menu suspenso do painel.
- A criação de parâmetros permite Enter com validação e continuidade da entrada pelo teclado. O preenchimento automático
  compartilhado de expressões permite seleção com Cima/Baixo, aceitação com Enter/Tab e dispensa com Escape;
  aceitar uma sugestão não envia um parâmetro nem aciona atalhos da vista.
- Barras de título de janelas móveis acompanham a paleta clara/escura do cabeçalho do aplicativo. Menus radiais do botão direito
  são opcionais em Preferências > Navegação; menus de contexto comuns continuam sendo o padrão.
- Distribuição para desktop: janelas do Windows em execução e recursos do executável usam o ícone do aplicativo;
  o empacotamento valida os recursos de ícones. O Nix instala lançadores, dados MIME e ícones gerados,
  com aplicativos de inicialização explícitos e descoberta de Vulkan-loader/drivers. As instruções do repositório Flatpak
  registram a fonte de execução exigida; verificações de versões exercitam a modelagem no desktop em X11/Wayland.
- A renderização OpenGL limita a suavização de serrilhado efetiva a 1x nos pipelines de interface, texturas e callbacks
  CAD para evitar quadros pretos. Outros backends mantêm a contagem de amostras solicitada, e
  as preferências armazenadas permanecem inalteradas.
- Regras de configuração: condições tipadas e ordenadas podem controlar parâmetros e supressões ou
  restringir valores. A prévia explica valores efetivos sem modificar o documento; aplicar
  regras valida as linhas atomicamente. Expressões de comprimento mantêm suas unidades de criação durante
  alterações das unidades do documento. A interface oferece condições estruturadas Todas/Qualquer/Não e ordenação de regras.
  Estilos de componentes e estilos semânticos certificados de faces estão disponíveis; origens não suportadas são rejeitadas
  atomicamente, e pintura certificada desatualizada de uma face não pode saltar para outra superfície. Veja [configurações](configurations.md).
- Material de chapa metálica: furos herdados, entalhes que atravessam arestas, cortes sobrepostos e ilhas
  preservadas compartilham os mesmos contornos na prévia plana, no DXF e nos sólidos desdobrados. Peças de material
  separadas sobrevivem ao desdobramento/redobra como um único grupo lógico de chapa, incluindo chapas divididas com flanges.
  Recortes feitos na planificação atravessam dobras em qualquer ângulo ou de forma curva: suas arestas se dobram em
  hélices e curvas sobre a dobra, com paredes perpendiculares à chapa. Bases de forma livre usam amostragem
  limitada.
  Cortes prismáticos paralelos de profundidade parcial mantêm o lado e a profundidade da cavidade durante transferência e
  desdobramento/redobra, com contornos separados de cavidades na prévia e no DXF. Interseção plana preserva
  furos exatos e ilhas mantidas, incluindo a união de vários perfis selecionados, e
  registra camadas complementares de profundidade como cavidades, mantendo a espessura da chapa. Usinagem inclinada
  ou que atravessa dobras não suportada é rejeitada atomicamente em vez de restaurar material posteriormente.
  Esboços se vinculam às faces planas de uma chapa (base e flanges, de qualquer lado) por painel, nas
  coordenadas da planificação, e a acompanham durante reconstruções e desdobramento/redobra.
  Uma flange na aresta distante de outra flange (um retorno) acompanha a extremidade fechada dessa flange e mantém
  distância da flange vizinha, e retornos de uma mesma operação são fechados entre si em meia-esquadria
  com a folga da regra onde suas flanges se encontram em um canto; retornos de operações separadas e
  flanges sobre retornos não são fechados entre si.
- Precisão de arredondamento por esfera rolante: referências independentes de conexões em T de cilindros deslocados verificam distância
  da superfície, tesselação realmente recortada, contatos com cilindros de suporte e tangência em duas
  escalas. O erro medido da peça recortada é inferior a 0,0001 mm; trata-se de uma verificação analítica
  de geometria. Ajustes com resolução insuficiente retornam um erro.
- O acionamento com contato verifica a trajetória entre extremidades livres, incluindo juntas vinculadas,
  reprodução e exportações de movimento esparsas. As verificações usam amostras limitadas de tesselação, não detecção
  contínua exata de colisões; paredes finas locais e pequenos detalhes continuam sendo limitações.
  Veja [limites de conjuntos de contato e recuperação](contact-sets.md).

## Pontes de esboços 3D

O desenho de esboços 3D encaixa em pontos de outros planos de esboço visíveis sem achatá-los sobre
o plano ativo. As extremidades das pontes capturam suas posições de origem; vínculos associativos de extremidades,
seleção de fios 3D por plano de construção e trilhos de guia de loft continuam sendo trabalho separado.
As setas do cabeçalho dos diálogos de operações recolhem e restauram o formulário sem descartar seleções
ou valores editados, incluindo o painel Aparência.
## Construção em caminhos 3D

Plano ao longo de caminho tem um seletor de caminho na vista, posição fracionária, prévia ao vivo e edição
na linha do tempo. Planos e pontos de construção resolvem fios 3D desenhados sem aplicar novamente seu plano
de esboço. Caminhos curvos usam segmentos amostrados; cadeias de várias curvas, distância absoluta
e posicionamento por objeto de referência continuam sendo trabalho separado, assim como trilhos de loft e linhas de centro.

## Propriedades do projeto (#1015)

- Projetos carregam projeto, número da peça, revisão, autor, organização, descrição e licença
  no objeto opcional `metadata` (Arquivo → Propriedades do projeto…, `document.metadata.*`,
  `inspect_design`). Cabeçalhos STEP/IGES e metadados 3MF usam os campos disponíveis no formato.
  Outras ferramentas mantêm seus dados sob uma chave DNS reversa, preservada ao abrir, editar
  e salvar, e nunca exportada. Veja [propriedades do projeto](book/src/guide/files.md#design-properties).
  Propriedades por componente (um número de peça por componente) podem usar o mesmo tipo.

## Desempenho de abertura de documentos

- A importação STEP no desktop prepara sólidos independentes consecutivos em lotes limitados com até
  oito workers, mantendo percurso serial, ordem dos resultados e identidades persistentes das fontes.
  O caminho web continua serial. Dados originais de teste com vários sólidos mostram tempos completos
  de importação 64–81% menores; peças de montagem com um só sólido não são distribuídas entre nós. Veja
  [medições e limites](perf.md#preparação-paralela-de-corpos-step-2026-10-12).
- Contêineres de projeto (#303, fase um) separam e eliminam duplicações de STEP, imagens brutas e
  matrizes binárias de malha sem perda, mantendo JSON simples como formato suportado para Salvar como/exportar.
  Medições de armazenamento de malhas geradas reduzem o trabalho de serialização e decodificação;
  um leitor com acesso por posição adia os blobs originais até depois da exibição provisória.
  Veja [formato do contêiner](design-container.md) e [medições de desempenho](perf.md).
- Salvamentos com dados já calculados e `cache: true` explícito podem incluir um cache B-rep nativo versionado por meio de registros de topologia
  e geometria próprios do SolidCraft, com um adaptador separado para truck e um leitor binário limitado. Certificados exatos
  de origem de faces e o histórico de operações sobrevivem a ciclos de codificação e decodificação. Uma imagem provisória
  pode ser gerada antes da leitura dos blobs originais; edição e medição aguardam
  a reconstrução autoritativa. Metadados em cache vinculam a prévia e a reconstrução ao
  mesmo documento apesar de gravações simultâneas no arquivo. Salvamentos padrão omitem esse cache e nunca forçam avaliação
  ou geração de malha sem cache; `preview: true` explícito continua sendo uma opção separada de malha já calculada.
  Chaves, limites de recursos e proteções de documento/revisão rejeitam resultados incompatíveis, hostis ou
  desatualizados. Um segundo núcleo de modelagem ainda exige seu próprio adaptador testado.
- Salvamentos com dados já calculados e `cache: true` explícito também podem conter uma miniatura PNG limitada de malhas já em cache.
  Início a lê sem reconstruir o projeto quando sua imagem local está ausente;
  miniaturas incompatíveis ou malformadas recorrem à geração normal. A imagem é
  independente do núcleo, e salvar nunca força geração de malha sem cache para produzi-la.

- A análise de desempenho de reconstruções sem cache separa leitura, geometria, posicionamento, geração de malha para exibição e
  miniaturas. Projeção do ponto mais próximo em linha reta e reutilização da avaliação exata de superfícies curvas
  reduzem trabalho repetido sem relaxar as verificações de geometria. Três execuções alternadas de benchmark
  otimizado reduzem o tempo mediano de reconstrução em 26% na caixa pública de sensor e 22%
  em uma montagem local de caixa; medidas de forma e topologia correspondem à compilação original.
  Veja [medições de desempenho](perf.md). Ainda restam custos de operações booleanas e importação sem cache.
- A geometria preparada de montagens sobrevive a seleções e mudanças de vista, enquanto alterações reais de geometria e
  posicionamento a invalidam. Uma medição de seleção de aresta em uma montagem de Pi cai de 9,24 s para
  0,57 s; a abertura inicial permanece inalterada. A análise real de desempenho no desktop Windows mede abertura
  mediana de 15,87 s, com 82% gastos na reconstrução de geometria; reduzir a abertura completa à metade continua pendente.

## Português brasileiro

- O catálogo offline `pt-BR` cobre todas as mensagens em inglês, está registrado em Preferências
  e é protegido pelas verificações de catálogo completo. Identificadores de comandos e nomes salvos de modelos
  mantêm seus valores originais. O livro em português brasileiro é publicado ao lado do inglês.

## Núcleo

- Novas tentativas bem-sucedidas de operações booleanas informam estratégia e ajustes numéricos
  como dados estruturados de inspeção; tentativas que podem mudar o resultado exato também mostram avisos. Os diagnósticos sobrevivem à avaliação
  em cache, desfazer/refazer e reabertura sem cache; o executor de robustez conta os eventos
  de novas tentativas por estratégia e informa qualquer truncamento do registro limitado. As verificações existentes
  de aceitação de geometria continuam em vigor.

- Cilindros de mesmo raio com eixos que se cruzam (um furo atravessando outro de mesmo
  tamanho, um T de tubos iguais) se tocam em dois pontos onde a interseção da malha não
  consegue traçar curvas e distinguir os quatro braços. Se a booleana comum falhar, ambas
  as faces são divididas pela linha do cilindro que passa pelos pontos de contato (sem mudar
  sólidos), a booleana é calculada e seu histórico composto sobre os operandos originais
  (#788). Linhas de divisão ficam como arestas nas paredes. Um cruzamento que ainda falha
  após dividir (raio 17.5 em bloco de 45 mm com ferramenta de 80 mm) continua recusado.

- A construção de curvas de interseção booleanas rejeita polilinhas compostas apenas por pontos, colapsadas e não finitas
  sem gerar panic. Regressões diretas do construtor mantêm extremidades abertas válidas
  e interseções esféricas fechadas; o gatilho interativo original continua não identificado.
- Casca de um corpo plano com degraus e unido não cria mais subdivisões horizontais em
  paredes contínuas. Novas paredes de Corte alinhadas aos eixos são costuradas usando seu suporte exato de
  construção e topologia compartilhada, preservando degraus reais e contornos intencionais. O caso local
  relatado passa de 86 para 49 faces com material e limites inalterados; regressões
  abrangem matéria-prima com degraus, ciclos de ida e volta STEP e projeções salvas de peças removidas.

truck (Apache-2.0) por trás de nossa própria crate `kernel`, com nossa operação de transição e novas tentativas/validação
de operações booleanas; veja `plan/adr/0001-geometry-kernel.md` (local) — resumo no README.
