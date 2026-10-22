# Trabalhando com instâncias individuais

Cada posicionamento de ocorrência tem um caminho completo de IDs de ocorrências, da raiz do projeto até esse posicionamento. Uma definição filha reutilizada sob várias instâncias de pais tem um caminho completo diferente em cada pai. Nomes são rótulos; caminhos identificam posicionamentos durante renomeações e registros de posição.

Selecione um posicionamento por `select.set`:

```json
{"items":[{"type":"occurrence","path":[2,4]}]}
```

Propriedades aceita o ID da ocorrência folha e o caminho completo. Mover pode usar coordenadas globais, incluindo um pivô global de rotação:

```json
{"occurrence":4,"path":[2,4],"space":"world","translate":[10,0,0],"capture":false}
```

O movimento global inclui posições pendentes de pais e filhos. O espaço padrão continua local ao pai para scripts existentes. Registrar posição confirma transformações pendentes; Restaurar posição as descarta. Copiar uma ocorrência copia a transformação efetiva daquela ocorrência selecionada, incluindo sua posição pendente.

Uma definição de ocorrência dentro de um componente é compartilhada por todas as instâncias desse componente pai. Mover, fixar, copiar ou excluir essa definição aninhada altera todas as instâncias de seus pais. Selecionar seu caminho exato determina o sistema global usado para movimento e Propriedades; não cria uma definição filha independente.

Ative um posicionamento para criar geometria compartilhada por seu sistema de coordenadas:

```json
{"component":3,"occurrence":4,"path":[2,4]}
```

A nova geometria pertence ao componente e aparece em todas as instâncias. O sistema de coordenadas selecionado determina como entradas globais se tornam geometria local do componente. Esboços planos e novos fios 3D são exibidos no posicionamento ativo durante a criação. A ativação somente por componente mantém o comportamento do primeiro posicionamento. O formato 10 armazena um marcador explícito de coordenadas Global ou Componente para amostras e pontos de ajuste de fios 3D. Fios antigos desenhados e desvinculados mantêm suas posições globais; vínculos mantidos de Incluir e de produtores de superfícies conservam seu significado de componente, incluindo referências perdidas. Romper um vínculo preserva seu marcador explícito. Ative a ocorrência desejada antes de fornecer entradas globais a um esboço explícito pertencente a um componente repetido e não ativo.

A visibilidade é uma configuração independente da vista para posicionamentos exatos. Oculte um caminho e todos os descendentes com `browser.visibility`:

```json
{"visible":false,"paths":[[2,4]]}
```

`visible:true` remove a configuração própria de ocultação desse caminho. Um ancestral oculto ainda oculta seus descendentes. Adicione `isolate:true` a uma lista de caminhos visíveis para ocultar outros ramos. Mostre a pasta de corpos para remover ocultações de ocorrências. Caminhos de seleção e ativação são estado da sessão. A visibilidade é salva com o projeto (corpos, posicionamentos, esboços e itens de origem ocultos), mas não é uma etapa de desfazer; o documento salvo também mantém IDs e transformações de ocorrências. Desfazer segue o comportamento existente de limpar a seleção.

Caminhos são verificados quanto a vínculos ausentes, IDs repetidos, incompatibilidades de componentes e profundidade máxima de 64. A expansão de descendentes rejeita mais de 100.000 posicionamentos ou corpos em vez de retornar resultados incompletos. Nomes de corpos calculados normalmente mantêm seus rótulos legíveis de ocorrência; colisões usam IDs completos de ocorrências para que visibilidade e comandos de corpos continuem acessando a origem correta.

Corpos posicionados mantêm a identidade de sua definição separada dos nomes exibidos. Materiais e aparências de corpos continuam compartilhados, com substituições de corpos tendo prioridade sobre substituições de componentes e cores de materiais. Substituições de faces acompanham a geometria posicionada, incluindo posições pendentes, e exportações pintadas mantêm essas cores.

Colar como novo copia o grafo de definições de componentes alcançável e seu histórico de operações. Filhos repetidos continuam compartilhados dentro da nova cópia, enquanto as definições, esboços e parâmetros gerados de dimensões/entradas copiados são independentes dos originais. Materiais e aparências de corpos, componentes e faces acompanham as identidades das definições copiadas. Nomes de saídas de padrão e espelhamento continuam explícitos, portanto operações posteriores de corpos acompanham os corpos copiados.

Colunas de configurações, valores de linhas e regras internas são duplicados para os alvos copiados. Aparências nativas certificadas de faces acompanham a nova origem de operações. Caminhos selecionados de alvo de Relevo cilíndrico e de autoria dentro do grafo copiado acompanham as ocorrências copiadas; um contexto externo incompatível de autoria é rejeitado antes da adoção.

