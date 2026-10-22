# Chapas metálicas

A aba CHAPA METÁLICA cria peças a partir de uma chapa de espessura constante sob uma **regra de chapa metálica**
(espessura, raio de dobra, fator K), definida em **Regras de chapa metálica** (`sheet.manage_rules`).

- **Flange** (`sheet.flange`): uma flange base a partir de um perfil de esboço, uma flange de contorno a partir de um esboço
  aberto ou uma flange de aresta a partir da aresta de uma peça.
- **Bainha** (`sheet.hem`) e **Dobrar** (`sheet.fold`) ao longo de uma linha de esboço.
- **Desdobrar / Redobrar** (`sheet.unfold`, `sheet.refold`) para trabalhar na peça plana.
- **Converter** (`sheet.convert`) transforma uma placa de espessura constante em chapa metálica ou, com
  arestas de dobra, um sólido ou corpo com casca.
- **Criar planificação** (`sheet.flat_pattern`) mostra a peça plana com suas linhas de dobra e uma
  tabela de dobras e a exporta como DXF ou CSV da tabela de dobras pelo seletor de salvamento.

As dobras seguem o fator K da regra, para que os comprimentos planos correspondam ao resultado de uma dobradeira. Os testes
conferem comprimentos planos com desenvolvimentos de dobra calculados manualmente.

Uma flange de aresta retira seu recuo apenas de sua própria aresta. Onde essa aresta termina em um canto
interno da base (a base continua além dela, como o braço de um L), o restante da base
permanece, e o **alívio de dobra** da regra (`relief_width` e `relief_depth`; Largura do alívio e
Profundidade do alívio em Regras de chapa metálica) é cortado nela ao lado da extremidade da dobra: uma ranhura da largura
do alívio, desde a profundidade do alívio antes da linha de dobra até uma largura do alívio além da
aresta distante da flange na planificação. Assim, a dobra e a flange ficam livres da base, tanto dobradas quanto
planas. Um alívio que não pode ser cortado (largura inferior a 0.01 mm, tão largo quanto a base ao lado da
dobra, ou uma ranhura que separaria a base em partes) é omitido, e a operação de flange informa um
aviso indicando onde.

Onde um recorte removeu parte de uma aresta (um canto retirado da placa, um entalhe no
meio de uma aresta), uma flange de aresta escolhida nessa aresta percorre o trecho que os recortes
deixaram mais próximo do ponto escolhido e retira seu recuo apenas desse trecho. A base do outro
lado de um entalhe mantém sua aresta, e escolher um ponto ali, na mesma operação ou em uma posterior, cria
uma flange própria. Onde ainda sobra base ao lado da extremidade da dobra nesse recorte, ela
recebe o alívio de dobra descrito acima. Uma aresta totalmente removida pelos recortes é recusada.

O recuo é cortado do contorno da base tal como foi desenhado: onde a linha de dobra cruza um
arco do contorno (uma placa com cantos arredondados, dobrada por dentro ou no meio), o arco continua
sendo um arco e termina ali, na planificação e no DXF. Uma curva de forma livre (elipse,
spline) em um contorno cortado por um recuo é acompanhada por arcos tangentes com erro máximo de 0.001 mm.

