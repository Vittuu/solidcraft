# Esboços e restrições

Um esboço fica em um plano: um plano de origem, um plano de construção ou uma face planar. Inicie um com
**Criar esboço** (`sketch.create`) e clique em um plano, ou escolha primeiro uma face. **Concluir esboço**
retorna ao modelo.

Em scripts, `XY`, `XZ`, `YZ` e seus aliases `Top`, `Front`, `Right` nomeiam planos de origem.
Esses nomes são reservados: um plano de construção não pode ser criado com um deles nem renomeado para um deles
(sem distinção entre maiúsculas e minúsculas). Se um projeto antigo já tiver um plano de construção com esse nome, um
argumento de texto que o nomeie é ambíguo e será rejeitado; use o outro alias (`XY` para `Top`)
para o plano de origem, o ID numérico da operação de construção para selecioná-la, ou um plano explícito
`{origin: [x,y,z], normal: [x,y,z]}`. Referências salvas existentes preservam seu significado;
abrir um projeto não reinterpreta esboços antigos.

As curvas se adaptam ao zoom atual e à escala de exibição. A seleção e o realce usam o mesmo
contorno visível, incluindo círculos pequenos na vista Ajustar e esboços em componentes rotacionados.
O sombreamento dos perfis acompanha essas curvas e preserva os furos. Aplicar zoom muda a exibição
sem alterar a geometria do esboço, suas cotas ou o histórico de desfazer.
Em janelas baixas ou telas com escala ampliada, as opções da Paleta de esboço rolam enquanto seu
estado de restrições e o botão **Concluir esboço** permanecem visíveis.

## Geometria

**Ponto** está disponível diretamente no painel **CRIAR** da barra de ferramentas padrão de Esboço. Clique nele e depois
clique no esboço para posicionar um ponto. Personalizações existentes da barra são mantidas; use
**Restaurar barra de ferramentas** no menu de contexto da barra para restaurar os botões padrão.

Linhas e polilinhas (L), retângulos (dois pontos R, três pontos, centro), círculos (centro C,
dois pontos, três pontos, tangente), arcos (três pontos, centro, tangente), polígonos, ranhuras, elipses,
splines por pontos de ajuste e pontos de controle, cônicas, pontos e texto. **Construção** (X) transforma curvas
em geometria de referência que não forma perfis. As ferramentas de modificação incluem aparar (T), estender, quebrar,
arredondamento, chanfro, deslocamento (O), espelhamento, padrões, mover/copiar e escala.

**Linha infinita** prolonga uma linha nas duas direções pela vista inteira, passando por seus dois pontos.
Ela é geometria de construção e não forma perfis. Em scripts, `sketch.infinite` recebe `curves`
(uma lista de IDs de linhas) e `value`: `true` ativa o tipo infinito, `false` restaura uma linha normal
finita; omitir `value` alterna o tipo.

A opção **Geometrias de construção** da Paleta de esboço oculta curvas de construção enquanto você
trabalha. Uma curva de construção oculta não pode ser selecionada por clique nem por caixa. Nada invisível
é selecionável. A opção pertence a um esboço e é salva com o projeto. Linhas de centro têm
um tipo de linha próprio e continuam visíveis. Geometria projetada com tipo de linha de construção,
como o traço de um plano, responde à opção **Geometrias projetadas**. Essa
opção altera apenas a exibição. Curvas de construção já estavam fora de todos os perfis.

Ferramentas de polígono mostram o contorno das arestas enquanto o segundo ponto é posicionado, para conferir a orientação
antes de concluir. Polígonos inscritos e circunscritos preservam seu círculo de
apoio e atualizam o contorno quando o valor **Lados** na entrada direta muda.

**Texto** usa a fonte integrada (Liberation Sans), a menos que outra seja fornecida: `sketch.text` e
`sketch.edit_text` recebem `font`, o nome da família ou o nome completo de uma fonte instalada ("DejaVu Sans"), ou o caminho
de um arquivo `.ttf` / `.otf`, e o diálogo **Editar texto** tem um campo **Fonte** para isso. Uma fonte desconhecida é um
erro, e a fonte é mantida com o texto no projeto, para que o texto continue editável nela. `SOLIDCRAFT_FONT_DIRS`
adiciona pastas àquelas pesquisadas para encontrar fontes instaladas. Os mesmos dois comandos recusam `text` que não seja
uma string, `at` que não seja composto por dois números e `height` ou `angle` que não sejam números (ou `height` que não seja
positivo), com uma mensagem, em vez de desenhar o texto com um valor padrão; quando omitidos, os parâmetros mantêm seus padrões.

