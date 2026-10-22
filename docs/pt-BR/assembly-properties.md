# Propriedades físicas de montagens

A ação **Propriedades** de um componente na árvore do modelo mede o posicionamento da montagem selecionada,
incluindo seus componentes aninhados. Selecione a raiz do projeto para medir todas as instâncias da
montagem. A janela mostra massa considerando os materiais, volume, área da superfície, centro de massa e
a caixa delimitadora global. Movimentos pendentes de ocorrências são incluídos sem registrá-los.

As mesmas medidas estão disponíveis por comandos:

```json
{"command":"component.properties","params":{"component":"Bracket"}}
{"command":"component.properties","params":{"component":"root"}}
{"command":"component.properties","params":{"component":"Subassembly","descendants":false}}
{"command":"occurrence.properties","params":{"occurrence":"Bracket:2"}}
```

`component.properties` usa o componente ativo por padrão e mede seu primeiro posicionamento.
`occurrence.properties` mede a instância escolhida. Uma ocorrência aninhada pode aparecer sob
várias instâncias de seu pai; forneça `path` (a lista completa de IDs de ocorrências desde a
raiz) para escolher um desses posicionamentos. Uma ocorrência ambígua sem caminho retorna um
erro. Ambos os comandos aceitam `descendants: false` para medir apenas corpos diretamente pertencentes ao componente.

Os resultados contêm uma lista `bodies` e um objeto `total`. A entrada de cada corpo identifica o nome
original do corpo, o componente proprietário e o caminho de ocorrências. A densidade do material físico é resolvida a partir
desse corpo original, portanto as cópias mantêm seu material e sua massa. Os valores usam milímetros, milímetros
quadrados, milímetros cúbicos e gramas; as coordenadas usam o sistema global.

Massa e centro de massa consideram a densidade de cada corpo. Volume e área são somados entre
os corpos posicionados, incluindo corpos sobrepostos e instâncias repetidas. As medidas usam a
medição de corpos do núcleo baseada em tesselação. Um componente vazio informa totais zero e
centro de massa e caixa delimitadora nulos. Uma falha na medição de um corpo retorna um erro em vez de
apresentar um total incompleto da montagem. Esses comandos mantêm geometria, posicionamentos pendentes
e histórico de desfazer inalterados.
