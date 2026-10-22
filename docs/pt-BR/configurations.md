# Configurações de projeto

A tabela de configurações armazena variantes nomeadas do projeto. Adicione colunas de parâmetros, supressão de operações,
material físico, aparência de corpo, aparência de componente e aparência semântica de face
com `config.configure` ou `config.column`.
O menu **Adicionar coluna** da Tabela de configurações do desktop oferece os mesmos alvos. Células de material e
aparência têm um seletor de **Biblioteca**; seus campos de texto também aceitam valores salvos exatos.

```json
{"command":"config.configure","params":{"params":["width"],"materials":["Box1"],"appearances":["Box1"]}}
{"command":"config.row","params":{"name":"Aluminum blue","values":{"param:width":"80 mm","material:Box1":"Aluminum","appearance:Box1":"Aluminum - Anodized Blue"}}}
{"command":"config.activate","params":{"row":"Aluminum blue"}}
```

Use o nome real do corpo na árvore do modelo no lugar de `Box1`. `config.table` retorna as chaves de colunas,
linhas, nome ativo e `matches`: se o projeto atual ainda tem os valores da linha ativa.
Alterar uma atribuição manualmente torna `matches` falso. Ativar outra linha pode ser desfeito.

Valores de material são nomes de `material.list` (sem diferenciar maiúsculas de minúsculas), com `Default` removendo a
atribuição de material físico do corpo. Valores de aparência são nomes de `appearance.library`,
uma string vazia ou `Default` para remover a aparência do corpo, ou um objeto de aparência personalizada:

```json
{"command":"config.row","params":{"name":"Transparent blue","from":"Aluminum blue","values":{"appearance:Box1":{"name":"Custom blue","color":[40,90,186],"opacity":0.4}}}}
```

A tabela armazena aparências personalizadas sem perder cor ou opacidade. `color` contém três valores
de byte, nomes aceitam até 128 caracteres Unicode e `opacity` deve ser finita e estar entre
zero e um. Os valores de estilo são validados ao
editar uma linha e novamente ao ativar uma linha salva. Um valor inválido deixa o projeto inalterado.
Remover uma aparência de corpo permite exibir a aparência do componente ou a cor do material;
aparências de faces mantêm sua prioridade existente. Alterações de material físico afetam a massa medida.

`config.row` aceita `from` para copiar outra linha, `rename` para renomear uma linha existente e
`delete` para removê-la. Os nomes devem ser únicos. Para remover uma coluna de estilo, use
`config.column` com `material` ou `appearance` definido como o corpo-alvo e `delete: true`.

Colunas de estilo de corpo usam nomes de corpos como alvo. As atribuições continuam armazenadas quando uma linha suprime a
operação que produz esse corpo e ficam visíveis quando ele retorna.

## Aparências de componentes e faces

**Adicionar coluna** oferece aparências de componentes e um submenu **Aparência de face** organizado por
corpo e função semântica. Você também pode selecionar uma face na vista antes de abrir o menu e usar
**Face selecionada em ...**. Essas atribuições afetam todas as ocorrências que compartilham a definição.
Colunas de componentes usam IDs estáveis de componentes, portanto renomear um componente preserva suas linhas.
Colunas de faces resolvem um corpo da definição por sua origem certificada, nunca por um índice de face exibida salvo.

```json
{"command":"config.configure","params":{"component_appearances":["Part"],"face_appearances":[{"body":"Box1","point":[5,5,10]}]}}
```

Use um ponto na superfície atual para capturar uma face. O menu leva seu alvo já certificado diretamente, evitando uma segunda seleção da face mais próxima em paredes concêntricas finas. Seleções em ocorrências globais são convertidas
para as coordenadas da definição compartilhada. `config.table` retorna chaves canônicas e rótulos
legíveis de alvos. Use a chave de face retornada ao editar uma linha ou excluir uma coluna de face:
`config.column` aceita `face_appearance: "<returned key>"` com `delete: true`.
Colunas de componentes podem ser removidas com `component_appearance: "Part"` e `delete: true`.

