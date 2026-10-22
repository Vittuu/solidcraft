# Referência de comandos do SolidCraft para agentes

Estes são os formatos dos comandos necessários para criar uma peça, para que um agente raramente precise chamar `list_commands`. O exemplo desenvolvido no final é executado em CI (`skill_reference_example_builds_as_documented` em `crates/mcp/src/tests.rs`).

- Comprimentos são em mm.
- Ângulos dentro de expressões recebem unidades: `"30 deg"`.
- Qualquer campo numérico pode ser uma expressão que nomeia parâmetros: `"w/2 - wall - 6"`.

## Planos de esboço: onde (x, y) do esboço fica no espaço global

| `plane` | x do esboço | y do esboço | normal (+distance) |
|---|---|---|---|
| `"XY"` | +X | +Y | +Z |
| `"XZ"` | +X | **-Z** | +Y |
| `"YZ"` | +Y | +Z | +X |
| `{origin, x_dir, y_dir}` | x_dir | y_dir | x_dir x y_dir |

- `offset: "expr"` em `sketch.create` move o plano ao longo da normal e acompanha parâmetros.
- **Parede frontal (-Y) de uma peça centrada na origem:**
  - Crie o esboço em `{"plane":"XZ","offset":"-d/2"}`.
  - Desenhe uma operação na altura z em y do esboço = **-z**.
  - Um corte com distância positiva entra na parede.
- **Face superior:** crie o esboço em `{"plane":"XY","offset":"h"}`.
- **Direção do furo:** `solid.hole {sketch}` perfura do plano do esboço ao longo de -normal. Um esboço de furo em z=0 sob uma peça perfura espaço vazio e ainda informa sucesso; coloque-o na face superior.

## IDs

- **Numeração:** cada criador retorna `created: {points, curves, constraints, dimensions}`. IDs são estáveis e numerados por esboço.
- **Curvas:** `l1`, `c1`, `a1`.
- **Pontos:** `p1`, `l1.start`, `l1.end`, `c1.center`, `a1.center` e a origem do esboço `"origin"`.
- **Retângulos:** `sketch.rectangle.*` também retorna `sides {bottom,right,top,left}` e `corners {bottom_left,...}`.
  - `rectangle.center` adiciona 2 diagonais de construção, então cada canto une 3 linhas. Seu ponto central é `p5`.
  - Por isso, `sketch.fillet {point}` falha nos cantos. Use `{a:"l1", b:"l4", radius}` ou o `solid.fillet` 3D.
- **Ranhuras:** `sketch.slot.overall` e `sketch.slot.center_to_center` criam curvas `l1, a1, l2, a2` e pontos `p1`-`p4`, `a1.center`, `a2.center`.
  - O papel `length` de `slot.overall` é o comprimento total. Os centros dos arcos ficam em ±(length/2 - width/2).
  - Após `length` e `width`, restam 3 dof. Para removê-los, aplique cotas horizontal e vertical de `origin` a `a1.center`, e uma cota vertical de `origin` a `a2.center`. Para uma ranhura ao longo de Y, o solucionador recusa a última cota por excesso de restrições; aplique a cota horizontal a `a2.center`.
  - `sketch.slot.center_point` não cria ponto central. Aplique a cota a um centro de arco.
- **Círculos:** `sketch.circle.center` cria `c1` e `c1.center`, sem ponto `p`.
- **Cotas** compartilham o contador `k` com restrições.
- **Restrições podem adicionar curvas:** uma restrição sobre um eixo, como `sketch.constraint.symmetry` com `line:"y"`, adiciona uma linha de construção e usa o próximo ID `l`. Leia `created` antes de continuar a contagem.
- **Excesso de restrições** é recusado com `that conflicts with the sketch: neither entity can move to satisfy it`. Por exemplo, simetria já torna dois círculos iguais.

## Cotas

```json
{"command":"sketch.dimension","params":{"entities":["origin","c1.center"],"type":"horizontal","value":"w/2 - wall - 6"}}
{"command":"sketch.dimension","params":{"entities":["c1"],"type":"diameter","value":"6"}}
{"command":"sketch.dimension","params":{"entities":["l1"],"value":"w"}}
```

Distâncias não têm sinal. Desenhe primeiro a geometria no lado correto, e a cota a manterá ali.

`sketch.draw.dimensioned` cria uma forma e suas cotas em uma etapa de desfazer. Os papéis das cotas dependem do criador:

| Criador | Papéis |
|---|---|
| `line` (exatamente 2 pontos) | `length`, `angle` |
| `rectangle.*` | `width`, `height` |
| `circle.*` | `radius` ou `diameter` |
| `arc.center_point` | `radius`, `sweep` |
| `slot.center_to_center`, `slot.overall` | `length`, `width` |
| `polygon.*` | `radius` |

```json
{"command":"sketch.draw.dimensioned","params":{"command":"sketch.rectangle.center","params":{"center":[0,0],"corner":[35,25]},"dimensions":{"width":"w","height":"d"}}}
```

