# type-safe-api

i was thinking of making some kind of api spec for type safe api's?

so basically we have a schema file that describes all the endpoints, their input and return types, when making a req we make a checksum of that schema and send it like a preflight, then the client checks their version and if they fail the req fails

then we also have a compiler that compiles your data from whatever language and then sends it over the network as bytes or some low storage format, then the client 'un-compiles' it this way we are sure to have type safe api's thoughts?

i think i could include a schema diff in the schema file, so either major, minor, patch or none, which describes the difference level the client is willing to accept?
