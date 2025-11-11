import React, { useEffect, useState } from 'react';
import { tauriService } from '../services/tauri';

export default function SettingsPage() {
  const [spotifyClientId, setSpotifyClientId] = useState('');
  const [spotifyClientSecret, setSpotifyClientSecret] = useState('');
  const [spotifyAuthUrl, setSpotifyAuthUrl] = useState<string>('');
  const [spotifyAuthCode, setSpotifyAuthCode] = useState<string>('');
  const [soundcloudClientId, setSoundcloudClientId] = useState('');
  const [shikimoriToken, setShikimoriToken] = useState('');
  const [discordBotToken, setDiscordBotToken] = useState('');
  const [status, setStatus] = useState('');
  const [cfg, setCfg] = useState<{ spotify_client: boolean; soundcloud: boolean; shikimori: boolean; discord: boolean } | null>(null);

  useEffect(() => {
    tauriService.getConfigStatus().then(setCfg).catch(() => setCfg(null));
    setSpotifyClientId(localStorage.getItem('spotify.client_id') || '');
    setSpotifyClientSecret(localStorage.getItem('spotify.client_secret') || '');
    setSoundcloudClientId(localStorage.getItem('soundcloud.client_id') || '');
    setShikimoriToken(localStorage.getItem('shikimori.token') || '');
    setDiscordBotToken(localStorage.getItem('discord.bot_token') || '');
  }, []);

  const saveSpotify = async () => {
    try {
      let url: string;
      if (cfg?.spotify_client) {
        url = await tauriService.spotifyBeginAuth('http://127.0.0.1:3000');
      } else {
        url = await tauriService.spotifyAuthenticate({
          client_id: spotifyClientId,
          client_secret: spotifyClientSecret,
          redirect_uri: 'http://127.0.0.1:3000',
        });
        localStorage.setItem('spotify.client_id', spotifyClientId);
        localStorage.setItem('spotify.client_secret', spotifyClientSecret);
      }
      setSpotifyAuthUrl(url);
      setStatus('Spotify credentials saved. Open login URL to continue.');
    } catch (e: any) {
      setStatus(`Spotify error: ${e}`);
    }
  };

  const completeSpotify = async () => {
    try {
      const res = await tauriService.spotifyCompleteAuth(spotifyAuthCode.trim());
      setStatus(res);
      setSpotifyAuthCode('');
    } catch (e: any) {
      setStatus(`Spotify complete error: ${e}`);
    }
  };

  const saveSoundcloud = async () => {
    try {
      await tauriService.soundcloudAuthenticate({
        client_id: soundcloudClientId,
        redirect_uri: 'http://127.0.0.1:3000',
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

  const field = (label: string, value: string, set: (v: string) => void, type: string = 'text', disabled = false) => (
    <label style={{ display: 'block' }}>
      <div className="muted" style={{ fontSize: 12, marginBottom: 6 }}>{label}</div>
      <input className="input" type={type} value={value} onChange={(e) => set(e.target.value)} disabled={disabled} />
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
              {cfg?.spotify_client ? (
                <>
                  {field('Client ID', 'Configured on backend', setSpotifyClientId, 'text', true)}
                  {field('Client Secret', 'Configured on backend', setSpotifyClientSecret, 'text', true)}
                </>
              ) : (
                <>
                  {field('Client ID', spotifyClientId, setSpotifyClientId)}
                  {field('Client Secret', spotifyClientSecret, setSpotifyClientSecret, 'password')}
                </>
              )}
              <div className="flex gap-2">
                <button className="btn btn-primary" onClick={saveSpotify}>Save + Get Login URL</button>
                {spotifyAuthUrl && (
                  <button
                    className="btn"
                    onClick={() => { window.location.href = spotifyAuthUrl; }}
                    title="Opens inside the app to auto-complete auth"
                  >
                    Open Spotify Login
                  </button>
                )}
              </div>
              {spotifyAuthUrl && (
                <div className="grid gap-2">
                  <label className="muted text-xs">Paste "code" from redirect URI</label>
                  <input className="input" placeholder="Authorization code" value={spotifyAuthCode} onChange={(e) => setSpotifyAuthCode(e.target.value)} />
                  <div><button className="btn btn-primary" onClick={completeSpotify}>Complete Spotify Auth</button></div>
                  <div className="muted text-xs" style={{wordBreak:'break-all'}}>
                    If the button does not open, open this URL manually:
                    <div>
                      <a className="btn" href={spotifyAuthUrl} target="_blank" rel="noreferrer">Open in browser</a>
                    </div>
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>

        <div className="col-span-12 md:col-span-6">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">SoundCloud</h3>
            <div className="grid gap-3">
              {cfg?.soundcloud ? field('Client ID', 'Configured on backend', setSoundcloudClientId, 'text', true) : field('Client ID', soundcloudClientId, setSoundcloudClientId)}
              <div><button className="btn btn-primary" onClick={saveSoundcloud}>Save SoundCloud</button></div>
            </div>
          </div>
        </div>

        <div className="col-span-12 md:col-span-6">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">Shikimori</h3>
            <div className="grid gap-3">
              {cfg?.shikimori ? field('Access Token', 'Configured on backend', setShikimoriToken, 'text', true) : field('Access Token', shikimoriToken, setShikimoriToken, 'password')}
              <div><button className="btn btn-primary" onClick={saveShikimori}>Save Shikimori</button></div>
            </div>
          </div>
        </div>

        <div className="col-span-12 md:col-span-6">
          <div className="card">
            <h3 className="text-lg font-semibold mb-2">Discord</h3>
            <div className="grid gap-3">
              {cfg?.discord ? field('Bot Token', 'Configured on backend', setDiscordBotToken, 'text', true) : field('Bot Token', discordBotToken, setDiscordBotToken, 'password')}
              <div><button className="btn btn-primary" onClick={saveDiscord}>Save Discord</button></div>
            </div>
          </div>
        </div>
      </div>

      {status && <p className="muted mt-4">{status}</p>}
    </div>
  );
}