## Operações

| Comando | Parâmetros mínimos | Armadilhas |
|---|---|---|
| `solid.extrude` | `distance` | `profiles` usa **todas as regiões** por padrão, então dois círculos concêntricos produzem um disco sólido. Escolha regiões com `[{"point":[x,y]}]` ou `[index]`. Outras opções: `operation` (new/join/cut/intersect/auto), `direction` (positive/negative/symmetric), `through_all: true`, `start_offset`. Um corte que não atinge o corpo informa sucesso. |
| `solid.box` | `length`, `width`, `height` | `center:[x,y,z]` ou `corner`. A forma mais rápida de criar o volume inicial de uma peça. |
| `solid.cylinder` | `radius` ou `diameter`, `height` | `base` é o centro da base (padrão origem), `axis` usa `Z` por padrão. Expressões funcionam em `base`. Sempre passe `operation`. |
| `sketch.slot.overall` / `sketch.slot.center_to_center` | `p0`, `p1` (extremidades totais / centros de arcos), `width` | IDs e dof estão na seção "IDs" acima. |
| `solid.fillet` / `solid.chamfer` | `edges:[[x,y,z]]`, `radius` / `distance` | A seleção se ajusta à aresta **mais próxima**, sem limite de distância nem erro; verifique se a quantidade de faces mudou. Uma seleção abrange toda a cadeia tangente: uma borda arredondada precisa de 1 ponto, não 8. |
| `solid.shell` | `faces:[[x,y,z]]`, `thickness` | Um ponto em cada face a remover, ou seu nome de `body_topology`. Arredonde primeiro, exceto uma aresta da face removida (veja Ordem em `SKILL.md`). |
| `solid.hole` | `position:[x,y,z]` em uma face + `diameter`, ou `sketch` (+ `points`) | `clearance:"M3"`. `depth` usa através de tudo por padrão. `type` é simple/drilled/counterbore/countersink. Um `countersink` exige `cs_diameter` e `cs_angle`; um `counterbore` exige `cb_diameter` e `cb_depth`. `position` é um ponto fixo. |
| `solid.pattern.rectangular` | `features`, `dir1`, `count1`, `spacing1` (+ `dir2`, `count2`, `spacing2`) | Espaçamento aceita expressões e acompanha parâmetros. |
| `solid.pattern.circular` | `features`, `axis` (`"Z"` ou `{origin, dir}`), `count` | `angle` usa 360 deg por padrão. Um padrão completo mantém o centro de massa no eixo; verifique seu raio com uma captura da vista superior. |
| `solid.mirror` | `features:[names]`, `plane` (`XY`/`XZ`/`YZ`, um plano de construção ou `{origin, normal}` cuja origem pode usar expressões de comprimento) | Acompanha parâmetros. Operações de espelhamento e padrão não podem ser espelhadas nem repetidas em padrão novamente. |

- **Nomes:** nomeie todas as operações (`"name":"Standoff"`) e mantenha nomes únicos entre esboços e operações. `timeline.roll_to {"feature":"Plate"}` escolhe a primeira correspondência, que pode ser o esboço.
- **Quantidades como parâmetros:** `parameters.add {"name":"n","expression":"6","unit":""}` cria um parâmetro sem unidade, e `"count":"n"` o usa em um padrão.
- **Furos em um círculo paramétrico de parafusos:**
  1. Corte um furo com `solid.cylinder {"diameter":6.6,"height":"T + 2","base":["bcd/2",0,-1],"operation":"cut","name":"BoltHole"}`.
  2. Repita com `solid.pattern.circular {"features":["BoltHole"],"axis":"Z","count":"n"}`.

  O raio e a quantidade acompanham seus parâmetros.
- **Várias alterações de parâmetros:** use um `batch` de etapas `parameters.change` em vez de uma chamada `set_parameter` por alteração.
- **Operações booleanas de primitivas:** um corte `solid.box` ou `solid.cylinder` que toca faces curvas do destino pode falhar com `the exact fallback ... takes planar faces only`. Use um perfil de esboço e um corte por extrusão, por exemplo um esboço de ranhura em vez de uma caixa mais 2 cilindros.

## Exemplo desenvolvido: placa paramétrica com 4 furos rebaixados (1 lote)

