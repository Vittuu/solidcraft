# A interface do kernel e o truck

`crates/kernel` é o kernel B-rep do SolidCraft. Ele encapsula
[truck](https://github.com/ricosjp/truck) (Apache-2.0) para topologia, geometria NURBS, operações booleanas,
tesselação e interpretação STEP, e é a única crate autorizada a usar o truck. Quem o chama recebe
tipos do SolidCraft (`Body`, índices de faces e arestas, malhas, medidas) e um
`KernelError` em vez de tipos do truck e panics.

## `guard`

O truck pode entrar em panic com geometrias que não consegue tratar. Cada chamada passa por
`solidcraft_kernel::guard`, que captura o panic e o transforma em
`KernelError::Internal`, deixando o modelo inalterado:

```rust
pub fn guard<T>(what: &str, f: impl FnOnce() -> Result<T>) -> Result<T>
```

Uma operação que falha mostra um erro na linha do tempo; ela nunca derruba o aplicativo.

## O que é próprio do SolidCraft

O truck não tem arredondamentos, chanfros, cascas nem edições de faces, então estes são código do SolidCraft no
kernel: arredondamentos de raio constante, por corda e de raio variável com concordâncias nos cantos, chanfros, casca e
deslocamento, Excluir face, Dividir face, Substituir face, costura, leitura e gravação STEP e IGES além do
interpretador do truck, e proveniência de faces (de qual face de entrada veio cada face de saída, para
[nomenclatura persistente](naming.md)).

## Crates truck incluídas no repositório

Cópias corrigidas de `truck-shapeops`, `truck-stepio` e `truck-meshalgo` ficam em `vendor/`,
conectadas por `[patch.crates-io]`. As correções resolvem problemas de robustez encontrados por nossos testes e pelo
conjunto de arquivos STEP. `vendor/` é excluído de nossas regras de fmt e clippy, e cada correção deve ser enviada
ao projeto original quando possível.

Cilindros iguais que se cruzam (`tangent_cylinders.rs`) se tocam em dois pontos, pelos quais
as interseções baseadas em malha do truck não conseguem passar. Quando uma booleana desse
par falha, o kernel divide ambas as faces cilíndricas pela linha que passa pelos pontos de
contato e calcula a booleana dos corpos divididos; a correção de shapeops que escolhe o
canto da face em um vértice onde várias curvas terminam conecta os braços do cruzamento.

Algumas asserções do truck só são executadas em compilações de depuração e podem falhar com geometria válida; por isso, testes e CI
usam `--release`.