Um corpo reatribuído de um componente externo recebe uma BaseFeature independente antes da primeira operação copiada que o modifica. Os modificadores copiados são então reproduzidos uma vez. O comando informa esses corpos congelados. Parâmetros públicos do usuário continuam abrangendo todo o projeto; vínculos externos mantidos de esboços, projeções e operações são informados. Juntas e registros de movimento totalmente dentro do grafo copiado de ocorrências recebem novas referências. Registros que atravessam sua fronteira são omitidos com avisos.

Copiar avalia em um documento temporário, verifica que corpos originais continuam inalterados e valida medidas dos corpos copiados e ocupação de material amostrada antes de adotá-lo. Operações destrutivas não suportadas entre fronteiras, faces externas configuradas sem origem semântica certificada, corpos de malha reatribuídos, linhas do tempo retrocedidas e limites excedidos de trabalho retornam erros sem uma cópia parcial. Uma cópia bem-sucedida seleciona a nova ocorrência, preserva seu componente ativo e posições pendentes não relacionadas e posiciona a cópia sob o caminho escolhido de ocorrência pai. Ative a nova ocorrência para criar operações nela. Colar como novo preserva o marcador Global ou Componente e as coordenadas armazenadas de cada fio 3D: fios de Componente acompanham o posicionamento copiado, enquanto fios Globais mantêm suas posições físicas globais.

Criar componentes a partir de corpos e Mover corpos para componente agora adicionam operações de transferência de propriedade na posição atual da linha do tempo. Uniões, cortes e seleções anteriores de faces mantêm seus sistemas de coordenadas originais de componentes. Na transferência, a geometria é convertida uma vez para o sistema de destino; operações posteriores criadas nele atuam no corpo transferido. Retroceder a linha do tempo para antes da transferência mostra o proprietário original. Materiais de corpos e identidade de faces sobrevivem à transferência, e uma aparência herdada do componente de origem se torna uma substituição de corpo para preservar seu visual.

Transferências preservam a posição global do corpo através de pais rotacionados ou transladados. Posições pendentes nos caminhos envolvidos de origem e destino são registradas atomicamente com a transferência, para que salvar/reabrir preserve essa posição; posições pendentes não relacionadas continuam pendentes. Desfazer e restaurar ponto de restauração recuperam as posições pendentes originais e o histórico de operações; refazer restaura a transferência registrada. A operação exige um posicionamento de origem e um de destino. Definições compartilhadas de instâncias com vários posicionamentos são rejeitadas porque uma única mudança de propriedade não pode preservar várias posições globais diferentes. Um componente que fornece histórico de transferência fora de sua subárvore não pode ser excluído até que esse histórico dependente seja removido.

Arquivos antigos sem operações de transferência mantêm seu comportamento salvo de atribuição e deslocamento. Nenhum instante de transferência é deduzido do mapa final de propriedade. Colar como novo congela geometria criada externamente após sua transferência de entrada bem-sucedida e remapeia transferências internas, depois reproduz modificadores posteriores uma vez. Todos os comandos de transferência validam e avaliam em um documento temporário antes da adoção.

Novos esboços vinculados a faces capturam o produtor do corpo selecionado, a face certificada e o caminho exato de ocorrências, junto com o posicionamento de autoria do esboço. Uma face suportada acompanha alterações de origem; uma origem ausente ou incompatível informa uma vinculação perdida em vez de escolher outra face próxima. Abrir Editar restaura a ocorrência de autoria salva como contexto da vista sem criar uma entrada de desfazer. Referências antigas salvas de faces apenas por ponto mantêm seu comportamento legado.

`sketch.create` aceita `plane:{face:[x,y,z],body:"world body name",index:faceIndex,path:[occurrenceIDs]}`. Coordenadas e planos personalizados são entradas globais. O caminho opcional deve corresponder ao corpo selecionado. Seleções de faces apenas por ponto devem ser resolvidas de forma única para uma vinculação exata suportada. Para uma face plana não suportada, adicione explicitamente `fixed:true` para manter seu plano atual; isso não cria projeção ativa do contorno. Faces curvas não podem se tornar vinculações planas por essa opção.

Use `sketch.redefine` com o ID do esboço e um novo plano para reparar ou converter uma vinculação. Sua prévia é descartável; Aplicar registra uma etapa de desfazer. Um `author_path` opcional repara explicitamente um posicionamento de autoria ausente. Vincular novamente um proprietário repetido a partir de outro componente ativo exige esse caminho. Alterar o alvo ou converter para um plano fixo desvincula apenas as bordas automáticas antigas da face, mantendo sua geometria atualmente avaliada e seus IDs de entidades; vínculos de projeção não relacionados continuam ativos. Posições pendentes nos caminhos envolvidos de alvo e autoria são registradas com a alteração, enquanto posições pendentes não relacionadas continuam pendentes.

Colar como novo remapeia alvos certificados internos e caminhos de autoria para as definições copiadas. Colar na linha do tempo informa alvos externos mantidos de faces e rejeita transladar essas vinculações até que uma conversão explícita para plano fixo estabeleça geometria independente. Renomear preserva a identidade certificada do corpo; outro produtor usando o mesmo nome de corpo não pode reparar uma vinculação perdida.
