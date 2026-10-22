# Entrada limitada de texturas (admissão de recursos)

`solidcraft_render::texture_input` é um utilitário independente de entrada para #900,
coordenado com #792 e #813. Aceita bytes PNG/JPEG em memória e fornece pixels RGBA8 completos.
Não altera documentos, lê arquivos, atribui materiais, acrescenta formato de pacote de recursos,
agenda trabalho, envia dados à GPU ou anuncia suporte a PBR/triplanar.

`TextureSource::new(bytes, role)` admite cabeçalhos limitados; `decode(cancelled)` valida
a imagem inteira antes de retornar `DecodedTexture`. Origens são imutáveis e compartilhadas.
Admitir o cabeçalho não comprova pixels válidos. Codecs PNG/JPEG executam operações limitadas
sobre a imagem inteira; o host deve usar seu próprio worker para não bloquear a interface.
Cancelamento é verificado antes/depois de decodificar e converter; não interrompe o codec
internamente. Não é a API cooperativa de linhas do utilitário HDR separado.

Papéis são `base_color`, `normal_open_gl`, `roughness`, `metallic`, `ambient_occlusion`.
A cor base declara interpretação sRGB; outros papéis mantêm valores dos canais de dados.
O utilitário não aplica função de transferência, normaliza normais, troca canais DirectX,
desempacota mapas combinados ou infere semântica de nomes de arquivos. Mapas ausentes,
padrões de material e escolhas de amostragem/base pertencem ao futuro adaptador do renderizador.

Limites e exclusões iniciais:

- Armazenamento codificado não vazio, incluindo capacidade do Vec, de no máximo 16 MiB.
- Somente PNG/JPEG, com marcador final PNG IEND / JPEG EOI obrigatório. Isso não constitui
  um validador completo de enquadramento do contêiner/dados posteriores.
- Dimensões positivas, cada uma no máximo 2048; RGBA8 completo tem no máximo 16 MiB.
- Limite de alocação do codec de 64 MiB, separado dos bytes codificados retidos e da conversão
  de saída. Não é garantia combinada de memória CPU/GPU.
- Somente canais de oito bits. Arquivos de dezesseis bits são recusados em vez de quantizados.
- Perfis ICC incorporados e orientação EXIF não trivial são recusados. É preciso converter
  explicitamente espaço de cor/normalizar orientação antes da admissão; não há transformação
  ICC nem correção gama PNG aqui. Cor base sem identificação é entrada sRGB declarada pelo
  chamador, não espaço de cor inferido ou certificado.
- Panics de decodificação/conversão de terceiros são contidos e buffers parciais nunca escapam.
  Alocação GPU, agendamento no navegador e gerações de documento/sessão são responsabilidades do host.

O contrato numérico de superfície opaca está disponível em `surface`; seu consumidor de
produto é integrado separadamente em #1046 sob #1004. O adaptador de textura/ambiente e
a propriedade dos recursos continuam como trabalho separado em #813. Tamanho de repetição/
rotação de madeira, sistemas locais à definição para veios, identidade durável de recursos,
recibos de fonte/licença, empacotamento de documento e rotação/exposição HDR de configurações
devem usar essa representação acordada. Este utilitário não reserva campos ou versão de
documento. A biblioteca escalar de #863 continua distinta.

Ambientes HDR usam `lighting::HdrSource` / `HdrDecodeTask` existentes, não esse caminho PNG/JPEG:
Radiance RGBE, no máximo 16 MiB e 2048x1024, lat-long padrão 2:1, radiância corrigida finita.
Veja [entrada HDR](render-hdr-input.md). Decodificar com sucesso não habilita IBL ou prévia GPU.
O executor atual de snapshot CPU permanece inalterado e não deve anunciar saída HDRI/PBR não suportada.

## Verificação de arquivos licenciados (2026-10-14)

Arquivos foram baixados fora do checkout apenas para validação, pela API pública com um
User-Agent que identifica SolidCraft. Nenhum recurso é distribuído por esta alteração.
[Recursos Poly Haven são CC0](https://polyhaven.com/license); integração da API ao vivo tem
condições separadas de crédito/identidade. Estes resultados verificam entrada, não imagens sombreadas:

| Recurso | Autor / origem | Resultado |
|---|---|---|
| Black Oak Veneer, PNG de cor base 1K | Jenelle van Heerden; [recurso](https://polyhaven.com/a/black_oak_veneer) | Recusado: precisão de canal de 16 bits está fora deste utilitário inicial. |
| Black Oak Veneer, JPEG de cor base 1K | Mesmo recurso/autor | Recusado: ICC incorporado exige conversão explícita de cor. |
| Joburg Central Sunset, Radiance HDR 1K | Greg Zaal e Dimitrios Savva; [recurso](https://polyhaven.com/a/sunset_jhbcentral) | Utilitário HDR existente valida integralmente 1024x512, 1.573.705 bytes codificados, 6.291.456 bytes RGB lineares. Não se alega resultado de iluminação. |

As recusas comprovam deliberadamente o trabalho restante de normalização, não uma integração
bem-sucedida de madeira. Valores MD5 do fornecedor foram conferidos com os downloads selecionados
e recibos SHA-256 locais guardados com metadados de fonte/licença/obtenção. Nenhum deles comprova
autenticação do publicador. Recursos distribuídos exigiriam entradas individuais em ATTRIBUTION.md;
recursos privados/do usuário precisam de seus próprios registros portáteis de proveniência.
Reutilize o conjunto inicial do mantenedor quando disponível, em vez de criar um concorrente.

Testes geram pequenos PNG/JPEG originais em código e verificam valores exatos dos canais,
interpretação de papéis, limites de dimensões/armazenamento, rejeição de precisão, truncamento,
cancelamento e reutilização da origem. Não comprovam desempenho GPU, comportamento interativo
do navegador ou o esquema final de pacotes de recursos.
