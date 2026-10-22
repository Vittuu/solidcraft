# 3MF com etiquetas de faces B-rep

Versão 1 (2026-10). Namespace `https://github.com/bherbruck/solidcraft/formats/3mf-face-tags/2026/10`,
prefixo `ft`.

Um corpo tesselado perde suas faces: uma malha simples informa onde está a superfície, não de qual face B-rep
cada triângulo veio. Ferramentas posteriores que geram malhas ou analisam uma peça precisam recuperar essa informação. Elas aplicam
cargas e condições de contorno a faces, mantêm contornos de faces como arestas de operações e agrupam
resultados por face. SolidCraft, portanto, exporta um 3MF em que cada triângulo carrega o índice da
face B-rep sobre a qual está (**Arquivo → Salvar como malha…**, formato **3MF com identificadores de face**, ou `file.export` com
`format: "3mf-tagged"`).

## Por que 3MF

O arquivo é um **pacote 3MF** comum (3MF Consortium, *3D Manufacturing Format Core
Specification* 1.3). As etiquetas são um atributo em um namespace próprio do SolidCraft. A especificação
Core permite esse atributo em `<triangle>` e `<object>`, e todos os consumidores o ignoram
porque o namespace não está listado em `requiredextensions`. Fatiadores, visualizadores e a própria importação
de malhas do SolidCraft abrem o arquivo como 3MF simples. Uma ferramenta que conhece o namespace também obtém as faces.
Um formato binário específico precisaria de sua própria especificação e leitores e não abriria
em nenhum outro lugar. Coordenadas decimais não perdem nada: gravadas na forma mais curta de ida e volta, cada
`f64` é lido de volta bit a bit.

## Exemplo

```xml
<?xml version="1.0" encoding="UTF-8"?>
<model unit="millimeter" xml:lang="en-US"
       xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02"
       xmlns:ft="https://github.com/bherbruck/solidcraft/formats/3mf-face-tags/2026/10">
 <metadata name="Application">SolidCraft</metadata>
 <resources>
  <object id="2" type="model" name="Body1" ft:faces="7">
   <mesh>
    <vertices>
     <vertex x="0" y="0" z="0"/>
     ...
    </vertices>
    <triangles>
     <triangle v1="0" v2="2" v3="1" ft:face="0"/>
     ...
    </triangles>
   </mesh>
  </object>
 </resources>
 <build>
  <item objectid="2"/>
 </build>
</model>
```

## Atributos

| Atributo | Em | Valor | Significado |
|---|---|---|---|
| `ft:faces` | `<object>` | inteiro ≥ 1 | Quantidade de faces B-rep do corpo. Todos os `ft:face` do objeto são menores que esse valor. Faces sem triângulos ainda contam. |
| `ft:face` | `<triangle>` | inteiro ≥ 0 | A face B-rep (baseada em 0, na ordem própria de faces do corpo) sobre a qual o triângulo está. |

Os triângulos de um objeto são todos etiquetados ou nenhum é. Um leitor trata um objeto sem etiquetas como uma
única face (etiqueta 0).

## O que o SolidCraft grava

- `unit="millimeter"`, Z para cima. Há um `<object type="model">` com um `<mesh>` por corpo,
  nomeado pelo corpo, e um `<item>` por objeto, sem transformação nem componentes.
- A tesselação é a malha de exportação (tolerância de corda 2·10⁻⁴ do tamanho do corpo). É soldada
  de forma estanque: vértices a uma distância de até 10⁻⁶ do tamanho do corpo são mesclados, e junções em T entre faces são
  divididas. Cada peça de um triângulo dividido mantém sua face. Triângulos são anti-horários vistos de
  fora, e triângulos degenerados são descartados.
- Coordenadas são os próprios valores `f64` da tesselação na forma mais curta de ida e volta, e vértices
  são compartilhados por valor exato.
- Cores usam os próprios `basematerials` do 3MF (`pid`/`p1`), como na exportação 3MF simples, independentemente
  das etiquetas.
- Os bytes são determinísticos: ordem fixa de partes, horários fixos de zip, nenhuma saída ordenada por hash.

## Versões

Uma versão posterior recebe um novo namespace (`…/3mf-face-tags/<year>/<month>`). Um leitor aceita as
versões que conhece e trata um namespace desconhecido de etiquetas de faces como não etiquetado.