Clique com o botão direito no interior preenchido ou no contorno de uma letra e escolha **Editar texto**. Você também pode
iniciar **Editar texto** primeiro e clicar na letra. Todos os contornos pertencentes a esse texto são
realçados enquanto o diálogo está aberto. Um perfil delimitado por várias fontes de texto ou outras
geometrias não pode identificar um único texto: use **Selecionar outro** para escolher seu contorno.
Altere o texto, a fonte, a linha de base X/Y, a altura ou o ângulo e confirme com **OK**; **Cancelar**
mantém o texto original. Posição e altura usam as unidades do documento.

Um texto pode seguir uma curva de seu esboço (seu **Caminho**). Essa curva vira construção:
as letras se apoiam nela e suas descendentes a atravessam; como curva normal, ela as cortaria
em pedaços e selecionar o texto escolheria esses pedaços. Retirar o texto da curva a mantém
como construção; **Normal/Construção** a restaura.

Arraste um contorno de texto para mover todas as letras juntas. Solte para concluir em uma etapa de desfazer,
ou pressione **Esc** durante o arraste para restaurar o texto original. O texto continua editável.
Para um deslocamento exato, selecione um contorno e escolha **Mover/copiar** em seu menu de contexto.
Informe **Distância X**, **Distância Y** e um **Ângulo** opcional em torno da linha de base do texto;
comprimentos aceitam as unidades do documento ou unidades explícitas, como `1 in`. Ative **Criar cópia** para
manter o original no lugar e criar um texto editável separadamente. **Cancelar** deixa o
projeto inalterado. Geometria projetada continua protegida contra arrastes comuns.


**Projetar** (P) e **Interseção** trazem arestas do modelo para o esboço como geometria vinculada que
acompanha mudanças posteriores. Criar um esboço em uma face projeta suas arestas automaticamente.
Uma aresta projetada continua sendo a aresta escolhida, onde quer que uma mudança posterior a mova; quando essa
aresta deixa de existir, a projeção mantém sua última forma e o esboço informa que ela foi perdida.
Para Projetar, selecione curvas ou pontos de origem visíveis de outro esboço, arestas do modelo,
faces, vértices ou corpos, ou eixos e planos, e pressione P. Toda a seleção suportada
é projetada em uma etapa de desfazer. Uma projeção rejeitada mantém a seleção para correção;
uma seleção vazia ou mista inicia a ferramenta comum de seleção. Seleções de esboços externos
preservam seu esboço de origem, mesmo quando o esboço ativo usa os mesmos nomes de pontos ou curvas.

Um esboço em um plano de construção criado a partir do modelo (por uma linha de esboço, por três
pontos, a meio caminho entre duas faces…) projeta, faz interseções, inclui geometria e contém texto como um esboço
em qualquer outro plano, e seus vínculos acompanham o plano quando um parâmetro o move.

Executado por um script ou pelo canal de controle, cada comando que adiciona geometria ao esboço retorna os
IDs do que adicionou (`created: {points, curves, constraints, dimensions}`). Os IDs são estáveis e
nunca reutilizados dentro de um esboço, mesmo após uma exclusão, permitindo que um script aplique cotas ou restrições à nova
geometria imediatamente.

Estender encontra interseções com elipses exatas, cônicas racionais e trechos de spline. Ele preserva o
suporte original da linha ou do círculo e estende apenas além da extremidade selecionada; restrições
conflitantes ou uma sobreposição ambígua deixam o esboço inalterado.

Cadeias convexas fechadas de linhas e arcos circulares tangentes aceitam deslocamentos para dentro que consomem
um canto arredondado. Quando o deslocamento ultrapassa o raio do canto, os suportes retos
vizinhos se encontram em um canto vivo. As linhas sobreviventes e os IDs de extremidades permanecem estáveis; os
arcos retirados retornam com seus IDs originais se uma edição posterior restaurar seu raio.
Seleções de perfil de contorno completo acompanham essa transição ao salvar e reabrir, para que um
lábio extrudado possa continuar associativo. Deslocamentos analíticos fechados existentes e elegíveis são atualizados
sem alterar sua geometria salva ou seus IDs.

