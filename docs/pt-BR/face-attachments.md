# Esboços vinculados a faces

Novos esboços vinculados a faces armazenam o corpo de origem selecionado, sua operação criadora, um certificado
exato da face e os caminhos de ocorrências de origem e autoria. A origem e a orientação escolhidas
do esboço são armazenadas no plano canônico da face de origem. Editar a altura da origem
ou mover seu posicionamento, portanto, atualiza o plano do esboço, preservando sua geometria
local e suas dimensões.

Fontes planas suportadas são faces exatas de Caixa, tampas e laterais retas de esboço de Extrusão Novo corpo e União
sem inclinação e faces planas de importações STEP imutáveis. Tampas de extrusão também identificam seus
contornos externos e furos por IDs de curvas de origem. Reordenar perfis não pode substituir a tampa de outro
perfil. Um perfil desenhado sobre o contorno projetado de um esboço em face (um retângulo
traçado sobre a face inteira ou uma faixa cujos lados ficam nas arestas da face) é identificado pelas
curvas desenhadas: quando uma curva desenhada e uma projetada cobrem a mesma aresta, a curva desenhada
é a origem. Duas curvas desenhadas, ou duas projeções, sobre uma aresta continuam ambíguas. Superfícies curvas não podem receber um esboço plano por esse fluxo de trabalho.

Todas as outras faces planas são vinculadas por seu nome exato no histórico (um certificado `lineage`) quando
têm um: o nome persistente que o histórico de faces do núcleo lhes dá, que informa o que criou a face
e a acompanha em todas as operações posteriores. Isso abrange paredes e piso de um Corte
(`F5:side:l1`, `F5:top`), uma face de chanfro (`F7:blend:…`), cada cópia criada por padrão ou espelhamento
(`F4:top@F6.2`: a face original e a instância da cópia), uma parede lateral que União mesclou a partir de duas
operações (nomeada pela primeira, respondendo à outra) e as peças em que uma operação posterior dividiu
uma face (todas as peças de um nome em um plano orientado são essa face). O plano de uma face com origem
rastreada é seu plano exato atual, medido em um sistema de coordenadas que depende apenas do plano: sua
origem onde a origem da definição se projeta sobre ele, seu eixo x sendo o primeiro dos eixos
X, Y e Z da definição que não seja muito inclinado a ele (`|cos| < 0.6`), projetado. Os certificados de construção
acima continuam tendo prioridade; um certificado de origem é usado onde nenhum deles nomeia a face de forma única. Cópias
carregam o certificado de construção de sua origem, portanto um esboço vinculado à origem antes de sua
repetição em padrão continua sendo resolvido para a origem: entre as faces que carregam um certificado, a face da própria operação
produtora é aquela cujo nome não é de uma cópia.

Quando as peças de um nome deixam de compartilhar um plano (Deslocar face elevou uma peça de um topo que um
corte havia dividido), cada peça é vinculada por seu próprio nome: o da face, qualificado pelas faces
ao seu redor (`F2:top#[F2:side:l4,F4:side:l4]`). As operações de plástico (Ressalto, Rebordo, Suporte, Encaixe por pressão)
mantêm o histórico do corpo ao qual se aplicam e nomeiam suas próprias faces pela peça que criam
(`F4:post:top`, `F5:rest:side:0`, `F6:arm:top`), portanto elas também recebem vinculações. Uma lateral que uma extrusão ampliou para
sobrepor um corpo alinhado antes de uni-lo (suas vizinhas se alongaram além dos cantos do esboço) ainda é
nomeada pela linha do esboço ao longo da qual corre, portanto uma parede que União mesclou a partir das paredes de duas extrusões
se vincula pelo nome da primeira e responde ao da outra. Uma lateral desenhada sobre várias curvas
do esboço ao mesmo tempo (dois quadrados lado a lado deixam duas linhas coincidentes na aresta
compartilhada; vários perfis adjacentes extrudados juntos deixam uma parede ao longo de linhas
colineares) recebe os nomes de todas as curvas sobre as quais se encontra: a primeira na ordem do
esboço fornece seu nome, e as outras são aliases. O conjunto é determinado pelo esboço, portanto
um esboço extrudado para cima e para baixo e unido (um logotipo) recebe um esboço vinculado em
cada parede, além das faces superiores e inferiores.

Faces cujo histórico não é exato (faces que uma operação reconstruiu sem informar seu histórico),
fontes ambíguas e peças de uma face dividida intencionalmente com Dividir face (cada peça é sua
própria face) exigem, em vez disso, um plano fixo explícito. O erro explica o motivo, e o diálogo Criar esboço oferece **Usar plano fixo**,
que posiciona o esboço no plano atual da face sem acompanhar alterações posteriores. Uma
face próxima ou paralela nunca é usada automaticamente para reparar uma nova vinculação.

