# Operações de sólidos

As operações ficam na **linha do tempo** e são reconstruídas de forma incremental a partir da primeira que mudou.
Cada uma pode ser editada (clique duplo), suprimida, renomeada, reordenada (se nenhuma de suas dependências
for movida para depois dela), usada como ponto de retorno e excluída. A exclusão pede confirmação quando outras operações dependem
dela.

| Painel | Operações |
|---|---|
| Criar | extrusão (um lado, dois lados, simétrica, inclinação, deslocamento inicial, até uma distância ou através de tudo; novo corpo, unir, cortar, interseção; paredes finas ao longo de curvas), revolução, varredura, transição, caixa, cilindro, esfera, toro, hélice, tubo, nervura, reforço, relevo, furo (simples, rebaixado, escareado, roscado; em faces ou pontos de esboço), rosca, padrões retangulares/circulares/ao longo de caminho, espelhamento |
| Modificar | pressionar/puxar, arredondamento, chanfro, casca, ângulo de saída, escala, combinar, dividir corpo, deslocar face, substituir face, mover/copiar, alinhar, remover |
| Construir | plano deslocado, plano em ângulo, plano ao longo de caminho |
| Inspecionar | medir, análise de seção, interferência, pente e mapa de curvatura, zebra, ângulo de saída, raio mínimo, isocurvas, centro de massa |

A extrusão seleciona automaticamente **Unir** para adições quando o componente ativo já tem um
sólido, incluindo conexões laterais em cotovelo e ressaltos para parafusos que encontram apenas parte de uma face.
Extrudar para dentro do material existente seleciona **Cortar**. Em um componente vazio, seleciona **Novo corpo**;
escolha **Novo corpo** explicitamente para manter uma adição separada. Alterar o padrão não modifica
operações já salvas em um projeto.

A inclinação dá ângulo às laterais de qualquer perfil, incluindo texto de esboço: linhas, arcos e curvas spline
ou cônicas se movem juntas para dentro ou para fora, e uma ponta plana menor que a inclinação se fecha em um ponto.
Em laterais spline e cônicas, a inclinação é aproximada: o contorno deslocado é uma curva
do mesmo tipo a até 2 % do deslocamento da inclinação (ou 10 µm) do contorno exato; curvas
que viram demais são antes divididas em trechos. Onde a curva se dobra com raio menor que
o deslocamento, o contorno dobra sobre si e é suavizado. Uma inclinação maior que os detalhes
do perfil (um traço, uma curva ou uma abertura mais estreita que ela) é recusada com esse
motivo; uma inclinação ou distância menor é construída.

**Extrusão fina** (`thin: true` em `solid.extrude`, a caixa Extrusão fina no diálogo) constrói
uma parede de espessura constante ao longo das curvas do esboço em vez de preencher perfis:
cadeias abertas (uma parede) e contornos fechados (um anel), linhas, arcos, círculos, elipses
e splines. No diálogo, um perfil selecionado representa as curvas ao seu redor (contorno e
furos). A parede fica centrada nas curvas, dentro, fora ou dos dois lados com duas espessuras
(`side`: `center`, `inside`, `outside`, `two_sides` com `thickness2`). O interior de um contorno
fechado é a área cercada; o de uma cadeia aberta é o lado para o qual ela se curva (dentro
de um U), e o de uma linha reta é a esquerda da primeira curva como desenhada. Curvas unidas
pelas extremidades formam uma cadeia com cantos em meia-esquadria; paredes que se cruzam
ou encontram (anel e cruz, colmeia) formam uma peça. Todos os limites, operações e destinos
de extrusão se aplicam, e inclinação para paredes de linhas retas ou círculos completos.
Uma parede mais espessa que o raio interno da curva é recusada. Paredes spline seguem
o deslocamento exato com erro máximo de 0.0001 mm.

