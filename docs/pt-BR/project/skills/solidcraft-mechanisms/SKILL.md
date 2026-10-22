---
name: solidcraft-mechanisms
description: Use ao criar, acionar ou animar uma montagem ou mecanismo no SolidCraft pelo servidor MCP — componentes, fixação, juntas e suas origens, grupos rígidos, vínculos de movimento, ciclos fechados como atuadores em painéis articulados ou mecanismos de barras, conjuntos de contato, estudos de movimento e graus de liberdade — ou quando um acionamento não move nada, para antes ou uma peça salta ao receber uma junta. Para modelar as peças, use solidcraft-modeling.
---

# Mecanismos no SolidCraft

## Visão geral
Uma montagem consiste em componentes posicionados por juntas. Planeje primeiro o mecanismo como pontos e eixos globais: cada pino, guia e dobradiça, quais peças conecta e o valor esperado em cada pose. Forneça as origens de junta por essas coordenadas e compare cada etapa com números calculados por você. Um acionamento bem-sucedido pode não ter movido nada. Formatos de comandos, exemplo, armadilhas e tamanhos de resposta estão em `reference.md`. Para criar as peças, use `solidcraft-modeling`.

A referência foi escrita para `main` após 0.11.1, que conta os graus de liberdade de ciclos fechados e propaga vínculos por cadeias em qualquer ordem de criação. Esta tradução acompanha as respostas disponíveis na base `0872eb9`: graus/mm com unidades, indicação de movimento e respostas compactas.

## Antes da primeira chamada
1. Leia `reference.md`.
2. **Abrir:** se o projeto ainda não estiver aberto, use `open {path}`. Quando o servidor estiver conectado ao aplicativo e o projeto atual estiver `dirty`, pergunte antes de abrir outro.
3. **Mapear:** use `inspect_design {"measure":false,"sketches":false}` e `execute joint.list`. `joint.list` mostra `fixed` (ocorrências fixadas, sempre incluindo a raiz, 0), `dof` por ocorrência, `conflicts`, juntas, vínculos, origens nomeadas e cadeias `floating`. `execute motion.list` lista os estudos de movimento.
4. **Projeto existente:** crie um `edit.checkpoint` em chamada própria antes de mover corpos para componentes. `edit.restore_checkpoint {checkpoint}` restaura o estado exato.
5. **Escreva o plano:**
   - componentes e qual será fixado
   - cada junta: tipo, duas ocorrências, ponto e eixo globais, limites
   - cada vínculo de movimento e sua razão
   - para cada ciclo, a relação a verificar, como curso do atuador versus ângulo da dobradiça, e onde deve parar
   - a pose a restaurar antes de salvar

## Criar por etapas
1. **Componentes e fixação.**
   - Corpos criados na raiz vão para componentes com `component.create {"name":..., "parent":"root", "activate":false}` e `component.move_bodies {"bodies":[...], "component":...}`.
   - Uma peça que se move em duas partes, como a camisa e a haste de um atuador, se torna dois componentes.
   - Fixe a base: `occurrence.ground {"occurrence":"Frame:1", "grounded":true}`. Ocorrências usam `<component>:<n>`. Comandos de juntas também aceitam o nome do componente (primeira ocorrência); `occurrence.ground` não aceita.
2. **Juntas de cadeia aberta em um lote.** Leia `occurrence.properties` de cada ocorrência móvel antes e depois. Se algo se mover, divida o lote para encontrar a junta responsável.
3. **Cada junta que fecha um ciclo em chamada própria.** Confira `conflicts` na resposta.
4. **Vínculos de movimento.**
5. **Conjuntos de contato e estudo de movimento.**

## Juntas
- **Origens por coordenadas globais:** `{"occurrence":..., "point":[x,y,z], "z":[ax,ay,az]}`. Sempre informe `occurrence`; sem ela, usa-se o corpo mais próximo. Use a mesma forma nos dois lados; `frame`, `face`, `circle` e origens nomeadas estão em `reference.md`.
- **Forneça `flip: true`** quando as duas origens tiverem o mesmo ponto e eixo globais, como ao conectar peças onde já estão. Sem isso, z de b se opõe a z de a, e b gira 180 graus.
- **Peças já posicionadas:** `joint.as_built {"type":..., "a":..., "b":..., "at":{origin}, "name":..., "limits":[[min,max]]}` mantém seu posicionamento.
- **Valores e limites** usam graus para ângulos e mm para distâncias. Um ângulo gira pela regra da mão direita em torno de z da junta. Um acionamento além dos limites próprios é limitado a eles, sem ser uma falha. Uma junta nova começa em 0: seus limites devem incluir 0, ou a peça gira até o limite mais próximo ao receber a junta. `value` aciona o primeiro valor; `values` aciona a lista informada. Valores omitidos mantêm os anteriores ou acompanham um ciclo fechado.
- **Toda cadeia deve alcançar algo fixado.** Uma peça não fixada nem conectada a uma peça fixada nunca é posicionada. O acionamento retorna `moved: false`, `reason` e, quando aplicável, `unconnected`; `joint.list` lista a cadeia em `floating`. `dof: 6` também a identifica. Corrija com fixação, uma junta à estrutura ou `joint.rigid_group {"occurrences":["Frame:1", ...]}`.
- **Respostas:** `joint.drive` retorna `values` como `joint.list`: nome, valor, `unit` e limites, em graus/mm. `quiet: true` omite as listas `dof` e `fixed` das respostas de juntas.

