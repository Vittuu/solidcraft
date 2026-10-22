# B-rep nativo portátil, esquema 2

Este é o modelo de dados de intercâmbio de geometria do SolidCraft em `solidcraft_geom::brep`.
Não depende de nenhum backend de modelagem. É a base do cache nativo
descartável de #303; não é um instantâneo de sessão editável nem substitui STEP para
intercâmbio externo. O leitor de contêiner, o codec binário e os adaptadores de núcleo são
camadas separadas. Salvamentos do aplicativo com dados já calculados podem emiti-lo como o cache nativo opcional
descrito no [formato de contêiner](design-container.md#cache-nativo-avaliado).

## Geometria e topologia

Coordenadas são IEEE-754 binary64, em milímetros, com sistema de mão direita e Z para cima. Ângulos são
radianos. Referências inteiras são índices locais à casca de 32 bits sem sinal, nunca endereços
do núcleo ou IDs de execução. A versão do esquema é um campo explícito de 32 bits sem sinal.
Um corpo contém cascas ordenadas; uma casca contém vértices, arestas e faces ordenados.
A ordem é mantida para o rastreamento de origem e a nomeação de faces do SolidCraft. Cascas fechadas e abertas
podem ser representadas; a contenção de material não é deduzida da ordem das cascas.

Uma aresta referencia seus vértices inicial/final no sentido direto de sua curva. Cada
face contém laços fechados ordenados de usos orientados de arestas. Um uso invertido troca
as extremidades da aresta. Os laços registram o percurso físico orientado pela face. `forward` da face inverte apenas a
normal da superfície; nunca troca parâmetros de superfície nem inverte esses laços armazenados.
Os adaptadores devem converter suas convenções internas de contorno separadamente. Arestas compartilhadas são armazenadas uma vez por casca. Arestas fechadas podem referenciar o mesmo vértice de extremidade; adaptadores individuais podem rejeitá-las.
O validador verifica referências e laços fechados conectados. Não certifica
fechamento estanque, coincidência geométrica, ausência de autointerseções ou estado confiável
de modelagem; a extração do backend tem requisitos adicionais.

Curvas são registros matemáticos identificados por etiquetas:

- `line`: pontos inicial/final, parâmetro linear de 0 a 1.
- `polyline`: pontos ordenados; o segmento i ocupa o intervalo de parâmetro de i a i+1.
- `spline`: vetor completo e não decrescente de nós e pontos de controle cartesianos. O grau é
  a quantidade de nós menos a quantidade de pontos de controle menos um.
- `rational_spline`: a mesma base com coeficientes homogêneos `[x*w,y*w,z*w,w]`,
  mantendo os coeficientes originais em vez de normalizá-los/reaproximá-los.
- `circle_arc` e `ellipse_arc`: um sistema de coordenadas ortonormal, raio/raios e um
  intervalo angular crescente. A posição é origin + x*rx*cos(t) + y*ry*sin(t).
- `intersection`: uma spline polinomial 3D comum explícita (nós e pontos de controle
  cartesianos), tolerância positiva de ajuste em milímetros e duas referências opcionais
  a faces locais à casca como indicações. Nenhuma superfície de suporte, guia, solucionador
  de backend ou convenção de busca de parâmetros é exigida para avaliá-la.

As superfícies incluem planos (origem e eixos independentes de parâmetros), splines polinomiais e
racionais de produto tensorial e registros analíticos explícitos. Sistemas analíticos de coordenadas
armazenam origem e eixos x/y ortonormais; z é o produto vetorial de x por y. Intervalos angulares são radianos:

- cilindro: raio, intervalo angular e intervalo de altura axial;
- cone: raio na origem, meio ângulo, intervalo angular e intervalo de altura; o raio na
  altura h é radius_at_origin + h*tan(half_angle);
- esfera: raio, intervalos de longitude e latitude;
- toro: raios maior/menor, intervalos de longitude e meridiano;
- revolução: perfil, origem, eixo unitário e intervalo angular. Rotacione o perfil
  em torno desse eixo usando a regra da mão direita. Não há matriz afim persistida
  nem sinalizador de orientação/troca de parâmetros do backend.

Posições de cilindro/cone são origin + h*z + r(h)*(cos(u)*x + sin(u)*y).
Posições de esfera são origin + r*(cos(v)*(cos(u)*x + sin(u)*y) + sin(v)*z).
Posições de toro são origin + (R+r*cos(v))*(cos(u)*x + sin(u)*y) + r*sin(v)*z.
Normais analíticas apontam para fora; a orientação da face pode invertê-las. Cada adaptador
deve preservar o lugar geométrico e os contornos explícitos orientados pela face, mesmo
quando sua execução usa outra parametrização.

## Validação e confiança

O esquema 2 limita a 1,024 cascas, 100,000 itens combinados de topologia/geometria, aninhamento
de geometria a 16, grau de spline a 16 e pontos de controle por dimensão de spline a
4,096. Números devem ser finitos e limitados em magnitude a 1e12. Ordem de nós, domínio ativo
de spline, multiplicidades, grades retangulares, pesos racionais positivos,
planos não degenerados, sistemas ortonormais de coordenadas, eixos unitários de revolução, raios positivos,
intervalos válidos angulares/de altura e referências a faces de interseção são verificados antes de chamar um adaptador. São proteções estruturais e de recursos,
não um certificado de validade geométrica. O leitor binário também deve limitar
bytes, alocação e aninhamento antes de criar essas estruturas.

Testes do esquema constroem e validam uma face triangular aberta sem construir nem
vincular um corpo do núcleo. Estruturas específicas do núcleo devem aparecer somente dentro de adaptadores;
nunca devem adquirir autoridade sobre nomes de campos ou codificações persistidos. Um
segundo backend ainda exige um adaptador implementado e testado antes de afirmar
interoperabilidade entre núcleos.

Certificados de origem de faces, histórico de operações, aparências e identidades persistentes
do documento pertencem a um envelope versionado de corpo/estado do SolidCraft em torno dessa geometria.
Devem ser preservados pelo codec de cache, não reconstruídos por contagens de faces ou
comparações de ponto flutuante. Um cache decodificado continua provisório até terminar a
reavaliação autoritativa. O documento e os blobs originais continuam suficientes para reabrir
quando um cache, uma representação, um produtor ou um adaptador não são suportados.

## Envelope binário de corpo, versão 3

`SCBREP03` é a assinatura de oito bytes do envelope. O esquema de geometria 2 fica aninhado em
`solid`; o esquema de metadados 1 fica aninhado em `metadata`. São números de versão
separados porque cada contrato pode evoluir independentemente. As assinaturas 01 e 02
foram protótipos de desenvolvimento; 02 introduziu o esquema explícito de geometria, enquanto
03 também separa metadados dos tipos Rust de execução. Versões não suportadas são
ausências de cache, não falhas de migração de arquivos de projeto.

O envelope usa seus próprios registros explicitamente versionados para cor, pintura, contornos
criados, certificados de origem de faces, histórico de operações e proteção de contornos.
Registros de plano, região e segmento também pertencem ao esquema de intercâmbio. Conversões manuais
exaustivas de adaptadores os separam dos tipos de execução FaceSource, History,
FaceProv, EdgeProv, Paint, Seg2, Region2 e Plane. Suas derivações serde ou
refatorações internas não podem redefinir silenciosamente os campos salvos. Nenhum registro serde do truck,
ponteiro, tesselação ou string STEP é gravado. Alterar a semântica de campos
exige uma mudança de versão de formato, mesmo que um tipo Rust mantenha seu nome.

Os valores usam etiquetas: null=0, false=1, true=2, inteiro com sinal=3, inteiro sem sinal=4,
binary64=5, string UTF-8=6, array=7, object=8. Números usam oito bytes
little-endian. Strings e coleções usam comprimentos little-endian de 32 bits sem sinal; chaves
de objetos são UTF-8 com prefixo de comprimento, emitidas em ordem lexicográfica. Os leitores rejeitam chaves
duplicadas, etiquetas desconhecidas, números não finitos e bytes excedentes ao final. Os limites são 64 MiB,
2,000,000 valores, profundidade 48, 100,000 entradas por coleção e 1 MiB por string.

O adaptador truck converte esses registros em topologia de execução somente após
a validação. Rejeita arestas fechadas de um único vértice, polilinhas independentes,
domínios de spline não fixados nas extremidades, superfícies portadoras com intervalo angular incompleto e conversões que
alterariam eixos de planos. Arcos de círculo/elipse usam lugares geométricos quadráticos racionais exatos;
o adaptador pode reparametrizá-los. Transformações de similaridade em revoluções são
incorporadas ao perfil, à origem e ao eixo (incluindo reflexões). Transformações que não são
de similaridade são rejeitadas até que exista uma conversão exata e independente para spline.
Inversões de normais e orientação de contornos do backend são convertidas separadamente.

Interseções procedurais são exportadas com um ajuste cúbico espacial limitado: alvo de ajuste de 1e-4 mm,
4,096 avaliações compartilhadas pelo corpo, no máximo 1,023 peças e
12 subdivisões por intervalo de origem. O alvo é verificado em posições amostradas;
não é um erro máximo certificado entre amostras. Não conseguir ajustar dentro desses
limites rejeita o cache. Uma curva ajustada decodificada é uma spline espacial comum;
a reexportação pode, portanto, usar o registro `spline` em vez de manter sua anotação
opcional de interseção. Essa aproximação é adequada apenas ao cache
provisório, nunca ao estado autoritativo de modelagem.

Corpos decodificados continuam provisórios; o aplicativo nunca deve usar um cache
não confiável como seu modelo editável. A integração no aplicativo o mantém provisório enquanto
reconstrói o documento e as fontes originais. Um segundo
backend precisa de seu próprio adaptador testado antes de afirmar interoperabilidade
entre núcleos.

O arquivo JSON de referência de caixa com arredondamento fixa o envelope decodificado da versão 3,
nomes de campos, etiquetas, topologia, geometria e metadados. Valores inteiros e estruturais
são exatos; coeficientes de ponto flutuante permitem arredondamento relativo/absoluto de plataforma de 1e-10.
A assinatura é verificada separadamente. Isso protege o contrato de intercâmbio contra
alterações acidentais por refatorações de execução.

## Exibição provisória limitada

`native_preview_mesh` amostra no máximo 32 pontos ao longo de cada aresta curva, limita
o contorno de cada face a 4,096 pontos e limita controles de spline e superfícies aninhadas
antes das buscas de parâmetros. As buscas usam no máximo 16 tentativas de refinamento. Perfis de
revolução envolvendo interseções procedurais são rejeitados nesse caminho de exibição.
O gerador separado de malha usa uma grade fixa de no máximo 16 subdivisões por direção
e conexões fixas de emendas periódicas; nunca aciona refinamento adaptativo de arestas,
superfícies ou proporção. Planos usam uma única célula de grade.

Antes da projeção, quem chama reserva um limite conservador de triângulos que abrange
pontos duplicados de contorno projetado, emendas periódicas e a grade do corpo
completo. A saída é verificada em relação a essa reserva e à finitude das coordenadas.
Faces ausentes ou limites excedidos causam uma ausência de cache. Essa malha é uma exibição aproximada,
somente de leitura; a tesselação normal de modelagem/exportação permanece inalterada.
