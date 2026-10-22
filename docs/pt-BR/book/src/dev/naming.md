# Nomenclatura persistente

Um arredondamento armazena as arestas que arredonda; um esboço, a face onde está; uma casca, as faces que abre.
Se essas referências fossem *índices* de faces e arestas, uma edição anterior que adicionasse uma face moveria o arredondamento
para outra aresta. O SolidCraft armazena **nomes** e os encontra novamente sempre que a linha do tempo
é avaliada.

## Por que os nomes seguem o histórico do kernel

Antes, os nomes eram calculados a partir da geometria: pontos amostrados de cada face nova eram procurados nas
faces da entrada da operação, e a maioria vencia; trechos de uma face dividida eram numerados conforme a posição
de seus centros. Essa suposição acerta na maioria das vezes e erra justamente quando importa: após
uma edição anterior que divide, une, espelha ou reduz uma face, pontos podem cair em uma vizinha,
centros podem trocar de ordem, e um arredondamento ou esboço se vincula silenciosamente a outra face. Nada
informa o usuário, o pior comportamento possível de uma ferramenta CAD.

O kernel agora informa, em cada operação, de onde veio cada face e aresta do resultado
(`crates/kernel/src/history.rs`: as origens de uma face e como ela mudou, a aresta de origem de uma aresta ou
as faces cujo encontro a criou). Os nomes seguem esse histórico, sendo exatos e determinísticos,
e quando uma referência não pode mais ser encontrada exatamente, a operação informa isso em vez de adivinhar.

## Como as faces de um corpo são nomeadas

`crates/doc/src/lineage.rs` compõe os históricos das operações de kernel executadas por uma operação do modelo (uma
ferramenta criada a partir de um esboço, cortada de cada destino, o corpo cortado separado em volumes) em uma
**linhagem** sobre operandos que podem ser nomeados:

| Operando | Suas faces são nomeadas |
|---|---|
| um corpo do modelo antes da operação | como estavam |
| uma cópia movida de um corpo (padrão, espelhamento) | `<name>@F<feature>.<instance>` |
| o lado da cavidade de uma casca expandida para fora | `F<feature>:inner:<name>` |
| um corpo criado pela operação | por papel: `F<feature>:side:<sketch curve>`, tampas `start`/`top`/`bottom`/`end`, lados de caixa `+x`…, faces importadas `face:<k>` |
| uma ferramenta copiada de uma operação repetida em padrão | `F<feature>:<role>@F<pattern>.<instance>` |

`crates/doc/src/history_names.rs` nomeia cada face do resultado a partir de suas origens: uma face mantida ou
recortada preserva o nome de sua origem; uma face de arredondamento ou chanfro é `F<feature>:blend:<edge>` da
aresta que substitui; a parede interna de uma casca é `F<feature>:inner:<face>`. A curva de esboço de um lado é a
curva sobre a qual o segmento do perfil está exatamente; nada é amostrado.

- **Trechos.** Faces que terminam com um nome são trechos de uma face e são diferenciadas pelas
  faces ao redor que as outras não têm: `F2:top#[F2:side:l4,F4:side:l4]` é o trecho do topo
  do bloco delimitado por sua parede esquerda e pela parede esquerda da ranhura. Isso resiste à ranhura se mover,
  crescer ou diminuir. Trechos que não podem ser diferenciados por nada ao seu redor são numerados (`#1`, `#2`): pelo que
  lhes deu origem quando isso difere (as duas metades da parede de um círculo); caso contrário, pela ordem do kernel e,
  então, não verificados.
- **Faces unidas.** Uma face formada por várias (uma união nivelada com o corpo) recebe o nome da face
  da qual todas são trechos ou, caso contrário, o nome da face de destino; ela também responde aos nomes das outras (aliases).
- **Arestas** são nomeadas pelas faces que separam, `<face A>|<face B>`, com `~k` quando várias arestas passam
  entre as mesmas duas faces (numeradas pela posição; uma referência por esse número é
  informada quando apenas a geometria a confirma).