As faces planas de um corpo de chapa metálica são a exceção: não têm certificado de
construção, portanto um esboço em uma delas se vincula ao painel pelo nome (`sheet:<body>:<panel>:<side>`)
e o encontra novamente após cada reconstrução, desdobramento e redobra. As bordas de espessura (a espessura da chapa
ao longo de uma aresta reta de painel) se vinculam da mesma forma, pela aresta ao longo da qual correm
(`<panel>:rim.l3` para uma linha de esboço-base, `<flange>:rim.far`, `rim.side0`, `rim.side1`), assim como
as extremidades planas de uma dobra (`<flange>:rim.bend0`, `rim.bend1`); o plano de uma borda aponta para fora do
painel, com x ao longo da aresta e y através da espessura. Extrusão e Pressionar/puxar a partir de uma face
de chapa usam a mesma vinculação. Se esse painel deixar de existir, o
esboço informa uma vinculação perdida, como acima. Veja o guia de chapas metálicas.

## Criar e reparar

Selecione uma face plana suportada e crie um esboço. A seleção no desktop fornece o corpo
exato e o índice da face, incluindo seu caminho de ocorrências. Quem chama comandos pode usar
`sketch.create` com `plane: {face: [x,y,z], body: "world body key", index: face_index}`;
o ponto está em coordenadas globais. Uma solicitação de face apenas por ponto deve encontrar uma fonte suportada
única. Uma solicitação não suportada ou ambígua falha com uma explicação em vez de criar
uma vinculação com identidade mais fraca.

Quando uma fonte é suprimida, excluída, substituída ou perde seu certificado, o esboço informa
uma vinculação perdida. Seu plano salvo é apenas uma guia visível de reparo: ele não produz um
esboço resolvido válido nem permite que operações posteriores sejam construídas em uma face substituta.

Use **Redefinir plano do esboço** para selecionar explicitamente uma nova face suportada. O rascunho pode ser
revisado antes de aplicar; cancelar mantém a vinculação existente intacta. A nova vinculação
mantém a geometria existente do esboço, IDs e dimensões. Quando a fonte selecionada muda, seu
contorno anterior projetado automaticamente se torna geometria comum no novo plano;
ele não é substituído por um novo contorno projetado. Vincular novamente à mesma fonte certificada
mantém seu contorno ativo. Vínculos de projeção criados manualmente mantêm suas próprias fontes.

Escolha um plano fixo explícito para desvincular o esboço. Isso preserva sua geometria e dimensões
locais atuais e converte apenas seu contorno de face projetado automaticamente em
geometria comum. Vínculos manuais de projeção continuam vinculados às suas próprias fontes. Um novo
esboço em plano fixo não tem projeção automática ativa de face.

## Ocorrências, posições pendentes e arquivos

A ocorrência de origem selecionada e a ocorrência de autoria do esboço são identidades separadas.
Instâncias repetidas, portanto, não podem trocar silenciosamente uma vinculação para o primeiro posicionamento.
Um caminho de autoria ausente também exige reparo explícito. Nova vinculação e criação registram
posições pendentes apenas nos caminhos de origem e autoria envolvidos. Comandos que falham restauram
essas posições pendentes; desfazer restaura o estado anterior de vinculação e posicionamento.

O formato de arquivo 11 introduziu essas vinculações opcionais, que o formato 12 também preserva. O formato 16
adiciona o certificado `lineage` (leitores antigos solicitam uma atualização em vez de encontrar um tipo
desconhecido); âncoras salvas com certificados de construção são resolvidas como antes. O formato 17 adiciona nomes
de peças qualificados pelas faces ao redor e os nomes de bordas de espessura e extremidades de dobras de chapa metálica; 0.8.0
solicita uma atualização ao abrir um projeto de formato 17. Projetos existentes com referências a faces
sem uma vinculação mantêm sua geometria legada e comportamento de busca ao abrir; eles
não são atualizados silenciosamente. Redefinir esse esboço em uma face selecionada suportada cria
a vinculação mais forte. Geometria legada não suportada pode, em vez disso, ser mantida em um plano
fixo explícito. Vinculações importadas também vinculam o resumo exato dos dados STEP, a entidade de item e a
entidade de face: substituir os dados não pode reutilizar uma vinculação antiga apenas porque um índice
de face ou uma forma parece igual.

## Perfis gerados por comandos de modelagem de faces

Extrusão, Revolução e Pressionar/puxar podem gerar um esboço interno a partir de uma face plana selecionada. Fontes
suportadas usam a mesma vinculação certificada e o mesmo contorno ativo de um esboço vinculado comum.
Para faces modificadas sem certificado de construção, esses comandos mantêm seu comportamento histórico
de face nomeada e contorno vinculado por um caminho privado de compatibilidade. A captura inicial
usa o corpo e o índice de face selecionados. Esses esboços auxiliares não têm a garantia mais forte de
identidade salva de uma vinculação certificada. Esse caminho de compatibilidade não altera os
requisitos para criar um esboço comum vinculado a face.