Funções estáveis de faces suportadas são faces de eixo de caixa, tampas e lateral não dividida de cilindro, tampas de
extrusão no plano do perfil e tampas de revolução unilateral. Tampas de extrusão com De diferente de zero ou Deslocamento inicial variável/Dois lados e
tampas de revolução simétrica/Dois lados ainda não são certificadas. Faces de malha, nomes de tampas ambíguos,
nomes genéricos derivados de índices e nomes de laterais de esboço baseados em heurísticas de curva mais próxima são
rejeitados. Extrusões Novo corpo sem inclinação também suportam laterais de linhas, círculos e arcos
com certificados exatos de construção de arestas de origem. Nomes comuns de faces e âncoras salvas de esboço ou Relevo mantêm sua nomeação legada; estilos rigorosos resolvem metadados separados de origem e também exigem a operação produtora original. Todas as peças construídas de uma mesma
curva de esboço compartilham uma aparência, incluindo emendas de círculos divididos; curvas coincidentes
distintas são ambíguas e não podem ser selecionadas. Modificadores posteriores não suportados removem
o certificado em vez de transferir a pintura por proximidade. Peças coplanares divididas de uma face semântica suportada compartilham
a aparência configurada. Se essa origem desaparecer enquanto o corpo permanecer, Aplicar e
Prévia rejeitam a linha atomicamente. Não há substituição pela face mais próxima; alterações comuns
posteriores de topologia também não podem transferir sua pintura para uma face substituta. Uma operação proprietária
suprimida mantém a atribuição inativa até retornar.

**Seguir aparência herdada**, na Biblioteca, remove apenas a substituição direta do componente
ou da face selecionada. A pintura de face tem prioridade sobre corpo, componente e material físico;
removê-la revela a pintura herdada ou importada. Renomeações de corpos mantêm os alvos de faces.
Excluir um componente ou a operação de origem remove seus alvos configurados e as células das linhas.
Excluir uma ocorrência preserva a configuração da definição compartilhada.

Faces STEP incorporadas usam certificados vinculados aos dados exatos armazenados, ao item de origem e à
entidade original de face STEP. Peças de emendas do núcleo dessa mesma face de origem compartilham a substituição.
Substituir os dados incorporados invalida o alvo antigo de face, mesmo quando a substituição
tem geometria idêntica ou reutiliza números de entidades. Renomeações de corpos e posicionamentos rígidos exatos
preservam a identidade de origem; alterações comuns de parâmetros reconstroem certificados de construção.

Cópias independentes de componentes duplicam colunas mapeadas de configuração, valores de linhas e regras
sem redirecionar a origem. Uma cópia independente dos mesmos dados STEP incorporados
preserva seu certificado de origem sob a nova operação. Copiar pintura rigorosa em um corpo
externo que exige reexportação ainda falha explicitamente porque esse exportador ainda não consegue
certificar o mapeamento de faces da origem para a saída.

A ativação avalia juntas todas as expressões de parâmetros da linha, para que a ordem das colunas não crie
ciclos temporários de dependências. Prévia e ativação rejeitam erros de geometria novos ou alterados
sem mudar o projeto. Um erro existente idêntico continua visível. Uma linha salva não pode
recriar um parâmetro excluído: remova suas referências de configuração antes de excluí-lo. Excluir
uma operação remove suas colunas de entrada e supressão e todas as regras referentes a essa operação.

## Regras ordenadas e prévia

Abra **Adicionar ou editar regra** abaixo da tabela. Nomeie a regra, escolha quando ela se aplica e então
escolha um parâmetro controlado, uma supressão controlada, uma faixa numérica ou valores permitidos. Campos
numéricos aceitam expressões comuns de projeto e unidades explícitas. **Editar**, os botões de seta,
a caixa de ativação e **Excluir** atualizam regras existentes. Escolha **Todas**, **Qualquer** ou **Não**
para criar condições compostas com controles aninhados, **Adicionar condição** e **Remover condição**.

