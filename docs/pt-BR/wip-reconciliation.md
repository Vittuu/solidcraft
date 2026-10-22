# Reconciliação de branches em andamento (#6)

As branches são anteriores aos IDs de comandos separados por pontos. Suas alterações restantes foram portadas para os
IDs com pontos e reconciliadas com a implementação atual.

| Branches | Resultado |
|---|---|
| `solidcraft-sk`, `solidcraft-d2` | Portados `62d22e2` e `e0b3138`: esboços/projeções reconhecem componentes posicionados; chaves de cache incluem posicionamentos; diálogos de operações movidas resolvem seleções globais; Juntas usa linhas da árvore do modelo. |
| `solidcraft-b4` | Portado `2c18dac`: imagens de referência têm componente proprietário, e origens nomeadas de juntas acompanham posicionamentos de componentes. |
| `solidcraft-asm` | Portado `bc05805`: feedback de encaixe de juntas, limites de acionamento, menus de juntas e controles de edição/reprodução de Estudo de movimento. |
| `solidcraft-shell`, `solidcraft-shell-b` | Setas da árvore do modelo, tríade compartilhada de Mover ocorrência, personalização da barra de ferramentas, seletor de espaços de trabalho e rótulos de estado já estão em main. Seus cenários continuam ativos; o cenário de seta de Juntas agora também está ativo. |
| `solidcraft-kdev3`, `solidcraft-kdev`, `solidcraft-kdev2` | Integradas anteriormente como `67c79ec`; as referências antigas já foram removidas. |
| `solidcraft-kdev4` | O trabalho anterior de robustez, casca, transições, wasm e medição já está presente. Portada a costura de prismas empilhados restante de `9f98449` e reparada a seleção de faces de contornos curvos. |
| `solidcraft-params(-build)` | Integrada anteriormente como `517b7a9`. |

As referências de origem do trabalho em andamento são mantidas até a integração deste lote. Exclua-as somente após a main
integrada conter as alterações reconciliadas e passar na CI.

A cobertura de regressões inclui esboços da raiz em componentes transladados e rotacionados, movimentos posteriores
de componentes, salvar/reabrir imagens de referência, origens de juntas durante movimentos pendentes, recarregamento de diálogos
de operações movidas, setas da árvore do modelo, construção do exemplo e cenários de interface. `cargo xtask ci` cobre testes
nativos, wasm, camadas e atribuição de recursos.
