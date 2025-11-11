import { invoke } from '@tauri-apps/api/core';

// Types
export type SpotifyAuthRequest = {
  client_id: string;
  client_secret: string;
  redirect_uri: string;
};

export type SoundcloudAuthRequest = {
  client_id: string;
  redirect_uri: string;
};

export type ShikimoriAuthRequest = {
  access_token: string;
};

export type DiscordAuthRequest = {
  bot_token: string;
};

export type DiscordMessageRequest = {
  channel_id: string;
  content: string;
};

export type AnimeEntry = {
  id: number;
  title: string;
  episodes_watched: number;
  total_episodes: number;
  status: string;
  rating?: number | null;
};

async function call<T>(cmd: string, payload?: any): Promise<T> {
  return invoke<T>(cmd, payload ?? {});
}

export const tauriService = {
  // Config
  getConfigStatus: () => call<{ spotify_client: boolean; soundcloud: boolean; shikimori: boolean; discord: boolean }>('get_config_status'),

  // Spotify
  spotifyAuthenticate: (req: SpotifyAuthRequest) => call<string>('spotify_authenticate', { request: req }),
  spotifyBeginAuth: (redirect_uri?: string) => call<string>('spotify_begin_auth', { redirect_uri }),
  spotifyCompleteAuth: (code: string) => call<string>('spotify_complete_auth', { code }),
  spotifyGetPlaylists: () => call<string[]>('spotify_get_playlists'),
  spotifyGetTracks: () => call<string[]>('spotify_get_tracks'),

  // SoundCloud
  soundcloudAuthenticate: (req: SoundcloudAuthRequest) => call<string>('soundcloud_authenticate', { request: req }),
  soundcloudGetTracks: () => call<any[]>('soundcloud_get_tracks'),

  // Shikimori
  shikimoriAuthenticate: (req: ShikimoriAuthRequest) => call<string>('shikimori_authenticate', { request: req }),
  shikimoriGetAnimeList: () => call<AnimeEntry[]>('shikimori_get_anime_list'),

  // Discord
  discordAuthenticate: (req: DiscordAuthRequest) => call<string>('discord_authenticate', { request: req }),
  discordSendMessage: (req: DiscordMessageRequest) => call<string>('discord_send_message', { request: req }),

  // Player
  playTrack: (track_url: string) => call<string>('play_track', { track_url }),
  pauseTrack: () => call<string>('pause_track'),
  setVolume: (volume: number) => call<string>('set_volume', { volume }),
};
