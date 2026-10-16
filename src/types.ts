export interface Options {
  port?: number;
  forwardAddresses?: Address[]|undefined;
  address?: string;
}
export interface Address {
  port: number;
  ip?: string;
}