Onde duas dobras de uma mesma operação de flange de aresta se encontram em um canto da base, a **forma de alívio
de canto** da regra (`corner_relief_shape`; Forma do alívio de canto em Regras de chapa metálica) define o que
é cortado ali. `notch`, o padrão, não corta nada adicional: as dobras terminam perpendicularmente no canto, o que
deixa um degrau na planificação entre a extremidade de cada dobra e a extremidade fechada de sua flange. `tear`
também não corta alívio, nem deixa degrau: ambas as dobras seguem até o canto onde as duas linhas de dobra
se encontram, e a extremidade de cada dobra é cortada em linha reta, na planificação, desse canto até onde começa a
extremidade fechada de sua flange. Na peça dobrada, as duas dobras se tocam no canto e abrem para a
folga da regra onde começam as flanges; a tabela de dobras lista uma dobra com rasgo entre os pontos médios de suas duas
extremidades. Um rasgo cujos cortes se cruzariam na planificação ou que faria uma dobra invadir a
outra na peça dobrada (flanges com ângulos muito diferentes) é omitido com um aviso, e o canto
mantém seu entalhe. `round`
e `rectangular` cortam um alívio com a dimensão transversal do **alívio de canto** da regra, centrado no canto onde
as duas linhas de dobra se encontram: retiram da base uma forma redonda com metade dessa dimensão como raio, ou um quadrado
com metade dessa dimensão como lado, e ambas as dobras param onde o alívio começa, cortadas perpendicularmente. Os
painéis de flange mantêm suas extremidades. O alívio aparece na peça dobrada, na planificação, na tabela de
dobras (as dobras ficam mais curtas) e no DXF (um alívio redondo é um arco). Em um canto que não é
reto, o redondo acompanha o ângulo da base e o quadrado vira um deltoide, com seus dois cortes
perpendiculares às arestas. Alterar a forma em uma regra altera as peças criadas com essa regra. Um
alívio que não cabe (dimensão transversal inferior a 0.01 mm, que elimina toda uma dobra, ou que alcança uma distância
de seu canto igual à de outra aresta da base) é omitido, e a operação de flange informa um
aviso indicando onde.

```json
{"command":"sheet.manage_rules","params":{"name":"Sheet 2 mm","thickness":"2 mm","k_factor":0.5,
 "bend_radius":"1 mm","corner_relief_shape":"round","corner_relief":"8 mm","active":true}}
```

A forma e o tamanho da regra são os padrões. Uma operação de flange de aresta pode escolher seus próprios
valores para os cantos formados por suas flanges: `corner_relief_shape` (`notch`, `round`, `rectangular` ou
`tear`) e `corner_relief` (o tamanho transversal, um comprimento ou uma expressão) em `sheet.flange`, ou
Forma do alívio de canto e Alívio de canto no diálogo Flange. No diálogo, Regra como forma e um tamanho
vazio usam os valores da regra; se um deles for omitido, segue a regra enquanto o outro usa a escolha da
operação. O alívio é aplicado somente entre flanges de uma mesma operação, portanto a escolha de uma
operação nunca se encontra com a de outra em um canto, e duas operações de flange de aresta da mesma
peça podem usar alívios diferentes sob uma mesma regra. Uma flange de base ou de contorno usa somente
os cantos da regra e recusa ambos os parâmetros. A folga da flange e os demais valores continuam vindo
da regra para todas as flanges. A escolha da operação é salva com o projeto (formato de arquivo 23).

```json
{"command":"sheet.flange","params":{"edges":[[60,0,2],[120,45,2],[60,90,2],[0,45,2]],
 "height":20,"corner_relief_shape":"round","corner_relief":"6 mm"}}
```

Onde as flanges de aresta de uma operação se encontram em um canto da base, seus painéis são fechados
um em direção ao outro em qualquer ângulo de dobra: cada extremidade é cortada perpendicularmente à chapa, em uma linha na planificação
escolhida de modo que as arestas mais próximas entre si (nas faces internas) fiquem paralelas, separadas pela
**folga** da regra, de cada lado do plano a meio caminho entre as flanges. Flanges a 72° recebem extremidades
inclinadas na planificação; cantos que não são retos fecham da mesma forma, e a 90° o canto é o mesmo
que a regra sempre produziu. Um canto que não pode ser fechado (folga inferior a 0.01 mm, flanges dobradas
em sentidos opostos, extremidades que se sobreporiam na planificação) permanece aberto, e a operação de flange
informa um aviso indicando onde. Uma flange adicionada por uma operação posterior deixa o canto como estava:
a flange anterior mantém seu comprimento e a nova termina em sua dobra.

