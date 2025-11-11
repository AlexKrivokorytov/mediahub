import React, { useEffect, useState } from 'react';
import { tauriService } from '../services/tauri';

export default function SettingsPage() {
  const [spotifyClientId, setSpotifyClientId] = useState('');
  const [spotifyClientSecret, setSpotifyClientSecret] = useState('');
  const [soundcloudClientId, setSoundcloudClientId] = useState('');
  const [shikimoriToken, setShikimoriToken] = useState('');
  const [discordBotToken, setDiscordBotToken] = useState('');
  const [status, setStatus] = useState('');

  useEffect(() => {
    setSpotifyClientId(localStorage.getItem('spotify.client_id') || '');
    setSpotifyClientSecret(localStorage.getItem('spotify.client_secret') || '');
    setSoundcloudClientId(localStorage.getItem('soundcloud.client_id') || '');
    setShikimoriToken(localStorage.getItem('shikimori.token') || '');
    setDiscordBotToken(localStorage.getItem('discord.bot_token') || '');
  }, []);

  const saveSpotify = async () => {
    try {
      await tauriService.spotifyAuthenticate({
        client_id: spotifyClientId,
        client_secret: spotifyClientSecret,
        redirect_uri: 'tauri://localhost/callback',
      });
      localStorage.setItem('spotify.client_id', spotifyClientId);
      localStorage.setItem('spotify.client_secret', spotifyClientSecret);
      setStatus('Spotify saved');
    } catch (e: any) {
      setStatus(`Spotify error: ${e}`);
    }
  };

  const saveSoundcloud = async () => {
    try {
      await tauriService.soundcloudAuthenticate({
        client_id: soundcloudClientId,
        redirect_uri: 'tauri://localhost/callback',
      });
      localStorage.setItem('soundcloud.client_id', soundcloudClientId);
      setStatus('SoundCloud saved');
    } catch (e: any) {
      setStatus(`SoundCloud error: ${e}`);
    }
  };

  const saveShikimori = async () => {
    try {
      await tauriService.shikimoriAuthenticate({ access_token: shikimoriToken });
      localStorage.setItem('shikimori.token', shikimoriToken);
      setStatus('Shikimori saved');
    } catch (e: any) {
      setStatus(`Shikimori error: ${e}`);
    }
  };

  const saveDiscord = async () => {
    try {
      await tauriService.discordAuthenticate({ bot_token: discordBotToken });
      localStorage.setItem('discord.bot_token', discordBotToken);
      setStatus('Discord saved');
    } catch (e: any) {
      setStatus(`Discord error: ${e}`);
    }
  };

  const field = (label: string, value: string, set: (v: string) => void, type: string = 'text') => (
    <label style={{ display: 'block' }}>
      <div className="muted" style={{ fontSize: 12, marginBottom: 6 }}>{label}</div>
      <input className="input" type={type} value={value} onChange={(e) => set(e.target.value)} />
    </label>
  );

  return (
    <div>
      <h2 className="text-2xl font-semibold mb-3">Settings</h2>
      <div className="grid grid-cols-12 gap-4">
        <div className="col-span-12 md:col-span-6">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">Spotify</h3>
            <div className="grid gap-3">
              {field('Client ID', spotifyClientId, setSpotifyClientId)}
              {field('Client Secret', spotifyClientSecret, setSpotifyClientSecret, 'password')}
              <div><button className="btn btn-primary" onClick={saveSpotify}>Save Spotify</button></div>
            </div>
          </div>
        </div>

        <div className="col-span-12 md:col-span-6">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">SoundCloud</h3>
            <div className="grid gap-3">
              {field('Client ID', soundcloudClientId, setSoundcloudClientId)}
              <div><button className="btn btn-primary" onClick={saveSoundcloud}>Save SoundCloud</button></div>
            </div>
          </div>
        </div>

        <div className="col-span-12 md:col-span-6">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">Shikimori</h3>
            <div className="grid gap-3">
              {field('Access Token', shikimoriToken, setShikimoriToken, 'password')}
              <div><button className="btn btn-primary" onClick={saveShikimori}>Save Shikimori</button></div>
            </div>
          </div>
        </div>

        <div className="col-span-12 md:col-span-6">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">Discord</h3>
            <div className="grid gap-3">
              {field('Bot Token', discordBotToken, setDiscordBotToken, 'password')}
              <div><button className="btn btn-primary" onClick={saveDiscord}>Save Discord</button></div>
            </div>
          </div>
        </div>
      </div>

      {status && <p className="muted mt-4">{status}</p>}
    </div>
  );
}
