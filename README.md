<div align="center">
<img src="https://user-images.githubusercontent.com/94945604/209778095-4f7be4fa-df2f-4e4c-8fa1-043a5473cc8e.png" width="150px" alt="Tabnews Rust Version" />
<h1>tabnews-rs</h1>
</div>

Um wrapper pra API do [Tabnews](https://www.tabnews.com.br)

## TODO:

- [x] Rotas `/contents`.
  - [x] Listar posts
    - [x] Da página inicial
    - [x] De um determinado usuário
  - [x] Listar comentários
  - [x] Thumbnail de um post
  - [x] Obter Tabcoins de um post
  - [x] Downvote/upvote em um post
  - [x] RSS
  - [x] Publicar posts/comentários
  - [x] Obter post raiz
  - [x] Obter post pai
- [x] Rotas `/user`
- [x] Rotas `/users`
  - [x] Editar perfil
  - [x] Criar perfil
  - [x] Listar informações de um usuário `/users/{username}`
  - [x] Listar todos os usuários
- [x] Rotas `/analytics`
- [x] Reescrever as docs de cada função para usar o formato com `TabnewsClient`
- [ ] Usar a url de homologação nos testes
- [ ] Escrever exemplos

## Agradecimentos

[tabnews-rs](https://github.com/fadiinho/tabnews-rs) foi inspirado pelas seguintes libs:

[Gustavosta](https://github.com/Gustavosta) - Criador da lib [TabNews.py](https://github.com/Gustavosta/TabNews.py)

[miguel-l-b](https://github.com/miguel-l-b) - Criador da lib [tabnews.ts](https://github.com/miguel-l-b/tabnews.ts)

## Licença

Projeto sob a licença [MIT](https://github.com/fadiinho/tabnews-rs/tree/main/LICENSE)
