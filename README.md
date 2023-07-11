# DDDating backend

> This is an overview of the project. Go to the submodule to see the implementation details of a specific service.

Dating application backend based on the gRPC microservices ecosystem and Clean Architecture principles

____

### Design features:

- *Rust* language
- *JWT* authentication
- *gRPC* based microservices
- *gRPC* based gateway

### Microservices

- [Gateway](./service/gateway)
- [Authentication](./service/authentication)
    - [Account](./service/account)
    - [Token](./service/token)
- [Conversation](./service/conversation)
- [Matchmaking](./service/matchmaking)
- [Profile](./service/profile)
- [Recommendation](./service/recommendation)
- [Safety](./service/safety)
- [Support](./service/support)