Editar uma extrusão preserva suas distâncias com sinal, entradas De objeto/Até objeto, expressões de inclinação
e corpos de destino explícitos. Edições de revolução preservam os eixos globais, de esboço e de linha explícita, ambas as
expressões de ângulo e os destinos Até objeto. Uma seleção **Todos os perfis** que não foi alterada continua
incluindo perfis adicionados depois. Cancelar restaura a linha do tempo completa; uma edição bem-sucedida é
uma única etapa de desfazer, incluindo alterações de parâmetros posteriores. Editar um componente posicionado mostra a prévia da
operação editada antes de seu histórico posterior. As guias de perfil de Relevo cilíndrico acompanham o
sistema de coordenadas da ocorrência escolhido para o esboço.

Perfis adaptativos selecionados recentemente armazenam as identidades completas das fontes do contorno externo e dos furos quando
todo o contorno pode ser qualificado. Alterar uma elipse ou um deslocamento periódico após a extrusão pode,
então, mudar a quantidade de trechos de spline e manter a mesma região de material selecionada. Um contorno parcial,
intersectado ou ambíguo não pode atender a essa referência de fonte completa; alterar as
identidades dos furos também faz a referência se perder. Seleções antigas de trechos de curvas preservam os multiconjuntos exatos
armazenados. Se uma operação antiga já perdeu seu perfil após uma mudança de subdivisão, edite
a operação e selecione novamente a região atual pretendida para capturar a nova identidade.

Novas extrusões simétricas de sólidos tratam a distância informada como a espessura total e a dividem igualmente
em torno do plano inicial: 10 mm vão de -5 a +5 mm em um esboço XY. Alternar de Um lado para
Simétrica mantém a espessura total informada. Através de tudo continua alcançando destinos nos dois
sentidos, e Dois lados mantém suas distâncias independentes. Extrusões simétricas salvas antes
dessa mudança mantêm suas distâncias originais por lado ao reabrir ou editar. Selecionar outra direção
no editor da operação usa a convenção atual de distância total.

Novas revoluções simétricas dividem o ângulo informado igualmente ao redor do perfil do esboço, independentemente
do sentido do eixo selecionado. Dois lados mede o segundo ângulo para trás e o
primeiro para frente ao longo da direção de varredura normal ao esboço. Perfis em lados opostos de
um eixo usam, cada um, sua própria direção de varredura. Projetos salvos antes do formato 11 mantêm seu posicionamento
angular original ao abrir, editar ou copiar; criar outra revolução usa o novo comportamento.
Até objeto segue essa mesma direção para cada perfil selecionado, incluindo faces de destino curvas.
Escolha um destino afastado do eixo para que sua posição angular fique definida.

Para **Relevo** cilíndrico, selecione uma face cilíndrica externa suportada e um perfil de esboço
tangente. **Baixo relevo** corta radialmente a parede. Perfis de linhas ortogonais, incluindo furos e
contornos côncavos, envolvem o cilindro com paredes radiais exatas; o perfil deve ficar afastado de emendas e extremidades.
Cilindros completos e luvas coaxiais são suportados. Capture as posições pendentes dos componentes
antes de aplicar. A operação armazena os caminhos das ocorrências do destino selecionado e do esboço de origem, de modo que edições
e reabertura preservem as instâncias escolhidas. Outras superfícies curvas e perfis de forma livre continuam
sem suporte. Um novo Relevo sem face explícita infere um único cilindro elegível suportado;
selecione a face quando o destino for ambíguo. Operações antigas salvas sem face preservam seu
posicionamento planar.

Um **Furo** com posicionamento **Único** perfura onde você escolhe uma face planar. Escolher um ponto sobre ou perto de um
ponto de um esboço visível que esteja nessa face, como o centro de um círculo, posiciona o furo
exatamente nesse ponto, como no posicionamento **Múltiplo**; **Selecionar outro** (pressionar e segurar) também
oferece a face no local clicado. Cantos de contornos de texto não contam, nem pontos projetados
enquanto o esboço oculta geometria projetada. O furo mantém esse local na face; para que
os furos acompanhem edições posteriores do esboço, posicione-os com **Múltiplo**.