As flanges de uma mesma operação de flange de aresta podem ter, cada uma, seu próprio ângulo e altura. Uma
entrada de `edges` é um ponto `[x, y, z]`, que usa `height` e `angle` da operação, ou um
objeto `{"point": [x, y, z], "angle": "105 deg", "height": "30 mm"}` cujos `angle` e `height`
(um deles ou ambos) substituem os valores para essa aresta. Eles são expressões como as da própria operação,
permitindo o uso de parâmetros, e cada um é uma entrada própria da operação (`Edge angle`, `Edge height`) após
`Height` e `Angle`. Os cantos entre essas flanges fecham como os de flanges iguais:
as extremidades seguem uma em direção à outra, separadas pela folga da regra, de cada lado do plano a meio caminho entre
as duas flanges, e a extremidade de uma flange mais alta mantém essa linha até sua própria ponta. Essa é a forma de
fechar um canto entre flanges de ângulos diferentes, porque flanges de operações separadas
não fecham uma em direção à outra. No diálogo Flange, selecionar mais de uma aresta mostra uma linha
por aresta com uma caixa de ângulo e outra de altura; uma caixa vazia usa o valor acima.

```json
{"command":"sheet.flange","params":{"edges":[[60,0,2],{"point":[120,45,2],"angle":"90 deg"},
 {"point":[60,90,2],"angle":"105 deg","height":"30 mm"},{"point":[0,45,2],"angle":"120 deg","height":"25 mm"}],
 "height":38.6,"angle":"75 deg"}}
```

Uma flange também pode partir da aresta distante de outra flange: um retorno, como um lábio virado para dentro
ao longo do topo das paredes de uma bandeja. Ele percorre essa aresta de ponta a ponta, acompanhando a
extremidade fechada da flange sobre a qual está, inclinada ou não. Dois retornos de uma mesma operação cujas flanges
se encontram em um canto da base também fecham um em direção ao outro, com a folga da regra. Eles
são cortados em meia-esquadria de cada lado de um plano que segue a média das normais dos dois painéis, divide ao meio
suas linhas de dobra e passa pela interseção das linhas onde os painéis começam. Retornos que estão em
um mesmo plano (retornos nivelados nas paredes de uma bandeja, qualquer que seja o ângulo das paredes) se encontram na
bissetriz do canto, como os lados de uma moldura. Cada painel termina em uma linha inclinada na planificação.
Cada dobra termina perpendicularmente onde toda ela mantém metade da folga em relação a esse plano, no máximo até
a extremidade da flange sobre a qual está, o que pode deixar um pequeno trecho da aresta dessa flange exposto.
Retornos virados em sentidos opostos passam um pelo outro. Um retorno que não encontra outro em um canto termina
junto com a flange sobre a qual está; onde a flange vizinha se inclina para dentro sobre ele, o retorno é
recuado até manter dessa flange a mesma distância que a extremidade da flange sobre a qual está.
Retornos que não podem ser fechados permanecem abertos com um aviso, como na base. Retornos de operações
separadas também não fecham um em direção ao outro: faça os retornos ao redor de um canto em uma só operação.
Um alívio de canto ou um rasgo não muda nada disso: ele deixa as extremidades fechadas das flanges como
estão, e os retornos sobre elas fecham sobre um canto com alívio ou rasgo como sobre um canto inteiro.

```json
{"command":"sheet.flange","params":{"edges":[[60,2,20],[118,45,20],[60,88,20],[2,45,20]],
 "height":12,"angle":"90 deg"}}
```

(Os quatro retornos de uma bandeja de 120 x 90 em chapa de 2 mm com paredes a 90° e altura 20: cada ponto é o
meio da aresta distante de uma parede em sua face interna, de modo que os retornos se voltem para dentro sobre a base.)

