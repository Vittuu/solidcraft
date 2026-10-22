# Estruturas de teste

`cargo xtask ci` executa tudo que uma pull request deve passar: fmt, `xtask assets`,
`xtask licences --check`, `xtask layers`, clippy com `-D warnings`, os testes com o perfil `ci`
e a verificação de compilação wasm.

| Estrutura | Onde | O que detecta |
|---|---|---|
| Testes unitários e do mecanismo | ao lado do código, `*_tests.rs` | resultados medidos: volume, área, contagens topológicas comparadas com valores calculados |
| Entrada hostil | `hostile_params_never_panic`, `crates/engine/src/tests.rs` | cada comando × cada parâmetro documentado × valores inválidos: erros, nunca panics |
| Fuzzing de esboços | `crates/engine/src/cmd/sketch_fuzz_tests.rs` | esboços aleatórios passando pelo solucionador e pelo localizador de perfis |
| Cenários de interface | `crates/ui-egui/tests/scenarios/*.json` | o aplicativo real, sem janela: cliques, teclas, arrastes, asserções |
| Testes de layout | `crates/ui-egui/tests/ui_layout.rs`, `window_widths.rs` | painéis e diálogos em todas as larguras de janela |
| Conjunto de arquivos de projeto | `apps/solidcraft-cli/tests/corpus` | todas as versões de arquivos salvos continuam abrindo e sendo avaliadas da mesma forma |
| Encerrar durante o salvamento | `apps/solidcraft-cli/tests/kill_mid_save.rs` | um salvamento encerrado a qualquer momento deixa um arquivo legível |
| Testes de referência | `cargo xtask oracle` → [docs/oracle.md](oracle.md) | peças reconstruídas a partir de `tests/oracle/*/recipe.json`, comparadas com valores esperados analíticos ou do FreeCAD |

## Alertas de dependências

Execuções agendadas e disparadas manualmente de CI têm um job separado **Alertas de dependências**;
novos alertas não fazem falhar pull requests sem relação com eles. Para executar a mesma verificação localmente:

```sh
cargo install cargo-deny --version 0.20.2 --locked
cargo deny --locked check --warn unmaintained advisories bans sources
```

O `deny.toml` versionado rejeita vulnerabilidades e registros/fontes git desconhecidos.
Crates sem manutenção, versões retiradas e versões duplicadas geram avisos; as licenças continuam cobertas
por `cargo xtask licences --check`. Revise falhas por alertas e atualize dependências, em vez
de adicionar exclusões amplas. A primeira verificação levou à atualização para quick-xml 0.41.0 por causa de
[RUSTSEC-2026-0194](https://rustsec.org/advisories/RUSTSEC-2026-0194) e
[RUSTSEC-2026-0195](https://rustsec.org/advisories/RUSTSEC-2026-0195).

A cobertura exclui a correspondência de versões das correções por caminho local: `truck-shapeops`,
`truck-stepio`, `truck-meshalgo`, `matext4cgmath` e `egui-wgpu` em `vendor/`. Seu código
versionado e suas correções ainda precisam ser comparados com os projetos originais de licença permissiva; uma verificação
verde não comprova sua segurança nem incorpora automaticamente uma correção do projeto original.

## Cenários de interface

Um cenário é uma lista JSON de etapas executadas no aplicativo real com um contexto egui sem janela:

```json
[
  {"start": "sketch.create"},
  {"click": {"plane": "XY"}},
  {"start": "sketch.rectangle.two_point"},
  {"click": {"sketch": [0, 0]}},
  {"click": {"sketch": [40, 30]}},
  {"expect": {"errors": 0}}
]
```

Um novo arquivo em `crates/ui-egui/tests/scenarios/` é descoberto automaticamente. Para registrar um bug
antes de corrigi-lo, adicione uma etapa `{"pending": "why"}`; o teste passa a ser ignorado com esse motivo, e
a correção remove a etapa.

## Perfil de CI

CI executa os testes com `--profile ci`: otimização leve, tabelas de linhas, sem LTO e com asserções de depuração
desativadas. Algumas asserções de depuração do truck falham com geometria válida; o perfil `ci` as desativa
e compila mais rápido que `--release`. Se um teste falhar apenas em uma compilação de depuração comum, informe
isso na PR.

Execute `cargo xtask test` para compilar com esse perfil e executar os binários comuns e os testes de documentação
com uma quantidade limitada de CPUs, incluindo os testes comuns do engine. Depois desse trabalho, quatro regressões
do engine que medem tempo ou recursos do processo rodam, cada uma, em seu próprio processo isolado, e a matriz
de ações roda sozinha. Seus limites originais não devem medir a contenção do runner.
Argumentos após `--` são passados a cada binário de teste, por exemplo `cargo xtask test -- shell`.
Com qualquer argumento desse tipo, toda a suíte do engine roda em sequência e isolada para preservar
os filtros e opções solicitados. Para executar os binários de teste em sequência, use
`cargo test --workspace --profile ci`.

## Divisão dos testes de CI e testes pesados

No GitHub, um job compila os testes em um arquivo
do [cargo-nextest](https://nexte.st/), e quatro jobs executam um quarto dos testes cada
(`--partition slice:N/4`, configurações em `.config/nextest.toml`); testes de documentação são executados em seu próprio job.
Pull requests pulam alguns testes pesados (cada um leva de um a quatro minutos); eles são executados a cada push para
`main`, nas execuções noturnas e em `cargo xtask test`:

- `solidcraft-engine`: `tests::hostile_params_never_panic`, tudo em `tests::robustness` e
  `tests::robustness_goldens` (pull requests que alteram `crates/kernel` executam sua parte `sample_*`),
  `pattern_suppress_tests::enclosure_arc_extinction_keeps_a_real_sharp_inner_lip_after_resize_and_reopen`,
  `pattern_suppress_tests::enclosure_datums_follow_wall_height_and_board_depth_with_material_proofs`,
  `cmd::plastic::tests::enclosure_snap_fit_follows_resized_front_wall`
- `solidcraft-kernel`: `tests::boolean_identity_on_tangent_primitives`
- cenários de `solidcraft-ui-egui`: `acceptance_sensor_enclosure`

A **matriz de ações** (`apps/solidcraft-cli/tests/action_matrix.rs`) tenta cada ação comum
(esboço, plano deslocado, pressionar/puxar, furo, ângulo de saída, casca, arredondamento, chanfro, geometria de construção, mover,
espelhamento, padrão) em cada face, aresta, vértice e corpo de um conjunto de projetos, comparando com uma
referência versionada: uma ação que passava nela e falha agora faz o teste falhar. Cada execução verifica uma
amostra fixa; o job noturno "Action matrix" executa tudo. Depois de uma correção, registre o que passa agora com
`cargo xtask action-matrix --bless`. Os resultados e as causas das falhas estão na
[página da matriz de ações](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/action-matrix.md).

Para executar exatamente o que uma pull request executa: `cargo nextest run --cargo-profile ci --profile ci`;
adicione `--ignore-default-filter` para o conjunto completo. Um novo teste que leva mais de um minuto em CI pode
entrar na lista (o `default-filter` em `.config/nextest.toml`) se proteger a robustez, em vez
do comportamento cotidiano.
