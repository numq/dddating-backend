<h1 align="center">DDDating backend</h1>

> This is an overview of the project. Go to the submodule to see the implementation details of a specific service.

Dating application backend based on the gRPC microservices ecosystem and Clean Architecture principles

____

[Try plugin for IntelliJ, make it easy to generate Protobuf code](https://plugins.jetbrains.com/plugin/21792-protobuf-blueprint)

[![Protobuf Blueprint](https://github.com/numq/protobuf-blueprint-plugin/blob/master/media/logo.png)](https://plugins.jetbrains.com/plugin/21792-protobuf-blueprint)

![Overview](./media/dddating-backend-overview.png)

## Installation

> docker-compose up -d

## Services:

> Details about the API methods are available in the README of each service

- [Gateway](./service/gateway) - Entrypoint via microservices aggregation
- [Authentication](./service/authentication) - Password based authentication functionality
    - [Account](./service/account) - User account management
    - [Token](./service/token) - User authentication session handling
- [Conversation](./service/conversation) - Chat between matches
- [Matchmaking](./service/matchmaking) - Likes, dislikes with expiration and backtrack
- [Profile](./service/profile) - User profile management
- [Recommendation](./service/recommendation) - Match candidates by filters
- [Safety](./service/safety) - Profile report system
- [Support](./service/support) - Ticket based report functionality

## Architecture:

- *Clean Architecture*
- *Domain-driven design (DDD)*
- *Reactive programming*
- *Rust* workspace
- *gRPC* based microservices
- *gRPC* based gateway

## Technologies:

- **Rust** language
- **tokio** async runtime
- **tonic** gRPC implementation
- **prost** Protocol Buffers implementation
- **RabbitMQ (lapin)** message brocker
- **MongoDB** NoSQL database
- **Redis** in-memory cache
- **JWT** based authentication

___

<p align="center">
  <a href="https://numq.github.io/support">
    <img src="https://api.qrserver.com/v1/create-qr-code/?size=112x112&data=https://numq.github.io/support&bgcolor=1a1b26&color=7aa2f7" 
         width="112" 
         height="112" 
         style="border-radius: 4px;" 
         alt="QR code">
  </a>
  <br>
  <a href="https://numq.github.io/support" style="text-decoration: none;">
    <code><font color="#bb9af7">numq.github.io/support</font></code>
  </a>
</p>