**Converter** também aceita um sólido ou corpo com casca: escolha a face que permanece fixa e as arestas
vivas que se tornam dobras (`bends`; Arestas de dobra no diálogo). As faces planas alcançadas a partir da
face fixa através dessas arestas se tornam a chapa. Um corpo com casca mantém suas paredes e sua
espessura; as faces de qualquer outro sólido se tornam o exterior da chapa, com a espessura da regra
aplicada para dentro do corpo. Cada aresta viva permanece onde as faces de sua dobra se encontrariam, para que a peça
mantenha seu volume envolvente, e a planificação corresponde aos comprimentos externos menos uma dedução por dobra,
2·(R + T)·tan(θ/2) − θ·(R + K·T). Uma dobra pode ser escolhida em qualquer uma das superfícies de uma casca; as dobras podem virar
em qualquer sentido, se suceder e assumir qualquer ângulo (as paredes de uma bandeja inclinada a 72°). Uma
aresta entre duas faces da chapa que não é dobra é aberta por um corte, com cada lado afastado pela metade da
**folga** da regra do plano a meio caminho entre os dois, como nos cantos de uma bandeja. Uma dobra
desenhada como chanfro (uma faixa plana entre duas faces, a corda do arco da dobra em modelos sem
raios de dobra) é escolhida no meio da faixa: as faces se encontram em sua aresta viva virtual,
em uma única dobra com o raio da regra, e a faixa deixa de ser uma face da chapa. Uma dobra com
raio modelado (um cilindro entre duas faces planas tangentes, como em peças de outros sistemas
ou uma aresta arredondada) é escolhida em qualquer ponto do cilindro, em qualquer superfície da
casca, e convertida da mesma forma. A dobra usa o raio e o fator K da regra, não o raio modelado;
a planificação é a produzida pela regra.

O resultado é um corpo de chapa comum: flanges de aresta, cortes, Desdobrar, a planificação e suas
exportações funcionam nele, e ele acompanha o sólido quando um parâmetro muda (as arestas e a
face fixa são encontradas por seus nomes). Sem arestas de dobra, Converter espera uma placa,
como antes. Nos dois casos, paredes arredondadas (furos, extremidades de ranhuras, cantos
arredondados e entalhes) permanecem círculos e arcos na planificação e no DXF, inclusive quando
um STEP as representa em trechos. Furos e ranhuras de cada face da chapa são seus recortes,
posicionados pelas dobras. Uma gravação em uma face (rebaixo com fundo plano e profundidade de
até metade da espessura) se torna uma cavidade, como em uma placa, em qualquer superfície da
casca. Ainda não são convertidos: faces curvas que não sejam dobras (chapa calandrada); em
corpos com arestas de dobra, um furo ou gravação que alcance uma dobra ou uma cavidade mais
profunda que metade da espessura (faça o corte depois de converter); e dobras que fechem um anel.

Uma placa pode ter rebaixos em uma face, como uma etiqueta gravada ou um rebaixo cilíndrico:
fundo plano, paredes perpendiculares à placa e profundidade de até metade da espessura. Cada
um se torna uma cavidade da chapa (veja abaixo), preservada na peça convertida. A chapa mantém
a espessura da placa; a planificação é a placa inteira com os rebaixos listados em `pockets`
(contorno, lado e profundidade). O DXF desenha o contorno na camada de cavidade correspondente à
profundidade, separado dos contornos de corte, para o laser marcar em vez de cortar. Os rebaixos
podem estar na face escolhida ou na oposta. Rebaixos mais profundos que metade da espessura,
em ambas as faces ou maiores que a face ao redor (uma placa mais fina com ressaltos) são
recusados com o motivo. Um rebaixo fica em uma abertura da face em que foi cortado; as faces
das abas de uma peça dobrada não são rebaixos. Uma peça dobrada é recusada como placa e deve
ser convertida com suas arestas de dobra. Uma dobra que atravesse uma cavidade é recusada;
portanto, mantenha dobras e flanges afastadas de uma gravação.

```json
{"command":"solid.box","params":{"length":100,"width":60,"height":30,"body_name":"Tray"}}
{"command":"solid.shell","params":{"faces":[[50,30,30]],"thickness":2}}
{"command":"sheet.convert","params":{"body":"Tray","face":[50,30,0],
 "bends":[[50,0,0],[100,30,0],[50,60,0],[0,30,0]]}}
```