```json
{"commands":[
 {"command":"parameters.add","params":{"name":"L","expression":"80 mm"}},
 {"command":"parameters.add","params":{"name":"W","expression":"50 mm"}},
 {"command":"parameters.add","params":{"name":"T","expression":"6 mm"}},
 {"command":"parameters.add","params":{"name":"inset","expression":"8 mm"}},
 {"command":"sketch.create","params":{"plane":"XY","name":"PlateSketch"}},
 {"command":"sketch.draw.dimensioned","params":{"command":"sketch.rectangle.center","params":{"center":[0,0],"corner":[40,25]},"dimensions":{"width":"L","height":"W"}}},
 {"command":"sketch.finish","params":{}},
 {"command":"solid.extrude","params":{"distance":"T","name":"Plate"}},
 {"command":"solid.fillet","params":{"edges":[[40,25,3],[-40,25,3],[40,-25,3],[-40,-25,3]],"radius":5,"name":"Corners"}},
 {"command":"sketch.create","params":{"plane":"XY","name":"HolePts","offset":"T"}},
 {"command":"sketch.point","params":{"point":[32,17]}},
 {"command":"sketch.dimension","params":{"entities":["origin","p1"],"type":"horizontal","value":"L/2 - inset"}},
 {"command":"sketch.dimension","params":{"entities":["origin","p1"],"type":"vertical","value":"W/2 - inset"}},
 {"command":"sketch.finish","params":{}},
 {"command":"solid.hole","params":{"sketch":"HolePts","diameter":3.4,"type":"counterbore","cb_diameter":6.5,"cb_depth":3.3,"name":"Hole"}},
 {"command":"solid.pattern.rectangular","params":{"features":["Hole"],"dir1":[-1,0,0],"count1":2,"spacing1":"L - 2*inset","dir2":[0,-1,0],"count2":2,"spacing2":"W - 2*inset","name":"Holes"}}
]}
```

**Volume esperado:**
- Placa: `L*W*T - 4*r²(1-π/4)*T`
- Cada furo: `π*1.7²*(T-3.3) + π*3.25²*3.3`
- Total: 23335.2 mm³, com 22 faces.

Após `set_parameter L=120`, o volume é 35335.2 mm³, e os furos permanecem nos cantos.

## Interpretando `measure`

- **Tolerância:** volumes analíticos e medidos concordam com erro de cerca de 0.001-0.01%; trate uma diferença tão pequena como correspondência. Arcos longos e rasos medem pior, cerca de 0.1% (R 1000 mm em 20 graus).
- **Quantidades de faces:** compare `faces` lógicas, não o número maior de `kernel.faces`.
- **Tipos de resposta:** `set_parameter` retorna `recomputed` como número. Uma resposta de lote repete cada etapa.
- **Aviso booleano:** um `warning` de operação como `Boolean retry: temporary coincident-face separation up to 2.050000e0 mm per face` significa que o kernel ajustou faces coincidentes. Exemplos são uma união nivelada em uma face, como um cilindro unido ao piso de uma caixa com casca, ou um corte inclinado que começa em uma face. Quando o volume ainda corresponde ao cálculo, o aviso é inofensivo. Quando não corresponde, a tentativa pode ter deixado a geometria errada, sem erro. Caso observado: em uma caixa com cantos arredondados, casca e borda arredondada, um apoio unido e seu furo piloto cortado como dois cilindros, repetidos em padrão 2x2, deixavam material nos furos em algumas alturas de apoio. Um ressalto com furo como um perfil de anel (dois círculos concêntricos, extrudar o anel) foi criado corretamente em todas as alturas testadas (3 a 20 mm). Outro tipo, `tool displacement`, move a ferramenta em cerca de 2e-4 mm.

## Custo das respostas (0.8, exemplo da placa)

| Chamada | Tamanho |
|---|---|
| `list_commands {}` | **Mais de 100 KB.** Nunca chame sem filtro; filtre por uma palavra específica, como `"hole"` ou `"slot"`. |
| `body_topology` | 25-45 KB |
| `inspect_design {}` | 12 KB, aumentando com os esboços |
| `inspect_design {"measure":false,"sketches":false}` | 4 KB: parâmetros, erros da linha do tempo, corpos |
| `sketch.inspect` | 5 KB por esboço pequeno |
| `parameters.list {"filter":"feature"}` | 3 KB |
| `measure {}` | 1 KB: volume, caixa delimitadora, centro de massa, contagens de faces/arestas/vértices |
| `sketch.solve` | menos de 100 B: dof, estado, quantidade de perfis |
| `screenshot {"source":"model","width":480,"height":320}` | uma imagem pequena. Sem `source`, um servidor conectado retorna a janela inteira do aplicativo e ignora o tamanho. |

## Erros

| Texto do erro | Correção |
|---|---|
| `... is not available right now: no sketch is being edited` | Execute primeiro `sketch.create` ou `sketch.edit {sketch}`. |
| `unknown sketch entity 'p1'` | Use os IDs de `created`. O centro de um círculo é `c1.center`. |
| `the corner must join exactly two lines` | É um canto de `rectangle.center`. Use a forma `{a, b}`. |
| `` `cs_diameter` must be a number or an expression `` | Escareamentos exigem `cs_diameter` e `cs_angle`, mesmo com `clearance`. |
| `unsupported dimension role for this creator` | Verifique a tabela de papéis acima. |
| `<Feature> cannot be patterned or mirrored yet` | Repita a operação original em padrão, por exemplo um padrão retangular 2x2. |
| `an edge is a point [x,y,z], {point} or {body, index}` | Uma seleção de aresta continha uma expressão. Use números simples. |
| `boolean Cut: the operation failed ... planar faces only` | Veja "Operações booleanas de primitivas" acima. |
