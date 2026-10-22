# Arte da interface

Edite os originais Rust de desenho em `source/`; a crate da interface os compila por
`crates/ui-egui/src/icons.rs`. Use a grade de 24 × 24 e cores semânticas de traço/preenchimento/destaque.
IDs de ícones e sua geometria ficam juntos em `source/icons.rs`;
IDs desconhecidos mantêm a alternativa existente. Adicione novas artes ali sem
manter um registro separado de IDs.

| Fonte | Arte | Quem utiliza |
| --- | --- | --- |
| `icons.rs` | Ícones da barra de ferramentas, navegador, menus e restrições | `icons::paint` |
| `glyphs.rs` | Marca, títulos, controles de reprodução, indicadores, fixador, seta, expansão, centro radial e cruz de remoção | barra de ferramentas, barra de título, linha do tempo, navegador, menus |
| `navigation.rs` | Eixos com sinal, faces/rótulos do cubo e setas | área de visualização/cubo de navegação |
| `markers.rs` | Mapeamentos/letras de restrições, marcadores de ajuste e pontas de seta de cotas | esboço, montagem e cotas |
| `pointer.rs` | Ponteiro e etiqueta do agente | cursor do agente |

Estado, layout, projeção, seleção e comandos de widgets permanecem em seus widgets.
Revise mudanças de desenho no aplicativo real nos temas claro e escuro.
Nenhum projeto nativo separado de design estava disponível para este conjunto. Preserve qualquer
trabalho nativo futuro e entradas vinculadas em `source/work/`, com atribuição.
Arte original é MIT OR Apache-2.0; a geometria dos títulos mantém atribuição ao VectorCraft
no `ATTRIBUTION.md` da raiz.