A `face` também pode ser o nome listado por `model.faces` (`"face":"F1:+z"`), evitando
calcular um ponto em uma face com furos ou contorno curvo: `model.faces` fornece para cada
face um `point` que fica sobre ela.

Para um desenho de fabricação ou uma planilha, `sheet.export_bend_table` retorna CSV por padrão; escolha
`format: "json"` para dados estruturados. Ele inclui a regra de chapa, espessura, ângulos de dobra,
raios, desenvolvimentos de dobra e coordenadas das linhas de centro na planificação. Comprimentos usam milímetros e ângulos usam
graus. Uma dobra é listada onde sua linha de centro tem material: uma encurtada por um recorte
informa o comprimento e as extremidades que restaram; uma dividida por recortes fornece uma linha para cada trecho,
todos com o número da dobra (a planificação e o DXF desenham os mesmos trechos como linhas de dobra).
Forneça `path` para salvar ou omita `path` para receber o conteúdo pela resposta
do comando. `body` usa o último corpo de chapa por padrão. Exportar não adiciona uma entrada à linha do tempo nem
consome uma etapa de desfazer.

```json
{"command":"sheet.export_bend_table","params":{"format":"csv","path":"bracket-bends.csv"}}
{"command":"sheet.export_dxf","params":{"path":"bracket-flat.dxf"}}
```

O DXF é um desenho AutoCAD 2000 (`AC1015`) em milímetros. As camadas `OUTLINE`, `BEND_UP` e
`BEND_DOWN` separam cortes de centros de dobra. Um furo redondo é `CIRCLE`, um arco circular
é `ARC`, uma elipse completa é um `ELLIPSE` e um arco de elipse é um `ELLIPSE` elíptico;
trechos de spline, contornos de texto e outras cônicas são `SPLINE`s do próprio grau, sem
escrever curvas como cordas. Para uma máquina que lê apenas linhas e arcos, passe
`"curves": "arcs"` a `sheet.export_dxf`: curvas livres passam a ser arcos tangentes e linhas
com erro máximo de 0.001 mm.

Perfis de flange base mantêm seus furos internos. Cortes passantes que cruzam uma aresta da chapa se tornam
entalhes no contorno plano; cortes sobrepostos e repetidos removem sua área combinada uma única vez.
A prévia plana, o DXF e o sólido desdobrado usam os mesmos contornos de material. Linhas e arcos
circulares permanecem exatos. Uma elipse, spline ou texto mantém a curva como foi desenhada na planificação
e no sólido desdobrado; o sólido dobrado a acompanha com arcos tangentes com erro máximo de 0.001 mm,
e as listas de pontos a gravam como cordas com erro máximo de 0.001 mm (o DXF mantém a curva, veja acima).
O JSON da planificação lista cada recorte duas vezes, na mesma ordem: como pontos (`cutouts`) e
de forma exata (`cutout_shapes`): um furo redondo como `{"circle": {"center", "radius"}}`, qualquer outro por seus
segmentos `line`, `arc` (centro, raio, início e varredura em graus) e `polyline` (uma
`polyline` para cada quarto de elipse, spline ou curva de glifo).

Cortes podem deixar pedaços de material separados, incluindo o centro de um corte em anel. Esses pedaços
permanecem como um único grupo lógico de chapa ao Desdobrar / Redobrar e são exportados como contornos planos separados.
Perfis grandes que excedem o orçamento de processamento de geometria informam um erro, em vez de
exportar um contorno parcial.

Um corte de Caixa, ou um corte de Cilindro cujo eixo seja a normal da chapa, usina a chapa como a
extrusão de sua projeção: através da chapa, ele é um recorte (no caso de um cilindro, redondo), e
parcialmente, uma cavidade. Um corte de primitiva em qualquer outro ângulo, ou através de uma dobra formada, é recusado.

