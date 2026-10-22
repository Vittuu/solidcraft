# Referência de mecanismos do SolidCraft para agentes

Estes são os formatos de comandos usados por uma montagem, para evitar consultas frequentes a `list_commands`. Todos passam por `execute` ou `batch`. O exemplo abaixo é coberto por `mechanisms_skill_example_moves_as_documented`, em `crates/mcp/src/tests.rs`.

- Valores de juntas, limites e chaves de estudos usam graus para ângulos e mm para distâncias. Expressões também funcionam: `"30 deg"`, `"stroke/2"`.
- Pontos e eixos usam coordenadas globais, na posição do modelo quando a junta é criada.

## Origens de junta

Cada um de `a` e `b` em `joint.create` aceita uma destas formas:

| Forma | Formato | Observações |
|---|---|---|
| Ponto | `{"occurrence":"Wing","point":[0,30,0],"z":[1,0,0]}` | Forma comum. `z` é o eixo da junta (padrão +Z). |
| Referencial | `{"occurrence":"Wing","frame":{"origin":[0,30,0],"z":[1,0,0],"x":[0,1,0]}}` | `x` fixa a referência do ângulo 0. |
| Face | `{"occurrence":"Wing","face":[0,20,3]}` | Centro da face plana que passa pelo ponto, com eixo na normal. Acompanha a face quando a peça muda. |
| Círculo | `{"occurrence":"Actuator rod","circle":[0,32,0]}` | Centro da aresta circular que passa pelo ponto, com eixo no cilindro. Escolha um ponto na borda. |
| Nomeada | `"Hinge on frame"` | Criada antes com `joint.origin {"name":..., "occurrence":..., "point":[...], "z":[...]}`. |

- `occurrence` aceita nome de ocorrência (`Wing:1`), ID ou nome de componente (primeira ocorrência). Sem ela, usa-se o corpo mais próximo do ponto, que pode ser a peça errada onde duas se tocam.
- **Use a mesma forma nos dois lados.** `point` deriva x de z. Um `frame` com `x` de um lado e um `point` do outro girou o painel do exemplo de referência em 90 graus ao receber a junta.
- **`flip`:** `false` (padrão) acopla as origens como duas faces: z de b fica oposto ao de a. Duas origens no mesmo ponto e com o mesmo `z` giram b em 180 graus, e uma junta que fecha um ciclo informa conflito ("off by 71 mm and 180 degrees" no relato SELENE). `true` alinha os eixos z, mantendo as peças onde estão. `joint.edit {"joint":..., "flip":true}` altera depois.

## Tipos de junta

| `type` | Valores, em ordem | Unidades |
|---|---|---|
| `rigid` | nenhum | |
| `revolute` | ângulo | deg |
| `slider` | distância ao longo de z | mm |
| `cylindrical` | ângulo, distância | deg, mm |
| `pin_slot` | ângulo, deslocamento | deg, mm |
| `planar` | x, y, ângulo | mm, mm, deg |
| `ball` | yaw, pitch, roll | deg |

O valor de uma deslizante cresce quando b se move ao longo de +z de sua origem. Dê o mesmo ponto às duas origens para que 0 seja a posição modelada.

## Comandos

```json
{"command":"component.create","params":{"name":"Actuator rod","parent":"root","activate":false}}
{"command":"component.move_bodies","params":{"bodies":["Rod"],"component":"Actuator rod"}}
{"command":"occurrence.ground","params":{"occurrence":"Frame:1","grounded":true}}
{"command":"joint.create","params":{"type":"slider","name":"Stroke","flip":true,"limits":[[-5,10]],"a":{"occurrence":"Actuator barrel","point":[0,0,-40],"z":[0,0.6,0.8]},"b":{"occurrence":"Actuator rod","point":[0,0,-40],"z":[0,0.6,0.8]}}}
{"command":"joint.as_built","params":{"type":"revolute","a":"Frame","b":"Wing 2","at":{"occurrence":"Frame","point":[0,0,0],"z":[1,0,0]},"name":"Hinge 2","limits":[[-60,0]]}}
{"command":"joint.rigid_group","params":{"occurrences":["Frame:1","Lamp:1"]}}
{"command":"joint.motion_link","params":{"a":"Hinge","b":"Hinge 2","ratio":-1}}
{"command":"joint.drive","params":{"joint":"Hinge","value":20}}
{"command":"joint.drive","params":{"joint":"Spindle","values":[30,5]}}
{"command":"joint.limits","params":{"joint":"Hinge","min":0,"max":60}}
{"command":"joint.edit","params":{"joint":"Hinge","flip":true}}
{"command":"joint.delete","params":{"link":0}}
{"command":"contact.create","params":{"occurrences":["Frame:1","Wing 2:1"],"name":"Wing 2 stop"}}
{"command":"contact.enable_sets","params":{}}
{"command":"motion.study","params":{"name":"Open","steps":10,"keys":[{"joint":"Hinge","points":[[0,0],[10,25]]}]}}
{"command":"motion.play","params":{"study":"Open","step":5}}
{"command":"motion.export","params":{"study":"Open","samples":3}}
{"command":"occurrence.properties","params":{"occurrence":"Wing:1"}}
```

