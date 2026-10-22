# Entrada Radiance com limites

`solidcraft_render::lighting` fornece um decodificador RGBE cooperativo controlado pelo chamador.
Isso é apenas preparação de entrada: não habilita comando de importação de ambiente, formato
de documento, material PBR, capacidade de iluminação, envio à GPU nem mecanismo de renderização de imagem.
O decodificador existente de imagens de referência aceita explicitamente apenas PNG/JPEG, portanto a unificação de
features Cargo não pode enviar HDR por seu caminho síncrono de carregamento da imagem inteira.
Essa é a base independente de entrada para #813 / #620, coordenada com o
trabalho de renderizador do mantenedor descrito em #792.

`HdrSource::new(Vec<u8>)` verifica um cabeçalho limitado usando o codec existente de metadados
de `image`. Sucesso **não** significa que o fluxo de pixels é válido. Use
`validate(cancelled)` para uma verificação síncrona do fluxo completo, ou crie
`HdrDecodeTask` e chame repetidamente `step(units, cancelled)` para preparação
cooperativa. Somente `Complete(DecodedHdr)` contém pixels prontos para publicação. Cancelamento
e erros liberam buffers parciais; chamadas seguintes retornam `Consumed`, inclusive
após uma conclusão bem-sucedida. A própria função de cancelamento deve ter execução limitada.
O consumidor controla o agendamento e não deve executar `validate` síncrono em uma
thread de interface sensível à latência.

Os limites de entrada são 16 MiB de capacidade do Vec codificado, cabeçalho de 64 KiB, 2048x1024 e
2.097.152 pixels. Apenas RGBE padrão `-Y +X`, proporção 2:1 e pixels quadrados são aceitos.
EXR, XYZE e outras orientações não são suportados. Uma chamada aceita de 1 a 16.384 unidades
de trabalho; cada unidade decodifica um pacote (no máximo 128 gravações de componentes) ou converte um
pixel. Um limite global de trabalho restringe sequências malformadas no estilo antigo. Canais corrigidos
devem ser finitos, não negativos e no máximo 100.000.000. Os limites são uma política intencional
de prévia, não uma afirmação sobre todos os arquivos Radiance válidos.

A alocação de pixels em ponto flutuante é de no máximo 24 MiB, com uma linha RGBE de no máximo 8 KiB
e uma fonte codificada compartilhada de no máximo 16 MiB. A construção reserva armazenamento em ponto flutuante
com possibilidade de falha; a inicialização ocorre de forma incremental. Essa é uma estimativa por tarefa,
não uma cota de memória do processo. O consumidor deve admitir jobs simultâneos, propriedade
das fontes, cópias decodificadas, conversão de cor, texturas GPU e mapas IBL dentro
de seu próprio orçamento total antes de começar.

A conversão RGBE segue a convenção de mantissa/expoente de `image` 0.25.10, sem
mapeamento de tons nem limitação a [0,1]. Valores armazenados são divididos por EXPOSURE
e COLORCORR do cabeçalho. Arquivos de entrada arbitrários não garantem unidades físicas.
`HdrInfo::primaries` preserva cromaticidades PRIMARIES explícitas; sua ausência é
indefinida. A saída é RGB linear no espaço de cor da fonte, **não implicitamente
sRGB linear**. Consumidores devem escolher e validar uma transformação para o espaço de trabalho;
a validação básica de finitude/intervalo das cromaticidades aqui não comprova um gamut inversível
nem fisicamente significativo. Esta ferramenta não faz conversão de gamut.

O adaptador de linhas é derivado do codec `image` de licença permissiva;
veja ATTRIBUTION.md, NOTICE e THIRD-PARTY-LICENSES.txt. Imagens de teste são fixtures originais
de bytes inline. Nenhuma imagem ou material de produto comercial é incluído.
