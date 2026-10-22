# Conjuntos de contato e movimento de juntas

Conjuntos de contato restringem **Acionar juntas**, a reprodução de estudos de movimento e amostras de movimento exportadas.
Corpos pertencentes a membros diferentes de um conjunto ativo são verificados ao longo do movimento
de junta solicitado. Uma colisão entre posições inicial e final livres pode, portanto, interromper o movimento,
incluindo o percurso de uma junta deslizante através de um obstáculo e o de uma junta revoluta passando por ele.

```json
{"command":"contact.create","params":{"occurrences":["Base:1","Carriage:1"]}}
{"command":"joint.drive","params":{"joint":"Rail","value":30}}
```

Um comando interrompido retorna `stopped_by_contact`, identificando o par de ocorrências e a
fração do movimento solicitado alcançada. O primeiro intervalo de colisão detectado é refinado
para deixar os corpos pouco antes da penetração, dentro da tolerância de contato da tesselação.
Vínculos de movimento e limites participam das verificações da trajetória. Mover um corpo entre
componentes preserva seu posicionamento para verificações de contato por meio do deslocamento do corpo.

Um acionamento também pode parar onde um ciclo fechado de juntas (quatro barras,
biela-manivela) não consegue mais fechar; nesse caso retorna `stopped_by_loop`, com a junta
cujo ciclo abre e a fração alcançada. Quando ambos se aplicam, para no primeiro e informa
apenas esse motivo. O diálogo Acionar juntas mostra o valor alcançado e o motivo na prévia,
e a linha de estado os repete após OK.

A trajetória usa amostras limitadas de geometria tesselada em cache. O espaçamento considera o
deslocamento efetivo das juntas, movimento angular, distância dos pivôs das juntas e larguras de corpos inteiros
ao longo de direções amostradas de faces. Isso lida com placas finas inclinadas, mantendo prático
o acionamento de corpos curvos. Não é detecção contínua exata de colisão: paredes localmente finas em
um corpo maior, direções de faces não amostradas e detalhes geométricos muito pequenos continuam sendo limitações.

Quando um movimento ou montagem excede o limite de amostragem, geometria ou trabalho de colisão, o
comando retorna um erro e restaura o documento. Use incrementos menores de acionamento ou um
conjunto de contato menor. Corpos de malha em conjuntos de contato são atualmente rejeitados para acionamento com verificação de trajetória.
Uma montagem já sobreposta no início mantém o comportamento de recuperação que permite
mover-se para fora desse posicionamento inválido. Desative o contato para mover livremente.

`motion.export` usa as mesmas verificações entre suas posições solicitadas, incluindo exportações esparsas,
e mantém o documento de origem inalterado. Verificações de contato interpolam valores de juntas entre
essas posições; não reconstroem quadros-chave intermediários do estudo de movimento omitidos pelos
passos de amostragem solicitados.