- `joint.as_built` recebe ocorrências, não origens. `at` é o referencial da junta (padrão: origem do componente de b).
- `joint.rigid_group` conecta todas as ocorrências à primeira com juntas rígidas na posição atual.
- `joint.motion_link`: b = ratio·a + offset, limitado aos limites de b. `offset` é um número em radianos para ângulos e mm para distâncias: 10 graus são `0.17453`. `ia`/`ib` escolhem um valor de uma junta com vários valores (padrão 0). `joint.delete {"link": n}` remove o vínculo n (índice em `links` de `joint.list`).
- `joint.drive`: `values` aciona os valores listados em ordem (`Spindle` acima é cilíndrica: 30 graus e 5 mm). `value` aciona o primeiro. Valores omitidos mantêm os anteriores ou acompanham um ciclo fechado; não são zerados. Valores são limitados aos limites da junta. A resposta usa objetos com `name`, `value`, `unit` e limites, como `joint.list`, em graus/mm.
- `moved` informa se alguma ocorrência mudou de pose. Quando a junta não pode mover nada, `reason` explica; `unconnected` identifica a peça de uma cadeia sem ligação a algo fixado. `joint.list` agrupa essas cadeias em `floating`. `quiet: true` omite `dof` e `fixed` das respostas de juntas.
- `motion.study {"study":..., ...}` edita um estudo; fornecer `keys` substitui todas as chaves.
- `motion.play` posiciona as juntas no passo do estudo (frações funcionam) e altera o projeto. `moved` informa movimento; `unmoved` lista juntas que não podem mover nada. Com `quiet: true`, retorna poses apenas das ocorrências movidas. `motion.export` não altera o projeto.
- `component.list {"compact":true}` usa contagens de corpos, operações, esboços e planos de construção, omite transformações identidade e lista filhos por ID de ocorrência. Acrescente `"bodies":true` para nomes de corpos em vez da contagem.

## Exemplo: um painel em um atuador

Uma estrutura segura um painel por uma dobradiça ao longo de X na origem. Um atuador vai do pino da raiz na estrutura, R = (0, 0, -40), ao pino da ponta no painel, T = (0, 30, 0), a 30 mm da dobradiça: comprimento de 50 mm, na direção (0, 0.6, 0.8). A haste desliza de -5 a +10 mm. Um vínculo de movimento espelha um segundo painel do outro lado.

```json
{"commands":[
 {"command":"solid.box","params":{"length":40,"width":10,"height":50,"corner":[-20,-10,-50],"operation":"new","body_name":"Post"}},
 {"command":"solid.box","params":{"length":40,"width":40,"height":10,"corner":[-20,0,-50],"operation":"new","body_name":"Base"}},
 {"command":"solid.box","params":{"length":40,"width":40,"height":3,"corner":[-20,0,0],"operation":"new","body_name":"Panel"}},
 {"command":"solid.box","params":{"length":40,"width":40,"height":3,"corner":[-20,-40,0],"operation":"new","body_name":"Panel 2"}},
 {"command":"solid.cylinder","params":{"radius":5,"height":20,"base":[0,0,-40],"axis":[0,0.6,0.8],"operation":"new","body_name":"Barrel"}},
 {"command":"solid.cylinder","params":{"radius":2,"height":35,"base":[0,9,-28],"axis":[0,0.6,0.8],"operation":"new","body_name":"Rod"}},
 {"command":"component.create","params":{"name":"Frame","parent":"root","activate":false}},
 {"command":"component.move_bodies","params":{"bodies":["Post","Base"],"component":"Frame"}},
 {"command":"component.create","params":{"name":"Wing","parent":"root","activate":false}},
 {"command":"component.move_bodies","params":{"bodies":["Panel"],"component":"Wing"}},
 {"command":"component.create","params":{"name":"Wing 2","parent":"root","activate":false}},
 {"command":"component.move_bodies","params":{"bodies":["Panel 2"],"component":"Wing 2"}},
 {"command":"component.create","params":{"name":"Actuator barrel","parent":"root","activate":false}},
 {"command":"component.move_bodies","params":{"bodies":["Barrel"],"component":"Actuator barrel"}},
 {"command":"component.create","params":{"name":"Actuator rod","parent":"root","activate":false}},
 {"command":"component.move_bodies","params":{"bodies":["Rod"],"component":"Actuator rod"}},
 {"command":"occurrence.ground","params":{"occurrence":"Frame:1","grounded":true}},
 {"command":"joint.create","params":{"type":"revolute","name":"Hinge","flip":true,"a":{"occurrence":"Frame","point":[0,0,0],"z":[1,0,0]},"b":{"occurrence":"Wing","point":[0,0,0],"z":[1,0,0]}}},
 {"command":"joint.create","params":{"type":"revolute","name":"Root pin","flip":true,"a":{"occurrence":"Frame","point":[0,0,-40],"z":[1,0,0]},"b":{"occurrence":"Actuator barrel","point":[0,0,-40],"z":[1,0,0]}}},
 {"command":"joint.create","params":{"type":"slider","name":"Stroke","flip":true,"limits":[[-5,10]],"a":{"occurrence":"Actuator barrel","point":[0,0,-40],"z":[0,0.6,0.8]},"b":{"occurrence":"Actuator rod","point":[0,0,-40],"z":[0,0.6,0.8]}}},
 {"command":"joint.create","params":{"type":"revolute","name":"Tip pin","flip":true,"a":{"occurrence":"Actuator rod","point":[0,30,0],"z":[1,0,0]},"b":{"occurrence":"Wing","point":[0,30,0],"z":[1,0,0]}}},
 {"command":"joint.as_built","params":{"type":"revolute","a":"Frame","b":"Wing 2","at":{"occurrence":"Frame","point":[0,0,0],"z":[1,0,0]},"name":"Hinge 2","limits":[[-60,0]]}},
 {"command":"joint.motion_link","params":{"a":"Hinge","b":"Hinge 2","ratio":-1}}
]}
```

