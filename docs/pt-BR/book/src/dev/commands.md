# O mecanismo de comandos

Cada ação visível para o usuário é um **comando**: um `CommandSpec` registrado em
`crates/engine/src/cmd/`, com um ID em letras minúsculas separado por pontos (`solid.extrude`, `sketch.create`,
`timeline.rollback`). A barra de ferramentas, os diálogos, o menu radial, a caixa S, os scripts, a CLI, o canal de
controle e o MCP executam comandos por `Session::execute(id, params)`. Comandos recebem parâmetros
JSON e nunca abrem diálogos; um diálogo é um formulário da interface que monta os parâmetros.

```rust
CommandSpec::new("solid.fillet", "Fillet", fillet)
    .at("SOLID", "MODIFY")      // toolbar tab and panel ("" = not on the toolbar)
    .icon("fillet")             // drawn in code, assets/icons/ui/source/icons.rs
    .key("F")                   // default shortcut
    .params("edges: [[x,y,z] | {body, index}]; radius: expr; …"),
```

`run` é `fn(&mut Session, &Value) -> Result<Value>`, e `enabled` é
`fn(&Session) -> Result<(), String>`; o texto de erro é a dica do botão desativado. O
texto de `params` é a documentação de uma linha exibida por `solidcraft-cli commands`, pelo
`list_commands` do MCP e pela [referência de comandos](../reference/commands.md).

Um comando que altera o projeto pode ser desfeito por padrão: a sessão captura o estado do documento
antes de executá-lo, e um comando que falha deixa o documento inalterado. Comandos de operações adicionam uma
operação à linha do tempo e deixam o documento avaliá-la; eles validam primeiro as entradas
(unidades, intervalos, referências) e retornam `BadParams` em vez de criar uma operação inválida.

Somente os IDs próprios do registro separados por pontos nomeiam comandos (a correspondência ignora maiúsculas e minúsculas); qualquer outro ID falha
com `unknown command`, que sugere até três IDs próximos: um erro de digitação (`solid.extrud`) ou um
ID que termine com a mesma palavra (`file.open`: você quis dizer `doc.open`?).

## Adicionando um comando

1. Adicione um `CommandSpec` ao arquivo adequado em `crates/engine/src/cmd/` e implemente `run`.
2. Escreva testes do mecanismo que meçam o resultado: volume, área e contagens de faces/arestas/vértices
   comparados com valores que você possa calcular manualmente.
3. O teste de entradas hostis (`hostile_params_never_panic` em `crates/engine/src/tests.rs`) executa
   seu comando automaticamente com cada parâmetro documentado definido para valores inválidos. Ele deve retornar
   erros, nunca entrar em panic.
4. Para trabalho de interface, adicione um cenário (veja [Estruturas de teste](testing.md)).
