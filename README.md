# F1 Game UDP Parser
![npm (scoped)](https://img.shields.io/npm/v/@racehub-io/f1game-udp-parser?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/racehub-io/f1game-udp-parser?style=for-the-badge)

## Installing
### Important!
**This is a native node module written in [Rust](https://www.rust-lang.org/install.html). To allow for cross compatibility it builds from source on installation.**
You will require Rust installed, before installing this module.
As soon as [Neon](https://neon-bindings.com/) is stable enough to compile through node-gyp for compiled releases we will migrate.


```
$ npm install @racehub-io/f1game-udp-parser
```

or

```
$ yarn add @racehub-io/f1game-udp-parser
```

## Usage

```
import { TelemetryClient, constants } from "@racehub-io/f1game-udp-parser";
// or: const { TelemetryClient, constants } = require('f1game-udp-parser');
const { DATA } = constants;

const client = new TelemetryClient({port: 20777, forwardAddresses: [{port:30500}]});
client.on(DATA, (packetVersion, packetType, packetData) => console.log(packetData));

client.start();

// and when you want to stop:
client.stop();
```

## ToDo
- [ ] Migrate publishing to node-gyp for compiled builds
- [ ] Move UDP socket and EventEmitter to Rust
- [ ] Improve packet dependent logic inside Rust parser
- [ ] Migrate to N-API backend with Neon
- [ ] Update Rust dependencies

## Community use

RaceHub source contributions are available under the scope in [LICENSING.md](LICENSING.md).
Local features do not require a RaceHub subscription.
Third-party games, SDKs, and optional hosting keep their own terms and costs.
See [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md).
No hosted service or maintenance schedule is included.
