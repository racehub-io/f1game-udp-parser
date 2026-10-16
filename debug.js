const { TelemetryClient, constants } = require('.');

const { DATA } = constants;

const client = new TelemetryClient({port: 30501, forwardAddresses: [{port:30500}]});
client.on(DATA, (packetVersion, packetType, packetData) => console.log(packetData));

client.start();