Arredondamentos e chanfros aceitam arestas retas entre faces planares, laços suaves completos, sequências de
arestas vizinhas com cantos em meia-esquadria ou esféricos e cadeias fechadas de arestas curvas (uma esfera
rolante). Veja as limitações conhecidas no roteiro para os demais casos.

**Curvatura (G2)** em **Tipo de raio** no diálogo Arredondamento cria uma transição de arestas retas entre
duas faces planas com uma seção quíntica exata. O plano tangente e a curvatura correspondem a cada
face lateral: a curvatura chega a zero nas duas linhas de contato, ao contrário de um arredondamento
circular G1. Cada extremidade deve encontrar uma única face plana perpendicular à aresta. Arestas/faces
curvas, extremidades inclinadas ou com várias faces e transições de canto que se intersectam não são
suportadas; elas retornam um erro em vez de recorrer a G1. **Raio equivalente** define o mesmo recuo
até o contato que um arredondamento circular (para um ângulo reto, o recuo é igual ao valor informado);
ele não é um raio de curvatura constante. Reduzir/Inclinar para caber não estão disponíveis para esse tipo.

Scripts e MCP usam o mesmo comando, por exemplo em uma caixa de 20 mm:
```json
{"command": "solid.fillet", "params": {"edges": [[20, 0, 10]], "radius": 2, "type": "curvature"}}
```
`type: "g2"` é um alias. Arredondamentos existentes continuam usando, por padrão, transições circulares G1 de raio constante.

As referências de operações a faces e arestas são armazenadas por nome, de modo que uma edição anterior que mova a geometria
as mantenha na face correta (veja [Nomenclatura persistente](../dev/naming.md)). Quando uma referência não pode
ser encontrada, a operação avisa, em vez de escolher silenciosamente outra face.

**Plano ao longo de caminho** aceita uma curva de esboço (incluindo um fio 3D desenhado) ou uma aresta do modelo.
Escolha o caminho na área de visualização e informe **Posição (0 a 1)**: 0 é o início do caminho, 1 é o fim,
e 0.5 posiciona o plano na metade de seu comprimento. O plano é perpendicular ao
segmento local do caminho. A prévia não altera o projeto; OK cria uma operação editável na linha do tempo.
O plano acompanha edições posteriores do caminho. Caminhos curvos usam sua polilinha amostrada; distância absoluta,
posicionamento por objeto de referência e cadeias de várias curvas ainda não estão disponíveis nesse diálogo.

## Editando uma operação por um script

Para `solid.pattern.circular`, `axis` aceita `X`, `Y`, `Z` ou `{origin, dir}`.
Uma origem explícita deve conter três coordenadas numéricas finitas em milímetros
(`[x, y, z]` ou `{x, y, z}`); omiti-la usa a origem global. Expressões e
referências a eixos de construção não são suportadas. Origens inválidas são rejeitadas, em vez de
serem silenciosamente substituídas por zero.

`timeline.inspect {feature}` retorna a `definition` de uma operação: os campos alterados por `timeline.edit`,
com seus valores atuais. `timeline.edit {feature, set}` mescla `set` nessa definição
(objetos são mesclados campo a campo; qualquer outro valor substitui o anterior), reconstrói a operação e
informa `recomputed` e `restored`. O tipo da operação não pode mudar, e a geometria do esboço é
editada com os comandos de esboço. Expressões são texto (`"30"`, `"30 mm"`, `"width / 2"`,
`"45 deg"`); um número sem unidade é um comprimento nas unidades do projeto, ou graus para um ângulo.

```json
{"command": "timeline.inspect", "params": {"feature": "Slab"}}
→ {"type": "ExtrudeFeature",
   "definition": {"sketch": 1, "profiles": {"type": "all"}, "operation": "new_body",
                  "extent": {"distance": "10", "direction": "positive"}},
   "inputs": [{"label": "Distance", "param": "d1", "expression": "10", "unit": "mm"}], …}
{"command": "timeline.edit", "params": {"feature": "Slab", "set": {"extent": {"distance": "12"}}}}
```

