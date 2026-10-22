# Configurações persistentes de renderização (R2)

Configurações nomeadas salvam a intenção comum de renderização no documento. Não iniciam uma
renderização, aplicam a câmera de navegação ou tornam disponível um backend indisponível.
O espaço de trabalho e os trabalhos integrados as consomem com suas próprias verificações de
capacidade. [PBR opaco](render-opaque-pbr.md) é uma resposta de aparência opcional; recursos de
textura e ambiente continuam como etapas futuras de [#620](https://github.com/bherbruck/solidcraft/issues/620);
veja [os limites da integração combinada](render-integration.md).

## Comandos

```json
{"command":"render.settings.set","params":{"name":"Studio","set":{"width":1600,"height":900,"exposure_ev":1.5}}}
{"command":"render.settings.get","params":{"name":"Studio"}}
{"command":"render.settings.get","params":{}}
```

`set` cria uma configuração nomeada com os padrões ou substitui os campos de nível superior
fornecidos em uma existente. Uma `camera` fornecida substitui a câmera inteira; não é uma
alteração recursiva. Campos omitidos mantêm os valores. Campos desconhecidos (incluindo os de
câmera), tipos e valores inválidos falham antes de alterar o documento. Um novo `set` vazio
salva os padrões; definir uma configuração existente com valores idênticos não faz alterações,
preservando o histórico de refazer e a revisão.
Cada edição efetiva é uma etapa de desfazer e marca o documento como alterado. Não há comando
para excluir ou renomear nesta etapa.

`get` com nome retorna `{name, setup}` ou erro se o nome não existir. Sem nome, retorna
`{setups, defaults}`. Leituras não avaliam o modelo nem alteram o histórico.

| Campo | Padrão | Contrato salvo |
|---|---|---|
| `backend` | `builtin` | ID válido de backend R1, no máximo 64 bytes ASCII; IDs indisponíveis sobrevivem à reabertura |
| `mode` | `preview` | Modo R1 `preview` / `final`; disponibilidade verificada por uma solicitação futura |
| `camera` | `Camera::default()` existente | Captura de alvo, yaw, pitch, roll, distância, fov e up; mm/radianos, padrão existente com Z para cima |
| `width`, `height` | 1024, 768 | Positivos, cada um no máximo 4096; produto no máximo 4.194.304 pixels |
| `format` | `png` | Formato de saída R1 (`png` / `exr`); salvar EXR não acrescenta um codificador |
| `exposure_ev` | 0 | Finito, de -32 a 32 |
| `environment_rotation` | 0 | Radianos finitos, magnitude no máximo 1.000.000 |
| `environment_intensity` | 1 | Finito, de 0 a 100 |
| `background` | [236, 240, 245] | Três valores de bytes sRGB, independentes da iluminação futura |

Coordenadas do alvo e distância positiva da câmera têm magnitude limitada a 1e9 mm;
ângulos, a 1e6 radianos. FOV é finito, `0 <= fov < pi`; zero significa ortográfico no
modelo existente de câmera. Ambos os eixos verticais existentes são preservados. Nomes são
aparados, não vazios, têm no máximo 128 bytes UTF-8 e não contêm caracteres de controle.
Um documento tem no máximo 32 configurações. A desserialização também executa a validação.

## Persistência e avaliação

O formato de documento 21 acrescenta `render: {setups: {name: setup}}`. Projetos de formato 20
e anteriores migram com um mapa vazio; o estado vazio de renderização é omitido ao salvar.
Geometria, aparências, opacidade e materiais físicos existentes são preservados. JSON simples,
contêineres de projeto e reprodução do diário de falhas usam a mesma representação de documento/comando.
Versões anteriores (0.11.0 lê até 20) rejeitam o formato 21 em vez de perder configurações
silenciosamente. Esta etapa acrescenta apenas intenção de renderização. Nenhuma capacidade
de execução, trabalho, textura ou dispositivo é salvo.

O mecanismo evita avaliar a geometria somente após comprovar que o documento difere apenas
nas configurações validadas, com o mesmo esboço ativo e movimentos pendentes. A comprovação
compara o documento completo após normalizar o campo de renderização; edições mistas de
geometria/aparência/posicionamento seguem a atualização conservadora normal. Ela usa clonagem
e comparação do documento, sem alegar tempo constante ou aceleração do aplicativo inteiro.
Ponteiros de renderização inalterados encerram antecipadamente a verificação de comandos normais.

Edições apenas de renderização, desfazer/refazer e restauração de checkpoint preservam o estado
do modelo, seu marcador de desatualização, a tesselação em cache e o estado global preparado.
Um resultado desatualizado de arraste ao vivo nunca se torna definitivo por uma edição de
configurações; o próximo comando que lê o modelo ainda o estabiliza normalmente. A revisão
avança para que consumidores futuros invalidem o trabalho de sombreamento. Acumulação real
de renderização e invalidação de recursos não são implementadas em R2.

Não há nova dependência ou recurso de terceiros. `doc` depende da crate pura `render`
existente para câmera compartilhada e identificadores R1, respeitando a regra L2 → L1.
