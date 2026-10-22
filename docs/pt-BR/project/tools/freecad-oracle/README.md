# Oráculo FreeCAD

Valores de referência de alguns casos em [`tests/oracle/`](../../tests/oracle/README.md) são calculados
criando o `recipe.json` do próprio caso uma segunda vez em outro kernel: FreeCAD
(OpenCASCADE). Esses casos têm `"source": "freecad <version>"` em seu `expected.json`.
Casos com respostas de forma fechada são calculados manualmente (`"source": "analytic"`,
veja [`tools/oracle-analytic/`](../oracle-analytic/README.md)).

## FreeCAD é uma ferramenta externa, não uma dependência

O FreeCAD é executado como programa separado (`freecadcmd`) que executa `build.py` pela API
pública de scripts Python do FreeCAD (Part, PartDesign, Sketcher, MeshPart). O SolidCraft não usa,
vincula, inclui no repositório nem distribui código, biblioteca ou binário do FreeCAD ou OpenCASCADE, e nada neste
diretório faz parte da compilação. Portanto, a licença LGPL do FreeCAD não se aplica ao
código do SolidCraft. Os scripts aqui são próprios; foram escritos somente com base na documentação pública da API
do FreeCAD, nunca em seu código-fonte.

Somente valores de nossas próprias receitas são produzidos: os números em `expected.json` (quantidade de corpos,
volume, área, centro de massa, caixa delimitadora) e a superfície triangulada em `reference.stl`.

## Arquivos

- `build.py`: executa dentro do FreeCAD. Lê o `recipe.json` de um caso e cria a peça com
  operações de modelagem do FreeCAD, seguindo o significado atribuído pelo SolidCraft à receita
  (`apps/solidcraft-cli/src/recipe.rs`): esboços (resolvidos pelo solucionador de esboços do FreeCAD quando a receita
  aplica restrições, caso contrário nas coordenadas registradas), extrusão (distância, simétrica, dois
  lados, através de tudo, inclinação), revolução, varredura, transição, furos (simples, ponta de broca, rebaixo cilíndrico,
  escareamento), padrões retangulares/circulares/ao longo de caminho e espelhamento (a ferramenta de cada operação de origem é movida
  e sua operação aplicada novamente), combinar, casca, ângulo de saída, arredondamento e chanfro (arestas escolhidas
  mais próximas do ponto da receita, cadeias tangentes acompanhadas), dividir corpo e planos de construção.
  Depois mede o resultado.
- `emit.py`: exibe um resultado ou o grava como `expected.json` / `reference.stl`.
- `run.sh`: o ponto de entrada.

## Execução

```sh
tools/freecad-oracle/run.sh tests/oracle/17-loft tests/oracle/42-overlapping-patterns   # print
tools/freecad-oracle/run.sh --write tests/oracle/17-loft                                # write
```

Sem `--write`, cada caso exibe uma linha. Com `--write`, grava `<case>/expected.json`
(`source`, `method`, `body_count`, `volume_mm3`, `area_mm2`, `center_of_mass`, `bbox` obtido de
`optimalBoundingBox()`) e `<case>/reference.stl` (binário, deflexão linear de 0.005 a 0.02 mm,
no máximo cerca de 400 KB; quando não é possível, o STL é omitido e `note` informa). Ele nunca
sobrescreve um `expected.json` cuja fonte seja `analytic`. Configurações de comparação por caso
(`compare`, `tolerance`, `note`) ficam em `CASE_META` de `build.py`.

`freecadcmd` é encontrado por `$FREECADCMD`, depois por `freecadcmd` em `PATH`, depois por um AppImage
extraído neste diretório (`squashfs-root/`, ignorado pelo git). Localmente: instale FreeCAD 1.x pela distribuição ou extraia o
AppImage (`./FreeCAD_*.AppImage --appimage-extract`) e aponte `FREECADCMD` para
`squashfs-root/usr/bin/freecadcmd`. Em CI (Debian/Ubuntu): `apt install freecad`, depois execute o
script; não é necessário monitor.

Os valores foram produzidos com FreeCAD 1.1.4. Outra versão do FreeCAD pode diferir nos últimos
dígitos; gere novamente apenas quando uma receita mudar e confira os novos valores com o SolidCraft
(`solidcraft-cli oracle`) antes de fazer commit.
