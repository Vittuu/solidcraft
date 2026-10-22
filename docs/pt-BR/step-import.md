# Importação STEP

SolidCraft lê STEP (ISO 10303-21; geometria AP203, AP214 e AP242) com seu próprio leitor em `crates/kernel/src/step_in` (escrito a partir dos padrões públicos; sem código STEP de terceiros). Um arquivo STEP se torna uma operação-base **Importar** na linha do tempo: contém o B-rep do arquivo (o texto STEP é mantido no projeto, para que ele reabra sem o arquivo), e operações posteriores — arredondamentos, furos, cortes desenhados em suas faces, cascas — são construídas sobre seus corpos.

- **Abrir**: Arquivo → Abrir (`doc.open`) aceita `.step`/`.stp` independentemente de maiúsculas e minúsculas e inicia um novo projeto; `solidcraft part.step`; `solidcraft-cli eval part.step`, `solidcraft-cli run script.json --in part.step`; a ferramenta MCP `open`.
- **Inserir** no projeto atual: Arquivo → Inserir STEP… ou SÓLIDO → INSERIR (`file.insert_step {path}`).
- **Arrastar e soltar** um arquivo STEP na janela: inserido quando o projeto tem operações; caso contrário, aberto.

Leitura: B-reps de sólidos manifold (com vazios), B-reps facetados, modelos de superfície baseados em cascas; planos, cilindros, cones, esferas, toros, superfícies B-spline e B-spline racionais (com nós, Bézier, uniformes, quase uniformes), superfícies de revolução e extrusão linear, superfícies recortadas; linhas, círculos, elipses, curvas B-spline, polilinhas, curvas recortadas/de superfície/de emenda; unidades de comprimento e ângulo (prefixos SI, unidades baseadas em conversão, como polegadas); nomes de produtos (nomes de corpos vêm do nome do sólido; caso contrário, do produto); cores de superfícies; montagens (ocorrências next-assembly-usage posicionadas por transformações definidas por itens e itens mapeados) — a árvore de produtos é mantida na operação para o marco de componentes. Arquivos não são confiáveis: tamanhos, contagens de entidades e aninhamento são limitados, e todas as chamadas ao núcleo são protegidas. O que não puder ser lido se torna um aviso na operação (uma face omitida deixa um corpo aberto); ainda não são lidos: superfícies deslocadas, arestas apenas de pcurve.

## Arquivos de outros sistemas CAD

Arquivos de sistemas diferentes variam na direção vertical e nos nomes dados aos sólidos.
`doc.open`, `file.insert_step` e a ferramenta MCP `open` aceitam duas opções para STEP e IGES;
sem elas, o arquivo é importado como foi escrito.

- **`up`**: `"z"` (padrão) mantém os eixos do arquivo. Muitos sistemas usam Y para cima, e a peça
  fica deitada no espaço Z-para-cima do SolidCraft. `"up": "y"` gira apenas os corpos do arquivo
  um quarto de volta em X (+90°: Y se torna Z), com uma operação Mover logo após Importar,
  a mesma que a oferta **Colocar em pé** da janela adiciona. Importação e giro formam uma única
  etapa de desfazer; a resposta identifica Mover em `stand_up`. Corpos que já estavam no projeto
  não giram. A janela oferece colocar a peça em pé após abrir um STEP, a menos que a chamada
  informe o eixo vertical (`up: "z"` ou `"y"`).
- **`names`**: como os corpos serão nomeados. `"solid"` (padrão) usa o nome do sólido no arquivo,
  ou o nome do produto quando o sólido não tem nome. Alguns sistemas dão ao sólido o nome da
  última operação (`Cut-Extrude3`), que pouco informa sobre a peça. `"product"` usa o nome do
  produto que contém cada corpo; `"instance"`, o da ocorrência na montagem (`Post-2`), ou o
  produto quando não há ocorrência. Um nome ausente ou de preenchimento recorre ao próximo
  disponível (instância, produto, sólido). Nomes repetidos são numerados normalmente (`Post`,
  `Post (2)`). Os nomes ficam na operação Importar e são mantidos ao reabrir o projeto. O padrão
  continua sendo o nome do sólido, pois a exportação STEP do SolidCraft grava ali o nome do corpo.

```json
{"command": "file.insert_step", "params": {"path": "frame.step", "up": "y", "names": "instance"}}
```

## Aparências importadas

Cores de corpos e faces importados são mantidas durante operações posteriores. Cores de corpos
continuam como alternativa sob aparências e materiais explícitos do projeto; cores de faces
acompanham nomes persistentes de faces. Uma face removida não tem atribuição de cor a
exibir no corpo resultante.

## IGES

Arquivos IGES 5.3 (`.igs`/`.iges`) abrem, são inseridos e podem ser soltos da mesma forma: `crates/kernel/src/iges_in.rs` (escrito a partir da especificação publicada) reexpressa o arquivo como STEP, e o leitor STEP o lê, portanto a operação contém texto STEP. Leitura: B-reps de sólidos manifold (186) com suas cascas, faces, laços, listas de arestas e vértices; superfícies recortadas (144) e delimitadas (143) e remendos B-spline simples, costurados em um sólido por suas arestas compartilhadas (um corpo aberto quando não fecham); linhas, arcos circulares, elipses, curvas compostas, curvas B-spline racionais; planos (108, 190), cilindros, cones, esferas, toros, superfícies B-spline racionais, superfícies de revolução e cilindros tabulados; matrizes de transformação; unidades; cores (314 e números predefinidos); nomes de corpos (propriedades de nome; caso contrário, rótulos). Ainda não são lidas: superfícies regradas e deslocadas, curvas sobre superfícies definidas apenas no espaço de parâmetros. A exportação (`file.export` para `.igs`) grava cada corpo como um B-rep de sólido manifold nas mesmas superfícies usadas pela exportação STEP, com seu nome e sua cor.
