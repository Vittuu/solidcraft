# Exportar a vista atual como DXF

Escolha **Arquivo → Exportar vista atual como DXF…**, escolha se deseja omitir as arestas ocultas ou
exportá-las como linhas tracejadas e, depois, escolha **Exportar DXF…** e um destino. A escolha das
arestas ocultas é lembrada entre execuções. O menu em japonês é **ファイル → 現在の表示をDXFでエクスポート…**.

O desenho usa a câmera no momento em que Exportar é pressionado, incluindo a rotação em torno do
eixo de visão, o eixo vertical do mundo e a perspectiva. Somente corpos visíveis são exportados,
nas posições atuais de seus componentes/ocorrências. Corpos ocultos não são desenhados nem ocultam
outros corpos. Toda a geometria projetada é exportada, incluindo a geometria além das bordas da
viewport. Grade, esboços, cotas, destaques de seleção e outras sobreposições são excluídos.
Desative a análise de seção antes de exportar.

As coordenadas são 2D, Z=0, em milímetros. Vistas ortográficas mantêm dimensões 1:1 no plano da
vista. Vistas em perspectiva mantêm o encurtamento aparente, projetado no plano que passa pelo
alvo da câmera; destinam-se a ilustrações, em vez de fabricação com dimensões. Curvas e silhuetas
curvas são aproximadas por segmentos retos, com tolerância padrão de corda no espaço do modelo
de 0.01 mm. Esta não é uma exportação de curvas analíticas exatas.

O DXF ASCII do AutoCAD 2000 contém as camadas `VISIBLE` (contínua) e, quando solicitado, `HIDDEN`
(traços de 2 mm / intervalos de 1 mm). Curvas ocultas usam entidades `LWPOLYLINE` conectadas com
geração contínua do tipo de linha, para que o padrão não recomece em cada corda. O padrão é
incorporado ao arquivo. As flags da polilinha seguem a
[referência DXF da Autodesk](https://help.autodesk.com/cloudhelp/2017/ENU/AutoCAD-DXF/files/GUID-748FC305-F3F2-4F74-825A-61F04D757A50.htm).

As arestas são divididas onde outra superfície as oculta, incluindo a oclusão parcial por outro
corpo. Arestas ocultas nunca são incluídas no modo de omissão. Diagonais de triângulos e costuras
suaves são excluídas; silhuetas de corpos curvos são incluídas. Desenhos muito complexos retornam
um erro em vez de uma exportação incompleta. Se a geometria atravessar o plano do olho da câmera
em perspectiva, afaste o zoom antes de exportar.

Chamadas programáticas usam `file.export_view_dxf` com `camera: Camera`, `hidden: "omit"` ou
`"dashed"`, `tolerance` opcional (0.0001–1 mm) e `path` opcional. Sem `path`, o comando retorna
o texto DXF. O comando não modifica o projeto nem seu histórico de desfazer.
