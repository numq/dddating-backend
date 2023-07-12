# DDDating backend

> This is an overview of the project. Go to the submodule to see the implementation details of a specific service.

Dating application backend based on the gRPC microservices ecosystem and Clean Architecture principles

____

[Try plugin for Intellij, make it easy to generate Protobuf code](https://plugins.jetbrains.com/plugin/21792-protobuf-blueprint)

[![Protobuf Blueprint](https://github.com/numq/protobuf-blueprint-plugin/blob/master/media/logo.png)](https://plugins.jetbrains.com/plugin/21792-protobuf-blueprint)

![Overview](./media/dddating-backend-overview.png)

### Features:

- *JWT* based authentication
- Secure account and public profile
- Filter based recommendations
- Like/dislike/backtrack with expiration
- Reactive conversation creation functionality for matches
- Profile report system
- Tickets based support functionality

### Architectural design:

- *Clean Architecture*
- *Domain-driven design (DDD)*
- *Rust* workspace
- *gRPC* based microservices
- *gRPC* based gateway

### Technologies:

- *Rust* language
- *tokio* async runtime
- *tonic* gRPC implementation
- *prost* Protocol Buffers implementation
- *RabbitMQ (lapin)* message brocker
- *MongoDB* NoSQL database
- *Redis* in-memory cache

### Microservices:

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