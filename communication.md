## This document describes the communication protocol used by FooBar daemon to communicate with clients

The daemon is a central "hub" that sends messages thorugh Unix (Domain) Sockets (of type stream). These sockets are bidirectional (aka full duplex) stream based communication. It has no frames / packets so that must be handled by us. 

A message is made up of 2 parts: header and body

### Header
| Size | Type | Reason |
| ----- | ----- | ---- |
| 32 bit | unsigned integer | Body size (excluding the rest of the header) |
| 8 bit  | unsigned integer | Opcode describing the general purpose of the message |

The header should be read in 1 (atomic) operation. 
If the body size is greater than 1MiB and the opcode is something that's not marked as large (see at opcodes), the connection should be immediately dropped and the incident logged.

### Body

The body should be as the opcode describes it. 

### Opcodes

| Code | Parameters | Description |
| :--- | :--------- | ----------: |
