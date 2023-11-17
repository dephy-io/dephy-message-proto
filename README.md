Messaging in DePHY
===

### The `dephy-proto` Package
There are packages for developpers to build applications with `protobuf` definitions in this repository.

```bash
cargo add dephy_proto # for Rust
npm install dephy-proto # for Node.js
```

The Rust crate is built with [prost](https://github.com/tokio-rs/prost) and published on [crates.io](https://crates.io/crates/dephy_proto).

The JavaScript/TypeScript package for Node.js and Deno is built with [pbkit](https://github.com/pbkit/pbkit) and published on [NPM](https://www.npmjs.com/package/dephy-proto).


### Direct Message
A `Direct Message` in DePHY is a message for DePHY devices and Edge Brokers to communicate directly in `Protocol Buffer` encoding.

It should be transferred in binary when interacting between the device and Edge Brokers’ service endpoints(MQTT or HTTP).

Usually, a DePHY device send Direct Messages to:
- `mqtt://path.to.dephy.edge:1883`
    - topic: `/dephy/signed_message`
    - the message binary should be the payload
- `http://path.to.dephy.edge:3883/dephy/signed_message`
    - method: `POST`
    - `content-type: application/x-dephy`
    - the message binary should be the HTTP body

# Routed Message
A `Routed Message` in DePHY is a [NoStr event](https://github.com/nostr-protocol/nips/blob/master/01.md) wrapping a `Direct Message` which will be redistributed by Edge Brokers in a `NoStr` decentralized network:
- The `kind` of the event should be `1111`.
- The `content` should be a Direct Message encoded in Base58 (a.k.a `base58(bytes(SignedMessage))`).
- It should contain these tags:
    - `c` is the marker of a Routed Message and should be always `dephy`
    - `dephy_from` is the DID string of the message sender
    - `dephy_to` is the DID string of the message recipent
    - `dephy_edge` is the DID string of the message forwarder

And it looks like this:
```json
{
  "id": "...",
  "pubkey": "...",
  "created_at": 0,
  "kind": 1111, // Magic!
  "content": "...", // base58(bytes(SignedMessage))
  "tags": [
    ["c", "dephy"],
    ["dephy_to", "did:dephy:0x..."], // did(direct_message.payload.to)
    ["dephy_from", "did:dephy:0x..."], // did(direct_message.payload.from)
    ["dephy_edge", "did:dephy:0x..."], // did(origin_edge)
  ],
  "sig": "..."
}
```

You don’t need to care about Routed Messages if you just build applications interacting with only devices and Edge Brokers.