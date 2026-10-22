# Matriz de ações

A matriz de ações testa todas as ações comuns em cada face, aresta, vértice e corpo dos corpos
finais de um conjunto de projetos reais, pelos mesmos comandos e parâmetros enviados pelo desktop,
e registra o que funciona. [`tests/action_matrix/baseline.json`](https://github.com/bherbruck/solidcraft/blob/main/tests/action_matrix/baseline.json)
é uma referência progressiva: uma ação que passou nela e falha agora é uma regressão e reprova o teste.
O teste é [`apps/solidcraft-cli/tests/action_matrix.rs`](https://github.com/bherbruck/solidcraft/blob/main/apps/solidcraft-cli/tests/action_matrix.rs).

## Execução

- **Toda execução de CI** (incluindo pull requests) verifica uma amostra fixa, uma ação em 40 pelo hash estável de
  seu nome, em todos os projetos, exceto os sete mais lentos de construir (`action_matrix_sample`, menos de um
  minuto).
- **À noite** (e manualmente, Executar workflow), o job "Action matrix" executa a matriz inteira em quatro
  partes (`action_matrix_full_1` a `_4`).
- **Localmente**: `cargo xtask action-matrix` executa tudo (uma execução registrada com `ACTION_MATRIX_JOBS=6` levou cerca de 22 minutos em uma máquina de 16 núcleos). Adicione `--bless` para
  regravar a referência após uma correção (ações que passaram a funcionar são listadas quando não usa essa opção) e
  `--results FILE` para manter todos os resultados com suas mensagens e o comando enviado.
  `ACTION_MATRIX_FILTER=oracle/23` executa apenas projetos cujo nome contém esse texto;
  `ACTION_MATRIX_JOBS` define quantos projetos são executados simultaneamente. O padrão é no máximo quatro projetos;
  cada processo filho recebe um limite de threads do Rayon igual às CPUs disponíveis divididas pelos jobs de projetos (no mínimo uma).
  Um `RAYON_NUM_THREADS` explícito mantém seu valor. Esses padrões também valem para execuções diretas
  com Cargo e nextest; `cargo xtask test` atribui o mesmo limite à matriz durante sua fase isolada.
  Para uma verificação local rápida, `ACTION_MATRIX_ACTIONS=edge.fillet,edge.chamfer` testa apenas essas ações (nunca
  use bless nessa execução), e `ACTION_MATRIX_VALIDITY=1` adiciona a verificação de corpo de cada ação aprovada (`Body::validity`)
  aos resultados.
  `ACTION_MATRIX_UNFOLD=1` também desdobra e dobra novamente cada corpo de chapa metálica após cada ação
  aprovada, registrando o resultado como `<action>+unfold`. Para verificar novamente um único projeto:
  `ACTION_MATRIX_DESIGN=oracle/69-sm-tray ACTION_MATRIX_OUT=out.jsonl ACTION_MATRIX_ACTIONS=face.hole
  cargo test --profile ci -p solidcraft-cli --test action_matrix action_matrix_worker -- --exact --ignored`
  (a partir de `apps/solidcraft-cli`).

Cada projeto é executado em seu próprio processo, para que uma ação travada (90 s sem resultado) ou que termine abruptamente seja
registrada como `timeout` ou `crash`, e o restante do projeto continue sendo executado. Carregar um projeto antes
de sua primeira ação pode levar até 300 s (os maiores levam várias vezes mais tempo em um executor de CI
sobrecarregado do que aqui). Nenhum levou esse tempo nesta execução.

Peças que compartilham um nome (`#2`, `~3`) são numeradas por sua posição, e os números de peças
quase no mesmo nível podem trocar entre execuções (#569). A referência progressiva, portanto, conta aprovações por família
de peças (o nome sem seus números) e amostra famílias inteiras, para que uma troca não seja uma regressão.

Cada causa em aberto abaixo tem uma pequena reprodução entre os testes ignorados no final de
`action_matrix.rs` (`cargo test --profile ci -p solidcraft-cli --test action_matrix -- --ignored`
lista quais ainda falham; ignore `action_matrix_` para excluir a própria matriz).

## O conjunto de projetos

- `corpus/*`: os arquivos antigos de projeto de [`apps/solidcraft-cli/tests/corpus`](project/apps/solidcraft-cli/tests/corpus/README.md).
- `examples/*`, `regressions/*`: os scripts em [`examples/`](https://github.com/bherbruck/solidcraft/tree/main/examples).
- `oracle/*`: as 76 receitas de referência, reconstruídas (`solidcraft-cli recipe`).
- `demo/enclosure_steps`: a caixa eletrônica construída pelo MCP;
  `demo/pi-case`: a caixa para Raspberry Pi 4 da demonstração do README 0.6, construída pela interface de desktop
  e salva por `examples/demo/enclosure_ui_mcp.py --story pi-0.6 --save`;
  `demo/pi-case-0.8`: a caixa da demonstração do README 0.8 (`--story pi`), com rebordo arredondado, texto
  gravado e material PETG;
  `demo/logo-up-down-join`: o logotipo do SolidCraft conforme relatado em #954, com um esboço
  extrudado para cima e para baixo e unido.
- `typical/*`: [`tests/action_matrix/designs`](https://github.com/bherbruck/solidcraft/tree/main/tests/action_matrix/designs), peças feitas da
  forma comum: extrusões empilhadas, caixa arredondada e com casca, botão por revolução, placa com um
  furo rebaixado repetido em padrão 3 × 2 e contorno chanfrado, suporte em L de chapa metálica com furos,
  união e corte de três corpos, loft sobre uma placa, varredura e um esboço de quadrados desenhados
  lado a lado, extrudado para cima e para baixo e unido (`up-down-join`).

## As ações

Cada uma é executada em uma cópia temporária do projeto. Passa quando o comando funciona, a linha do tempo
não tem novos erros e qualquer esboço que crie está vinculado à sua face (válido, não um esboço legado de face
reencontrado por um ponto). Uma seleção de face é o que o desktop envia: o corpo global, o índice
da face, o caminho de ocorrências e um ponto dentro da face; nunca `fixed`.

| Entidade | Ação | Comando e parâmetros |
|---|---|---|
| face plana | `face.sketch` | `sketch.create` na face |
| | `face.offset_plane` | `construct.plane.offset`, 10% do tamanho do corpo |
| | `face.angle_plane` | `construct.plane.perpendicular` através da aresta reta mais longa da face |
| | `face.midplane` | `construct.plane.midplane` com a face paralela oposta mais próxima |
| | `face.press_pull`, `face.extrude` | `solid.press_pull` / `solid.extrude` (`face`, join), 5% do tamanho (0.5 a 5 mm) |
| | `face.hole` | `solid.hole` bem dentro da face (0.8 × o caminho até sua aresta mais próxima, no máximo 6 mm), através de tudo |
| | `face.draft_neutral` | `solid.draft` das paredes perpendiculares à face, 2°, com o plano da face neutro |
| | `face.shell` | `solid.shell` removendo a face, 1% do tamanho (0.1 a 0.5 mm) |
| cilindro, cone, toro, esfera | `curved.axis` | `construct.axis.through_cylinder` (exceto esferas) |
| | `curved.tangent_plane` | `construct.plane.tangent` |
| aresta | `edge.fillet`, `edge.chamfer` | somente a aresta, 10% da aresta mais curta em suas extremidades (não arestas tangentes, onde não há nada para criar uma transição) |
| | `edge.axis` | `construct.axis.edge` (arestas retas) |
| | `edge.normal_plane` | `construct.plane.along_path`, no meio |
| | `edge.project` | um esboço no plano de origem em que aparece mais longa, `sketch.project` |
| vértice | `vertex.point` | `construct.point.vertex` |
| | `vertex.plane_three_points` | `construct.plane.three_points` com dois vizinhos ao longo de suas arestas |
| corpo | `body.move`, `body.copy`, `body.mirror`, `body.pattern` | `solid.move` 10 mm, e com `copy` ao lado; `solid.mirror` em torno de YZ; `solid.pattern.rectangular` 2 × |

Criar esboço na tampa de um cilindro é coberto por `face.sketch` na tampa plana. Corpos são limitados a
120 faces, 240 arestas e 120 vértices cada. Entidades são nomeadas por seus nomes persistentes
(`F2:top`, arestas `face|face`, vértices pelas faces ao redor).

**A rodada de edição**: uma aprovação em seis também altera um parâmetro de controle (o primeiro parâmetro do usuário;
caso contrário, um de operação, em 10%, um cuja mudança o próprio projeto suporte) e verifica se a nova operação ainda
é resolvida; é registrada como `<action>+edit`.

## Resultados

**30567 de 35576 ações passam (85.9%)** em 104 projetos; a rodada de edição passa em 1370 de 1386.

### Por categoria

| Categoria | Ações | Aprovadas | Taxa |
|---|---:|---:|---:|
| corpo | 472 | 471 | 99.8% |
| curva | 1848 | 1814 | 98.2% |
| aresta | 18763 | 17469 | 93.1% |
| face | 9323 | 5679 | 60.9% |
| vértice | 5170 | 5134 | 99.3% |

### Por ação

| Ação | Ações | Aprovadas | Taxa | Rodada de edição |
|---|---:|---:|---:|---:|
| `body.copy` | 118 | 118 | 100.0% | 6 / 6 |
| `body.mirror` | 118 | 118 | 100.0% | 1 / 1 |
| `body.move` | 118 | 118 | 100.0% | 3 / 3 |
| `body.pattern` | 118 | 117 | 99.2% | 8 / 8 |
| `curved.axis` | 906 | 884 | 97.6% | 39 / 39 |
| `curved.tangent_plane` | 942 | 930 | 98.7% | 44 / 44 |
| `edge.axis` | 2897 | 2846 | 98.2% | 140 / 140 |
| `edge.chamfer` | 3465 | 2863 | 82.6% | 122 / 123 |
| `edge.fillet` | 3465 | 2896 | 83.6% | 126 / 126 |
| `edge.normal_plane` | 4468 | 4413 | 98.8% | 190 / 192 |
| `edge.project` | 4468 | 4451 | 99.6% | 223 / 223 |
| `face.angle_plane` | 1004 | 980 | 97.6% | 51 / 51 |
| `face.draft_neutral` | 895 | 246 | 27.5% | 8 / 8 |
| `face.extrude` | 1123 | 465 | 41.4% | 25 / 26 |
| `face.hole` | 1123 | 726 | 64.6% | 40 / 43 |
| `face.midplane` | 686 | 674 | 98.3% | 25 / 26 |
| `face.offset_plane` | 1123 | 586 | 52.2% | 26 / 26 |
| `face.press_pull` | 1123 | 465 | 41.4% | 25 / 26 |
| `face.shell` | 1123 | 953 | 84.9% | 43 / 45 |
| `face.sketch` | 1123 | 584 | 52.0% | 27 / 27 |
| `vertex.plane_three_points` | 2048 | 2012 | 98.2% | 80 / 85 |
| `vertex.point` | 3122 | 3122 | 100.0% | 118 / 118 |

### Por projeto

| Projeto | Ações | Aprovadas | Taxa |
|---|---:|---:|---:|
| corpus/components-arm | 593 | 354 | 59.7% |
| corpus/issue-489-stacked-join | 243 | 227 | 93.4% |
| corpus/joints-hinge | 344 | 258 | 75.0% |
| corpus/m0-box-fillet-hole | 170 | 158 | 92.9% |
| corpus/m0-revolve-primitives | 344 | 300 | 87.2% |
| corpus/modern-path-pattern | 299 | 294 | 98.3% |
| corpus/modern-plastic | 830 | 634 | 76.4% |
| corpus/modern-sheet-metal | 394 | 273 | 69.3% |
| corpus/move-bodies | 262 | 262 | 100.0% |
| corpus/naming-split-pieces | 285 | 224 | 78.6% |
| corpus/split-body-pieces | 304 | 280 | 92.1% |
| demo/enclosure_steps | 1070 | 991 | 92.6% |
| demo/pi-case | 1780 | 1500 | 84.3% |
| examples/bracket | 202 | 191 | 94.6% |
| examples/sensor_enclosure | 2020 | 1852 | 91.7% |
| oracle/01-box | 131 | 131 | 100.0% |
| oracle/02-cylinder | 41 | 41 | 100.0% |
| oracle/03-box-through-hole | 155 | 151 | 97.4% |
| oracle/04-revolve-vase | 101 | 79 | 78.2% |
| oracle/05-taper-extrude | 95 | 87 | 91.6% |
| oracle/06-symmetric-extrude | 131 | 131 | 100.0% |
| oracle/07-fillet-all-edges | 249 | 249 | 100.0% |
| oracle/08-chamfer | 206 | 188 | 91.3% |
| oracle/09-shell | 274 | 265 | 96.7% |
| oracle/10-rect-pattern-holes | 419 | 415 | 99.0% |
| oracle/11-circular-pattern-flange | 209 | 201 | 96.2% |
| oracle/12-mirror-feature | 193 | 181 | 93.8% |
| oracle/13-combine-join | 231 | 202 | 87.4% |
| oracle/14-combine-cut | 160 | 146 | 91.2% |
| oracle/15-combine-intersect | 108 | 78 | 72.2% |
| oracle/16-sweep | 64 | 56 | 87.5% |
| oracle/17-loft | 90 | 64 | 71.1% |
| oracle/18-hole-features | 286 | 262 | 91.6% |
| oracle/19-l-bracket-constrained | 187 | 187 | 100.0% |
| oracle/20-slot-constrained | 106 | 100 | 94.3% |
| oracle/21-draft | 123 | 99 | 80.5% |
| oracle/22-split-body | 80 | 56 | 70.0% |
| oracle/23-bracket-multifeature | 450 | 381 | 84.7% |
| oracle/24-user-parameters | 155 | 151 | 97.4% |
| oracle/25-sketch-triangle | 102 | 100 | 98.0% |
| oracle/26-sketch-belt | 129 | 125 | 96.9% |
| oracle/27-sketch-trapezoid | 178 | 173 | 97.2% |
| oracle/28-blind-pocket-fillets | 361 | 330 | 91.4% |
| oracle/29-two-sided-and-partial-revolve | 314 | 256 | 81.5% |
| oracle/30-fillet-after-shell | 421 | 406 | 96.4% |
| oracle/31-fillet-l-all-edges | 375 | 371 | 98.9% |
| oracle/32-boss-plate-fillets | 174 | 167 | 96.0% |
| oracle/33-fillet-tangent-chain | 184 | 166 | 90.2% |
| oracle/34-chamfer-then-fillet | 312 | 255 | 81.7% |
| oracle/35-fillet-curved-curved | 73 | 70 | 95.9% |
| oracle/36-corner-blend-two-radii | 242 | 236 | 97.5% |
| oracle/37-pyramid-apex-blend | 169 | 144 | 85.2% |
| oracle/38-coincident-cylinders | 41 | 41 | 100.0% |
| oracle/39-tee-pipe | 119 | 118 | 99.2% |
| oracle/40-box-sphere-torus | 226 | 192 | 85.0% |
| oracle/41-tube-radial-holes | 497 | 495 | 99.6% |
| oracle/42-overlapping-patterns | 1094 | 926 | 84.6% |
| oracle/43-cut-splits-body | 262 | 254 | 96.9% |
| oracle/44-shell-l-bracket | 396 | 390 | 98.5% |
| oracle/45-shell-boss-plate | 335 | 322 | 96.1% |
| oracle/46-shell-two-open-faces | 278 | 268 | 96.4% |
| oracle/47-draft-curved-pocket | 221 | 179 | 81.0% |
| oracle/48-pulley | 298 | 274 | 91.9% |
| oracle/49-helical-sweep | 36 | 18 | 50.0% |
| oracle/50-sweep-tight-bend | 77 | 61 | 79.2% |
| oracle/51-loft-rect-circle-offset | 90 | 64 | 71.1% |
| oracle/52-circular-pattern-fillet | 267 | 226 | 84.6% |
| oracle/53-path-pattern | 299 | 294 | 98.3% |
| oracle/54-mirror-body-fillets | 239 | 209 | 87.4% |
| oracle/55-tapped-hole-cosmetic | 179 | 167 | 93.3% |
| oracle/56-holes-on-curved-face | 143 | 123 | 86.0% |
| oracle/57-hole-pattern-sketch-points | 485 | 456 | 94.0% |
| oracle/58-phone-stand | 635 | 552 | 86.9% |
| oracle/59-enclosure-base | 1029 | 942 | 91.5% |
| oracle/60-pipe-flange | 378 | 356 | 94.2% |
| oracle/61-shaft-keyway | 310 | 231 | 74.5% |
| oracle/62-heat-sink | 1781 | 1524 | 85.6% |
| oracle/63-hinge-knuckle | 432 | 353 | 81.7% |
| oracle/64-sketch-tangent-arc-chain | 173 | 142 | 82.1% |
| oracle/65-sketch-gear-tooth | 170 | 144 | 84.7% |
| oracle/66-sketch-linkage | 201 | 195 | 97.0% |
| oracle/67-sm-base-flange | 131 | 105 | 80.2% |
| oracle/68-sm-edge-flange | 263 | 187 | 71.1% |
| oracle/69-sm-tray | 919 | 599 | 65.2% |
| oracle/70-sm-flange-45 | 261 | 185 | 70.9% |
| oracle/71-sm-custom-rule | 394 | 278 | 70.6% |
| oracle/72-sm-contour-flange | 394 | 273 | 69.3% |
| oracle/73-sm-hem | 395 | 272 | 68.9% |
| oracle/74-sm-unfold-refold | 619 | 439 | 70.9% |
| oracle/75-sm-bracket-holes | 479 | 323 | 67.4% |
| oracle/76-sm-thickness-variant | 263 | 187 | 71.1% |
| regressions/join_struts_on_bore_wall | 544 | 424 | 77.9% |
| regressions/loft_cut_beside_round_hole | 386 | 305 | 79.0% |
| regressions/oblique_cut_of_cylinder | 306 | 269 | 87.9% |
| regressions/oblique_cut_of_cylinder_hang | 306 | 270 | 88.2% |
| regressions/shallow_loft_cut_from_top_face | 307 | 264 | 86.0% |
| typical/filleted-shelled-box | 402 | 386 | 96.0% |
| typical/holes-pattern-chamfer | 536 | 480 | 89.6% |
| typical/loft | 218 | 180 | 82.6% |
| typical/multi-body-join-cut | 309 | 284 | 91.9% |
| typical/revolved-part | 161 | 153 | 95.0% |
| typical/sheet-metal-bracket | 311 | 230 | 74.0% |
| typical/stacked-extrude | 283 | 275 | 97.2% |
| typical/sweep | 280 | 185 | 66.1% |

## Causas

Todas as falhas, agrupadas por causa (5025 no total). Faces que não podem receber um esboço vinculado falham em quatro
ações cada (`face.sketch`, `face.offset_plane` e `face.press_pull` e `face.extrude`, que
criam ali um esboço legado de face).

| Falhas | Issue | Causa | Ações | Projetos | Exemplo |
|---:|---|---|---|---:|---|
| 986 | #556 | Arredondamento ou chanfro de uma aresta cujas extremidades encontram faces curvas, inclinadas ou múltiplas | `edge.chamfer` 496, `edge.fillet` 490 | 46 | corpus/components-arm `ArmBody (Arm:2)/F4:+x\|F4:+y` |
| 684 | #495 | Faces de espessura (borda) de chapa metálica não têm vinculação | `face.extrude` 202, `face.press_pull` 202, `face.offset_plane` 140, `face.sketch` 140 | 12 | corpus/modern-sheet-metal `Body1/F2:face:0` |
| 649 | #390 | Ângulo de saída rejeita paredes cujas vizinhas não são perpendiculares à direção de extração (arredondamentos, outras paredes), ou cantos que mudam após aplicar o ângulo de saída | `face.draft_neutral` 649 | 65 | corpus/components-arm `ArmBody (Arm:2)/F4:+x` |
| 384 | #494 | Revolução, Deslocar face, Ressalto e outras operações de plástico descartam as origens de faces do corpo | `face.offset_plane` 100, `face.sketch` 100, `face.extrude` 92, `face.press_pull` 92 | 11 | corpus/m0-revolve-primitives `Body1/F2:end` |
| 380 | #493 | Paredes de corte, faces de furos e faces de chanfros não têm origem | `face.extrude` 95, `face.offset_plane` 95, `face.press_pull` 95, `face.sketch` 95 | 17 | corpus/split-body-pieces `Body1 (1)/F4:side:l1` |
| 336 | #557 | Cópias de padrão e espelhamento compartilham ou perdem as origens de suas faces | `face.extrude` 84, `face.offset_plane` 84, `face.press_pull` 84, `face.sketch` 84 | 6 | demo/pi-case `Body1/F6:top` |
| 285 | #558 | Seleções em um componente movido ou colado não encontram o alvo (geometria de construção, Furo, Pressionar/puxar, Projetar) | `edge.normal_plane` 55, `edge.axis` 51, `vertex.plane_three_points` 36, `face.extrude` 34 | 2 | corpus/components-arm `ArmBody (Arm:2)/F4:+x` |
| 226 | #559 | Um Furo passante ao longo de uma parede ou ao lado de faces curvas: o corte falha | `face.hole` 226 | 46 | corpus/m0-revolve-primitives `Body1/F2:end` |
| 155 | #560 | Furo em uma face de chapa metálica é rejeitado | `face.hole` 155 | 12 | corpus/modern-sheet-metal `Body1/F2:face:0` |
| 146 | #561 | Faces de Loft, Varredura e Espiral não têm origem | `face.offset_plane` 38, `face.sketch` 38, `face.extrude` 35, `face.press_pull` 35 | 10 | oracle/16-sweep `Tube/F3:end` |
| 116 | #431 | Arredondamento ou chanfro ao longo de uma aresta curva: a esfera não cabe ao longo da aresta | `edge.chamfer` 68, `edge.fillet` 48 | 16 | corpus/modern-sheet-metal `Body1/F2:face:14\|F2:face:16` |
| 100 | #492 | Uma face dividida em peças por uma operação posterior | `face.extrude` 25, `face.offset_plane` 25, `face.press_pull` 25, `face.sketch` 25 | 5 | corpus/issue-489-stacked-join `Body1/F2:top#[F2:side:l1,F4:side:l5]` |
| 98 | #491 | Uma face que União mesclou a partir de faces de duas operações | `face.sketch` 27, `face.offset_plane` 25, `face.extrude` 23, `face.press_pull` 23 | 12 | corpus/issue-489-stacked-join `Body1/F2:side:l2` |
| 86 |  | Outras rejeições de Casca: uma face removida encontrando um arredondamento tão grande quanto a parede, faces de forma livre (loft), paredes mais finas que o dobro da casca (esperado, não registrado) | `face.shell` 86 | 22 | corpus/modern-plastic `Body1/F3:face:15` |
| 84 | #566 | Casca de corpos de varredura e chapa metálica: um canto cujas faces não se encontram da mesma forma | `face.shell` 84 | 12 | oracle/30-fillet-after-shell `Box/F3:inner:F2:start` |
| 80 | #562 | Pressionar/puxar ao lado de arredondamentos e vizinhas curvas: a união falha | `face.extrude` 40, `face.press_pull` 40 | 13 | demo/enclosure_steps `Enclosure/F2:start` |
| 69 |  | Transição maior que as faces ao redor (o raio de 10% não cabe; esperado, não registrado) | `edge.chamfer` 38, `edge.fillet` 31 | 11 | corpus/modern-plastic `Body1/F2:inner:F1:+x\|F3:face:19` |
| 56 | #563 | Ângulo de saída e Extrusão inclinada descartam as origens das faces | `face.extrude` 14, `face.offset_plane` 14, `face.press_pull` 14, `face.sketch` 14 | 3 | oracle/05-taper-extrude `Frustum/F2:start` |
| 48 | #564 | O rebordo que Casca deixa não tem origem | `face.extrude` 12, `face.offset_plane` 12, `face.press_pull` 12, `face.sketch` 12 | 8 | demo/pi-case `Body1/F2:top` |
| 18 | #567 | Eixo ou plano tangente em uma ponta de broca: nenhuma face em NaN | `curved.axis` 9, `curved.tangent_plane` 9 | 3 | oracle/18-hole-features `Plate/F5:face:15` |
| 12 | #565 | Dividir corpo e Combinar interseção descartam as origens das faces | `face.offset_plane` 4, `face.sketch` 4, `face.extrude` 2, `face.press_pull` 2 | 1 | oracle/22-split-body `Cyl (1)/F2:bottom` |
| 9 | #568 | Geometria de construção em arestas, faces e vértices do modelo é perdida quando um parâmetro a move | `vertex.plane_three_points+edit` 5, `edge.normal_plane+edit` 2, `face.midplane+edit` 1, `face.shell+edit` 1 | 4 | demo/enclosure_steps `Enclosure/v[F2:top & F4:inner:F2:side:l4 & F4:inner:F3:blend:F2:side:l` |
| 9 |  | Eixo através de uma face que o núcleo reconhece como cilindro, mas o comando não (extremidades de ranhuras da caixa de sensor; ainda não registrado) | `curved.axis` 9 | 1 | examples/sensor_enclosure `Lid/F15:blend:F14:side:l2\|F14:side:l3` |
| 7 |  | Outras falhas na rodada de edição: o corte de furo e tamanhos de transição acima, e uma extrusão da face superior da caixa M0 cujo perfil não é reencontrado (ainda não registrado) | `face.hole+edit` 3, `face.extrude+edit` 1, `face.press_pull+edit` 1, `edge.chamfer+edit` 1 | 4 | corpus/m0-box-fillet-hole `Body1/F2:top` |
| 2 |  | Uma aresta de hélice projetada ao longo de seu eixo é um ponto (esperado) | `edge.project` 2 | 1 | oracle/50-sweep-tight-bend `Bend/F3:side:c1#1\|F3:side:c1#2~2` |

O estado do histórico de faces (`crates/kernel/src/history.rs`, [vinculações a faces](face-attachments.md)):
uma face plana recebe um esboço vinculado quando o corpo carrega um certificado exato de origem para ela
(tampas e laterais de extrusão, caixas, paredes internas de cascas, faces STEP, painéis de chapa metálica) ou, caso contrário,
quando seu nome persistente acompanha o histórico exato (um certificado `lineage`): paredes e pisos de cortes,
chanfros, cópias de padrão e espelhamento, laterais mescladas por União, revoluções, lofts, varreduras, ângulos de saída, inclinações,
rebordos de cascas, faces das operações de plástico e as faces ao redor, e peças de uma face dividida (pelo
nome próprio da peça quando Deslocar face a afastou do plano das demais). Painéis de chapa metálica, suas
bordas de espessura e as extremidades planas de dobras se vinculam pelo nome. O que ainda rejeita esboços é o que cria faces
sem registrar seu histórico (alguns cortes, Dividir corpo, Interseção): pressionar/puxar e
extrusão a partir de face então recorrem a um esboço legado de face reencontrado por um ponto.
