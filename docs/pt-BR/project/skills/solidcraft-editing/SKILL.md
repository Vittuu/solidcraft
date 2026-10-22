---
name: solidcraft-editing
description: Use ao alterar, corrigir ou ampliar um projeto existente do SolidCraft por seu servidor MCP — um arquivo .solidcraft ou o projeto aberto no aplicativo. Pedidos comuns incluem redimensionar, mover um corte, alterar um furo ou apoio, adicionar arredondamento ou chanfro, tornar um valor fixo paramétrico, ou corrigir uma operação que mostra erro após uma alteração ou falha do aplicativo.
---

# Edição no SolidCraft

## Visão geral
Edite um projeto existente pelos parâmetros em vez de recriá-lo. Cada cota de esboço e entrada de operação já é um parâmetro (`d1`, `d2`, ...), então a maioria das edições é uma `parameters.change` por alteração, enviadas juntas em um lote. Mapeie o projeto com poucas chamadas compactas, em vez de percorrer a linha do tempo operação por operação. Para formatos de comandos e armadilhas, use `reference.md` da skill `solidcraft-modeling`.

## Mapear o projeto (3-4 chamadas)
1. **Abrir:** se o projeto ainda não estiver aberto, execute `open {path}`. Quando o servidor estiver conectado ao aplicativo e o projeto atual estiver `dirty`, pergunte antes de abrir qualquer coisa.
2. **Visão geral:** `inspect_design {"measure":false,"sketches":false}` lista os parâmetros do usuário, as cotas de esboço (`d1`, `d2`, ... com expressões), as operações em ordem e seus erros.
3. **Entradas de operações:** `execute parameters.list {"filter":"feature"}` fornece cada entrada de operação como um objeto, por exemplo `{"name":"d9","expression":"(6) / 2","feature":"Cylinder1","input":"Radius"}`. `inspect_design` as omite.
   - Identifique cotas de esboço pela expressão.
   - Se houver ambiguidade, use `sketch.inspect {sketch}` apenas nesse esboço; ele também mostra os pontos em coordenadas globais.
4. **Referência inicial:** `measure {}`, para prever cada alteração em relação a ela.

`timeline.inspect {feature}` serve apenas para dois casos:
- alterar mais que um número: perfis, direção, operação ou arestas
- verificar se uma posição acompanha um parâmetro, por exemplo a `base` de um cilindro escrita como `width/2 - wall - 6`

Inspecione uma operação de cada tipo, não todas as cópias.

## Alterar
- **Um número:** `parameters.change {"name":"d9","expression":"..."}`.
  - **Para tornar um valor fixo paramétrico:** adicione primeiro um parâmetro do usuário (`parameters.add standoff_d`), depois defina cada entrada como expressão dele (`standoff_d/2`).
  - **Várias alterações:** um `batch`.
- **Um parâmetro do usuário:** `set_parameter` ou `parameters.change`. Todas as expressões que o usam acompanham.
- **Estrutura:** `timeline.edit {"feature":..., "set":{...}}`, com os formatos de campos mostrados por `timeline.inspect`.
- **Uma nova operação** fica no final da linha do tempo. Para inseri-la antes:
  1. `timeline.roll_to {"position":N}`. Uma posição é mais segura que um nome, pois um esboço e uma operação podem compartilhar um.
  2. Adicione a operação.
  3. `timeline.roll_to {}` para retornar ao fim.

  Tudo após a inserção é recalculado, e uma operação que funcionava pode passar a falhar. Caso observado: arredondamento de canto após inserir um chanfro.
- **Edições arriscadas ou em várias etapas:** crie `edit.checkpoint` primeiro em uma chamada própria. `edit.restore_checkpoint {checkpoint}` retorna ao estado exato. Um `batch` desfaz a si mesmo; chamadas `execute` separadas não.
- **Excluir:** primeiro `timeline.dependents {feature}`. Ele lista o que será excluído junto e o que falharia.
- **Texto ambíguo:** "2 mm mais alto" em um valor escrito como `wall + 6` pode ser relativo (`wall + 8`) ou absoluto.
  - Preserve a intenção do projeto, que é a interpretação relativa.
  - Informe ao usuário a interpretação usada. Isso importa principalmente quando o mesmo pedido também altera `wall`, pois as duas interpretações produzem alturas diferentes.
- **Novas constantes**, como dimensão de chanfro, entram em parâmetros do usuário, para que ele possa alterá-las depois.

## Verificar
1. **Prever, depois medir uma vez.** Envie as alterações independentes em um lote, preveja a mudança total de volume e execute `measure` uma vez.
   - Um movimento não altera volume. Para confirmar um esboço movido, leia as coordenadas globais dos pontos com `sketch.inspect`. Você também pode prever o deslocamento do centro de massa, `Δcom = V_moved · Δz / V_body`, mas somente quando nada mais no lote altera o volume.
   - Se corresponder com erro de cerca de 0.01%, está concluído.
   - Caso contrário, restaure o checkpoint e aplique as alterações uma por vez, medindo após cada uma.
   - Um corte ou chanfro que deixa o volume inalterado não atingiu o corpo.
2. **Estado da linha do tempo:** execute `inspect_design {"measure":false,"sketches":false}`. Todos os `error` e `warning` devem ser null. Se algo quebrou, restaure ou use `undo` antes de tentar qualquer outra coisa.
3. **Operações internas:** capture uma vista superior, `{"source":"model","view":"top","width":480,"height":320}`. Quando o servidor está conectado, `view` também gira a câmera do usuário. A vista isométrica oculta apoios atrás das paredes.
4. **Ainda paramétrico:** se o usuário quiser mantê-lo paramétrico, altere uma dimensão de controle, use `measure` e restaure o valor.

## Quando algo já está quebrado
- **Uma operação tem `error` após abrir:** leia com `timeline.inspect`, corrija a entrada ou definição e recalcule.
- **O aplicativo falhou:**
  - `crash_reports` lista os relatórios, e `crash_reports {id}` fornece o comando que falhou.
  - `execute doc.recovery_list` e depois `doc.recover {id}` reabrem o projeto salvo automaticamente.
  - Nunca repita o comando que causou a falha sem alterá-lo.

## Salvar
`save {"path":"...-v2.solidcraft"}` grava uma cópia. `save {}` sobrescreve o arquivo aberto; faça isso somente quando o usuário pedir.

## Sinais de alerta

| Ideia | Faça isto |
|---|---|
| "timeline.inspect em cada operação para entendê-la" | `parameters.list {"filter":"feature"}` mostra todas as entradas em uma chamada. |
| "Excluir e remodelar a operação" | Altere a entrada `dN` ou use `timeline.edit`. O histórico e seleções posteriores permanecem intactos. |
| "A inserção funcionou, terminou" | Verifique os erros de todas as operações posteriores. |
| "Pontos de aresta com expressões, para o chanfro acompanhar a dimensão" | Eles são rejeitados, e seleções armazenadas já acompanham mudanças de dimensão. |
| "Selecionar todas as 8 arestas da borda" | Uma seleção em uma cadeia tangente arredonda ou chanfra a cadeia inteira. |
| "Medir depois de cada edição" | Um lote, um `measure` comparado com a previsão somada. Divida apenas quando não corresponder. |
