# Capacidades dos mecanismos de renderização (R1)

Esta página descreve o contrato de metadados R1. Hosts instalados da vista GPU e de
trabalhos CPU de produto o ampliam com perfis completos; preparar entrada HDR não acrescenta
iluminação. Veja [os limites das fases integradas](render-integration.md).

`render.backends` descobre metadados de renderização sem abrir uma janela, sondar uma GPU,
avaliar o modelo nem alterar o documento, a câmera, o estado de alterações pendentes ou o histórico de desfazer/refazer.
Ele também mantém desatualizado um modelo que ficou assim por um gesto de esboço em andamento.

```sh
solidcraft-cli exec render.backends '{}'
```

O canal de controle e `execute` do MCP usam o mesmo comando do mecanismo. Os parâmetros devem ser
um objeto vazio; um valor null de parâmetros é normalizado para `{}` pelo mecanismo. Outros
parâmetros falham com um erro normal de comando.

## Esquema de descoberta

O comando retorna este objeto JSON (versão 1 do esquema):

```json
{
  "schema_version": 1,
  "default_backend": "builtin",
  "backends": [{
    "id": "builtin",
    "label": "Integrated",
    "availability": {"status": "available"},
    "executors": [{
      "id": "cpu-raster",
      "availability": {"status": "available"},
      "profiles": [{
        "mode": "preview",
        "operation": "image",
        "formats": ["png"],
        "features": []
      }],
      "limits": {
        "max_dimension": 4096,
        "max_pixels": 4194304,
        "max_scratch_bytes": 268435456
      },
      "scratch": {
        "sample_scale": 2,
        "sample_bytes_per_pixel": 8,
        "output_bytes_per_pixel": 8,
        "encoding_bytes_per_pixel": 8
      }
    }]
  }]
}
```

Um mecanismo ou executor indisponível tem, em vez disso,
`"availability": {"status": "unavailable", "reason": "..."}`. Esses são descritores de
execução, não configurações do arquivo de projeto. IDs de executores têm escopo em seu mecanismo.
IDs de mecanismo/executor têm de 1 a 64 bytes ASCII: uma letra minúscula seguida por letras
minúsculas, dígitos, `.`, `_` ou `-`. A construção e a desserialização validam os IDs.

Sem hosts de execução, o mecanismo integrado descreve o caminho existente de captura por rasterização
em CPU. Seu perfil `preview`/`image` exporta PNG; ele não declara HDRI, PBR,
refração, renderização progressiva nem um integrador de qualidade final. A área de visualização GPU
existente se registra separadamente quando a interface instala seus dispositivos/pipelines reais.
A vista GPU de produto e o host CPU de imagem instalados suportam o [consumidor PBR opaco](render-opaque-pbr.md)
posterior; isso não altera o descritor raster estático nem habilita HDRI/refração/renderização
final. CPU e GPU são caminhos de execução, não famílias
separadas de mecanismos selecionáveis pelo usuário.

## Resolução pura de solicitações

`solidcraft_render::resolve_request` aceita uma fatia de descritores e um `RenderRequest`.
Ele apenas valida e resolve metadados: não aloca uma imagem, inicia um job,
redireciona `snapshot` nem chama um renderizador. Uma solicitação pode ser desserializada de:

```json
{
  "backend": "builtin",
  "mode": "preview",
  "operation": "image",
  "width": 640,
  "height": 480,
  "format": "png",
  "features": [],
  "policy": "strict"
}
```

`backend` usa `builtin` por padrão quando ausente/null. `features` usa `[]`, e
`policy` usa `strict`. Modo, operação e dimensões inteiras positivas são
obrigatórios. Uma imagem exige um formato suportado; uma área de visualização exige
`format` ausente/null. Campos desconhecidos, valores de enumeração, IDs inválidos e valores com tipos incorretos fazem
a desserialização falhar. Dimensões são verificadas pelo resolvedor. Nenhum campo de ponto flutuante
é introduzido na solicitação.

Os modos são `quick`, `preview`, `final`; as operações são `viewport`, `image`; os formatos
são `png`, `exr`. Capacidades exigidas podem ser `hdri`, `pbr`, `refraction`, `progressive`.
Esses nomes definem solicitações, não promessas de suporte. Cada executor lista perfis completos de
modo/operação com seus formatos e capacidades. Capacidades nunca são
combinadas entre executores para criar um perfil sem suporte.

A resolução rejeita IDs duplicados de mecanismos, IDs de executores dentro de um mecanismo e perfis
duplicados de modo/operação. O registro é limitado a 32 mecanismos, 16 executores por
mecanismo e 16 perfis por executor; uma solicitação permite no máximo 32 entradas de capacidades.
Executores disponíveis são considerados em ordem lexicográfica de ID, e o primeiro executor
que atende à solicitação inteira é selecionado. A ordem do registro não determina prioridade.
Os formatos/capacidades do perfil retornado são ordenados, para que sua ordem de declaração não
altere o resultado resolvido. Se todos os candidatos falharem, o erro do primeiro executor correspondente
é retornado; um perfil sem suporte é informado quando nenhum tem essa combinação.

O valor resolvido contém `backend`, `executor`, `profile`, `width`, `height`,
`format`, `estimated_scratch_bytes` e `warnings`. Na política estrita, os avisos ficam
vazios e nenhuma configuração solicitada é reduzida. Erros são valores tipados `ResolveError`:
IDs/solicitação/registro inválidos, IDs duplicados, mecanismo desconhecido/indisponível, nenhum executor
disponível, perfil/formato/capacidade sem suporte, dimensões inválidas, limites excedidos ou
estouro de tamanho. R1 não tem política de alternativa automática nem API pública de jobs.

## Política de admissão de recursos

Os limites padrão do novo resolvedor são valores conservadores de política, não capacidade medida
do adaptador: de 1 a 4096 pixels por dimensão, no máximo 4.194.304 pixels de saída e um orçamento
estimado de 256 MiB para buffers de pixels. Cada executor fornece `RenderLimits` configuráveis e
um `ScratchLayout`. Aritmética verificada cobre dimensões, superamostragem, produtos de buffers,
somas e o tamanho de alocação endereçável do destino, incluindo wasm32.

Para o caminho existente de rasterização por CPU, a estimativa é de 48 bytes por pixel de saída: quatro
superamostras de 8 bytes cada (RGBA8 mais profundidade f32), uma imagem de saída de 8 bytes e uma
reserva de 8 bytes para uma cópia RGBA e a saída codificada. Em 640 × 480, são 14.745.600
bytes; em 2048 × 2048, são 201.326.592 bytes. Uma solicitação de 4096 × 4096 excede o limite
de pixels, embora cada dimensão seja permitida.

Essa é uma estimativa de admissão, não um limite total de memória do processo. Ela exclui alocações de cena,
textura, geometria projetada, estruturas de aceleração e internas de codecs.
Hospedeiros de execução futuros devem impor sua própria política completa de alocação/cancelamento.
O rasterizador legado, capturas da CLI e área de visualização GPU mantêm o comportamento
e os limites existentes; nenhum deles é encaminhado por esse resolvedor em R1.

O futuro espaço de trabalho Renderização, configurações persistentes, hospedeiros de jobs e mecanismos opcionais são
acompanhados em [#620](https://github.com/bherbruck/solidcraft/issues/620). A base implementada
tem seu escopo definido por [#642](https://github.com/bherbruck/solidcraft/issues/642).
