# Exportação URDF

Um projeto com componentes e juntas é exportado como descrição de robô em URDF (Unified Robot
Description Format), o formato lido por ROS, Gazebo, Isaac Sim, PyBullet e MuJoCo. Cada peça
se torna um elo com sua malha, massa e inércia. Cada junta se torna uma junta URDF, ou uma pequena
cadeia delas, que move as peças da mesma forma que o SolidCraft.

- **Desktop:** **Arquivo → Exportar URDF…**
- **Comandos:** `file.export {"path": "robot.urdf"}` (ou `"format": "urdf"`)
- **MCP:** a ferramenta `export` com um caminho `.urdf` ou `format: "urdf"`
- **CLI:** `solidcraft-cli run script.json --out robot.urdf`

URDF sempre exporta a montagem inteira, portanto `bodies` é rejeitado.

## Arquivos

Ao lado de `robot.urdf`, a exportação grava um STL binário por componente, em metros:

- `robot-<component>.stl` contém todos os corpos do componente. Os corpos do componente raiz ficam
  em `robot-base.stl`.
- `robot-<component>-collision.stl` aparece apenas quando um componente mistura sólidos fechados com
  superfícies abertas: contém os fechados, que são a geometria de colisão.

Todos os caminhos são verificados antes de qualquer gravação, incluindo em relação às pastas que um agente pode
usar (`allow_paths`). As malhas são gravadas primeiro e o `.urdf` por último. Se uma gravação falhar no
meio, o erro identifica os arquivos já gravados e aquele que falhou.

## Parâmetros

| Parâmetro | Padrão | Significado |
|---|---|---|
| `mesh_prefix` | nenhum | Prefixo de cada nome de arquivo de malha. Deixe vazio para carregadores que resolvem caminhos em relação à pasta do URDF (Isaac Sim, PyBullet, MuJoCo, yourdfpy). RViz e outras ferramentas ROS precisam de `package://<pkg>/meshes/` ou de uma pasta absoluta. |
| `effort` | 1000 | Limite de esforço da junta, em N (deslizamento) ou N m (rotação). |
| `velocity` | 10 | Limite de velocidade da junta, em m/s ou rad/s. |

URDF exige `effort` e `velocity` em todas as juntas móveis. Quando não são fornecidos, os
padrões são gravados e um aviso informa isso. Um valor que não seja um número positivo é rejeitado.

## Elos

- **`base_link`** contém os corpos do componente raiz e permanece fixo no espaço global. Todas as
  ocorrências fixadas se conectam a ele por uma junta `fixed` em seu local atual.
- **Um elo por posicionamento de um componente.** Um componente posicionado uma vez dá seu nome ao
  elo (`Gripper`). Um posicionado várias vezes dá os nomes das ocorrências (`Finger:1` se torna
  `Finger_1`).
- **Nomes** usam apenas `A-Z a-z 0-9 _` (arquivos de malha também mantêm `-`). Elos, juntas e arquivos compartilham
  um conjunto de nomes, porque o Gazebo rejeita um elo e uma junta com o mesmo nome. Um nome
  já usado recebe `_2`, `_3` e assim por diante. Nomes de juntas são reservados primeiro, para que uma junta mantenha o
  nome que você deu a ela. Nomes de arquivos de malha são comparados sem diferenciar maiúsculas de minúsculas, pois Windows e macOS
  ignoram essa diferença.
- **Massa e inércia** vêm da malha e da densidade do material de cada corpo: massa em kg,
  centro de massa e tensor de inércia em torno dele no sistema de coordenadas do elo, em kg m². Corpos do
  mesmo elo são combinados pelo teorema dos eixos paralelos.
- **Superfícies abertas** são desenhadas, mas não acrescentam massa nem colisão, com um aviso que as identifica.
- **Um elo sem massa** (somente superfícies, nenhum corpo ou um elo intermediário de uma cadeia) recebe uma massa
  simbólica de 1e-6 kg e inércia de 1e-12 kg m², porque o Gazebo descarta elos sem massa e rompe
  a cadeia nesse ponto.