Essa transição exige um contorno convexo fechado com junções tangentes e uma região
não vazia certificada. Um arco consumido, referenciado individualmente por uma restrição ou outro
deslocamento, não pode ser retirado. Criar outro deslocamento a partir de um contorno já com cantos vivos e arcos retirados também
exige romper primeiro sua associação. Cadeias abertas, não convexas e mistas de forma livre mantêm
seu comportamento existente. Romper associação de deslocamento congela o contorno atual, incluindo
centros privados dos arcos e indicadores de construção, para edição independente.

Deslocamentos de elipses, cônicas e splines suaves acompanham sua fonte após alterações de cotas,
arrastes e reabertura de um projeto. Sua spline cúbica armazenada tem um limite de erro da distância normal
de 0.002 mm ao longo de todo o trecho. Tangentes singulares, cúspides de curvatura, possíveis auto-interseções e
subdivisões excessivas são recusadas. Desloque essas curvas individualmente: junções de cadeias mistas
exigem atualmente geometria separada. `sketch.offset.break` libera um deslocamento para edição
independente, e desfazer restaura sua associação. Deslocamentos antigos amostrados de forma independente permanecem independentes.
Deslocamentos abertos suaves e deslocamentos fechados suaves recém-criados podem ser deslocados novamente, incluindo refinamento
limitado após uma edição da fonte. Novas saídas fechadas usam um ajuste cúbico periódico com tangente
e curvatura contínuas na emenda. Romper associação de deslocamento preserva essa interpolação,
e seu ponto de fechamento permanece unido ao ser arrastado. Excluir uma saída também remove seus
pontos privados de ajuste sem uso; excluir uma fonte com um dependente mantido é recusado.

Para splines fechadas por ajuste explícitas, `sketch.spline.fit_point` aceita `periodic: true` e pelo
menos três amostras cíclicas distintas. Ele fecha o laço automaticamente. Chamadas existentes e
splines fechadas salvas anteriormente preservam a interpolação natural; deslocamentos fechados associativos antigos
também preservam seu ajuste natural salvo. Uma emenda legada sem normal única ainda recusa
outro deslocamento. A intenção periódica é armazenada em cada curva e permanece ao salvar, reabrir e romper a associação.
Ao desenhar uma spline por ajuste, clique no ponto inicial depois de três amostras distintas para fechá-la
suavemente. O botão direito conclui um ajuste natural comum. Deslocamento aceita tanto alças de elipse
quanto curvas densas ajustadas; o segundo clique define o lado e a distância à curva.

Ao desenhar em um esboço planar, **Construção** (X) muda o tipo de linha sem
encerrar a ferramenta nem perder pontos escolhidos e valores de entrada direta. A prévia e a indicação ao passar o cursor
mantêm visíveis os traços de construção e os padrões traço-ponto das linhas de centro. Com curvas selecionadas,
Construção altera essas curvas como antes.

Ative **Esboço 3D** na Paleta de esboço para desenhar linhas, splines por ajuste e pontos fora do plano.
Ao posicionar esses pontos, você pode ajustá-los a pontos de outros esboços visíveis. Para uma linha de
ligação, inicie Linha e clique em uma extremidade em cada esboço de origem; cada extremidade usa seu plano
de origem, mesmo quando esse plano difere do esboço ativo. Esses ajustes capturam posições;
eles não criam vínculos associativos com os pontos de origem.
Nova geometria 3D acompanha o posicionamento do componente de seu esboço, incluindo a ocorrência selecionada
e alterações pendentes de posição. Clique em um ponto de ajuste desenhado para mostrar sua tríade de movimento; as setas movem
ao longo dos eixos globais X, Y e Z. Scripts também fornecem coordenadas globais a `sketch.line3d`,
`sketch.spline3d`, `sketch.point3d` e `sketch.move3d`. Ative a ocorrência desejada antes
de editar um esboço explícito pertencente a um componente com vários posicionamentos.

Fios 3D antigos desenhados ou desvinculados mantêm suas posições globais salvas. Fios incluídos e vinculados a superfícies
com um marcador explícito de componente mantêm esse significado depois que o vínculo é rompido. Arquivos antigos
cujos vínculos se perderam após uma atualização entre componentes com escopo incorreto podem exigir um
novo vínculo quando a fonte estiver disponível; seus valores de pontos armazenados são preservados ao abrir.

## Restrições e cotas

**Horizontal/vertical** aceita um clique em uma linha ou cliques em dois pontos distintos do esboço,
incluindo a origem, extremidades e centros de círculos. Para dois pontos, escolhe alinhamento horizontal
ou vertical a partir de suas posições atuais. Esc cancela um primeiro ponto pendente.

