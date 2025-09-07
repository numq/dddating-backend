<h1 align="center">DDDating backend</h1>

<br>

<div align="center" style="display: grid; justify-content: center;">

|                                                                  🌟                                                                   |                  Support this project                   |               
|:-------------------------------------------------------------------------------------------------------------------------------------:|:-------------------------------------------------------:|
|  <img src="https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/32/bitcoin.png" alt="Bitcoin (BTC)" width="32"/>  | <code>bc1qs6qq0fkqqhp4whwq8u8zc5egprakvqxewr5pmx</code> | 
| <img src="https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/32/ethereum.png" alt="Ethereum (ETH)" width="32"/> | <code>0x3147bEE3179Df0f6a0852044BFe3C59086072e12</code> |
|  <img src="https://raw.githubusercontent.com/ErikThiart/cryptocurrency-icons/master/32/tether.png" alt="USDT (TRC-20)" width="32"/>   |     <code>TKznmR65yhPt5qmYCML4tNSWFeeUkgYSEV</code>     |

</div>

<br>

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