- **Cor:** a cor do primeiro corpo, como material do elemento visual.

## Juntas

| Junta do SolidCraft | URDF |
|---|---|
| Rígida | `fixed` |
| Revoluta | `revolute` em torno de z com limites, ou `continuous` sem eles |
| Deslizante | `prismatic` ao longo de z |
| Cilíndrica | `revolute` z, depois `prismatic` z |
| Pino-ranhura | `prismatic` x, depois `revolute` z |
| Plana | `prismatic` x, `prismatic` y, depois `revolute` z |
| Esférica | `revolute` z, `revolute` y, depois `revolute` z |

- **Cadeias:** uma junta com vários valores se torna uma junta URDF por valor. Entre elas ficam
  elos chamados `<joint>_link1`, `<joint>_link2`. As etapas anteriores à última são chamadas
  `<joint>_dof1`, `<joint>_dof2`, e a última mantém o nome da junta.
- **Valores:** cada valor de junta URDF equivale ao valor do SolidCraft de que se originou: o mesmo ângulo
  (em radianos) ou distância (em metros). Isso vale independentemente do lado da junta mais próximo da
  base. Quando a árvore chega a uma junta pelo lado B, a cadeia percorre o sentido inverso com seus eixos
  invertidos, para que os valores ainda correspondam.
- **Limites** são gravados como `lower <= upper` em radianos ou metros. Uma etapa deslizante sem
  limite recebe ±1000 m, pois URDF exige um, com um aviso.
- **Vínculos de movimento** não são gravados como `<mimic>`. O valor próprio de cada junta é livre no URDF.

O URDF é gravado com os valores atuais de juntas do projeto, com cada peça onde é exibida,
incluindo movimentos ainda não registrados. A exceção é uma peça posicionada pelas juntas: ela é exportada onde suas
juntas a posicionam, pois a recolocam assim que uma delas se move, e um aviso informa isso. Mover uma junta em um simulador posiciona
as peças que o solucionador de juntas coloca onde **Acionar juntas** do SolidCraft as colocaria.

## A árvore

URDF é uma árvore conectada a `base_link`. A exportação a constrói da mesma forma que o solucionador de juntas percorre
a montagem.

1. A partir de `base_link` e das ocorrências fixadas, segue as juntas para fora.
2. Um grupo de peças unidas entre si, mas a nenhuma peça fixada, é fixado onde está pelo
   lado A de sua primeira junta, como o solucionador faz quando nada está fixado. Um aviso informa que a
   peça não está fixada.
3. Uma peça que nenhuma junta alcança é fixada onde está. Uma peça aninhada se conecta à peça pai, para
   mover-se com ela, como no SolidCraft. Uma de nível superior se conecta a `base_link`, com um aviso.
4. Uma junta que fecharia um ciclo é omitida, com um aviso, pois uma árvore não pode contê-la.
5. Uma junta cujos pontos de encaixe não podem mais ser resolvidos (por exemplo, a face em que estava desapareceu) é
   omitida, com um aviso que a identifica.

Quando um componente é posicionado várias vezes, o que está aninhado nele aparece em cada posicionamento. No
URDF, o primeiro posicionamento é aquele que suas juntas movem; os demais são cópias fixas em seu
pai, na posição atual, com um aviso quando a peça copiada tem uma junta. (No SolidCraft,
as cópias se movem juntas, o que um URDF só poderia expressar com `<mimic>`.) A exportação para de copiar após
64 níveis ou 100 000 posicionamentos, com um aviso.

## Fora da cobertura

- `<mimic>` para vínculos de movimento.
- Formas simplificadas de colisão: a malha de colisão é a malha visual.
- Exportação SDF e MJCF e importação URDF.
- Movimento de juntas dentro das cópias de um componente posicionado várias vezes.
- Dinâmica de acionamento: `effort` e `velocity` são limites que você escolhe.
