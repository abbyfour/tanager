import { SubsonicAPI } from "npm:subsonic-api";
import * as config from "../config.ts";

export class Subsonic {
  private API: SubsonicAPI;

  constructor() {
    this.API = new SubsonicAPI({
      url: config.url,
      auth: {
        username: config.username,
        password: config.password,
      },
    });
  }

  public async getNowPlaying() {
    return await this.API.getNowPlaying();
  }

  public async getQueue() {
    return await this.API.getPlayQueue();
  }
}
