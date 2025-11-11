import { invoke } from "@tauri-apps/api/core";


export const tauriService = {
  async authenticateSpotify(client_id: string, client_secret: string) {
    return invoke('spotify_authenticate', { request: { client_id, client_secret, redirect_uri: '' } });
  },

  async playTrack(track_url: string) {
    return invoke('play_track', { track_url });
  }
};
