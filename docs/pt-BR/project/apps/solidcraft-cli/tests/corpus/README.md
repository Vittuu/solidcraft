# Conjunto de arquivos de projeto

Projetos salvos por versões antigas do SolidCraft. `tests/design_corpus.rs` abre cada um com a
compilação atual e verifica que seja avaliado como quando foi salvo (o `.expected.json` ao lado
é o resultado de `solidcraft-cli eval` desse arquivo pela compilação que o salvou: corpos, volumes, áreas,
caixas delimitadoras globais), que seja atualizado para o formato atual e que variantes danificadas e
hostis nunca entrem em panic.

| Arquivo | Salvo por | Período |
|---|---|---|
| `m0-box-fillet-hole` | 3714342 | M0: esboço, parâmetros, extrusão, arredondamento, corte |
| `m0-revolve-primitives` | 3714342 | M0: revolução, primitivas, combinar, chanfro, edição de linha do tempo |
| `components-arm` | 97d03e3 | componentes e ocorrências: mover, copiar, Colar novo, fixar |
| `joints-hinge` | c2309eb | juntas: dobradiça de revolução acionada por parâmetro |
| `modern-sheet-metal` | bd98035 | flange de contorno de chapa metálica (referência 72) |
| `modern-path-pattern` | bd98035 | padrão ao longo de caminho (referência 53) |
| `modern-plastic` | bd98035 | casca, lábio, ressalto para parafuso, apoio, encaixe por pressão, regra de plástico |
| `naming-split-pieces` | 9a979cf | nomes do formato 13: uma ranhura divide o topo; arredondamento, chanfro e deslocar face nomeiam seus trechos por número (`F2:top#2`) |
| `move-bodies` | 00baf1d | `solid.move` antes de poder copiar: dois corpos, um transladado no lugar, outro transladado e girado 90 deg ao redor de Z (sem chave `copy` em nenhum lugar do arquivo) |
| `split-body-pieces` | v0.3.2 (07e263d) | uma ranhura ao longo de X corta um bloco em dois corpos; um arredondamento, um chanfro e uma aparência referenciam as partes pelo nome do corpo (`Body1`, `Body1 (1)`) |
| `issue-489-stacked-join` | v0.6.0 (809ecf7), desktop | uma extrusão de 70 × 84, depois uma faixa de 70 × 62 unida à face superior, com esboço parcialmente sobre as arestas projetadas da face (issue #489; valores esperados de main em 2880525) |
| `issue-616-lid-ribs` | v0.7.0 (a9879fa), CLI | uma tampa arredondada com um anel de saia, uma janela e uma ranhura passante, depois quatro nervuras unidas à placa sobre a saia, alinhadas com os lados da janela; v0.7.0 deslocava as nervuras e deixava uma fenda através da saia e da placa (issue #616; script `examples/regressions/lid_ribs_flush_with_window.json`, valores esperados da correção) |

Os scripts que os criaram estão em `scripts/` (`issue-489-stacked-join` foi desenhado no aplicativo de
desktop e não tem script; `issue-616-lid-ribs` é criado por seu script de regressão). Cada face plana de cada corpo final também deve aceitar um esboço vinculado e um
plano deslocado; as faces que ainda não aceitam estão listadas com suas issues em `design_corpus.rs`.

Nunca gere novamente um arquivo com uma compilação mais recente: o propósito é que tenha sido gravado pela antiga. Adicione novos arquivos para novos períodos.
