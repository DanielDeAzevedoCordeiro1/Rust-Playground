# Mathfix (Apenas para aprendizado)

## Se trata de uma simples lib para testar a integracao e uso em outras linguagens.

A ideia e compreender como uma lib e consumida por outras linguagens.
Compreender assuntos como:

[ABI](https://doc.rust-lang.org/reference/abi.html) Define como código binário se comunica.   
[FFI](https://doc.rust-lang.org/nomicon/ffi.html) Permite chamar código de outra linguagem via um ABI comum.  
[cdylib](https://doc.rust-lang.org/reference/linkage.html) Crate type.  
[ctypes](https://docs.python.org/3/library/ctypes.html) API do [Python](https://www.python.org/doc/) para chamar bibliotecas nativas (ABI C)




### Como usar:

A principio ela so pode ser usada em um binario Rust.

Clone a Lib
```bash
git clone https://github.com/DanielDeAzevedoCordeiro1/Rust-Playground.git
```

Acesse o projeto
```bash
cd Rust-Playground
```

## Testando a lib usando Rust

Voce pode testar a lib e/ou alterar o main.rs para testar todas as funcoes. Use o cargo e siga o seguinte passo a passo:

Acesse o projeto rust-test
```bash
cd rust-test
```

Rode o projeto
```bash
cargo run
```

## Testando a lib usando Python

Voce tambem testar usando codigo Python. obs: Nao e necessario ter o python instalado , apenas o docker.

Va para a raiz do projeto (Rust-Playground) e rode o comando (Este comando ira realizar o build da lib e depois move-la para o diretorio python-test):
```bash
bash ./build-python-lib.sh
```

Depois rode este comando (Ele ira gerar uma imagem da (lib/main.py) e subira um container docker):
```bash
bash ./build-docker.sh
```

# Extra

Caso queira alterar o arquivo .py sem precisar gerar outra imagem. Instale o nano no seu container e utilize o nano para editar o arquivo main.py .

Baixe o nano no container python
```bash
apt update && apt install -y nano
```

Altere o arquivo:
```bash
nano main.py
```

Rode novamente:
```bash
python main.py
```