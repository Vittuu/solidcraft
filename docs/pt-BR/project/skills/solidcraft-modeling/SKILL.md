---
name: solidcraft-modeling
description: Use ao criar uma nova peça, carcaça, suporte, placa, flange ou qualquer outro sólido 3D no SolidCraft por seu servidor MCP (ferramentas como batch, execute, measure, screenshot), ou quando uma sessão do SolidCraft estiver usando muitas chamadas de ferramentas ou tokens.
---

# Modelagem no SolidCraft

## Visão geral
Defina primeiro a peça em números. Depois crie-a em poucas chamadas grandes de `batch` e verifique cada etapa com `measure` comparando com valores calculados por você. Uma criação sem problemas usa 3-4 lotes de construção, cerca de 10-13 chamadas com as verificações. Formatos de comandos, eixos de planos, IDs, tamanhos de respostas e armadilhas conhecidas estão em `reference.md` ao lado deste arquivo, dispensando explorar o catálogo de comandos.

## Antes da primeira chamada
1. Leia `reference.md`.
2. Execute `inspect_design {"measure":false,"sketches":false}`, mesmo que considere a sessão vazia.
   - As instruções do servidor SolidCraft terminam com `Backend: connected to the SolidCraft app` quando ele controla o aplicativo aberto do usuário, e com `Backend: headless` nos demais casos.
   - Se o projeto tiver operações na linha do tempo, pergunte antes de chamar `new_design` ou `open`, pois ambos descartam trabalho não salvo. Você também pode criar dentro do projeto aberto.
3. Escreva um plano:
   - os parâmetros
   - o posicionamento: centralize a peça na origem para que padrões e espelhamentos usem os planos
   - cada operação, com o ponto global a usar em cada seleção de aresta ou face
   - o volume esperado e a caixa delimitadora

## Criar
- **Etapas:** use um lote por etapa:
  1. parâmetros, esboço, cotas, concluir e a primeira operação
  2. arredondamentos e casca
  3. furos, cortes e padrões
- **Falhas:** um lote é tudo ou nada. Em caso de falha, leia `failed.index` e `failed.error`, corrija apenas essa etapa e envie o lote novamente.
- **Etapas arriscadas:** coloque uma etapa arriscada em seu próprio lote, para que uma falha não desfaça o trabalho ao redor. Isso inclui operações booleanas entre faces curvas que se tocam e qualquer coisa que você ainda não fez.
- **Tamanho da resposta:** em projetos grandes, passe `"verbose": false` para `batch` e `execute`.
- **IDs de esboço são previsíveis,** então etapas posteriores no mesmo lote podem usá-los:
  - Cada esboço conta cada tipo a partir de 1 (`l1`, `c1`, `a1`, `p1`).
  - Conte o que as etapas anteriores desse esboço criaram. Por exemplo, `rectangle.center` cria `l1`-`l6` e `p1`-`p5`.
  - Não divida um lote apenas para ler os IDs retornados.
- **Posições que devem acompanhar um parâmetro** vêm de:
  - cotas de esboço para `"origin"` com valores de expressão (`"w/2 - wall - 6"`)
  - campos de expressão como `solid.cylinder base:["w/2-6","d/2-6","wall"]`

  Um número simples nunca acompanha um parâmetro.
- **Repetições:** `solid.pattern.rectangular` aceita espaçamento por expressão; `solid.mirror` aceita `XY`/`XZ`/`YZ`, um plano de construção ou `{origin, normal}` (as coordenadas da origem podem ser expressões de comprimento, como `["half", 0, 0]`). Uma operação de espelhamento ou padrão não pode ser espelhada nem repetida em padrão novamente.
- **Seleções de arestas e faces** se ajustam à aresta ou face mais próxima. Não há limite de distância nem erro, então calcule o ponto médio exato da geometria (a aresta vertical de canto de uma caixa centrada é `[w/2, d/2, h/2]`).
  - Seleções devem ser números simples.
  - Uma seleção é resolvida uma vez e depois acompanha sua aresta quando os parâmetros mudam.
  - Uma face pode ser fornecida por nome em vez de ponto (`faces:["F1:+z"]`, `face` de `sheet.convert`). `body_topology` lista o `name` de cada face, um `point` sobre ela (o centroide de uma face com furo fica no furo) e, para cilindros, `{origin, axis, radius}`.
- **Perfis:** extrusão usa todas as regiões por padrão. Para um anel ou ressalto com furo, passe `profiles`.
- **Ordem:** arredonde as arestas externas antes de criar a casca, para que ela acompanhe os arredondamentos. Uma aresta da face removida pela casca, como a borda de uma caixa aberta, é a exceção: arredonde-a depois da casca, pois a casca pode falhar nela.

## Verificar
- **Cada etapa:** execute `measure {}` (cerca de 1 KB) e compare volume, caixa delimitadora e quantidade de faces com seus números. Um corte ou furo que deixa o volume inalterado não atingiu o corpo, mesmo informando sucesso.
- **Estado da linha do tempo:** `inspect_design {"measure":false,"sketches":false}` mostra `error` e `warning` de cada operação.
- **Teste de parâmetro:** execute `set_parameter`. Espere `errors: []`, depois `measure` novamente; o centro de massa e a caixa delimitadora devem se mover nas quantidades calculadas.
- **Posições:** `measure` não informa onde uma operação fica.
  - Em uma peça assimétrica, verifique o deslocamento de centro de massa calculado.
  - Em uma peça simétrica (layout espelhado ou repetido em padrão), o centro de massa não muda; capture a vista superior e compare distâncias em pixels com uma dimensão conhecida.
  - `body_topology` é o último recurso: custa 25-45 KB e divide cilindros em meias faces.
- **Aparência final:** uma `screenshot {"source":"model","view":"iso","width":480,"height":320}`. Use `"view":"top"` para operações dentro das paredes, ocultadas pela isométrica.
  - `width` e `height` se aplicam apenas a renderizações do modelo.
  - Quando o servidor está conectado ao aplicativo, `source` usa por padrão a janela inteira e `view` também gira a câmera do usuário; passe `"source":"model"` e omita `view`, a menos que o usuário queira mover a câmera.

## Concluir
`export {path}` grava `.step` para CAD, `.3mf`/`.stl` para impressão ou `.urdf` (uma montagem com juntas para simuladores de robôs). `save {path}` grava `.solidcraft`.

## Sinais de alerta

| Ideia | Faça isto |
|---|---|
| "Listar todos os comandos primeiro" | `reference.md` ou `list_commands {"filter":"<distinctive word>"}`. A lista sem filtro passa de 100 KB. |
| "Verificar posições com body_topology" | Centro de massa e caixa delimitadora de `measure` comparados com seus números. |
| "Inspecionar o esboço" | `sketch.solve` retorna graus de liberdade e quantidade de perfis em menos de 100 bytes. Execute `sketch.inspect` sozinho, nunca dentro de um lote grande. |
| "new_design para começar do zero" | Quando o servidor estiver conectado ao aplicativo, pergunte primeiro. |
| "Informou ok, então funcionou" | Somente um `measure` que corresponde aos valores calculados conta. |