O solucionador é próprio do SolidCraft (Gauss–Newton amortecido com análise de graus de liberdade). Restrições:
coincidência, horizontal/vertical, paralelismo, perpendicularidade, tangência, igualdade, concentricidade, colinearidade,
ponto médio, simetria, fixação, curvatura (G2) e polígono. **Cota de esboço** (D) adiciona cotas lineares,
alinhadas, horizontais, verticais, de raio, diâmetro e ângulo. Cada uma é um parâmetro
(`d1`, `d2`, …), podendo conter uma expressão.

Pressione D, clique no que deseja medir e digite o valor: assim que o clique define a
cota (uma linha, círculo ou arco, ou o segundo ponto ou linha de uma distância ou ângulo), sua
caixa de valor recebe o foco do teclado com o valor medido selecionado, e a digitação o substitui. Números,
unidades (`1 in`), expressões (`2*width`) e nomes de parâmetros funcionam; letras vão para a caixa,
sem acionar atalhos de ferramentas. Enquanto você digita, o texto da cota acompanha o cursor, e Enter o posiciona
ali (ou em sua posição padrão se você não moveu o cursor) em uma etapa de desfazer. Esc
cancela a cota.

Você ainda pode posicionar o texto primeiro. Antes de digitar, um clique em um espaço vazio do esboço coloca o
texto ali e mantém o foco na caixa de valor; um clique em outro ponto ou linha transforma a
cota em uma distância ou ângulo; Tab deixa os elementos escolhidos aguardando um clique para posicionar o
texto.

Entre dois pontos, ou em uma linha, que não estejam alinhados com um eixo, a posição escolhida para a
cota determina o que ela mede: na direção entre eles, a distância entre eles; acima ou abaixo
deles, a distância horizontal; ao lado deles, a distância vertical.

Uma cota que o esboço não pode receber, porque a geometria já está fixada por outras restrições
e cotas, é adicionada como cota de referência: ela aparece entre
parênteses, mede o esboço e não tem um valor editável.

Para **Coincidência**, escolha dois pontos, um ponto e uma curva (em qualquer ordem), ou duas
linhas retas distintas. Duas linhas unem o par de extremidades mais próximo; a primeira
linha escolhida permanece no lugar quando as restrições existentes permitem. Escolha as extremidades
explicitamente quando quiser conectar outro par.

Horizontal/vertical também se aplica imediatamente a uma linha selecionada ou a dois pontos distintos
selecionados. Use Shift+clique para adicionar o segundo ponto antes de escolher a restrição.

Para Paralelismo, Perpendicularidade, Igualdade, Tangência, Concentricidade e Colinearidade, selecione exatamente duas
curvas do esboço (Shift+clique para adicionar a segunda) e escolha a restrição para aplicá-la imediatamente.
Você também pode iniciar a restrição primeiro e depois escolher as curvas. Uma seleção incompleta ou mista
deixa a ferramenta pronta para a seleção comum.

Coincidência aceita uma seleção exata de dois pontos ou de um ponto e uma curva. Ponto médio
aceita um ponto e uma linha, em qualquer ordem de seleção. Simetria aceita dois pontos distintos
e uma linha de simetria, independentemente de onde a linha apareça na seleção.
Também pode usar a origem ou outro ponto como centro de simetria: cada par é unido por uma
linha de construção cujo ponto médio é esse centro. Em scripts, `sketch.constraint.symmetry`
recebe `point: "origin"` (ou o ID de outro ponto); `line: "x"` ou `"y"` usa um eixo do próprio
esboço, salvo quando uma curva já tiver esse ID. Dois círculos ou arcos têm centros simétricos
e raios iguais; para duas linhas, as extremidades correspondentes se tornam simétricas.
Restrições bem-sucedidas limpam a seleção; restrições rejeitadas a preservam.

Geometria com restrições insuficientes é azul e geometria totalmente restringida fica escura; o estado
mostra os graus de liberdade restantes. Uma restrição ou cota que deixaria o
esboço com restrições excessivas é recusada em vez de aplicada. Arraste um ponto para movê-lo; o solucionador mantém todas as
restrições durante o arraste.

## Perfis

Laços fechados viram perfis automaticamente, incluindo regiões divididas por linhas que se cruzam e
ilhas dentro de contornos. Extrusão, revolução, varredura e transição selecionam perfis.