## Movimento vinculado e fechado
- **Vínculos:** `joint.motion_link {"a":..., "b":..., "ratio":r, "offset":o}` define b = r·a + o, limitado pelos limites de b. `offset` continua em radianos para ângulos (mm para distâncias), ao contrário dos valores de junta. Uma peça espelhada usa `ratio: -1` e limites espelhados. Cadeias (a aciona b, b aciona c) funcionam em qualquer ordem de criação. Acione apenas a cabeça da cadeia.
- **Ciclos fechados:** um atuador entre uma estrutura e um painel articulado usa três juntas além da dobradiça:
  - pino de revolução na raiz (estrutura e camisa)
  - deslizante (camisa e haste), com z ao longo do atuador da raiz à ponta nas duas origens, ambas no pino da raiz, e o curso como limites
  - pino de revolução na ponta (haste e painel), que fecha o ciclo

  Quatro barras e biela-manivela seguem o mesmo padrão. Acione a dobradiça; pinos e deslizante acompanham.
- **Paradas:** no fim do curso, o acionamento para com `stopped_by_loop: {joint, fraction}`, identificando a junta que não pôde acompanhar. Um ciclo que não fecha é listado em `conflicts`, e nada nele se move.
- **dof** considera o ciclo: painel, camisa e haste informam 1 cada. Em uma pose na qual o atuador se alinha ao caminho do pino da ponta, uma peça pode informar 0 e ainda se mover; acione um pouco e liste novamente.

## Contato e estudos de movimento
- **Contato:** `contact.create {"occurrences":[...], "name":...}` e `contact.enable_sets {}`. O acionamento para onde dois corpos do conjunto atravessariam um ao outro, com `stopped_by_contact: {a, b, fraction}`. `contact.check {}` lista os pares sobrepostos agora.
- **Estudo:** `motion.study {"name":..., "steps":N, "keys":[{"joint":..., "points":[[step, value], ...]}]}`. Valores em graus/mm, interpolados linearmente. Defina chaves apenas nas juntas que acionam; vínculos e ciclos acompanham.
- **Reproduzir:** `motion.play {"study":..., "step":s}` posiciona as juntas e altera o projeto. `moved` informa movimento; `unmoved` identifica juntas que não podem mover nada. `quiet: true` retorna poses apenas das ocorrências que se moveram. `motion.export {"study":..., "samples":n}` retorna poses sem alterar o projeto.

## Verificar
- **Cada junta:** `execute occurrence.properties {"occurrence":"Wing:1"}` (menos de 1 KB) dá a caixa global e o centro de massa da ocorrência. Adicionar a junta não deve movê-los.
- **Cada acionamento:** calcule o valor esperado de cada seguidor e confira em `joint.list` ou nos `values` de `joint.drive`, com `unit`. Para um atuador da raiz R à ponta T em um painel articulado em H: `L(θ) = |T(θ) - R|`, com T girado por θ em torno do eixo por H; o curso é `L(θ) - L(0)`. Um ciclo fechado coincide até 1e-6 mm.
- **Cada parada:** resolva onde o limite é atingido e compare com o valor alcançado. A bisseção coloca a parada a aproximadamente 0.01 graus.
- **Vínculos:** b deve ser exatamente r·a + o, salvo quando excede seus limites; então b fica no limite e deixa de acompanhar o espelhamento.
- **Movimento completo:** use `motion.play` em alguns passos ou um `motion.export` e compare as poses com o plano.
- **Visualizar:** conectado ao aplicativo, `screenshot` após `motion.play` mostra a janela naquele passo.

## Salvar
- Retorne à pose planejada (`motion.play` no passo 0 ou `joint.drive`) e use `save {path}`.
- Exportar `.urdf` omite cada junta que fecha um ciclo, pois URDF é uma árvore.

## Sinais de alerta

| Ideia | Faça isto |
|---|---|
| "As origens estão no mesmo ponto, então `flip` não importa" | Passe `flip: true`, ou a peça gira 180 graus. |
| "O acionamento retornou valores, então moveu algo" | Confira `moved`, `reason`, `unconnected` e as cadeias `floating`; uma cadeia flutuante não move nada. |
| "Interpretar `joint.drive` como uma lista de radianos" | Leia os objetos com `value` e `unit`: as respostas atuais usam graus/mm. O `offset` de vínculo angular continua em radianos. |
| "Definir chaves para todas as juntas do ciclo" | Defina apenas a junta que aciona; o ciclo acompanha. |
| "Conferir a pose com uma listagem completa de componentes" | Use `occurrence.properties` para uma ocorrência; para explorar a montagem, `component.list {"compact":true}` omite listas extensas e transformações identidade. |
| "Abrir o projeto com `file.open`" | Use a ferramenta `open` ou `doc.open`; não existe `file.open`. |