**Esperado:**
- Após o lote, `joint.list` informa `dof` 1 para Wing, Wing 2, camisa e haste (ciclo e vínculo deixam um movimento), 0 para Frame e nenhum `conflicts`.
- **Comprimento do atuador:** o pino da ponta vai para `(0, 30 cos θ, 30 sin θ)`, portanto `L(θ)² = 2500 + 2400 sin θ` e `L(0) = 50`. O curso é `L(θ) - 50`.
- `joint.drive {"joint":"Hinge","value":20}`: o curso é `L(20°) - 50 = 7.6268` mm, e Hinge 2 fica em -20. `occurrence.properties {"occurrence":"Wing:1"}` coloca o centro de massa do painel em (0, 18.281, 8.250), e o de Wing 2 em (0, -18.281, 8.250).
- `joint.drive {"joint":"Hinge","value":45}` para em 27.28 graus com `stopped_by_loop: {"joint":"Stroke", ...}`: a haste alcança +10 mm quando `L(θ) = 60`, em `sin θ = 11/24`, θ = 27.280 graus.

## Em escala: SELENE

A referência original relata o mesmo padrão nas asas solares do veículo SELENE (`Rover-v7.solidcraft`, de 0xlillith, 36 ocorrências e 1353 operações):
- **Atuadores separados:** cada um dos quatro atuadores era um componente. Camisa e haste, com os olhais e pinos que as acompanham, foram para componentes próprios com `component.create` e `component.move_bodies`.
- **Ciclos:** por atuador, um pino de revolução na raiz (estrutura e camisa), uma deslizante com limites de curso `[[-6, 0.4]]` mm e um pino de revolução na asa (haste e asa). Todos com `flip: true`, origens por ponto global, z em X nos pinos e ao longo do atuador (raiz à ponta) na deslizante.
- **Vínculos:** as seis rodas acompanham a primeira (`ratio: 1`); a inclinação de estibordo espelha a de bombordo (`ratio: -1`).
- **Módulos flutuantes:** em v7, 18 componentes (pedestais, aviônica, tampa e outros) não estavam fixados nem conectados; acionar torre ou antena não movia nada. Um `joint.rigid_group` com a estrutura resolveu isso.
- **Ocultos:** as mangueiras flexíveis do braço são corpos rígidos em um elo, e seus esboços de varredura ficam parados; ambos se afastam quando o braço se move. `browser.visibility {"items":[...], "visible":false}` os ocultou, e `{"folder":"joints","visible":false}` ocultou os símbolos das juntas na gravação.
- **Números:** a dobradiça de inclinação de bombordo fica em (0, -140, 158), ao longo de X; a raiz do atuador em (x, -91, 134) e o pino na asa em (x, -180.41, 169.71). O curso a 20 graus é -2.87 mm, e a haste chega a -6 mm em 32.230 graus. No relato original, o aplicativo concorda: acionar a 20 graus retorna -2.87 mm, e em direção a 45 graus para em 32.23 graus com `stopped_by_loop`.

## Armadilhas