**Prévia da linha** reconstrói um projeto temporário e mostra expressões efetivas e regras correspondentes ou
ignoradas. Valores controlados e restritos identificam a regra responsável. Ela mantém a geometria
atual, a linha ativa e o histórico de desfazer inalterados. **Aplicar** ou **Ativar** confirma
a variante validada como uma ação que pode ser desfeita.

As regras são executadas na ordem da lista. Cada condição vê os controles anteriores; restrições de regras
correspondentes se aplicam todas aos valores finais. Uma faixa inclui suas extremidades. Valores permitidos comparam
valores numéricos com dimensões correspondentes, portanto `1 cm` equivale a `10 mm`. Um ângulo nunca equivale a um
comprimento. Expressões sem unidade usam a unidade do parâmetro-alvo ou a unidade do documento para entradas de
comprimento de operações. A igualdade usa tolerância relativa/absoluta de `1e-9` em unidades internas.
O seletor **Unidades de comprimento** do editor de regras define o significado de expressões de comprimento
de operações sem unidade; unidades explícitas prevalecem sobre ele. As regras capturam esse contexto de unidades ao serem criadas. Alterar as unidades do documento
preserva expressões mistas como `width + 1` sem tratar `width * 2` como um multiplicador
de comprimento. Células salvas de comprimento de operação sem unidade e limites de regras recebem a unidade anterior
explícita; unidades explícitas e expressões de origem não literais mantêm seu texto. Uma linha ativa
correspondente recalcula seus valores controlados quando as unidades mudam. Parâmetros do usuário mantêm suas próprias unidades.

As regras são recalculadas em alterações configuradas, **Aplicar** e **Prévia da linha**. Alterações comuns de parâmetros
podem deixar a configuração sem correspondência e não impõem restrições continuamente.
Controles entre unidades armazenam comprimentos numéricos efetivos; suas expressões de origem salvas são reavaliadas
no próximo Aplicar ou Prévia, em vez de acompanhar ao vivo alterações comuns posteriores de parâmetros.
Células de origem salvas são preservadas: desativar um controle restaura a escolha original da linha.

```json
{"command":"config.rule","params":{"rule":{"id":"depth-rule","name":"Large parts use deeper stock","condition":{"type":"parameter","parameter":"width","operator":"ge","expression":"80 mm"},"effect":{"type":"drive_parameter","parameter":"depth","expression":"width / 2"}}}}
{"command":"config.rule","params":{"rule":{"id":"width-range","name":"Stock width","condition":{"type":"always"},"effect":{"type":"parameter_range","parameter":"width","minimum":"10 mm","maximum":"100 mm"}}}}
{"command":"config.preview","params":{"row":"Aluminum blue"}}
{"command":"config.rule","params":{"id":"width-range","index":0}}
```

Configure a coluna `depth` antes de adicionar esse controle de exemplo. Condições podem referenciar qualquer
parâmetro existente do projeto; alvos de efeitos devem ser colunas configuradas. `config.rules` lista
as regras. `config.rule` substitui uma regra com o mesmo ID, exclui com `id` e `delete: true`
ou a move para um `index` iniciado em zero. Editar uma regra valida todas as linhas salvas antes de confirmar;
editar uma linha valida essa linha. Geometria inválida, ciclos de dependências, unidades incompatíveis
ou restrições deixam o documento anterior intacto e explicam a falha. Exclua as regras
antes de remover a última linha. Renomeações de parâmetros atualizam alvos e expressões de regras; referências
de regras impedem a exclusão do parâmetro até serem removidas.

As tabelas suportam no máximo 128 regras; condições permitem oito níveis e 64 nós, e listas de valores
permitidos suportam até 128 expressões. Esses limites são verificados antes da decodificação
das regras salvas. Regras de aparência/material controlados e avaliação repetida de ponto fixo estão fora
do escopo atual de regras.

Projetos recém-salvos usam o formato de arquivo 12, incluindo vinculações duráveis opcionais a faces,
identidades analíticas de contornos de deslocamento e referências adaptativas qualificadas de perfis inteiros, incluindo
furos. Projetos antigos são atualizados ao abrir sem
alterar seus valores existentes de configuração. Versões antigas do SolidCraft devem ser atualizadas para
abrir projetos recém-salvos.