- **Volumes separados.** Quando um corte deixa um corpo em partes, o conjunto é nomeado primeiro, para distinguir trechos de uma face em
  corpos diferentes, e os corpos (`Bar`, `Bar (1)`) são atribuídos na ordem de seu menor nome
  de face, não na ordem de cascas do kernel (que uma edição anterior pode trocar; veja
  [Projetos antigos](#projetos-antigos)).

Os nomes são calculados sob demanda, na primeira solicitação, e compartilhados pelos clones de um corpo.

### Onde não há histórico

Um corpo criado por uma operação que ainda não informa histórico (alguns caminhos do kernel não retornam nenhum; veja
as listas de passagem do kernel e as operações de doc que ainda não o compõem: chapa metálica, plástico
e operações de superfície, relevo, nervura, substituir face, o corte facetado de rosca), ou uma face cujo papel
não seja conhecido exatamente (os lados de uma transição, o perfil gerado de uma hélice ou tubo, a ferramenta de um furo), é
nomeado da forma antiga, pela geometria, e seus nomes são marcados como **não verificados** (`model.faces` informa
`exact: false`). Uma referência resolvida por um nome não verificado quando seu ponto armazenado já não
está na face informa isso: `face ... was found by geometry, not exact history; check it`.

## Como uma referência encontra sua face novamente

`crates/doc/src/name_lookup.rs`, nesta ordem:

1. a face com esse nome;
2. a única face que responde a ele: unida a ela (a operação informa a união), ou o trecho
   nomeado, ainda delimitado pelas faces listadas em seu nome;
3. seus trechos (ela foi dividida ou está em vários corpos): o mais próximo do ponto da
   referência, e a operação informa a divisão;
4. nada: a operação recorre à geometria que registrou e informa que a face não existe mais.

Um nome de aresta é encontrado como a aresta entre as duas faces que nomeia, cada uma encontrada como acima: quando
exatamente uma aresta passa entre elas, ela é a aresta desejada, mesmo depois de uma divisão de uma de suas faces.

O vínculo de um esboço com uma aresta projetada (`sketch.project {edge: [x, y, z]}`, além de Interseção e
Incluir geometria 3D de uma aresta) mantém o nome da aresta ao lado de seu ponto. O vínculo é essa aresta
para onde quer que uma alteração a leve; no caso de uma aresta dividida, é o trecho mais próximo do ponto do vínculo,
e o esboço informa isso. Quando nenhuma aresta tem mais o nome, o vínculo é perdido e mantém sua última
geometria, como acontece com uma face: ele não usa qualquer aresta que agora esteja mais próxima. Um vínculo salvo
sem nome por uma versão que mantinha apenas o ponto é encontrado pela aresta mais próxima de seu ponto, como
sempre foi, e recebe o nome dessa aresta quando seu esboço for editado novamente.

Um comando que recebe uma face como ponto pode receber o nome listado por `model.faces`
(`face.delete`, `solid.shell`, `solid.draft`, `solid.offset_face`, `solid.split_face`, `faces`
de `solid.replace_face`, `face` de `sheet.convert`). A mesma busca é usada (`naming::locate_one`:
a face com o nome ou a única que responde por ele, no corpo fornecido ou em qualquer corpo;
um nome que encontra várias faces é recusado). O comando usa um ponto nessa face
(`naming::named_face_point`: o incentro do maior triângulo da face, movido para a superfície
de plano, cilindro, cone, esfera ou toro), armazenado pela operação como um ponto escolhido,
com o nome que recebe como antes. Nada novo é salvo no projeto. `model.faces` lista esse
ponto para todas as faces.

Um furo posicionado em uma face é a exceção a "o ponto mais próximo do ponto da referência":
sua posição é um local, não uma seleção. O furo acompanha sua face ao longo de seu próprio eixo, então ele
se move com a face quando ela se move, e quando a face não alcança mais a posição, ele permanece
onde foi colocado e a operação informa isso, em vez de deslizar para a aresta da face e perfurar
meio furo ali. (Furos em pontos de esboço acompanham seu esboço.)

## Projetos antigos

Projetos salvos antes do formato 14 armazenavam nomes calculados a partir da geometria, e alguns deles nomeiam uma
face de forma diferente do histórico (a parede externa de um tubo era nomeada pela curva de esboço mais próxima
de seu centro, seu círculo interno). A migração desse projeto marca os nomes (`?F2:top#2`), e eles
são encontrados entre os nomes fornecidos pela geometria, como a versão que os salvou os encontrava: um projeto antigo
é avaliado como foi salvo. Selecionar a face novamente armazena seu nome exato.

Suas operações também continuam nomeando as partes em que dividem um corpo na ordem de cascas do kernel
(`Feature::shell_piece_order`, definido pela migração): os nomes de corpos armazenados depois delas (corpos de arredondamento e
chanfro, destinos, participantes, materiais, aparências, juntas) significam as partes nessa
ordem. Operações adicionadas depois nomeiam suas partes a partir das faces.

## O que ainda falta

- Certificados (`FaceSource`, as identidades de vinculação estrita por trás de esboços em faces e
  estilos de face configurados) ainda só são preservados onde o mapeamento de fontes planares do kernel os mantém;
  transportá-los por `History::face_sources` permitiria que uma vinculação certificada acompanhasse uma face remodelada por uma
  união posterior, algo hoje recusado de propósito.
- Operações sem histórico, listadas acima, mantêm os nomes geométricos não verificados.