| Sintoma | Causa e correção |
|---|---|
| A peça gira 180 graus ao receber a junta; o ciclo informa "off by ... and 180 degrees" | `flip` omitido. Passe `flip: true` ou `joint.edit {"joint":..., "flip":true}`. |
| A peça gira até um limite assim que recebe a junta | Uma junta nova começa em 0 e é limitada aos seus limites. Inclua 0: `[[10, 60]]` gira uma dobradiça 10 graus. |
| A peça gira outro ângulo ao receber a junta | Origens com formas ou eixos x diferentes. Use `point` + `z` nos dois lados ou `joint.as_built`. |
| `joint.drive` ou `motion.play` não move nada | Confira `moved`, `reason`, `unconnected`, `unmoved` e as cadeias `floating`. Uma cadeia sem ligação a algo fixado (`dof: 6`) precisa ser fixada ou conectada, por exemplo com `joint.rigid_group` à estrutura. |
| Os números de `joint.drive` parecem estar em radianos | As respostas atuais usam objetos com `value` e `unit`, em graus/mm, como `joint.list`. Não aplique conversão de radianos a esses valores. |
| O acionamento para antes, com `stopped_by_loop` | Um ciclo atingiu um limite ou não fecha além dali; `joint` identifica a junta. Compare os limites com o cálculo do curso. |
| Uma peça do ciclo informa `dof: 0` na pose modelada, mas se move ao acionar | `dof` é contado na pose atual. Uma peça cujo movimento desaparece ali (atuador alinhado ao caminho da ponta, que só se estende) conta 0. Acione um pouco e liste novamente. |
| Alterar só o primeiro valor de uma junta com vários valores | `value` aciona o primeiro; `values` aciona os informados. Os demais mantêm seus valores ou acompanham o ciclo, sem zerar. |
| Um ângulo vinculado difere muito mais que seu `offset` | `joint.motion_link` lê `offset` angular em radianos: `"offset":10` adiciona 573 graus. |
| Uma junta vinculada deixa de acompanhar | Seu valor é limitado aos próprios limites. Hinge 2 tem `[[-60, 0]]`; acionar Hinge abaixo de 0 a mantém em 0. Dê limites espelhados à junta espelhada. |
| O acionamento para antes, com `stopped_by_contact` | Dois corpos do conjunto de contato se tocam. `fraction` informa a fração alcançada. |
| `occurrence.ground` não encontra `Frame` | Aceita nomes de ocorrência: `Frame:1`. |
| `a joint needs two different occurrences` | As origens encontraram a mesma peça. Informe `occurrence` nos dois lados. |
| "unknown command" para `file.open` | Abra com a ferramenta `open` ou `doc.open {path}`. |
| Mangueiras, cabos ou molas rígidos se afastam durante o movimento | Corpos não se dobram. Oculte com `browser.visibility` ou modele por elo. |
| Renderizar um script sobre um projeto existente | `solidcraft-cli snapshot SCRIPT --in design` aceita o projeto inicial na base atual; use `solidcraft-cli run SCRIPT --in design --save out.solidcraft` quando também quiser salvar o resultado. |
| A exportação URDF omite uma junta | Exportar `.urdf` omite, com aviso, cada junta que fecha um ciclo, pois URDF é uma árvore. |

**Respostas atuais.** A base `0872eb9` já inclui as mudanças que o texto inglês original descrevia como pendentes: valores de `joint.drive`/`motion.play` com unidades em graus/mm, indicação de movimento, cadeias `floating` e `quiet: true`. Esta tradução também acompanha a preservação de valores omitidos, `snapshot --in` e `component.list` compacto; não reutiliza as armadilhas antigas sobre radianos, valores zerados ou entrada ignorada.

## Custo de resposta

Tamanhos aproximados relatados na referência original, antes das opções compactas; não são medições desta tradução:

| Chamada | Tamanho |
|---|---|
| `component.list` completo | **140 KB** no SELENE (transformação 4x4 por ocorrência). Prefira `{"compact":true}` ou `occurrence.properties`. |
| `joint.list` | cerca de 0.5 KB por junta: 16 KB para 29 juntas |
| `motion.play` sem `quiet` | cerca de 0.2 KB por ocorrência e passo: 9 KB no SELENE |
| `motion.export` | o mesmo por amostra; informe `samples`, ou `path` para gravar um arquivo |
| `joint.create`, `joint.drive`, `joint.motion_link` | lista `dof` de todas as ocorrências, cerca de 40 B cada; `quiet: true` a omite |
| `occurrence.properties` | menos de 1 KB por ocorrência |

## Mostrar o movimento

Conectado ao aplicativo, `motion.play {"study":..., "step":s}` move a vista do usuário, e `screenshot` captura a janela em cada passo. Para gravar fora do MCP, use o canal de controle do aplicativo ([protocolo](../../../control-protocol.md)): `engine.execute` com `motion.play`, depois `ui.screenshot` com `path`, uma vez por quadro.