Furo usina a chapa da mesma forma a partir de uma face plana de qualquer painel, da base ou de uma flange, em
qualquer lado, dobrado ou desdobrado: perpendicular a esse painel, um furo passante (simples ou roscado) é um
recorte redondo da planificação e um `CIRCLE` no DXF, e um rebaixo cilíndrico ou um furo cego
de fundo plano é uma cavidade redonda. Um furo inclinado em relação ao painel ou perfurado na aresta da
chapa, um escareamento e uma ponta de broca são recusados.

Cortes de extrusão paralelos e sem inclinação também preservam cavidades rasas ao Desdobrar / Redobrar.
O fundo e as paredes laterais da cavidade permanecem como geometria sólida; cortes sobrepostos de lados opostos
se tornam um furo passante onde removem toda a espessura. O JSON da planificação lista
`pockets` com seu lado, profundidade e coordenadas de espessura inferior/superior em milímetros.
Contornos de cavidade no DXF usam as camadas `POCKET_BOTTOM_Z…_TO_…_MM`, `POCKET_TOP_Z…_TO_…_MM` ou
`POCKET_INTERNAL_Z…_TO_…_MM`. São operações de usinagem, portanto não trate toda
camada de cavidade como um contorno de corte passante ao importar o desenho em uma ferramenta CAM.

Extrusão com Interseção em uma base reta ou chapa desdobrada mantém o material dentro tanto da
projeção da ferramenta atual quanto de seu intervalo de profundidade, incluindo furos internos, ilhas separadas
e cavidades anteriores. Uma interseção rasa registra as camadas removidas acima e abaixo do
material mantido como cavidades superiores e inferiores. A regra de chapa mantém a espessura original do material bruto;
o sólido restante pode ser mais fino. Uma interseção completamente vazia remove a chapa e
seus metadados juntos. O DXF registra a profundidade de cavidades nos nomes de camadas; importar esse DXF como
geometria comum de esboço não reconstrói automaticamente as profundidades de usinagem. Cada
camada de profundidade positiva mantida ou removida deve atender à resolução de extrusão de 0.000001 mm do
kernel; camadas menores informam um erro em vez de serem eliminadas por arredondamento. Até objeto pode
manter uma ferramenta recortada até um prisma reto, usando sua projeção realmente recortada. Uma ferramenta
curva de profundidade variável é recusada antes de alterar a geometria da chapa. Perfis de cavidades podem acompanhar painéis planares durante a conformação,
mas uma cavidade que cruza uma zona de dobra ainda não pode ser redobrada. Usinar material de dobra já formada,
cortes inclinados de chapa e outras operações 3D de Corte/Interseção não suportadas em chapa informam um erro antes de alterar
o projeto; elas não podem restaurar silenciosamente material removido durante uma reconstrução posterior da chapa.

Um esboço pode ser desenhado em uma face plana da chapa: na base ou em uma flange, de qualquer lado. Escolha
a face (ou forneça a `sketch.create` o ponto `plane: {face: [x, y, z]}`), e o esboço se vincula
a esse painel em vez de a uma face de origem certificada. Ele mantém o nome da face,
`sheet:<body>:<panel>:<top|bottom>` (o painel é `base` ou a identificação da operação de flange), e encontra
a face novamente sempre que a chapa é reconstruída: após uma mudança de parâmetro, como a espessura ou
o ângulo de uma flange, ao Desdobrar / Redobrar, depois de reabrir o arquivo e quando o corpo é
renomeado. As coordenadas do esboço são as da planificação, então um ponto desenhado em uma flange fica onde
a planificação e seu DXF o apresentam, e na face inferior y segue no sentido oposto, visto
de baixo. Quando a face desaparece (sua flange é suprimida ou excluída), o esboço informa uma vinculação
perdida em vez de passar para outra face; **Redefinir plano do esboço** o vincula a outra
face. `fixed: true` continua congelando o plano escolhido no espaço.
