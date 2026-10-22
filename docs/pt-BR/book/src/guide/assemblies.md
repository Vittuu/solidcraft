# Componentes e juntas

Um projeto é uma árvore de **componentes**. **Novo componente** (`component.create`) adiciona um, e
**Criar componentes a partir de corpos** (`component.from_bodies`) transforma corpos em componentes. Cada
componente tem sua própria origem, seus corpos, esboços e entradas da linha do tempo. O **componente ativo**
(o botão de opção no navegador) recebe novos esboços e operações; os demais são desenhados
esmaecidos.

Um componente é posicionado por suas **ocorrências**. Colar cria outra instância do mesmo
componente, e **Colar novo** cria uma cópia independente. Mova uma ocorrência com a tríade de
movimento e depois use **Capturar posição** para mantê-la ou **Reverter** para retornar à posição anterior. O SolidCraft pergunta antes
do próximo comando se um movimento não foi capturado. **Fixar** prende uma ocorrência no lugar.

Colar novo copia definições de componentes aninhados, esboços, históricos de operações e parâmetros de cota
gerados. Instâncias repetidas de componentes filhos permanecem compartilhadas dentro da nova cópia. Materiais,
aparências e registros de juntas/movimento inteiramente internos recebem referências copiadas. Os parâmetros
do usuário válidos para todo o projeto permanecem compartilhados; vínculos externos mantidos e registros de movimento
que cruzam os limites e são omitidos são informados. Vínculos destrutivos não suportados impedem a cópia inteira. Veja
[detalhes de instâncias individuais e cópias](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/occurrence-workflows.md).

**Juntas** (`joint.create`, J) conectam dois componentes por origens de junta que se ajustam a faces, arestas,
círculos e vértices: rígida, de revolução, deslizante, cilíndrica, pino-ranhura, planar e esférica, com
deslocamentos, inversão e limites. **Junta na posição atual** mantém a posição atual, **Grupo rígido** fixa
componentes entre si, **Acionar juntas** move uma junta ao longo de seus valores, e **Vínculo de movimento**
acopla duas juntas. Vínculos podem formar cadeias (uma junta aciona outra, que aciona uma
terceira), e um valor percorre até 64 vínculos em qualquer ordem de criação. Conjuntos de contato impedem o movimento onde corpos atravessariam uns aos outros,
**Estudo de movimento** define valores de juntas em pontos de uma linha do tempo, e **Interferência** mostra volumes
sobrepostos em vermelho.

Juntas podem fechar um ciclo, como em um mecanismo de quatro barras ou biela-manivela.
Ao acionar uma junta, as outras acompanham para que todas continuem conectadas, e o mecanismo
permanece no modo de montagem inicial (um mecanismo aberto de quatro barras não muda para
o cruzado). Se o movimento ultrapassaria o ponto em que o ciclo ainda fecha, como acionar
um balancim além de sua amplitude, ele para na última posição válida e informa `stopped_by_loop`.
Um ciclo que não consegue fechar é listado em `conflicts` e nada nele se move. `joint.list`
informa quantos graus de liberdade cada peça ainda tem com os ciclos fechados: as peças
móveis de quatro barras compartilham um, e um ciclo travado por junta rígida não tem nenhum.
O diálogo Acionar juntas informa quando um acionamento para antes e por quê. Na prévia,
uma linha sob os valores mostra o valor alcançado e o motivo (o ciclo que não fecha além
dali, ou as duas peças separadas pelo contato); após OK, a linha de estado o repete.
A caixa de valor mantém o que você digitou. O diálogo abre na junta selecionada no navegador ou,
quando nenhuma está selecionada, na primeira junta que se move.

O solucionador posiciona o que alcança a partir da raiz e dos componentes fixados. Uma cadeia de
juntas que não se conecta a nada fixado nunca é posicionada; acionar uma de suas juntas não move
nada. O acionamento informa isso: `joint.drive` responde `moved: false`, com um `reason` e a peça
`unconnected` ("Pedestal:1 is not connected to anything grounded"); o diálogo Acionar juntas mostra
a mesma informação, `motion.play` lista essas juntas em `unmoved`, e `joint.list` identifica cada
cadeia em `floating`. Fixe uma peça da cadeia ou conecte-a a uma peça fixada (uma junta rígida na
posição atual preserva seu posicionamento), e ela passa a se mover.

`joint.drive` retorna os valores da junta como `joint.list`: em graus e milímetros, com unidades e
limites; `moved` informa se alguma peça se moveu. Por padrão, as origens se acoplam como duas faces
que se encontram: o eixo z de B fica oposto ao de A. Com `flip: true`, os eixos se alinham, de modo
que duas origens no mesmo ponto e com o mesmo z permanecem como estão. Em scripts com muitos
passos, `quiet: true` omite os graus de liberdade por peça das respostas de juntas; em
`motion.play`, `quiet: true` retorna as poses apenas das peças que se moveram.

**Inserir projeto** (`component.insert_design {path, translate?}`) adiciona um componente vinculado a
outro projeto salvo, de modo que cada peça possa permanecer em seu próprio documento. Os corpos sólidos da peça são
inseridos como estão nesse projeto e são mantidos na montagem, para que ela abra mesmo sem
o arquivo da peça. O vínculo é armazenado em relação à pasta da montagem depois que a montagem é salva.
Após uma mudança no arquivo da peça, **Atualizar vínculos** (`component.update_links`) o lê novamente.
**Romper vínculo** (`component.break_link`) mantém os corpos e descarta a referência ao arquivo, e
`component.links` lista os vínculos e informa se os arquivos foram encontrados. Um componente vinculado contém
apenas os corpos da peça: a linha do tempo, os parâmetros e os dados de chapa metálica (planificação, tabela de
dobras) permanecem no projeto da própria peça.

A exportação STEP grava a árvore de componentes como uma montagem.

Clique com o botão direito em um componente no Navegador e escolha **Propriedades** para inspecionar
massa considerando os materiais, volume, área de superfície, centro de massa e limites globais. Descendentes e
instâncias repetidas contribuem para os totais da montagem; movimentos pendentes são incluídos. Um componente
usa seu primeiro posicionamento; `occurrence.properties` seleciona uma instância específica. As Propriedades da raiz
medem a montagem completa. Os volumes são somados, incluindo corpos sobrepostos, em vez de
medir sua união. O mesmo fluxo está disponível por `component.properties` e
`occurrence.properties`; veja
[detalhes de posicionamento e medição](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/assembly-properties.md).