`inputs` lista as entradas nomeadas da operação. `parameters.change {name: "d1", expression}` define uma
delas diretamente. Campos opcionais não definidos não aparecem na definição; ainda
podem ser adicionados por `set`. Os campos das operações comuns:

| Operação | Campos |
|---|---|
| Extrusão | `extent`: {`distance`, `direction`: positive\|negative\|symmetric, `distance2`, `start_offset`, `through_all`, `taper`, `to`, `to_offset`, `from`}; `operation`: new_body\|join\|cut\|intersect; `targets`; `profiles`; `sketch`; `thin`: {`curves`, `thickness`, `side`: center\|inside\|outside\|two_sides, `thickness2`} |
| Revolução | `angle`, `angle2`, `symmetric`, `axis`, `to`, `operation`, `targets`, `profiles`, `sketch` |
| Arredondamento | `radius`, `edges` (um ponto em cada aresta), `body`, `style` |
| Chanfro | `distance`, `distance2` ou `angle`, `flip`, `edges`, `body` |
| Caixa | `length`, `width`, `height`, `corner`, `operation` |
| Cilindro | `radius`, `height`, `base`, `axis`, `operation` |
| Furo | `diameter`, `depth`, `position`, `direction`, `hole` (simple, drilled, counterbore, countersink), `thread`, `to` |
| Casca | `thickness`, `faces`, `direction`, `body`, `tangent_chain` |
| Ângulo de saída | `angle`, `faces`, `neutral`, `pull`, `body` |
| Deslocar face | `distance`, `faces`, `body` |
| Padrão | `pattern` (seu tipo e quantidades/espaçamento), `features`, `bodies` |
| Espelhamento | `plane`, `features`, `bodies`, `combine` |

Para qualquer outra operação, `timeline.inspect` mostra os campos.

Quando um comando recebe uma face como ponto (`face.delete`, `solid.shell`, `solid.draft`,
`solid.offset_face`, `solid.split_face`, `faces` de `solid.replace_face` e `face` de
`sheet.convert`), scripts podem fornecer o nome listado por `model.faces`:
`{"command": "face.delete", "params": {"faces": ["F2:side#1", "F2:side#2"], "body": "Plate"}}`.
`model.faces` também lista um `point` em cada face que a seleciona (o centroide de uma face
com furo fica no furo; o de um meio cilindro fica dentro do cilindro) e, para uma face
cilíndrica, seu `cylinder`: `origin` e `axis` do eixo e `radius`. Um nome que corresponde a
várias faces (pedaços de uma face dividida ou faces de vários corpos) é recusado; `body`
indica em qual corpo procurar.

Pressione **Delete** em qualquer elemento selecionado (corpos, faces, esboços, operações, planos, componentes,
imagens de referência). A ação é executada como uma etapa que pode ser desfeita e lista as operações que seriam excluídas junto ou falhariam.

Extrusão de face, Revolução de face e Pressionar/puxar preservam perfis gerados vinculados. Quando uma face planar modificada não tem certificado de construção, esses perfis internos usam o comportamento legado de vinculação e projeção por face nomeada, para que edições anteriores ainda atualizem o plano e o contorno. Esse caminho de compatibilidade é privado dessas ferramentas de modelagem: novos esboços de usuário vinculados exigem uma face certificada ou um plano explicitamente fixo. Vinculações internas legadas podem acompanhar a face correspondente mais próxima após mudanças de topologia; inspecione seu resultado depois de grandes edições anteriores. Ative a ocorrência pretendida antes de gerar um perfil de face a partir de um componente repetido; capture uma posição de origem movida separadamente antes de criar um perfil legado entre componentes.

Pressionar/puxar com valor negativo em faces prismáticas alinhadas suportadas preserva furos passantes redondos,
inclusive depois de uma extrusão unida e em uma ocorrência de componente rotacionada. O corte particiona
o material atual em seções axiais, preservando o furo ao desfazer, refazer e reabrir.
Paredes que não podem ser qualificadas como uma extrusão paralela ainda usam o caminho booleano geral
e podem informar uma operação de superfícies coincidentes não suportada de forma atômica.
