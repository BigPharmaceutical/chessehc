# Chessehc Protocol

## **WIP**

**THIS IS A WORK-IN-PROGRESS!**  
This specification is not complete!  
Things will change!

## Notation

This is about notation within this documentation.

Indexing starts from zero.

Ranges are inclusive.

Strings are utf-8 encoded.

Numbers are big-endian.

Trees are represented by lists detailing the ID of each part and the bit(s) to identify its branches in brackets.  
For example: `1 - account (2-3)` means that ID 1 is related to accounts and its branches are identified by bits 2-3 (numbers 0-3).

## [Requests](./request.md)

## [Responses](./response.md)

## Underlying Protocol

This protocol runs on top of the WebSocket protocol.

## [Authentication](./authentication.md)
