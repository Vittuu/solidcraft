# Contrato numérico de superfície opaca

Primeira etapa numérica de [#1004](https://github.com/bherbruck/solidcraft/issues/1004),
sob #813/#620. `solidcraft_render::surface` é uma referência original, independente de dispositivo,
para o futuro consumidor integrado. Ela não altera aparências, comandos, formatos de documento,
capturas ou renderização da vista existentes. Nenhum backend anuncia
PBR devido à existência deste módulo. Nenhum código, conjunto de dados ou arquivo de recurso externo é adicionado.

## Admissão e cor

`OpaqueSurface::new` aceita canais de refletância sRGB linear em [0, 1], fração metálica
em [0, 1], rugosidade perceptual em [0.045, 1] e IOR relativo do dielétrico
em [1, 3]. Todos os valores devem ser finitos. Campos privados impedem contornar a admissão;
futuros adaptadores de comandos/desserialização devem chamar esse construtor. O limite inferior de
rugosidade é um limite explícito de prévia, não um espelho perfeito limitado silenciosamente.
Esses são limites de renderização, não uma afirmação de que todos os materiais reais cabem neste modelo.

`srgb8_to_linear` decodifica canais de cor existentes com a função de transferência sRGB por trechos,
incluindo seu trecho linear inicial. Papéis de textura, outras primárias de origem, radiância HDR,
exposição, mapeamento de tons e codificação de saída são responsabilidades separadas. Densidade
mecânica, constantes elásticas, medidas dependentes das condições e eixos de material
não são entradas deste contrato visual.

## Resposta e limitações

`brdf(normal, view, light)` retorna a resposta RGB em esterradianos inversos. As direções
apontam para fora da superfície e são normalizadas pelo escalonamento da componente máxima, permitindo
magnitudes extremas finitas de entrada sem overflow. Direções nulas/não finitas falham;
direções tangentes à superfície ou abaixo dela retornam zero. Consumidores aplicam radiância
incidente e o cosseno da luz exatamente uma vez, depois exposição/transferência de exibição separadamente.

O termo especular usa GGX isotrópico, mascaramento/sombreamento Smith separável e Fresnel
de Schlick. Alfa é a rugosidade perceptual ao quadrado. A refletância normal do dielétrico é
`((ior - 1)/(ior + 1))²`; a fração metálica a interpola em direção à cor base e
remove o substrato difuso. A álgebra avalia `G1/cos(theta)` diretamente para
evitar divisão instável em ângulos rasantes sem limitar o cosseno a um epsilon.

O termo difuso é Lambert ponderado por `(1-metallic) * (1-F(n·view)) * (1-F(n·light))` por
canal. Essa atenuação simétrica de entrada/saída preserva a reciprocidade e evita
adicionar difusão não atenuada à reflexão rasante. É uma aproximação conservadora de
prévia; não há reflexão interna nem compensação de múltiplo espalhamento por microfacetas.
O modelo pode perder energia, especialmente em metais rugosos. Não é uma BSDF em camadas exata,
um modelo espectral de condutor nem um ajuste de material derivado de medições.

Não são fornecidos transmissão, refração, absorção, filme fino, anisotropia, espalhamento subsuperficial,
mapeamento de texturas, IBL, cáusticas ou integração de caminhos. Em particular, este
modelo opaco não deve ser usado como substituto de vidro baseado em opacidade.

## Evidências e critérios restantes

Os testes usam valores analíticos independentes de incidência normal, pares de direções recíprocas,
entradas rasantes/de escala extrema, admissão inválida e valores de referência sRGB. A quadratura
determinística do hemisfério de 256×256 verifica refletância branca em frações metálicas
0/0.5/1, rugosidade 0.25/0.5/1, IOR 1/1.5/3 e cossenos de vista 0.2/1, com margem numérica
de 0.025. Essa grade finita não é uma prova universal de conservação de energia e
não resolve os lóbulos mais estreitos admitidos. Quadratura por importância de lóbulos estreitos
e paridade de dispositivo continuam sendo critérios de aceitação dos consumidores.

Uma atenuação difusa inicial baseada apenas no vetor intermediário falhou no teste de dielétrico branco rasante
(resposta integrada de 1.2324 com rugosidade 0.25 e cosseno de vista 0.2). A formulação simétrica de
entrada/saída a substitui; a tolerância do teste não foi relaxada para aceitar
aquele resultado.

Próximo passo: conectar esse significado por definições de aparência pertencentes ao projeto, comandos
de atribuição existentes e cenas de produto; testar persistência, desfazer e massa inalterada;
instalar os consumidores GPU/CPU realmente integrados e registrar exemplos nativos originais.
Só então anunciar capacidades dos executores. Executores não suportados devem recusar a
nova resposta atomicamente, em vez de renderizar silenciosamente a aproximação escalar legada.
Nenhuma captura de tela é alegada para esta etapa exclusivamente numérica.
