import * as dgram from 'dgram';
import * as constants from './constants';

import { EventEmitter } from 'events';

import { AddressInfo } from 'net';
import { Address, Options } from './types';

const parser = require('../native/index.node');

const DEFAULT_PORT = 20777;
const FORWARD_ADDRESSES = undefined;
const ADDRESS = 'localhost';

const { DATA } = constants;

class TelemetryClient extends EventEmitter {
  port: number;
  forwardAddresses?: Address[];
  address: string;
  socket?: dgram.Socket;

  constructor(opts: Options = {}) {
    super();

    const {
      port = DEFAULT_PORT,
      forwardAddresses = FORWARD_ADDRESSES,
      address = ADDRESS,
    } = opts;

    this.port = port;
    this.forwardAddresses = forwardAddresses;
    this.address = address;
    this.socket = dgram.createSocket('udp4');
  }

  /**
   *
   * @param {Number} packetFormat
   * @param {Number} packetId
   */
  static getPacketSize(packetFormat: number, packetId: number) {
    const {PACKET_SIZES} = constants;
    const packetValues = Object.values(PACKET_SIZES);
    return packetValues[packetId][packetFormat];
  }

  /**
   *
   * @param {Buffer} message
   */
  static parseBufferMessage(message: Buffer):
      any {

    const packet = parser.parseMessage(message);

    const {packetVersion, packetType, packetData: fullPacket} = packet;

    // emit parsed message
    const packetData = fullPacket[packetVersion][packetType];
    return {packetVersion, packetType, packetData};
  }

  /**
   *
   * @param {Buffer} message
   */
  handleMessage(message: Buffer) {
    if (this.forwardAddresses) {
      // bridge message
      this.bridgeMessage(message);
    }

    const parsedMessage = TelemetryClient.parseBufferMessage(message);

    if (!parsedMessage || !parsedMessage.packetData) {
      return;
    }

    // emit parsed message
    this.emit(DATA, parsedMessage.packetVersion, parsedMessage.packetType, parsedMessage.packetData);
  }

  /**
   *
   * @param {Buffer} message
   */
  bridgeMessage(message: Buffer) {
    if (!this.socket) {
      throw new Error('Socket is not initialized');
    }
    if (!this.forwardAddresses) {
      throw new Error('No ports to bridge over');
    }
    for (const address of this.forwardAddresses) {
      this.socket.send(
          message, 0, message.length, address.port, address.ip || '0.0.0.0');
    }
  }

  /**
   * Method to start listening for packets
   */
  start() {
    if (!this.socket) {
      return;
    }

    this.socket.on('listening', () => {
      if (!this.socket) {
        return;
      }

      const address = this.socket.address() as AddressInfo;
      console.log(
          `UDP Client listening on ${address.address}:${address.port} 🏎️`);
      this.socket.setBroadcast(true);
    });

    this.socket.on('message', (m) => this.handleMessage(m));
    this.socket.bind({
      port: this.port,
      exclusive: false,
    });
  }

  /**
   * Method to close the client
   */
  stop() {
    if (!this.socket) {
      return;
    }

    return this.socket.close(() => {
      console.log(`UDP Client closed 🏁`);
      this.socket = undefined;
    });
  }
}

export { TelemetryClient, constants };