Uma operação lembra um perfil escolhido pelas curvas ao redor dele, e não pelo local clicado:
`"profiles": [{"curves": ["l1", "l2", "a1", "l3"]}]` (ou `[{"curves": ["c1"]}]` para um círculo)
seleciona a região cujo laço externo é formado exatamente por essas curvas, ou pelos trechos delas que
outras curvas recortam. Ela é encontrada novamente a cada recálculo, para que a operação mantenha a mesma região
quando um parâmetro move a geometria. Ilhas dentro da região são seus furos. Quando nenhuma região, ou
mais de uma (ambas as metades de um círculo cortado por uma linha), é delimitada exatamente por essas curvas, a
operação informa um erro em vez de usar outra região. Perfis escolhidos por índice ou ponto
(`[{"point": [x, y]}]`) são armazenados da mesma forma quando a operação é criada.

## Unidades de medida

Altere o projeto atual por **Configurações do documento > Unidades** no navegador.
A preferência de unidades se aplica a novos projetos. Anotações existentes de esboço, comprimentos
exibidos durante o desenho e Medir acompanham as unidades do projeto atual sem redimensionar a geometria.
Polegadas são exibidas com pelo menos três casas decimais e preservam um quarto dígito significativo;
uma precisão maior de exibição pode ser selecionada nas Preferências. Valores de comprimento sem unidade digitados
na ferramenta de cotas usam as unidades do projeto atual. Unidades explícitas, como `25.4 mm`,
continuam válidas em um projeto em polegadas. Cotas angulares mantêm suas unidades angulares. Literais em polegadas digitados deliberadamente com quatro casas
preservam zeros finais. Controles de distância de componentes/imagens de referência e medições de inspeção, biblioteca de peças
e planificação também acompanham as unidades do projeto; expressões de parâmetros e nomes de padrões
métricos mantêm as unidades explícitas com que foram definidos.

### Revisando resultados de espelhamento, padrão e deslocamento

Espelhar, Padrão retangular/circular e Deslocamento mostram a etapa de seleção ativa na
janela compartilhada dessas ferramentas. Clique em uma entrada para alterá-la; clique novamente em um
objeto selecionado ou use Ctrl/Cmd-clique para removê-lo. Cada entrada tem seu próprio botão Limpar.
As entradas de origem mostram uma contagem, e referências únicas identificam a linha de espelhamento,
a direção ou o centro escolhidos. A seleção por caixa também funciona enquanto a janela está aberta.
Ferramentas de modificação de esboço usam a geometria do esboço que está sendo editado.

A geometria proposta do esboço aparece em **verde** antes da confirmação. Altere as entradas ou os
valores para atualizar a prévia e pressione **OK**. **Cancelar** descarta a prévia. O centro de um
padrão circular pode ser um ponto existente ou uma posição clicada em uma área vazia do esboço.

Padrões de esboço mantêm cada instância ligada à geometria de origem: mover a origem atualiza
as cópias, e uma cópia não pode se deformar independentemente. Raios de círculos e elipses
acompanham a origem. Escolha o centro de um círculo para manter o padrão circular centrado
nele. Um centro em espaço vazio vira um ponto de esboço que aceita cota ou coincidência.
Clique duas vezes no indicador do padrão (no centro circular ou ao lado da origem retangular),
ou clique com o botão direito na origem/cópia e escolha **Editar padrão de esboço**, para
alterar quantidade, espaçamento, direção ou ângulo total. Edições confirmadas preservam IDs
de instâncias sobreviventes; Cancelar descarta a prévia. Configurações sobrevivem a salvar/
reabrir e desfazer/refazer. Excluir uma origem, cópia ou relação rompe a associação e deixa
a geometria restante independente. Padrões de versões antigas continuam como cópias
independentes; recrie-os para acrescentar associações.

As distâncias de deslocamento são cotas controladoras editáveis: clique duas vezes em uma cota ou
altere seu parâmetro de modelo para redimensionar o deslocamento sem mover as curvas de origem.
O lado selecionado, a distância e a associação são preservados ao salvar/reabrir e desfazer/refazer.
Deslocamentos simétricos compartilham uma distância controladora; Dois lados tem uma distância
independente para cada lado. Todas as curvas ou cadeias desconectadas selecionadas são deslocadas.
Romper uma associação mantém sua geometria atual e remove a cota que controla essa associação.
Deslocamentos antigos mantêm suas distâncias fixas salvas.
