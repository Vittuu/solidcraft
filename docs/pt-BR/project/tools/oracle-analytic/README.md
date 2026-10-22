# Valores de referência analíticos

Valores esperados de forma fechada para os casos em [`tests/oracle/`](../../tests/oracle/README.md) cuja forma
permite esse cálculo: prismas, cilindros, cones e troncos, esferas e calotas, toros e sólidos de revolução
(Pappus), arredondamentos em arestas retas (raio constante ou variável ao longo da aresta), cascas,
padrões desses elementos e chapa metálica dobrada (partes planas mais dobras como setores anulares). Cada script é
Python 3 simples (somente biblioteca padrão), com uma função por caso. Executar um script na raiz do repositório regrava o
`expected.json` de seus casos com `"source": "analytic"` e um texto `method` contendo as fórmulas:

```sh
python3 tools/oracle-analytic/cases_basic.py        # 01-29, 82: boxes, holes, revolves, sketches, thin walls
python3 tools/oracle-analytic/cases_features.py     # 32-66: fillets, shells, sweeps, patterns
python3 tools/oracle-analytic/cases_sheet_metal.py  # 67-76: sheet metal
python3 tools/oracle-analytic/cases_fillets.py      # 77-81: variable, chord-length, tapered and spline fillets
```

Onde um valor depende de uma convenção do SolidCraft, e não apenas da geometria (canto
interno vivo de uma casca, alívio de canto de chapa metálica, local de medição da altura de flange), `method` ou
`note` informa a convenção usada. Nunca altere um valor para corresponder à saída do SolidCraft: se
discordarem, descubra qual está errado.
