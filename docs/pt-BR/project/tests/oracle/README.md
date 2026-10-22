# Testes de referência (o oráculo)

Cada diretório é um caso:

- `recipe.json`: a peça, escrita no formato de receita do SolidCraft (esboços em coordenadas
  locais ao esboço, operações com seus parâmetros, faces e arestas selecionadas por um ponto global).
  `cargo xtask oracle` a repete pelo mecanismo de comandos do SolidCraft.
- `expected.json`: o que a peça deve medir e de onde vêm esses números.
- `reference.stl` (opcional): uma triangulação fina da peça esperada, usada para verificar a
  própria forma (distância de superfície), não apenas os totais.

Execute todos os casos com `cargo xtask oracle`; ele grava `docs/oracle.md`.

## expected.json

```json
{
  "source": "analytic",
  "method": "box 40 x 30 x 20: V = abc, A = 2(ab + bc + ca)",
  "body_count": 1,
  "volume_mm3": 24000.0,
  "area_mm2": 5200.0,
  "center_of_mass": [20.0, 15.0, 10.0],
  "bbox": { "min": [0.0, 0.0, 0.0], "max": [40.0, 30.0, 20.0] },
  "faces": 6
}
```

| Campo | Significado |
|---|---|
| `source` | `"analytic"` (geometria de forma fechada calculada manualmente; `method` fornece as fórmulas) ou `"freecad <version>"` (criado da mesma receita por `tools/freecad-oracle/`). |
| `method` | Como os números foram obtidos: fórmulas para casos analíticos, a construção para os do FreeCAD. |
| `body_count` | Quantidade de corpos sólidos. |
| `volume_mm3`, `area_mm2` | Totais de todos os corpos. |
| `center_of_mass` | Centroide de todos os corpos ponderado pelo volume (densidade uniforme). |
| `bbox` | Caixa delimitadora de todos os corpos alinhada aos eixos. |
| `faces` | Opcional. Somente onde a geometria determina a quantidade: uma face é um trecho suave máximo de uma superfície (a lateral de um cilindro completo é uma face; vizinhas coplanares são uma face). |
| `compare` | Opcional. Restringe a comparação a estas chaves (por exemplo `["volume_mm3", "area_mm2"]` onde os kernels criam legitimamente trechos de canto diferentes); `note` explica o motivo. |
| `tolerance` | Substituições opcionais: `rel` (volume, área; padrão 1e-3), `abs_mm` (centro de massa e caixa delimitadora, padrão 1e-3 × a diagonal da caixa), `surface_mm` (distância da superfície STL). |
| `note` | Observações opcionais. |

Nenhum valor aqui foi medido em, ou exportado de, um produto CAD comercial.
