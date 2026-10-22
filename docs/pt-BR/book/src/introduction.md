# SolidCraft

[English](https://bherbruck.github.io/solidcraft/)

O SolidCraft é um aplicativo CAD 3D paramétrico de código aberto escrito inteiramente em Rust.
Crie um esboço em um plano, aplique restrições e cotas, transforme perfis
em sólidos com operações em uma linha do tempo e altere um parâmetro para ver toda a linha do tempo
ser reconstruída.

![A placa de exemplo no SolidCraft](generated/sample-plate.png)

Ele funciona como:

- um **aplicativo para desktop** para Windows e Linux (egui + wgpu);
- um **aplicativo web** no navegador (wasm, WebGPU ou WebGL2), [experimente aqui](https://bherbruck.github.io/solidcraft/app/);
- uma **CLI sem interface gráfica**, `solidcraft-cli`, que executa scripts de comandos, faz medições e exporta;
- um **servidor MCP** com o qual agentes de IA modelam, sem interface gráfica ou controlando o aplicativo em execução.

Tudo o que o usuário pode fazer é um *comando* com parâmetros JSON. A barra de ferramentas, os diálogos, os scripts,
a CLI, o canal de controle e o MCP executam os mesmos comandos. Assim, tudo o que você pode clicar também pode
ser automatizado em um script, testado ou delegado a um agente.

O SolidCraft está em uma fase inicial. Esboços, modelagem de sólidos, parâmetros, componentes com juntas, chapas metálicas
e um subconjunto de ferramentas para carcaças plásticas funcionam; superfícies, desenhos técnicos, CAM e simulação estão adiados. O
[roteiro](https://github.com/bherbruck/solidcraft/blob/main/docs/pt-BR/ROADMAP.md) traz os detalhes, a
[lista de operações](dev/features.md) relaciona todos os comandos por área e os
[testes de referência](dev/oracle.md) conferem os resultados de modelagem com valores analíticos e do FreeCAD.

Os projetos `.solidcraft` existentes e os comandos de `solidcraft-cli` mantêm seus nomes e comportamento.

Este livro tem duas partes: o **guia do usuário**, para quem modela com o SolidCraft, e o
**guia do desenvolvedor**, para quem trabalha no projeto. A [referência de comandos](reference/commands.md) é
gerada a partir do código a cada compilação.

O SolidCraft é licenciado sob MIT OR Apache-2.0. Ele é independente: seu comportamento é definido a partir de documentação
pública, convenções gerais de CAD e comentários dos usuários, e os testes de referência usam valores
analíticos e ferramentas de código aberto. O SolidCraft não tem vínculo com a Autodesk; Fusion é uma marca registrada
da Autodesk, Inc.
